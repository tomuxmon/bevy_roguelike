use crate::components::*;
use bevy::prelude::*;

// TODO: macro for a repeating code
// TODO: trait for a repeating update(Attributes) function
// TODO: investigate why Changed<_> does not work as intended (includes non changes)

pub fn attributes_update_action_points(
    mut actors: Query<(&Attributes, &mut ActionPoints, Changed<Attributes>)>,
) {
    for (atr, mut ap, _) in actors.iter_mut() {
        ap.update(atr);
    }
}
pub fn attributes_update_hit_points(
    mut actors: Query<(&Attributes, &mut HitPoints, Changed<Attributes>)>,
) {
    for (atr, mut hp, _) in actors.iter_mut() {
        hp.update(atr);
    }
}
pub fn attributes_update_attack_stats(
    mut actors: Query<(&Attributes, &mut AttackStats, Changed<Attributes>)>,
) {
    for (atr, mut atk, _) in actors.iter_mut() {
        atk.update(atr);
    }
}
pub fn attributes_update_defense_stats(
    mut actors: Query<(&Attributes, &mut DefenseStats, Changed<Attributes>)>,
) {
    for (atr, mut das, _) in actors.iter_mut() {
        das.update(atr);
    }
}

pub fn attributes_update_field_of_view(
    mut actors: Query<(&Attributes, &mut FieldOfView, Changed<Attributes>)>,
) {
    for (atr, mut fov, _) in actors.iter_mut() {
        fov.update(atr);
    }
}
