use crate::components::Vector2D;
use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct ModifyHPEvent {
    pub id: Entity,
    pub amount: i32,
}

impl ModifyHPEvent {
    pub fn new(id: Entity, amount: i32) -> Self {
        Self { id, amount }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SpendAPEvent {
    pub id: Entity,
    pub amount: u32,
}

impl SpendAPEvent {
    pub fn new(id: Entity, amount: u32) -> Self {
        Self { id, amount }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CameraFocusEvent {
    pub position: Vector2D,
}
impl CameraFocusEvent {
    pub fn new(position: Vector2D) -> Self {
        Self { position }
    }
}
