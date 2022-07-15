use super::{AttributeType, Attributes, LinearFormula};
use bevy::{prelude::*, reflect::FromReflect};
use std::{fmt::Debug, hash::Hash};

pub const HP_FULL_MIN: i16 = 20;
pub const HP_REGEN_READY_DEFAULT: i16 = 128;
pub const HP_REGEN_INCREMENT_MIN: i16 = 16;

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
    pub fn new(
        full_formula: LinearFormula<A>,
        regen_increment_formula: LinearFormula<A>,
        atr: &Attributes<A>,
    ) -> Self {
        let full = HP_FULL_MIN + (full_formula.compute(atr) * 20.) as i16;
        Self {
            is_alive: true,
            full,
            current: full,
            regen_ready: HP_REGEN_READY_DEFAULT,
            regen_current: 0,
            regen_increment: HP_REGEN_INCREMENT_MIN
                + (regen_increment_formula.compute(atr) * 16.) as i16,
            full_formula,
            regen_increment_formula,
        }
    }
    pub fn update(&mut self, atr: &Attributes<A>) {
        let current_ratio = self.current as f32 / self.full as f32;
        self.full = HP_FULL_MIN + (self.full_formula.compute(atr) * 20.) as i16;
        self.current = (current_ratio * self.full as f32) as i16;
        self.regen_ready = HP_REGEN_READY_DEFAULT;
        self.regen_increment =
            HP_REGEN_INCREMENT_MIN + (self.regen_increment_formula.compute(atr) * 16.) as i16;
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
