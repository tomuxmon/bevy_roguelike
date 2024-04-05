use crate::{components::*, events::*, systems::*};
use bevy::prelude::*;
use bevy_roguelike_states::AppState;
use std::marker::PhantomData;

pub struct RoguelikeCombatPlugin<K: DamageKind, A: AttributeType> {
    _phantom: PhantomData<(K, A)>,
}

impl<K: DamageKind, A: AttributeType> Default for RoguelikeCombatPlugin<K, A> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<K: DamageKind, A: AttributeType> Plugin for RoguelikeCombatPlugin<K, A> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                attributes_update_action_points::<K, A>.run_if(in_state(AppState::InGame)),
                attributes_update_hit_points::<K, A>.run_if(in_state(AppState::InGame)),
            ),
        )
        .add_systems(
            Update,
            (
                attack::<K, A>.run_if(in_state(AppState::InGame)),
                spend_ap::<A>.run_if(in_state(AppState::InGame)),
            ),
        )
        .add_systems(
            PostUpdate,
            (
                damage_hit_points::<A>.run_if(in_state(AppState::InGame)),
                idle_rest::<A>
                    .after(damage_hit_points::<A>)
                    .run_if(in_state(AppState::InGame)),
            ),
        )
        .register_type::<Attributes<A>>()
        .register_type::<A>()
        .register_type::<ActionPoints<A>>()
        .register_type::<ActionPointsDirty>()
        .register_type::<HitPoints<A>>()
        .register_type::<HitPointsDirty>()
        .register_type::<Multiplier<A>>()
        .register_type::<LinearFormula<A>>()
        .register_type::<Rate<A>>()
        .register_type::<ActionCost<A>>()
        .register_type::<Damage<K, A>>()
        .register_type::<Protect<K, A>>()
        .register_type::<Resist<K>>()
        .register_type::<Protection<K, A>>()
        .register_type::<Resistance<K>>()
        .register_type::<Evasion<A>>()
        .register_type::<Block<K, A>>()
        .register_type::<StatsComputed<K, A>>()
        .register_type::<StatsComputedDirty>()
        .register_type::<K>()
        .add_event::<SpendAPEvent>()
        .add_event::<ActionCompletedEvent>()
        .add_event::<AttackEvent>()
        .add_event::<IdleEvent>()
        .add_event::<DeathEvent>()
        .add_event::<DamageHitPointsEvent>();
    }
}
