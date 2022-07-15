use super::{AttributeType, Attributes};
use bevy::{prelude::*, reflect::FromReflect};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

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
pub struct Multiplier<A: AttributeType> {
    /// multiplier taking into account amount of governing attribute.
    /// multiplier = 100; attribute = 10; will result in multiplier equal to 1.
    pub multiplier: u8,
    /// attribute that is taken into account when calculating the multiplier
    pub attribute: A,
}
impl<A: AttributeType> Multiplier<A> {
    pub fn compute(&self, attributes: &Attributes<A>) -> f32 {
        attributes.get(self.attribute) as f32 * self.multiplier as f32 / 1000.
    }
}

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
pub struct LinearFormula<A: AttributeType> {
    // TODO: include scale factor here
    // pub scale: f32,
    pub multipliers: Vec<Multiplier<A>>,
}
impl<A: AttributeType> LinearFormula<A> {
    pub fn new(multipls: impl IntoIterator<Item = Multiplier<A>>) -> Self {
        Self {
            multipliers: Vec::from_iter(multipls),
        }
    }
    pub fn empty() -> Self {
        Self {
            multipliers: Vec::new(),
        }
    }
    pub fn compute(&self, attributes: &Attributes<A>) -> f32 {
        if self.multipliers.is_empty() {
            return 1.;
        }
        self.multipliers
            .iter()
            .map(|m| m.compute(attributes))
            .sum::<f32>()
    }
}
