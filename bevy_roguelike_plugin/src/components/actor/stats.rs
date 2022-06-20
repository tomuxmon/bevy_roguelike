use crate::components::{Block, Damage, Evasion, Protection, Resistance};
use bevy::{prelude::*, reflect::FromReflect, utils::HashMap};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, iter::Sum, ops::Add};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
#[reflect_value(PartialEq, Serialize, Deserialize)]
pub enum AttributeType {
    Strength,
    Dexterity,
    Inteligence,
    Toughness,
    Perception,
    Willpower,
}
impl Default for AttributeType {
    fn default() -> Self {
        Self::Strength
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Attributes {
    pub list: HashMap<AttributeType, u8>,
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
            list: HashMap::from_iter(vec![
                (AttributeType::Strength, strength),
                (AttributeType::Dexterity, dexterity),
                (AttributeType::Inteligence, inteligence),
                (AttributeType::Toughness, toughness),
                (AttributeType::Perception, perception),
                (AttributeType::Willpower, willpower),
            ]),
        }
    }

    pub fn get(&self, attribute_type: AttributeType) -> u8 {
        *self.list.get(&attribute_type).unwrap_or(&0)
    }
}
impl Default for Attributes {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0)
    }
}
impl Add for Attributes {
    type Output = Attributes;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            list: HashMap::from_iter(self.list.into_iter().map(|(t, v)| (t, v + rhs.get(t)))),
        }
    }
}
impl Sum for Attributes {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Attributes::default(), |acc, a| acc + a)
    }
}
impl Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.list
                .iter()
                .map(|(&attribute_type, &amount)| format!("{:?} +{}", attribute_type, amount))
                .fold("".to_string(), |acc, x| format!("{}, {}", x, acc))
        )
    }
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct ActionPointsDirty;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ActionPoints {
    turn_ready: i16,
    current: i16,
    increment: i16,
}
impl ActionPoints {
    pub const MOVE_COST_DEFAULT: i16 = 100;
    pub const IDLE_COST_DEFAULT: i16 = 64;
    pub const TURN_READY_DEFAULT: i16 = 128;
    pub const INCREMENT_MIN: i16 = 64;

    pub fn new(atr: &Attributes) -> Self {
        Self {
            turn_ready: ActionPoints::TURN_READY_DEFAULT,
            increment: ActionPoints::INCREMENT_MIN
                + (atr.get(AttributeType::Dexterity) as i16) * 7
                + (atr.get(AttributeType::Willpower) as i16) * 3,
            current: 0,
        }
    }
    pub fn update(&mut self, atr: &Attributes) {
        self.turn_ready = ActionPoints::TURN_READY_DEFAULT;
        self.increment = ActionPoints::INCREMENT_MIN
            + (atr.get(AttributeType::Dexterity) as i16) * 7
            + (atr.get(AttributeType::Willpower) as i16) * 3;
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
            + atr.get(AttributeType::Toughness) as i16 * 4
            + atr.get(AttributeType::Strength) as i16
            + (atr.get(AttributeType::Willpower) as f32 / 2.) as i16;
        Self {
            full,
            current: full,
            regen_ready: HitPoints::REGEN_READY_DEFAULT,
            regen_current: 0,
            regen_increment: HitPoints::REGEN_INCREMENT_MIN
                + atr.get(AttributeType::Toughness) as i16 * 4
                + atr.get(AttributeType::Strength) as i16 * 2
                + atr.get(AttributeType::Willpower) as i16,
        }
    }
    pub fn update(&mut self, atr: &Attributes) {
        let current_ratio = self.current as f32 / self.full as f32;
        self.full = HitPoints::FULL_MIN
            + atr.get(AttributeType::Toughness) as i16 * 4
            + atr.get(AttributeType::Strength) as i16
            + (atr.get(AttributeType::Willpower) as f32 / 2.) as i16;
        self.current = (current_ratio * self.full as f32) as i16;
        self.regen_ready = HitPoints::REGEN_READY_DEFAULT;
        self.regen_increment = HitPoints::REGEN_INCREMENT_MIN
            + atr.get(AttributeType::Toughness) as i16 * 4
            + atr.get(AttributeType::Strength) as i16 * 2
            + atr.get(AttributeType::Willpower) as i16;
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
        self.regen_current += self.regen_increment;
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
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct StatsComputedDirty;

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct StatsComputed {
    pub attributes: Attributes,
    pub protection: Protection,
    pub resistance: Resistance,
    pub evasion: Evasion,
    pub block: Vec<Block>,
    pub damage: Vec<Damage>,
}
