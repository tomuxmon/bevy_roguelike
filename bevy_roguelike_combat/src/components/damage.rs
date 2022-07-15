use super::{ActionCost, AttributeType, Attributes, LinearFormula, Rate};
use bevy::{
    prelude::*,
    reflect::{FromReflect, GetTypeRegistration},
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fmt::Display, hash::Hash, ops::Range};

/// Type of damage that can be inflicted by actors of the environment.
pub trait DamageKind:
    Component
    + Copy
    + Clone
    + Eq
    + Hash
    + Debug
    + Display
    + Default
    + Reflect
    + FromReflect
    + GetTypeRegistration
{
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct DamageList<K: DamageKind, A: AttributeType> {
    pub list: Vec<Damage<K, A>>,
}

/// Information about damage that can be calculated based on actor attributes.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Damage<K: DamageKind, A: AttributeType> {
    pub kind: K,
    pub amount: Range<i32>,
    pub amount_multiplier: LinearFormula<A>,
    pub hit_cost: ActionCost<A>,
    pub hit_chance: Rate<A>,
}

impl<K: DamageKind, A: AttributeType> Damage<K, A> {
    pub fn compute(&self, attributes: &Attributes<A>, rng: &mut StdRng) -> i32 {
        (self.amount_roll(rng) as f32 * self.amount_multiplier.compute(attributes)) as i32
    }
    fn amount_roll(&self, rng: &mut StdRng) -> i32 {
        if !self.amount.is_empty() {
            rng.gen_range(self.amount.clone())
        } else {
            self.amount.start
        }
    }
}
