use crate::{components::*, events::*};
use bevy::prelude::*;

pub fn apply_hp_modify(mut actors: Query<&mut HitPoints>, mut dmg_rdr: EventReader<ModifyHPEvent>) {
    for e in dmg_rdr.iter() {
        if let Ok(mut hp) = actors.get_mut(e.id) {
            hp.apply(e.amount);
        }
    }
}
pub fn gather_action_points(mut actors: Query<(&mut ActionPoints, &mut TurnState)>) {
    for (mut ap, mut ts) in actors
        .iter_mut()
        .filter(|(_, ts)| **ts == TurnState::Collect)
    {
        *ts = if ap.current_add() > ap.turn_ready_to_act() {
            TurnState::Act
        } else {
            // NOTE: not yet ready to perform turn.
            // skip this turn.
            TurnState::End
        };
    }
}
pub fn turn_end_now_gather_ap(mut actors: Query<&mut TurnState>) {
    if actors.iter().all(|ts| *ts == TurnState::End) {
        actors.for_each_mut(|mut ts| {
            *ts = TurnState::Collect;
        });
    }
}
