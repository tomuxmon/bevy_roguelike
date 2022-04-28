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
pub struct CameraFocusEvent {
    pub position: IVec2,
}
impl CameraFocusEvent {
    pub fn new(position: IVec2) -> Self {
        Self { position }
    }
}
