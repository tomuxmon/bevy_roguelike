use super::{AttributeType, Attributes, LinearFormula};
use bevy::{prelude::*, reflect::FromReflect};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Rate<A: AttributeType> {
    /// A chance to perform action modifier where 100 means a normal chance.
    pub amount: u8,
    pub multiplier: LinearFormula<A>,
}
impl<A: AttributeType> Rate<A> {
    pub fn compute(&self, attributes: &Attributes<A>) -> i32 {
        (self.multiplier.compute(attributes) * self.amount as f32) as i32
    }
}
