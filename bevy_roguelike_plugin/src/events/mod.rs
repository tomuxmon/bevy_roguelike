use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct ActEvent {
    pub id: Entity,
    pub delta: IVec2,
}

// TODO: turn into Act component with is_dirty or is_used
impl ActEvent {
    pub fn new(id: Entity, delta: IVec2) -> Self {
        Self { id, delta }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
}
impl AttackEvent {
    pub fn new(attacker: Entity, defender: Entity) -> Self {
        Self { attacker, defender }
    }
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
    pub destination: IVec2,
}

impl MoveEvent {
    pub fn new(id: Entity, destination: IVec2) -> Self {
        Self { id, destination }
    }
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
