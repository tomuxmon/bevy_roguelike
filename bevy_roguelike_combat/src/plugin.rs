use crate::{events::*, stats::*, stats_derived::*, systems::*};
use bevy::{ecs::schedule::StateData, prelude::*, utils::HashSet};

pub struct RogulikeCombatPlugin<S> {
    pub state_running: S,
}

impl<S: StateData> Plugin for RogulikeCombatPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::on_update(self.state_running.clone())
                .with_system(attributes_update_action_points)
                .with_system(attributes_update_hit_points),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::on_update(self.state_running.clone()).with_system(attack),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::on_update(self.state_running.clone())
                .with_system(apply_damage_hit_points)
                .with_system(idle_rest.after(apply_damage_hit_points)),
        )
        .register_type::<Attributes>()
        .register_type::<AttributeType>()
        .register_type::<ActionPoints>()
        .register_type::<ActionPointsDirty>()
        .register_type::<HitPoints>()
        .register_type::<HitPointsDirty>()
        .register_type::<DamageKind>()
        .register_type::<AttributeMultiplier>()
        .register_type::<Formula>()
        .register_type::<Rate>()
        .register_type::<ActionCost>()
        .register_type::<Damage>()
        .register_type::<Protect>()
        .register_type::<Resist>()
        .register_type::<Protection>()
        .register_type::<Resistance>()
        .register_type::<Evasion>()
        .register_type::<Block>()
        .register_type::<Evasion>()
        .register_type::<StatsComputed>()
        .register_type::<StatsComputedDirty>()
        .register_type::<Vec<DamageKind>>()
        .register_type::<Vec<Protect>>()
        .register_type::<HashSet<Resist>>()
        .add_event::<AttackEvent>()
        .add_event::<IdleEvent>()
        .add_event::<DeathEvent>()
        .add_event::<DamageHitPointsEvent>();
    }
}
