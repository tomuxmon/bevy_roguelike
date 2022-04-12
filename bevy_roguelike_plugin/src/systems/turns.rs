use crate::{components::*, events::*};
use bevy::prelude::*;

pub fn apply_hp_modify(
    mut actors: Query<&mut Capability>,
    mut dmg_rdr: EventReader<ModifyHPEvent>,
) {
    for e in dmg_rdr.iter() {
        if let Ok(mut cp) = actors.get_mut(e.id) {
            cp.hp_apply(e.amount);
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
pub fn turn_end_now_gather_or_die(
    mut cmd: Commands,
    mut actors: Query<(Entity, &mut TurnState, &Capability)>,
) {
    if actors.iter().all(|(_, ts, _)| *ts == TurnState::End) {
        actors.for_each_mut(|(e, mut ts, cp)| {
            if cp.hp_current() > 0 {
                *ts = TurnState::Collect;
            } else {
                // TODO: animated death
                cmd.entity(e).despawn_recursive();
            }
        });
    }
}
