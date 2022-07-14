use crate::stats_derived::LinearFormula;
use bevy::{
    prelude::*,
    reflect::{FromReflect, GetTypeRegistration},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fmt::Display, hash::Hash, iter::Sum, ops::Add};
use strum::IntoEnumIterator;

pub trait AttributeType:
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
    + IntoEnumIterator
{
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Attributes<A: AttributeType> {
    pub list: HashMap<A, u8>,
}
// TODO: default should fill the full list with attributes;

impl<A: AttributeType> Attributes<A> {
    pub fn get(&self, attribute_type: A) -> u8 {
        *self.list.get(&attribute_type).unwrap_or(&0)
    }
    pub fn with_all(value: u8) -> Self {
        Self {
            list: HashMap::from_iter(A::iter().map(|a| (a, value))),
        }
    }
}
impl<A: AttributeType> Default for Attributes<A> {
    fn default() -> Self {
        Self::with_all(0)
    }
}

impl<A: AttributeType> Add for Attributes<A> {
    type Output = Attributes<A>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            list: HashMap::from_iter(self.list.into_iter().map(|(t, v)| (t, v + rhs.get(t)))),
        }
    }
}
impl<A: AttributeType> Sum for Attributes<A> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Attributes::default(), |acc, a| acc + a)
    }
}
impl<A: AttributeType> Display for Attributes<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.list
                .iter()
                .map(|(&attribute_type, &amount)| format!("{} +{}", attribute_type, amount))
                .fold("".to_string(), |acc, x| format!("{}, {}", x, acc))
        )
    }
}

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
    // TODO: move consts out. no need to be generic here
    pub const MOVE_COST_DEFAULT: i16 = 100;
    pub const IDLE_COST_DEFAULT: i16 = 64;
    pub const TURN_READY_DEFAULT: i16 = 128;
    pub const INCREMENT_MIN: i16 = 64;

    pub fn new(increment_formula: LinearFormula<A>, atr: &Attributes<A>) -> Self {
        Self {
            turn_ready: ActionPoints::<A>::TURN_READY_DEFAULT,
            increment: ActionPoints::<A>::INCREMENT_MIN
                + (increment_formula.compute(atr) * 64.) as i16,
            increment_formula,
            current: 0,
        }
    }
    pub fn update(&mut self, atr: &Attributes<A>) {
        self.turn_ready = ActionPoints::<A>::TURN_READY_DEFAULT;
        self.increment =
            ActionPoints::<A>::INCREMENT_MIN + (self.increment_formula.compute(atr) * 64.) as i16;
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

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct HitPointsDirty;
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct HitPoints<A: AttributeType> {
    is_alive: bool,
    full: i16,
    current: i16,

    regen_ready: i16,
    regen_current: i16,
    regen_increment: i16,

    full_formula: LinearFormula<A>,
    regen_increment_formula: LinearFormula<A>,
}
impl<A: AttributeType> HitPoints<A> {
    // TODO: move consts out. no need to be generic here
    pub const FULL_MIN: i16 = 20;
    pub const REGEN_READY_DEFAULT: i16 = 128;
    pub const REGEN_INCREMENT_MIN: i16 = 16;

    pub fn new(
        full_formula: LinearFormula<A>,
        regen_increment_formula: LinearFormula<A>,
        atr: &Attributes<A>,
    ) -> Self {
        let full = HitPoints::<A>::FULL_MIN + (full_formula.compute(atr) * 20.) as i16;
        Self {
            is_alive: true,
            full,
            current: full,
            regen_ready: HitPoints::<A>::REGEN_READY_DEFAULT,
            regen_current: 0,
            regen_increment: HitPoints::<A>::REGEN_INCREMENT_MIN
                + (regen_increment_formula.compute(atr) * 16.) as i16,
            full_formula,
            regen_increment_formula,
        }
    }
    pub fn update(&mut self, atr: &Attributes<A>) {
        let current_ratio = self.current as f32 / self.full as f32;
        self.full = HitPoints::<A>::FULL_MIN + (self.full_formula.compute(atr) * 20.) as i16;
        self.current = (current_ratio * self.full as f32) as i16;
        self.regen_ready = HitPoints::<A>::REGEN_READY_DEFAULT;
        self.regen_increment = HitPoints::<A>::REGEN_INCREMENT_MIN
            + (self.regen_increment_formula.compute(atr) * 16.) as i16;
    }

    pub fn apply(&mut self, amount: i16) -> i16 {
        self.current = i16::min(self.current + amount, self.full);
        if self.current <= 0 {
            self.is_alive = false;
        }
        self.current
    }
    pub fn current(&self) -> i16 {
        self.current
    }
    pub fn percent(&self) -> f32 {
        self.current as f32 / self.full as f32
    }
    pub fn regen(&mut self) {
        self.regen_ratio(1.);
    }
    pub fn regen_ratio(&mut self, ratio: f32) {
        self.regen_current += (self.regen_increment as f32 * ratio) as i16;
        if self.regen_current > self.regen_ready {
            let amount = self.regen_current / self.regen_ready;
            let rem = self.regen_current % self.regen_ready;
            self.apply(amount);
            self.regen_current = rem;
        }
    }
    pub fn full(&self) -> i16 {
        self.full
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}
