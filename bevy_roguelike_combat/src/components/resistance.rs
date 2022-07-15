use super::DamageKind;
use bevy::{prelude::*, reflect::FromReflect};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fmt::Display, hash::Hash};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Component,
    Reflect,
    FromReflect,
    Serialize,
    Deserialize,
)]
#[reflect(Component)]
pub struct Resist<K: DamageKind> {
    pub kind: K,
    /// Resistance amount in percents. 100 means fully resists specified [`DamageKind`].
    pub percent: u8,
}
impl<K: DamageKind> Display for Resist<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} +{}", self.kind, self.percent)
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Resistance<K: DamageKind> {
    /// defines resistance in percents per damage kind.
    pub amounts: Vec<Resist<K>>,
}
impl<K: DamageKind> Resistance<K> {
    pub fn new(resistances: impl IntoIterator<Item = Resist<K>>) -> Self {
        Self {
            amounts: Vec::from_iter(resistances),
        }
    }
    pub fn ingest(&mut self, other: &Resistance<K>) -> &mut Resistance<K> {
        // todo: fix me . instead match on DamageKind
        self.amounts.extend(other.clone().amounts);
        self
    }
}
impl<K: DamageKind> Display for Resistance<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.amounts
                .iter()
                .map(|r| format!("{}", r))
                .fold("".to_string(), |acc, x| format!("{}, {}", x, acc))
        )
    }
}
