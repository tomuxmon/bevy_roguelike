use crate::components::*;
use bevy::prelude::*;
use bevy_roguelike_combat::*;

// TODO: move to bevy_roguelike_turns?
pub fn action_completed(
    mut actors: Query<(&mut TurnState, &mut HitPoints<RogueAttributeType>)>,
    mut action_completed_reader: EventReader<ActionCompletedEvent>,
) {
    for e in action_completed_reader.iter() {
        if let Ok((mut ts, mut hp)) = actors.get_mut(e.id) {
            *ts = TurnState::End;
            hp.regen();
        }
    }
}

// TODO: move to bevy_roguelike_turns
pub fn gather_action_points(
    mut actors: Query<(&mut ActionPoints<RogueAttributeType>, &mut TurnState)>,
) {
    actors.par_iter_mut().for_each_mut(|(mut ap, mut ts)| {
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
pub fn turn_end_now_gather(mut actors: Query<&mut TurnState>) {
    if actors.iter().all(|ts| *ts == TurnState::End) {
        actors.par_iter_mut().for_each_mut(|mut ts| {
            *ts = TurnState::Collect;
        });
    }
}
