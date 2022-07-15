use super::{ActionCost, AttributeType, Attributes, Damage, DamageKind, Rate};
use bevy::{prelude::*, reflect::FromReflect};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
