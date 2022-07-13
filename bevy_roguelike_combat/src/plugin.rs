use crate::{events::*, stats::*, stats_derived::*, systems::*};
use bevy::{ecs::schedule::StateData, prelude::*};
use std::marker::PhantomData;

pub struct RoguelikeCombatPlugin<S, K: DamageKind> {
    pub state_running: S,
    pub phantom_1: PhantomData<K>,
}

impl<S: StateData, K: DamageKind> Plugin for RoguelikeCombatPlugin<S, K> {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::on_update(self.state_running.clone())
                .with_system(attributes_update_action_points::<K>)
                .with_system(attributes_update_hit_points::<K>),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::on_update(self.state_running.clone()).with_system(attack::<K>),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::on_update(self.state_running.clone())
                .with_system(damage_hit_points)
                .with_system(idle_rest.after(damage_hit_points)),
        )
        .register_type::<Attributes>()
        .register_type::<AttributeType>()
        .register_type::<ActionPoints>()
        .register_type::<ActionPointsDirty>()
        .register_type::<HitPoints>()
        .register_type::<HitPointsDirty>()
        .register_type::<AttributeMultiplier>()
        .register_type::<Formula>()
        .register_type::<Rate>()
        .register_type::<ActionCost>()
        .register_type::<Damage<K>>()
        .register_type::<Protect<K>>()
        .register_type::<Resist<K>>()
        .register_type::<Protection<K>>()
        .register_type::<Resistance<K>>()
        .register_type::<Evasion>()
        .register_type::<Block<K>>()
        .register_type::<Evasion>()
        .register_type::<StatsComputed<K>>()
        .register_type::<StatsComputedDirty>()
        .register_type::<K>()
        .add_event::<AttackEvent>()
        .add_event::<IdleEvent>()
        .add_event::<DeathEvent>()
        .add_event::<DamageHitPointsEvent>();
    }
}
