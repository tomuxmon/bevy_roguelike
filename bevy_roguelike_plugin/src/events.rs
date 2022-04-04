use crate::components::Vector2D;
use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct MoveEvent {
    pub id: Entity,
    pub destination: Vector2D,
}

impl MoveEvent {
    pub fn new(id: Entity, destination: Vector2D) -> Self {
        Self { id, destination }
    }
}
