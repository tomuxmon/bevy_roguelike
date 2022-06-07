use crate::components::Team;
use bevy::prelude::*;

// TODO: turn into Act component with is_dirty or is_used
#[derive(Debug, Copy, Clone)]
pub struct ActEvent {
    pub id: Entity,
    pub delta: IVec2,
}

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

impl SpendAPEvent {
    pub fn new(id: Entity, amount: i16) -> Self {
        Self { id, amount }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IdleEvent {
    pub id: Entity,
}

impl IdleEvent {
    pub fn new(id: Entity) -> Self {
        Self { id }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MoveEvent {
    pub id: Entity,
    pub team: Team,
    pub from: IVec2,
    pub to: IVec2,
}

#[derive(Debug, Copy, Clone)]
pub struct PickUpItemEvent {
    pub picker: Entity,
}

impl PickUpItemEvent {
    pub fn new(picker: Entity) -> Self {
        Self { picker }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DropItemEvent {
    pub droper: Entity,
    pub item: Entity,
}

impl DropItemEvent {
    pub fn new(droper: Entity, item: Entity) -> Self {
        Self { droper, item }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CameraFocusEvent {
    pub position: IVec2,
}
impl CameraFocusEvent {
    pub fn new(position: IVec2) -> Self {
        Self { position }
    }
}
