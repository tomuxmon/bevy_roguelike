use super::{AttributeType, Attributes, LinearFormula};
use bevy::{prelude::*, reflect::FromReflect};
use std::{fmt::Debug, hash::Hash};

pub const AP_MOVE_COST_DEFAULT: i16 = 100;
pub const AP_IDLE_COST_DEFAULT: i16 = 64;
pub const AP_TURN_READY_DEFAULT: i16 = 128;
pub const AP_INCREMENT_MIN: i16 = 64;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct ActionPointsDirty;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ActionPoints<A: AttributeType> {
    turn_ready: i16,
    current: i16,
    increment: i16,
    increment_formula: LinearFormula<A>,
}
impl<A: AttributeType> ActionPoints<A> {
    pub fn new(increment_formula: LinearFormula<A>, atr: &Attributes<A>) -> Self {
        Self {
            turn_ready: AP_TURN_READY_DEFAULT,
            increment: AP_INCREMENT_MIN + (increment_formula.compute(atr) * 64.) as i16,
            increment_formula,
            current: 0,
        }
    }
    pub fn update(&mut self, atr: &Attributes<A>) {
        self.turn_ready = AP_TURN_READY_DEFAULT;
        self.increment = AP_INCREMENT_MIN + (self.increment_formula.compute(atr) * 64.) as i16;
    }

    pub fn turn_ready_to_act(&self) -> i16 {
        self.turn_ready
    }
    pub fn current(&self) -> i16 {
        self.current
    }
    pub fn current_add(&mut self) -> i16 {
        self.current += self.increment;
        self.current
    }
    pub fn current_minus(&mut self, cost: i16) -> i16 {
        self.current -= cost;
        self.current
    }
    pub fn increment(&self) -> i16 {
        self.increment
    }
}
