use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct ItemPickUpEvent {
    pub picker: Entity,
}

#[derive(Debug, Copy, Clone)]
pub struct ItemDropEvent {
    pub droper: Entity,
    pub item: Entity,
}
