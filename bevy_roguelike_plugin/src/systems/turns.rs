use crate::{components::*, events::*};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use bevy::utils::HashMap;
use bevy_inventory::Equipment;
use bevy_inventory::Inventory;
use bevy_inventory::ItemType;
use bevy_inventory_ui::InventoryDisplayOwner;
use bevy_roguelike_combat::*;
use map_generator::*;

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
            idle_writer.send(IdleEvent { id: e.id });
            continue;
        }
        if let Ok((team, pt)) = actors.get(e.id) {
            let dest = **pt + e.delta;
            if !map.is_in_bounds(dest) || map[dest] != Tile::Floor {
                idle_writer.send(IdleEvent { id: e.id });
                continue;
            }
            if let Some(other_team) = team_pt.get(&dest) {
                if *other_team == *team {
                    // NOTE: can not move into a tile ocupied by a team mate
                    idle_writer.send(IdleEvent { id: e.id });
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
                move_writer.send(MoveEvent {
                    actor: e.id,
                    team: *team,
                    from: **pt,
                    to: dest,
                });
            }
        }
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

// TODO: move to bevy_roguelike_turns
pub fn spend_ap(
    mut actors: Query<(&mut ActionPoints, &mut TurnState, &mut HitPoints)>,
    mut ap_reader: EventReader<SpendAPEvent>,
) {
    for e in ap_reader.iter() {
        if let Ok((mut ap, mut ts, mut hp)) = actors.get_mut(e.id) {
            if ap.current_minus(e.amount) < ap.turn_ready_to_act() {
                *ts = TurnState::End;
                hp.regen();
            }
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

// TODO: move to bevy_roguelike_turns
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
// TODO: move to bevy_roguelike_turns
pub fn turn_end_now_gather(pool: Res<ComputeTaskPool>, mut actors: Query<&mut TurnState>) {
    if actors.iter().all(|ts| *ts == TurnState::End) {
        actors.par_for_each_mut(&*pool, 16, |mut ts| {
            *ts = TurnState::Collect;
        });
    }
}
