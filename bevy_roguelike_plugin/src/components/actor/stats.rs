use crate::components::{AttackBoost, DefenseBoost};
use bevy::prelude::*;
use std::ops::Add;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Attributes {
    pub strength: u8,
    pub dexterity: u8,
    pub inteligence: u8,
    pub toughness: u8,
    pub perception: u8,
    pub willpower: u8,
}
impl Attributes {
    pub fn new(
        strength: u8,
        dexterity: u8,
        inteligence: u8,
        toughness: u8,
        perception: u8,
        willpower: u8,
    ) -> Self {
        Self {
            strength,
            dexterity,
            inteligence,
            toughness,
            perception,
            willpower,
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ActionPoints {
    turn_ready: i16,
    current: i16,
    increment: i16,
}
impl ActionPoints {
    pub const DELTA_COST_MOVE_DEFAULT: i16 = 100;
    pub const IDLE_COST_DEFAULT: i16 = 64;
    pub const TURN_READY_DEFAULT: i16 = 128;
    pub const INCREMENT_MIN: i16 = 64;

    pub fn new(atr: &Attributes) -> Self {
        Self {
            turn_ready: ActionPoints::TURN_READY_DEFAULT,
            increment: ActionPoints::INCREMENT_MIN
                + (atr.dexterity as i16) * 7
                + (atr.willpower as i16) * 3,
            current: 0,
        }
    }
    pub fn update(&mut self, atr: &Attributes) {
        self.turn_ready = ActionPoints::TURN_READY_DEFAULT;
        self.increment =
            ActionPoints::INCREMENT_MIN + (atr.dexterity as i16) * 7 + (atr.willpower as i16) * 3;
    }

    pub fn turn_ready_to_act(&self) -> i16 {
        self.turn_ready
    }
    pub fn current(&self) -> i16 {
        self.current
    }
    pub fn current_add(&mut self) -> i16 {
        self.current = self.current + self.increment;
        self.current
    }
    pub fn current_minus(&mut self, cost: i16) -> i16 {
        //TODO: Too much of defending overflows it into negative side. fix it
        self.current -= cost;
        self.current
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct HitPoints {
    full: i16,
    current: i16,

    regen_ready: i16,
    regen_current: i16,
    regen_increment: i16,
}
impl HitPoints {
    pub const FULL_MIN: i16 = 20;
    pub const REGEN_READY_DEFAULT: i16 = 128;
    pub const REGEN_INCREMENT_MIN: i16 = 64;

    pub fn new(atr: &Attributes) -> Self {
        let full = HitPoints::FULL_MIN
            + (atr.toughness as i16) * 4
            + atr.strength as i16
            + (atr.willpower as f32 / 2.) as i16;
        Self {
            full,
            current: full,
            regen_ready: HitPoints::REGEN_READY_DEFAULT,
            regen_current: 0,
            regen_increment: HitPoints::REGEN_INCREMENT_MIN
                + (atr.toughness as i16) * 4
                + (atr.strength as i16) * 2
                + (atr.willpower as i16),
        }
    }
    pub fn update(&mut self, atr: &Attributes) {
        let current_ratio = self.current as f32 / self.full as f32;
        self.full = HitPoints::FULL_MIN
            + (atr.toughness as i16) * 4
            + atr.strength as i16
            + (atr.willpower as f32 / 2.) as i16;
        self.current = (current_ratio * self.full as f32) as i16;
        self.regen_ready = HitPoints::REGEN_READY_DEFAULT;
        self.regen_increment = HitPoints::REGEN_INCREMENT_MIN
            + (atr.toughness as i16) * 4
            + (atr.strength as i16) * 2
            + (atr.willpower as i16);
    }

    pub fn apply(&mut self, amount: i16) -> i16 {
        self.current = i16::min(self.current + amount, self.full);
        self.current
    }
    pub fn current(&self) -> i16 {
        self.current
    }
    pub fn percent(&self) -> f32 {
        self.current as f32 / self.full as f32
    }
    pub fn regen(&mut self) {
        self.regen_current = self.regen_current + self.regen_increment;
        if self.regen_current > self.regen_ready {
            let amount = self.regen_current / self.regen_ready;
            let rem = self.regen_current % self.regen_ready;
            self.apply(amount);
            self.regen_current = rem;
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct AttackStats {
    // TODO: should only work in conjunction with a weapon (or fists / claws / tentacles ...)
    damage: i16,
    // TODO: weapon should influence it
    cost: i16,
    // TODO: weapon should influence it
    rate: i16,
}
impl AttackStats {
    pub const COST_MAX: i16 = 128;
    pub const COST_MIN: i16 = 36;
    pub const DAMAGE_MIN: i16 = 1;
    pub const RATE_MIN: i16 = 8;

    pub fn new(atr: &Attributes) -> Self {
        Self {
            cost: i16::max(
                AttackStats::COST_MAX - atr.dexterity as i16,
                AttackStats::COST_MIN,
            ),
            damage: AttackStats::DAMAGE_MIN + atr.strength as i16,
            rate: AttackStats::RATE_MIN + atr.dexterity as i16,
        }
    }
    pub fn update(&mut self, atr: &Attributes) {
        self.cost = i16::max(
            AttackStats::COST_MAX - atr.dexterity as i16,
            AttackStats::COST_MIN,
        );
        self.damage = AttackStats::DAMAGE_MIN + atr.strength as i16;
        self.rate = AttackStats::RATE_MIN + atr.dexterity as i16;
    }
    pub fn damage(&self) -> i16 {
        self.damage
    }
    pub fn cost(&self) -> i16 {
        self.cost
    }
    pub fn rate(&self) -> i16 {
        self.rate
    }
}
impl Add<AttackBoost> for AttackStats {
    type Output = AttackStats;

    fn add(self, rhs: AttackBoost) -> Self::Output {
        AttackStats {
            damage: self.damage + rhs.damage(),
            cost: self.cost + rhs.cost(),
            rate: self.rate + rhs.rate(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct DefenseStats {
    absorb: i16,
    cost: i16,
    rate: i16,
}
impl DefenseStats {
    pub const ABSORB_MIN: i16 = 0;
    pub const COST_MAX: i16 = 36;
    pub const COST_MIN: i16 = 8;
    pub const RATE_MIN: i16 = 2;

    pub fn new(atr: &Attributes) -> Self {
        Self {
            absorb: DefenseStats::ABSORB_MIN
                + (atr.toughness as f32 / 4. + atr.willpower as f32 / 16.) as i16,
            cost: i16::max(
                DefenseStats::COST_MAX - atr.dexterity as i16,
                DefenseStats::COST_MIN,
            ),
            rate: DefenseStats::RATE_MIN
                + (atr.dexterity as f32 / 4. + atr.willpower as f32 / 16.) as i16,
        }
    }
    pub fn update(&mut self, atr: &Attributes) {
        self.absorb = DefenseStats::ABSORB_MIN
            + (atr.toughness as f32 / 4. + atr.willpower as f32 / 16.) as i16;
        self.cost = i16::max(
            DefenseStats::COST_MAX - atr.dexterity as i16,
            DefenseStats::COST_MIN,
        );
        self.rate = DefenseStats::RATE_MIN
            + (atr.dexterity as f32 / 4. + atr.willpower as f32 / 16.) as i16;
    }
    pub fn absorb(&self) -> i16 {
        self.absorb
    }
    pub fn cost(&self) -> i16 {
        self.cost
    }
    pub fn rate(&self) -> i16 {
        self.rate
    }
}
impl Add<DefenseBoost> for DefenseStats {
    type Output = DefenseStats;

    fn add(self, rhs: DefenseBoost) -> Self::Output {
        DefenseStats {
            absorb: self.absorb + rhs.absorb(),
            cost: self.cost() + rhs.cost(),
            rate: self.rate + rhs.rate(),
        }
    }
}
