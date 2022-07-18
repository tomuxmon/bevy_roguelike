#![allow(clippy::forget_non_drop)] // https://github.com/bevyengine/bevy/issues/4601
use super::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Combat<K: DamageKind, A: AttributeType> {
    attributes: Attributes<A>,
    ap: ActionPoints<A>,
    hp: HitPoints<A>,
    damage: DamageList<K, A>,
    protection: Protection<K, A>,
    evasion: Evasion<A>,
    resistance: Resistance<K>,
    stats: StatsComputed<K, A>,
    stats_dirty: StatsComputedDirty,
}

#[allow(clippy::too_many_arguments)]
impl<K: DamageKind, A: AttributeType> Combat<K, A> {
    pub fn new(
        attributes: &Attributes<A>,
        ap_increment_formula: LinearFormula<A>,
        hp_full_formula: LinearFormula<A>,
        hp_regen_increment_formula: LinearFormula<A>,
        damage: DamageList<K, A>,
        protection: Protection<K, A>,
        evasion: Evasion<A>,
        resistance: Resistance<K>,
    ) -> Self {
        Self {
            attributes: attributes.clone(),
            ap: ActionPoints::new(ap_increment_formula, attributes),
            hp: HitPoints::new(hp_full_formula, hp_regen_increment_formula, attributes),
            damage,
            protection,
            evasion,
            resistance,
            stats: StatsComputed::default(),
            stats_dirty: StatsComputedDirty {},
        }
    }
}
