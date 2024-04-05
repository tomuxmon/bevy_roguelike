use super::{AttributeType, Attributes, DamageKind, LinearFormula};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fmt::Display, hash::Hash};

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect, Serialize, Deserialize,
)]
pub struct Protect<K: DamageKind, A: AttributeType> {
    pub kind: K,
    pub amount_multiplier: LinearFormula<A>,
    pub amount: i32,
}

impl<K: DamageKind, A: AttributeType> Protect<K, A> {
    pub fn compute(&self, attributes: &Attributes<A>) -> i32 {
        (self.amount as f32 * self.amount_multiplier.compute(attributes)) as i32
    }
}
impl<K: DamageKind, A: AttributeType> Display for Protect<K, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: also write formula if present
        write!(f, "{} +{}", self.kind, self.amount)
    }
}

/// Protective Value (PV) or the amount of direct damage negated
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Protection<K: DamageKind, A: AttributeType> {
    pub amounts: Vec<Protect<K, A>>,
}
impl<K: DamageKind, A: AttributeType> Protection<K, A> {
    pub fn new(protections: impl IntoIterator<Item = Protect<K, A>>) -> Self {
        Self {
            amounts: Vec::from_iter(protections),
        }
    }
    pub fn extend(&mut self, other: &Protection<K, A>) -> &mut Protection<K, A> {
        self.amounts.extend(other.clone().amounts);
        self
    }
}
impl<K: DamageKind, A: AttributeType> Display for Protection<K, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.amounts
                .iter()
                .map(|p| format!("{}", p))
                .fold("".to_string(), |acc, x| format!("{}, {}", x, acc))
        )
    }
}
