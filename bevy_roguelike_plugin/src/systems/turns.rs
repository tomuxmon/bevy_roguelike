use crate::resources::TeamMap;
use crate::{components::*, events::*};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use bevy::utils::HashMap;
use map_generator::*;
use rand::prelude::*;

pub fn act(
    actors: Query<(&Team, &Damage, &Attributes, &Vector2D)>,
    enemies: Query<(Entity, &Team, &Vector2D)>,
    // TODO: ActComponent instead of ActEvent
    mut act_reader: EventReader<ActEvent>,
    mut attack_writer: EventWriter<AttackEvent>,
    mut ap_spend_writer: EventWriter<SpendAPEvent>,
    mut move_writer: EventWriter<MoveEvent>,
    mut idle_writer: EventWriter<IdleEvent>,
    map: Res<Map>,
    mut team_map: ResMut<TeamMap>,
) {
    let delta_costs = HashMap::from_iter(vec![
        (IVec2::new(0, 1), ActionPoints::DELTA_COST_MOVE_DEFAULT),
        (IVec2::new(0, -1), ActionPoints::DELTA_COST_MOVE_DEFAULT),
        (IVec2::new(-1, 0), ActionPoints::DELTA_COST_MOVE_DEFAULT),
        (IVec2::new(1, 0), ActionPoints::DELTA_COST_MOVE_DEFAULT),
        (IVec2::new(0, 0), ActionPoints::IDLE_COST_DEFAULT), // stay put - skip turn
    ]);
    for e in act_reader.iter() {
        if !delta_costs.contains_key(&e.delta) {
            log::error!("delta with no cost! {:?}", e.delta);
            continue;
        }
        if e.delta == IVec2::new(0, 0) {
            ap_spend_writer.send(SpendAPEvent::new(e.id, delta_costs[&e.delta]));
            idle_writer.send(IdleEvent::new(e.id));
            continue;
        }
        if let Ok((team, atk, atrib, pt)) = actors.get(e.id) {
            let dest = **pt + e.delta;
            if !map.is_in_bounds(dest) || map[dest] != Tile::Floor {
                ap_spend_writer.send(SpendAPEvent::new(e.id, delta_costs[&IVec2::new(0, 0)]));
                idle_writer.send(IdleEvent::new(e.id));
                continue;
            }
            if let Some(other_team) = team_map[dest] {
                if other_team == *team {
                    // NOTE: can not move into a tile ocupied by a team mate
                    ap_spend_writer.send(SpendAPEvent::new(e.id, delta_costs[&IVec2::new(0, 0)]));
                    idle_writer.send(IdleEvent::new(e.id));
                    continue;
                } else {
                    if let Some((enemy_entity, _, _)) =
                        enemies.iter().find(|(_, t, p)| *t != team && ***p == dest)
                    {
                        ap_spend_writer.send(SpendAPEvent::new(e.id, atk.hit_cost.compute(atrib)));
                        attack_writer.send(AttackEvent::new(e.id, enemy_entity))
                    } else {
                        log::error!("nothing to attack at {:?} (TeamMap has bugs).", dest);
                    }
                }
            } else {
                team_map[dest] = Some(*team);
                ap_spend_writer.send(SpendAPEvent::new(e.id, delta_costs[&e.delta]));
                move_writer.send(MoveEvent::new(e.id, dest));
            }
        }
    }
}

pub fn attack(
    attackers: Query<(&Damage, &Vector2D, Option<&Equipment>)>,
    defenders: Query<(&Protection, &Vector2D, Option<&Equipment>)>,
    mut cmd: Commands,
    mut attack_reader: EventReader<AttackEvent>,
    mut ap_spend_writer: EventWriter<SpendAPEvent>,
    mut rng: ResMut<StdRng>,
) {
    for e in attack_reader.iter() {
        if let Ok((atack, _apt, a_eqv)) = attackers.get(e.attacker) {
            if let Ok((defense, dpt, d_eqv)) = defenders.get(e.defender) {
                // let mut a_boosts = vec![];
                // if let Some(inv) = a_inventory {
                //     for i in inv.iter_some() {
                //         if let Ok(ab) = attack_boosts.get(i) {
                //             a_boosts.push(*ab);
                //         }
                //     }
                // }
                // let mut d_boosts = vec![];
                // if let Some(inv) = d_inventory {
                //     for i in inv.iter_some() {
                //         if let Ok(ab) = defense_boosts.get(i) {
                //             d_boosts.push(*ab);
                //         }
                //     }
                // }
                // let defense = *defense + d_boosts.iter().sum();
                // let atack = *atack + a_boosts.iter().sum();

                // TODO: fix me

                // if !rng.gen_ratio(defense.rate().min(atack.rate()) as u32, atack.rate() as u32) {
                //     // TODO: spawn attack animation
                //     cmd.spawn().insert(ModifyHP::new(
                //         **dpt,
                //         -i16::max(atack.damage() - defense.absorb(), 0),
                //     ));
                // } else {
                //     // TODO: spawn miss animation
                //     ap_spend_writer.send(SpendAPEvent::new(e.defender, defense.cost()));
                //     log::info!("attack miss at {}", dpt);
                // }
            } else {
                log::error!("no defender found.");
            }
        } else {
            log::error!("no attacker found.");
        }
    }
}

pub fn apply_hp_modify(
    mut cmd: Commands,
    mut actors: Query<(Entity, &Vector2D, &mut HitPoints)>,
    hp_mod: Query<(Entity, &ModifyHP)>,
    mut team_map: ResMut<TeamMap>,
) {
    for (e, hpm) in hp_mod.iter() {
        if let Some((ee, pt, mut hp)) = actors.iter_mut().find(|(_, p, _)| ***p == hpm.location) {
            hp.apply(hpm.amount);
            if hp.current() <= 0 {
                // TODO: animated death
                cmd.entity(ee).despawn_recursive();
                team_map[**pt] = None;
            }
        }
        cmd.entity(e).despawn_recursive();
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

pub fn idle_rest(mut actors: Query<&mut HitPoints>, mut idle_reader: EventReader<IdleEvent>) {
    for e in idle_reader.iter() {
        if let Ok(mut hp) = actors.get_mut(e.id) {
            hp.regen();
        }
    }
}

pub fn do_move(
    mut actors: Query<(&mut Vector2D, &mut FieldOfView)>,
    mut move_reader: EventReader<MoveEvent>,
    mut team_map: ResMut<TeamMap>,
) {
    for e in move_reader.iter() {
        if let Ok((mut pt, mut fov)) = actors.get_mut(e.id) {
            let pt_old = **pt;
            *pt = Vector2D::from(e.destination);
            team_map[pt_old] = None;
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
