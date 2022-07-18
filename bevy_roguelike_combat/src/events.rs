use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Debug, Copy, Clone)]
pub struct SpendAPEvent {
    pub id: Entity,
    pub amount: i16,
}

#[derive(Debug, Copy, Clone)]
pub struct ActionCompletedEvent {
    pub id: Entity,
}

impl SpendAPEvent {
    pub fn new(id: Entity, amount: i16) -> Self {
        Self { id, amount }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IdleEvent {
    pub id: Entity,
}

// NOTE: a clunky component to transfer damage
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Component)]
pub struct DamageHitPointsEvent {
    pub defender: Entity,
    pub amount: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct DeathEvent {
    pub actor: Entity,
}
