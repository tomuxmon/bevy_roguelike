use crate::{components::*, events::*};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use bevy::utils::HashMap;
use bevy_inventory::Equipment;
use bevy_inventory::Inventory;
use bevy_inventory::ItemType;
use bevy_inventory_ui::InventoryDisplayOwner;
use map_generator::*;
use rand::prelude::*;

pub fn act(
    actors: Query<(&Team, &Vector2D)>,
    enemies: Query<(Entity, &Team, &Vector2D)>,
    // TODO: ActComponent instead of ActEvent
    mut act_reader: EventReader<ActEvent>,
    mut attack_writer: EventWriter<AttackEvent>,
    mut move_writer: EventWriter<MoveEvent>,
    mut idle_writer: EventWriter<IdleEvent>,
    map: Res<Map>,
) {
    let team_pt: HashMap<_, _> = actors.iter().map(|(t, p)| (**p, *t)).collect();
    for e in act_reader.iter() {
        if e.delta == IVec2::new(0, 0) {
            idle_writer.send(IdleEvent::new(e.id));
            continue;
        }
        if let Ok((team, pt)) = actors.get(e.id) {
            let dest = **pt + e.delta;
            if !map.is_in_bounds(dest) || map[dest] != Tile::Floor {
                idle_writer.send(IdleEvent::new(e.id));
                continue;
            }
            if let Some(other_team) = team_pt.get(&dest) {
                if *other_team == *team {
                    // NOTE: can not move into a tile ocupied by a team mate
                    idle_writer.send(IdleEvent::new(e.id));
                    continue;
                } else if let Some((enemy_entity, _, _)) =
                    enemies.iter().find(|(_, t, p)| *t != team && ***p == dest)
                {
                    attack_writer.send({
                        AttackEvent {
                            attacker: e.id,
                            defender: enemy_entity,
                        }
                    })
                } else {
                    log::error!("nothing to attack at {:?} (... has bugs).", dest);
                }
            } else {
                let move_event = MoveEvent {
                    actor: e.id,
                    team: *team,
                    from: **pt,
                    to: dest,
                };
                move_writer.send(move_event);
            }
        }
    }
}

pub fn attack(
    attackers: Query<(&Vector2D, &StatsComputed)>,
    defenders: Query<(&Vector2D, &StatsComputed, &ActionPoints)>,
    mut cmd: Commands,
    mut attack_reader: EventReader<AttackEvent>,
    mut ap_spend_writer: EventWriter<SpendAPEvent>,
    mut rng: ResMut<StdRng>,
) {
    for e in attack_reader.iter() {
        let (attacker_pt, attacker_stats) = if let Ok(attacker) = attackers.get(e.attacker) {
            attacker
        } else {
            log::info!(
                "Attacker Not Found (id: {:?}). Probably died recently.",
                e.attacker
            );
            return;
        };
        let (defender_pt, defender_stats, defender_ap) =
            if let Ok(defender) = defenders.get(e.defender) {
                defender
            } else {
                log::info!(
                    "Defender Not Found (id: {:?}). Probably died recently.",
                    e.defender
                );
                return;
            };

        if attacker_stats.damage.is_empty() {
            log::error!("attacker has no damage.");
            return;
        }

        let rng = &mut *rng;

        let damage = if attacker_stats.damage.len() == 1 {
            &attacker_stats.damage[0]
        } else {
            &attacker_stats.damage[rng.gen_range(0..attacker_stats.damage.len())]
        };

        // TODO: spawn attack animation (based on damage.kind)

        // NOTE: attacker should spend AP regardles of outcome
        let attack_cost = damage.hit_cost.compute(&attacker_stats.attributes);
        ap_spend_writer.send(SpendAPEvent::new(e.attacker, attack_cost));
        log::trace!("attacking from {} with cost {}", attacker_pt, attack_cost);

        // NOTE: negative AP is ok as long as we are close to zero (not reaching i16::MIN).
        if defender_ap.current() > 0 {
            let (evaded, evade_cost) = defender_stats.evasion.try_evade(
                damage,
                &defender_stats.attributes,
                &attacker_stats.attributes,
                rng,
            );

            if evaded {
                // TODO: spawn evade animation (MISS on the enemy)
                ap_spend_writer.send(SpendAPEvent::new(e.defender, evade_cost));
                log::trace!("attack evaded {} with cost {}", defender_pt, evade_cost);
                return;
            } else {
                // TODO: roll crit hit (hit rate vs evade rate)
            }

            for block in defender_stats.block.iter() {
                let (blocked, block_cost) = block.try_block(
                    damage,
                    &defender_stats.attributes,
                    &attacker_stats.attributes,
                    rng,
                );
                if blocked {
                    // TODO: spawn block animation
                    ap_spend_writer.send(SpendAPEvent::new(e.defender, block_cost));
                    log::trace!("attack blocked {} with cost {}", defender_pt, block_cost);
                    return;
                }
            }
        }

        let mut true_damage = damage.compute(&attacker_stats.attributes, rng);
        log::trace!(
            "attack damage raw {} (roll from {:?})",
            true_damage,
            damage.amount
        );

        // NOTE: apply protection and only then resistance
        for protect in defender_stats
            .protection
            .amounts
            .iter()
            .filter(|p| p.kind == damage.kind)
        {
            true_damage -= protect.compute(&defender_stats.attributes);
        }
        if true_damage < 1 {
            log::trace!(
                "damage negated with protection. damage after protection {}",
                true_damage
            );
            // TODO: spawn clinc animation
            return;
        }

        let resist = defender_stats
            .resistance
            .amounts
            .iter()
            .filter(|r| r.kind == damage.kind)
            .map(|r| r.percent)
            .sum::<u8>()
            .min(100) as f32
            / 100.;

        true_damage = (true_damage as f32 * (1. - resist)) as i32;

        cmd.spawn().insert({
            ModifyHP {
                location: **defender_pt,
                amount: -true_damage as i16,
            }
        });
        log::trace!("attack damage {}", true_damage);
    }
}

