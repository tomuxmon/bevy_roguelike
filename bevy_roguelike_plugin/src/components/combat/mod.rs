use super::{AttributeType, Attributes};
use bevy::{prelude::*, reflect::FromReflect, utils::HashSet};
use rand::prelude::*;
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
        *attributes.get(self.attribute) as f32 * self.multiplier as f32 / 1000.
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Formula {
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
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
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
    // pub fn roll_against(
    //     &self,
    //     other: &Rate,
    //     self_attributes: &Attributes,
    //     other_attributes: &Attributes,
    //     rng: &mut StdRng,
    // ) -> bool {
    //     let self_chance = self.compute(self_attributes);
    //     let other_chance = other.compute(other_attributes);
    //     !rng.gen_ratio(other_chance.min(self_chance) as u32, self_chance as u32)
    // }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct ActionCost {
    /// cost in action points, [`super::ActionPoints::TURN_READY_DEFAULT`] being one single turn
    pub cost: i16,
    /// formula to compute the multiplier. using inverted compute.
    pub multiplier_inverted: Formula,
}
impl ActionCost {
    pub fn compute(&self, attributes: &Attributes) -> i16 {
        (self.cost as f32 / self.multiplier_inverted.compute(attributes)) as i16
    }
}

/// Information about damage that can be calculated based on actor attributes.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Damage {
    pub kind: DamageKind,
    pub amount: Range<i32>,
    pub amount_multiplier: Formula,
    pub hit_cost: ActionCost,
    pub hit_chance: Rate,
}
impl Damage {
    pub fn compute(&self, attributes: &Attributes, rng: &mut StdRng) -> i32 {
        (rng.gen_range(self.amount.clone()) as f32 * self.amount_multiplier.compute(attributes))
            as i32
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
pub struct Protect {
    pub kind: DamageKind,
    pub amount_multiplier: Option<Formula>,
    pub amount: i32,
}

impl Protect {
    pub fn compute(&self, attributes: &Attributes) -> i32 {
        (self.amount as f32
            * if let Some(formula) = &self.amount_multiplier {
                formula.compute(attributes)
            } else {
                1.
            }) as i32
    }
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
    pub fn extend(&mut self, other: &Protection) -> &mut Protection {
        self.amounts.extend(other.clone().amounts);
        self
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
    pub kind: DamageKind,
    /// Resistance amount in percents. 100 means fully resists specified [`DamageKind`].
    pub percent: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect, Serialize, Deserialize)]
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
    pub fn ingest(&mut self, other: &Resistance) -> &mut Resistance {
        // todo: fix me . instead match on DamageKind
        self.amounts.extend(other.clone().amounts);
        self
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
impl Evasion {
    /// will try to evade damage. returns true and cost if evaded. if not returns false and zero.
    pub fn try_evade(
        &self,
        damage: &Damage,
        self_attributes: &Attributes,
        attacker_attributes: &Attributes,
        rng: &mut StdRng,
    ) -> (bool, i16) {
        let rate_evade = self.chance.compute(self_attributes);
        let rate_hit = damage.hit_chance.compute(attacker_attributes);
        let evaded = rng.gen_ratio(rate_evade.min(rate_hit) as u32, rate_hit as u32);
        bevy::log::trace!(
            "evade rate {}, hit rate {}, evaded {}",
            rate_evade,
            rate_hit,
            evaded
        );
        let cost = if evaded {
            self.cost.compute(self_attributes)
        } else {
            0
        };
        (evaded, cost)
    }
}

/// Block works on specified damage types. Works together with [Rate].
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Block {
    // blocks specific damage type?
    pub block_type: Vec<DamageKind>,
    pub cost: ActionCost,
    /// Block chance. Compared against [`Damage::hit_rate`].
    pub chance: Rate,
}

impl Block {
    /// will try to block damage when block type matches. returns true and cost if blocked. if not returns false and zero.
    pub fn try_block(
        &self,
        damage: &Damage,
        self_attributes: &Attributes,
        attacker_attributes: &Attributes,
        rng: &mut StdRng,
    ) -> (bool, i16) {
        if !self.block_type.iter().any(|k| *k == damage.kind) {
            return (false, 0);
        }
        let rate_block = self.chance.compute(self_attributes);
        let rate_hit = damage.hit_chance.compute(attacker_attributes);
        let blocked = rng.gen_ratio(rate_block.min(rate_hit) as u32, rate_hit as u32);
        bevy::log::trace!(
            "block rate {}, hit rate {}, blocked {}",
            rate_block,
            rate_hit,
            blocked
        );
        let cost = if blocked {
            self.cost.compute(self_attributes)
        } else {
            0
        };
        (blocked, cost)
    }
}
