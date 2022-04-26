use crate::resources::{Map, MapOptions, TeamMap, Tile};
use crate::{components::*, events::*};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::utils::HashMap;

pub fn act(
    mut cmd: Commands,
    actors: Query<(Entity, &Team, &Capability, &Vector2D)>,
    mut act_reader: EventReader<ActEvent>,
    mut ap_writer: EventWriter<SpendAPEvent>,
    mut move_writer: EventWriter<MoveEvent>,
    map: Res<Map>,
    mut team_map: ResMut<TeamMap>,
) {
    let delta_costs = HashMap::from_iter(vec![
        (IVec2::new(0, 1), 900),
        (IVec2::new(0, -1), 900),
        (IVec2::new(-1, 0), 900),
        (IVec2::new(1, 0), 900),
        (IVec2::new(0, 0), 451), // stay put - skip turn
    ]);
    for e in act_reader.iter() {
        if e.delta == IVec2::new(0, 0) {
            ap_writer.send(SpendAPEvent::new(e.id, delta_costs[&e.delta]));
            continue;
        }
        if let Ok((_, team, cp, pt)) = actors.get(e.id) {
            let dest = **pt + e.delta;
            if !map.is_in_bounds(dest) || map[dest] != Tile::Floor {
                continue;
            }
            if let Some(other_team) = team_map[dest] {
                if other_team == *team {
                    // NOTE: can not move into a tile ocupied by a team mate
                    continue;
                } else {
                    ap_writer.send(SpendAPEvent::new(e.id, cp.attack_cost()));
                    cmd.spawn().insert(ModifyHP::new(dest, -cp.attack_damage()));
                }
            } else {
                team_map[dest] = Some(*team);
                ap_writer.send(SpendAPEvent::new(e.id, delta_costs[&e.delta]));
                move_writer.send(MoveEvent::new(e.id, dest));
            }
        }
    }
}

pub fn apply_hp_modify(
    mut cmd: Commands,
    mut actors: Query<(Entity, &Vector2D, &mut Capability)>,
    hp_mod: Query<(Entity, &ModifyHP)>,
    mut team_map: ResMut<TeamMap>,
) {
    for (e, hpm) in hp_mod.iter() {
        if let Some((ee, pt, mut cp)) = actors.iter_mut().find(|(_, p, _)| ***p == hpm.location) {
            cp.hp_apply(hpm.amount);
            if cp.hp_current() <= 0 {
                // TODO: animated death
                log::info!("death to {:?}!", ee);
                cmd.entity(ee).despawn_recursive();
                team_map[**pt] = None;
            }
        }
        cmd.entity(e).despawn();
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
    mut team_map: ResMut<TeamMap>,
) {
    for e in move_reader.iter() {
        if let Ok((mut pt, mut tr, mut fov)) = actors.get_mut(e.id) {
            let z = tr.translation.z;
            let pt_old = **pt;
            tr.translation = map_options.to_world_position(e.destination).extend(z);
            *pt = Vector2D::from(e.destination);
            team_map[pt_old] = None;
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
