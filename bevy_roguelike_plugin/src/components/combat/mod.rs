use super::{AttributeType, Attributes};
use bevy::{prelude::*, reflect::FromReflect, utils::HashSet};
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Type of damage that can be inflicted by actors or environment.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
#[reflect_value(PartialEq, Serialize, Deserialize)]
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
pub struct AttributeMultiplier {
    /// multiplier taking into account amount of governing attribute.
    /// multiplier = 100; attribute = 10; will result in multiplier equal to 1.
    pub multiplier: u8,
    /// attribute that is taken into account when calculating the multiplier
    pub attribute: AttributeType,
}
impl AttributeMultiplier {
    pub fn compute(&self, attributes: &Attributes) -> f32 {
        let amount = match self.attribute {
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

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Formula {
    // TODO: instead use MAshMap<AttributeType, u8>
    pub multipliers: HashSet<AttributeMultiplier>,
}
impl Formula {
    pub fn new(multipls: impl IntoIterator<Item = AttributeMultiplier>) -> Self {
        Self {
            multipliers: HashSet::from_iter(multipls),
        }
    }
    pub fn compute(&self, attributes: &Attributes) -> f32 {
        self.multipliers
            .iter()
            .map(|m| m.compute(attributes))
            .sum::<f32>()
    }
}

/// applies to action being performed
#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Rate {
    /// A chance to perform action modifier where 100 means a normal chance.
    pub amount: u8,
    pub multiplier: Formula,
}

impl Rate {
    pub fn compute(&self, attributes: &Attributes) -> i32 {
        (self.multiplier.compute(attributes) * self.amount as f32) as i32
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ActionCost {
    /// cost in action points, [`super::ActionPoints::TURN_READY_DEFAULT`] being one single turn
    pub cost: i16,
    // TODO: should be inverted formula where more of attribute means smaller multiplier
    pub cost_multiplier: Formula,
}
impl ActionCost {
    pub fn compute(&self, attributes: &Attributes) -> i16 {
        (self.cost_multiplier.compute(attributes) * self.cost as f32) as i16
    }
}

/// Information about damage that can be calculated based on actor attributes.
#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct Damage {
    pub kind: DamageKind,
    pub amount: Range<i32>,
    pub amount_multiplier: Formula,
    pub hit_cost: ActionCost,
    pub hit_chance: Rate,
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
pub struct Protect {
    pub kind: DamageKind,
    // Option<Formula> and no need for additional struct?
    pub amount_multiplier: Formula,
    pub amount: i32,
    // TODO: also include static protect without multiplier
}

/// Protective Value (PV) or the amount of direct damage negated
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Protection {
    pub amounts: Vec<Protect>,
}
impl Protection {
    pub fn new(protections: impl IntoIterator<Item = Protect>) -> Self {
        Self {
            amounts: Vec::from_iter(protections),
        }
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
pub struct Resist {
    kind: DamageKind,
    /// Resistance amount in percents. 100 means fully resists specified [`DamageKind`].
    percent: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct Resistance {
    /// defines resistance in percents per damage kind.
    pub amounts: HashSet<Resist>,
}
impl Resistance {
    pub fn new(resistances: impl IntoIterator<Item = Resist>) -> Self {
        Self {
            amounts: HashSet::from_iter(resistances),
        }
    }
}

/// Evasion works on any damage type.
#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct Evasion {
    /// Cost in action points, [`super::ActionPoints::TURN_READY_DEFAULT`] being one single turn.
    /// Usually it should be close to 0.3 of the turn (posibility to avoid 3 attacks per turn).
    /// Usually dexterity should influence it
    pub cost: ActionCost,
    /// Evasion chance. Compared against [`Damage::hit_rate`].
    pub chance: Rate,
}

/// Block works on specified damage types. Works together with [Rate].
#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect)]
pub struct Block {
    // blocks specific damage type?
    pub block_type: Vec<DamageKind>,
    pub cost: ActionCost,
    /// Block chance. Compared against [`Damage::hit_rate`].
    pub chance: Rate,
}
