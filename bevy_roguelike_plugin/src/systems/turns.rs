use crate::resources::{Map, MapOptions, Tile};
use crate::{components::*, events::*};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::utils::HashMap;

pub fn act(
    mut actors: Query<(Entity, &Team, &Capability, &Vector2D)>,
    mut act_reader: EventReader<ActEvent>,
    mut hp_writer: EventWriter<ModifyHPEvent>,
    mut ap_writer: EventWriter<SpendAPEvent>,
    mut move_writer: EventWriter<MoveEvent>,
    map: Res<Map>,
) {
    let delta_costs = HashMap::from_iter(vec![
        (IVec2::new(0, 1), 900),
        (IVec2::new(0, -1), 900),
        (IVec2::new(-1, 0), 900),
        (IVec2::new(1, 0), 900),
        (IVec2::new(0, 0), 451), // stay put - skip turn
    ]);

    let ocupied = HashMap::from_iter(actors.iter().map(|(e, t, _, p)| (**p, (e, *t))));

    for e in act_reader.iter() {
        if let Ok((_, team, cp, pt)) = actors.get_mut(e.id) {
            // let delta = Vector2D::from(e.delta);
            // if delta != Vector2D::minmin() {}
            let mut cost = delta_costs[&e.delta];
            let dest = **pt + e.delta;
            if !map.is_in_bounds(dest) || map[dest] != Tile::Floor {
                continue;
            }
            let other = ocupied.get(&dest);
            // NOTE: can not move into a tile ocupied by a team mate
            if other.is_some() && other.unwrap().1 == *team && e.delta != IVec2::new(0, 0) {
                continue;
            }
            // TODO: instead of 'delta != ..' check on is_same_id
            if other.is_some() && e.delta != IVec2::new(0, 0) {
                cost = cp.attack_cost();
                hp_writer.send(ModifyHPEvent::new(other.unwrap().0, -cp.attack_damage()));
            } else {
                if e.delta != IVec2::new(0, 0) {
                    move_writer.send(MoveEvent::new(e.id, dest));
                }
            }
            ap_writer.send(SpendAPEvent::new(e.id, cost));
        }
    }
}

pub fn apply_hp_modify(
    mut cmd: Commands,
    mut actors: Query<&mut Capability>,
    mut hp_reader: EventReader<ModifyHPEvent>,
) {
    for e in hp_reader.iter() {
        if let Ok(mut cp) = actors.get_mut(e.id) {
            cp.hp_apply(e.amount);
            if cp.hp_current() <= 0 {
                // TODO: animated death
                log::info!("death to {:?}!", e.id);
                cmd.entity(e.id).despawn_recursive();
            }
        }
    }
}

pub fn spend_ap(
    mut actors: Query<(&mut Capability, &mut TurnState)>,
    mut ap_reader: EventReader<SpendAPEvent>,
) {
    for e in ap_reader.iter() {
        if let Ok((mut cp, mut ts)) = actors.get_mut(e.id) {
            if cp.ap_current_minus(e.amount) < cp.ap_turn_ready_to_act() {
                *ts = TurnState::End;
            }
        }
    }
}

pub fn do_move(
    mut actors: Query<(&mut Vector2D, &mut Transform, &mut FieldOfView)>,
    mut move_reader: EventReader<MoveEvent>,
    map_options: Res<MapOptions>,
) {
    for e in move_reader.iter() {
        if let Ok((mut pt, mut tr, mut fov)) = actors.get_mut(e.id) {
            let z = tr.translation.z;
            tr.translation = map_options.to_world_position(e.destination).extend(z);
            *pt = Vector2D::from(e.destination);
            fov.is_dirty = true;
        }
    }
}

pub fn gather_action_points(
    pool: Res<AsyncComputeTaskPool>,
    mut actors: Query<(&mut Capability, &mut TurnState)>,
) {
    actors.par_for_each_mut(&*pool, 16, |(mut cp, mut ts)| {
        if *ts == TurnState::Collect {
            *ts = if cp.ap_current_add() > cp.ap_turn_ready_to_act() {
                TurnState::Act
            } else {
                // NOTE: not yet ready to perform turn.
                // skip this turn.
                TurnState::End
            };
        }
    });
}
pub fn turn_end_now_gather(pool: Res<AsyncComputeTaskPool>, mut actors: Query<&mut TurnState>) {
    if actors.iter().all(|ts| *ts == TurnState::End) {
        actors.par_for_each_mut(&*pool, 16, |mut ts| {
            *ts = TurnState::Collect;
        });
    }
}
