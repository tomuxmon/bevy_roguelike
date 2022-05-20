use super::{AttributeType, Attributes};
use bevy::prelude::*;
use std::ops::Range;

/// Type of damage that can be inflicted by actors or environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum DamageKind {
    /// phisical crushing damage
    Blunt,
    /// phisical puncturing damage
    Pierce,
    /// phisical cut damage
    Slash,
    /// elemental heat damage
    Fire,
    /// elemental cold damage
    Cold,
    /// elemental electrical damage
    Lightning,
}
impl Default for DamageKind {
    fn default() -> Self {
        Self::Blunt
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Formula {
    /// attribute that is taken into account when calculating the multiplier
    pub governing_attribute: AttributeType,
    /// multiplier taking into account amount of governing attribute.
    /// multiplier = 100; attribute = 10; will result in multiplier equal to 1.
    pub multiplier: u8,
    // could be Vec<(AttributeType, u8)> to be able to include more attributes per formula
}
impl Formula {
    pub fn new(governing_attribute: AttributeType, multiplier: u8) -> Self {
        Self {
            governing_attribute,
            multiplier,
        }
    }
    pub fn compute(&self, attributes: &Attributes) -> f32 {
        let amount = match self.governing_attribute {
            AttributeType::Strength => attributes.strength,
            AttributeType::Dexterity => attributes.dexterity,
            AttributeType::Inteligence => attributes.inteligence,
            AttributeType::Toughness => attributes.toughness,
            AttributeType::Perception => attributes.perception,
            AttributeType::Willpower => attributes.willpower,
        };
        amount as f32 * self.multiplier as f32 / 1000.
    }
}

/// applies to action being performed
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Rate {
    pub multiplier: Formula,
    /// A chance to perform action modifier where 100 means a normal chance.
    pub amount: u8,
}

impl Rate {
    pub fn new(multiplier: Formula, amount: u8) -> Self {
        Self { multiplier, amount }
    }
    pub fn compute(&self, attributes: &Attributes) -> i32 {
        (self.multiplier.compute(attributes) * self.amount as f32) as i32
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ActionCost {
    pub cost_multiplier: Formula,
    /// cost in action points, [`super::ActionPoints::TURN_READY_DEFAULT`] being one single turn
    pub cost: i16,
}
impl ActionCost {
    pub fn new(cost_multiplier: Formula, cost: i16) -> Self {
        Self {
            cost_multiplier,
            cost,
        }
    }
    pub fn compute(&self, attributes: &Attributes) -> i16 {
        (self.cost_multiplier.compute(attributes) * self.cost as f32) as i16
    }
}

/// Information about damage that can be calculated based on actor attributes.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Damage {
    pub kind: DamageKind,
    pub amount: Range<i32>,
    pub amount_multiplier: Formula,
    pub hit_cost: ActionCost,
    pub hit_chance: Rate,
}
impl Damage {
    pub fn new(
        kind: DamageKind,
        amount: Range<i32>,
        amount_multiplier: Formula,
        hit_cost: i16,
        hit_cost_multiplier: Formula,
        hit_chance_amount: u8,
        hit_chance_multiplier: Formula,
    ) -> Self {
        Self {
            kind,
            amount,
            amount_multiplier,
            hit_cost: ActionCost::new(hit_cost_multiplier, hit_cost),
            hit_chance: Rate::new(hit_chance_multiplier, hit_chance_amount),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct Protection {
    kind: DamageKind,
    amount_multiplier: Formula,
    amount: i32,
}

impl Protection {
    pub fn new(kind: DamageKind, amount_multiplier: Formula, amount: i32) -> Self {
        Self {
            kind,
            amount_multiplier,
            amount,
        }
    }
}

/// Protective Value (PV) or the amount of direct damage negated
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct Protections {
    pub amounts: Vec<Protection>,
}
impl Protections {
    pub fn new(amounts: Vec<Protection>) -> Self {
        Self { amounts }
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct Resistance {
    kind: DamageKind,
    /// Resistance amount in percents. 100 means fully resists specified [`DamageKind`].
    percent: u8,
}
impl Resistance {
    pub fn new(kind: DamageKind, percent: u8) -> Self {
        Self { kind, percent }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct Resistances {
    /// defines resistance in percents per damage kind.
    pub amounts: Vec<Resistance>,
}
impl Resistances {
    pub fn new(amounts: Vec<Resistance>) -> Self {
        Self { amounts }
    }
}

/// Evasion works on any damage type.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Evasion {
    pub cost: ActionCost,
    /// Evasion chance. Compared against [`Damage::hit_rate`].
    pub chance: Rate,
}
impl Evasion {
    /// Cost in action points, [`super::ActionPoints::TURN_READY_DEFAULT`] being one single turn.
    /// Usually it should be close to 0.3 of the turn (posibility to avoid 3 attacks per turn).
    /// Usually dexterity should influence it
    pub fn new(
        cost: i16,
        cost_multiplier: Formula,
        chance_amount: u8,
        chance_multiplier: Formula,
    ) -> Self {
        Self {
            cost: ActionCost::new(cost_multiplier, cost),
            chance: Rate::new(chance_multiplier, chance_amount),
        }
    }
}
/// Block works on specified damage types. Works together with [Rate].
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct Block {
    // blocks specific damage type?
    pub block_type: Vec<DamageKind>,
    pub cost: ActionCost,
    /// Block chance. Compared against [`Damage::hit_rate`].
    pub chance: Rate,
}

impl Block {
    pub fn new(
        block_type: Vec<DamageKind>,
        cost: i16,
        cost_multiplier: Formula,
        chance_amount: u8,
        chance_multiplier: Formula,
    ) -> Self {
        Self {
            block_type,
            cost: ActionCost::new(cost_multiplier, cost),
            chance: Rate::new(chance_multiplier, chance_amount),
        }
    }
}
