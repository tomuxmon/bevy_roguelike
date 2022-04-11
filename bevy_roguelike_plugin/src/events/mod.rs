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
