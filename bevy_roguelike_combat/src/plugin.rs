use crate::{events::*, stats::*, stats_derived::*, systems::*};
use bevy::{ecs::schedule::StateData, prelude::*};
use std::marker::PhantomData;

pub struct RoguelikeCombatPlugin<S, K: DamageKind, A: AttributeType> {
    pub state_running: S,
    pub phantom_1: PhantomData<K>,
    pub phantom_2: PhantomData<A>,
}

impl<S: StateData, K: DamageKind, A: AttributeType> Plugin for RoguelikeCombatPlugin<S, K, A> {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::on_update(self.state_running.clone())
                .with_system(attributes_update_action_points::<K, A>)
                .with_system(attributes_update_hit_points::<K, A>),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::on_update(self.state_running.clone()).with_system(attack::<K, A>),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::on_update(self.state_running.clone())
                .with_system(damage_hit_points::<A>)
                .with_system(idle_rest::<A>.after(damage_hit_points::<A>)),
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
        .add_event::<AttackEvent>()
        .add_event::<IdleEvent>()
        .add_event::<DeathEvent>()
        .add_event::<DamageHitPointsEvent>();
    }
}
