use super::{ActionCost, AttributeType, Attributes, Damage, DamageKind, Rate};
use bevy::prelude::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Block works on specified damage types. Works together with [Rate].
#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect, Serialize, Deserialize)]
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
