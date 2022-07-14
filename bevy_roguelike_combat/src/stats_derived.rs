use super::stats::*;
use bevy::{
    prelude::*,
    reflect::{FromReflect, GetTypeRegistration},
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fmt::Display, hash::Hash, ops::Range};

/// Type of damage that can be inflicted by actors of the environment.
pub trait DamageKind:
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
{
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
pub struct Multiplier<A: AttributeType> {
    /// multiplier taking into account amount of governing attribute.
    /// multiplier = 100; attribute = 10; will result in multiplier equal to 1.
    pub multiplier: u8,
    /// attribute that is taken into account when calculating the multiplier
    pub attribute: A,
}
impl<A: AttributeType> Multiplier<A> {
    pub fn compute(&self, attributes: &Attributes<A>) -> f32 {
        attributes.get(self.attribute) as f32 * self.multiplier as f32 / 1000.
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
pub struct LinearFormula<A: AttributeType> {
    // TODO: include scale factor here
    // pub scale: f32,
    pub multipliers: Vec<Multiplier<A>>,
}
impl<A: AttributeType> LinearFormula<A> {
    pub fn new(multipls: impl IntoIterator<Item = Multiplier<A>>) -> Self {
        Self {
            multipliers: Vec::from_iter(multipls),
        }
    }
    pub fn empty() -> Self {
        Self {
            multipliers: Vec::new(),
        }
    }
    pub fn compute(&self, attributes: &Attributes<A>) -> f32 {
        if self.multipliers.is_empty() {
            return 1.;
        }
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

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct DamageList<K: DamageKind, A: AttributeType> {
    pub list: Vec<Damage<K, A>>,
}

/// Information about damage that can be calculated based on actor attributes.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Damage<K: DamageKind, A: AttributeType> {
    pub kind: K,
    pub amount: Range<i32>,
    pub amount_multiplier: LinearFormula<A>,
    pub hit_cost: ActionCost<A>,
    pub hit_chance: Rate<A>,
}

impl<K: DamageKind, A: AttributeType> Damage<K, A> {
    pub fn compute(&self, attributes: &Attributes<A>, rng: &mut StdRng) -> i32 {
        (self.amount_roll(rng) as f32 * self.amount_multiplier.compute(attributes)) as i32
    }
    fn amount_roll(&self, rng: &mut StdRng) -> i32 {
        if !self.amount.is_empty() {
            rng.gen_range(self.amount.clone())
        } else {
            self.amount.start
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
pub struct Protect<K: DamageKind, A: AttributeType> {
    pub kind: K,
    pub amount_multiplier: LinearFormula<A>,
    pub amount: i32,
}

impl<K: DamageKind, A: AttributeType> Protect<K, A> {
    pub fn compute(&self, attributes: &Attributes<A>) -> i32 {
        (self.amount as f32 * self.amount_multiplier.compute(attributes)) as i32
    }
}
impl<K: DamageKind, A: AttributeType> Display for Protect<K, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: also write formula if present
        write!(f, "{} +{}", self.kind, self.amount)
    }
}

/// Protective Value (PV) or the amount of direct damage negated
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
pub struct Protection<K: DamageKind, A: AttributeType> {
    pub amounts: Vec<Protect<K, A>>,
}
impl<K: DamageKind, A: AttributeType> Protection<K, A> {
    pub fn new(protections: impl IntoIterator<Item = Protect<K, A>>) -> Self {
        Self {
            amounts: Vec::from_iter(protections),
        }
    }
    pub fn extend(&mut self, other: &Protection<K, A>) -> &mut Protection<K, A> {
        self.amounts.extend(other.clone().amounts);
        self
    }
}
impl<K: DamageKind, A: AttributeType> Display for Protection<K, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.amounts
                .iter()
                .map(|p| format!("{}", p))
                .fold("".to_string(), |acc, x| format!("{}, {}", x, acc))
        )
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
pub struct Resist<K: DamageKind> {
    pub kind: K,
    /// Resistance amount in percents. 100 means fully resists specified [`DamageKind`].
    pub percent: u8,
}
impl<K: DamageKind> Display for Resist<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} +{}", self.kind, self.percent)
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Resistance<K: DamageKind> {
    /// defines resistance in percents per damage kind.
    pub amounts: Vec<Resist<K>>,
}
impl<K: DamageKind> Resistance<K> {
    pub fn new(resistances: impl IntoIterator<Item = Resist<K>>) -> Self {
        Self {
            amounts: Vec::from_iter(resistances),
        }
    }
    pub fn ingest(&mut self, other: &Resistance<K>) -> &mut Resistance<K> {
        // todo: fix me . instead match on DamageKind
        self.amounts.extend(other.clone().amounts);
        self
    }
}
impl<K: DamageKind> Display for Resistance<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.amounts
                .iter()
                .map(|r| format!("{}", r))
                .fold("".to_string(), |acc, x| format!("{}, {}", x, acc))
        )
    }
}

/// Evasion works on any damage type.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Component, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Evasion<A: AttributeType> {
    /// Cost in action points, [`super::ActionPoints::TURN_READY_DEFAULT`] being one single turn.
    /// Usually it should be close to 0.3 of the turn (posibility to avoid 3 attacks per turn).
    /// Usually dexterity should influence it
    pub cost: ActionCost<A>,
    /// Evasion chance. Compared against [`Damage::hit_rate`].
    pub chance: Rate<A>,
}
impl<A: AttributeType> Evasion<A> {
    /// will try to evade damage. returns true and cost if evaded. if not returns false and zero.
    pub fn try_evade<K: DamageKind>(
        &self,
        damage: &Damage<K, A>,
        self_attributes: &Attributes<A>,
        attacker_attributes: &Attributes<A>,
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
pub struct Block<K: DamageKind, A: AttributeType> {
    // blocks specific damage type?
    pub block_type: Vec<K>,
    pub cost: ActionCost<A>,
    /// Block chance. Compared against [`Damage::hit_rate`].
    pub chance: Rate<A>,
}

impl<K: DamageKind, A: AttributeType> Block<K, A> {
    /// will try to block damage when block type matches. returns true and cost if blocked. if not returns false and zero.
    pub fn try_block(
        &self,
        damage: &Damage<K, A>,
        self_attributes: &Attributes<A>,
        attacker_attributes: &Attributes<A>,
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

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct StatsComputedDirty;

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct StatsComputed<K: DamageKind, A: AttributeType> {
    pub attributes: Attributes<A>,
    pub protection: Protection<K, A>,
    pub resistance: Resistance<K>,
    pub evasion: Evasion<A>,
    pub block: Vec<Block<K, A>>,
    pub damage: Vec<Damage<K, A>>,
}
