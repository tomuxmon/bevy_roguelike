use super::{AttributeType, Attributes, LinearFormula};
use bevy::{prelude::*, reflect::FromReflect};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// applies to action being performed
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct ActionCost<A: AttributeType> {
    /// cost in action points, [`super::ActionPoints::TURN_READY_DEFAULT`] being one single turn
    pub cost: i16,
    /// formula to compute the multiplier.
    pub multiplier_inverted: LinearFormula<A>,
}
impl<A: AttributeType> ActionCost<A> {
    pub fn compute(&self, attributes: &Attributes<A>) -> i16 {
        (self.cost as f32 / self.multiplier_inverted.compute(attributes)) as i16
    }
}
