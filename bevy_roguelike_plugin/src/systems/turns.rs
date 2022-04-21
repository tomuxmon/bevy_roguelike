use crate::{components::*, events::*};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;

pub fn apply_hp_modify(
    mut cmd: Commands,
    mut actors: Query<&mut Capability>,
    mut dmg_rdr: EventReader<ModifyHPEvent>,
) {
    for e in dmg_rdr.iter() {
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
    mut ap_rdr: EventReader<SpendAPEvent>,
) {
    for e in ap_rdr.iter() {
        if let Ok((mut cp, mut ts)) = actors.get_mut(e.id) {
            if cp.ap_current_minus(e.amount) < cp.ap_turn_ready_to_act() {
                *ts = TurnState::End;
            }
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
