use super::*;
use bevy::prelude::*;
use std::fmt::Debug;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct StatsComputedDirty;

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct StatsComputed<K: DamageKind, A: AttributeType> {
    pub attributes: Attributes<A>,
    pub protection: Protection<K, A>,
    pub resistance: Resistance<K>,
    pub evasion: Evasion<A>,
    pub block: Vec<Block<K, A>>,
    pub damage: Vec<Damage<K, A>>,
}