pub fn apply_hp_modify(
    mut cmd: Commands,
    mut actors: Query<(Entity, &Vector2D, &mut HitPoints)>,
    hp_mod: Query<(Entity, &ModifyHP)>,
    mut death_writer: EventWriter<DeathEvent>,
) {
    for (e, hpm) in hp_mod.iter() {
        if let Some((actor_entity, _, mut hp)) =
            actors.iter_mut().find(|(_, p, _)| ***p == hpm.location)
        {
            hp.apply(hpm.amount);
            if hp.current() <= 0 {
                death_writer.send(DeathEvent {
                    actor: actor_entity,
                });
            }
        }
        cmd.entity(e).despawn_recursive();
    }
}

pub fn death_read<I: ItemType>(
    mut cmd: Commands,
    mut death_reader: EventReader<DeathEvent>,
    actors: Query<(&Vector2D, &Name, &HitPoints, &Inventory, &Equipment<I>)>,
    inventory_displays: Query<(Entity, &InventoryDisplayOwner)>,
) {
    for death in death_reader.iter() {
        if let Ok((pt, name, _hp, inventory, equipment)) = actors.get(death.actor) {
            for item_entity in inventory
                .iter_some()
                .chain(equipment.iter_some().map(|(_, item)| item))
            {
                // NOTE: manually droping without itemDropEvent.
                // dirty way but...
                cmd.entity(item_entity).insert(*pt);
            }
            for (ui_node_entity, owner) in inventory_displays.iter() {
                if owner.actor == death.actor {
                    cmd.entity(ui_node_entity).despawn_recursive();
                }
            }
            // TODO: animated death
            // different animation based on negative percent of current hp
            bevy::log::info!("death to {} (id: {:?}) at {}", name, death.actor, pt);
            cmd.entity(death.actor).despawn_recursive();
        } else {
            bevy::log::error!("Death to {:?}. But actor bedead not found.", death.actor);
        }
    }
}

pub fn spend_ap(
    mut actors: Query<(&mut ActionPoints, &mut TurnState)>,
    mut ap_reader: EventReader<SpendAPEvent>,
) {
    for e in ap_reader.iter() {
        if let Ok((mut ap, mut ts)) = actors.get_mut(e.id) {
            if ap.current_minus(e.amount) < ap.turn_ready_to_act() {
                *ts = TurnState::End;
            }
        }
    }
}

pub fn idle_rest(
    mut actors: Query<&mut HitPoints>,
    mut idle_reader: EventReader<IdleEvent>,
    mut ap_spend_writer: EventWriter<SpendAPEvent>,
) {
    for e in idle_reader.iter() {
        ap_spend_writer.send(SpendAPEvent::new(e.id, ActionPoints::IDLE_COST_DEFAULT));
        if let Ok(mut hp) = actors.get_mut(e.id) {
            hp.regen();
        }
    }
}

pub fn try_move(
    mut actors: Query<(&mut Vector2D, &Team, &mut FieldOfView)>,
    mut move_reader: EventReader<MoveEvent>,
    mut ap_spend_writer: EventWriter<SpendAPEvent>,
) {
    let mut team_pt: HashMap<_, _> = actors.iter().map(|(p, t, _)| (**p, *t)).collect();
    for e in move_reader.iter() {
        if let Ok((mut pt, _tt, mut fov)) = actors.get_mut(e.actor) {
            if let Some(_team) = team_pt.get(&e.to) {
                bevy::log::trace!(
                    "trying to move from {} to {} by {:?}. location already ocupied",
                    e.from,
                    e.to,
                    e.actor
                );
                continue;
            }
            ap_spend_writer.send(SpendAPEvent::new(e.actor, ActionPoints::MOVE_COST_DEFAULT));
            team_pt.entry(e.to).insert(e.team);
            *pt = Vector2D::from(e.to);
            fov.is_dirty = true;
        }
    }
}

pub fn gather_action_points(
    pool: Res<ComputeTaskPool>,
    mut actors: Query<(&mut ActionPoints, &mut TurnState)>,
) {
    actors.par_for_each_mut(&*pool, 16, |(mut ap, mut ts)| {
        if *ts == TurnState::Collect {
            *ts = if ap.current_add() > ap.turn_ready_to_act() {
                TurnState::Act
            } else {
                // NOTE: not yet ready to perform turn.
                // skip this turn.
                TurnState::End
            };
        }
    });
}
pub fn turn_end_now_gather(pool: Res<ComputeTaskPool>, mut actors: Query<&mut TurnState>) {
    if actors.iter().all(|ts| *ts == TurnState::End) {
        actors.par_for_each_mut(&*pool, 16, |mut ts| {
            *ts = TurnState::Collect;
        });
    }
}
