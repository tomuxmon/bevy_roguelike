use crate::{components::*, events::*};
use bevy::log;
use bevy::prelude::*;

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
pub fn gather_action_points(mut actors: Query<(&mut Capability, &mut TurnState)>) {
    for (mut cp, mut ts) in actors
        .iter_mut()
        .filter(|(_, ts)| **ts == TurnState::Collect)
    {
        *ts = if cp.ap_current_add() > cp.ap_turn_ready_to_act() {
            TurnState::Act
        } else {
            // NOTE: not yet ready to perform turn.
            // skip this turn.
            TurnState::End
        };
    }
}
pub fn turn_end_now_gather(mut actors: Query<&mut TurnState>) {
    if actors.iter().all(|ts| *ts == TurnState::End) {
        actors.for_each_mut(|mut ts| {
            *ts = TurnState::Collect;
        });
    }
}
