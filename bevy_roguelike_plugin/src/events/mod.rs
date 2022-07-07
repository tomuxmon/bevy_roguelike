use crate::components::Team;
use bevy::prelude::*;

// TODO: turn into Act component with is_dirty or is_used
#[derive(Debug, Copy, Clone)]
pub struct ActEvent {
    pub id: Entity,
    pub delta: IVec2,
}

#[derive(Debug, Copy, Clone)]
pub struct MoveEvent {
    pub actor: Entity,
    pub team: Team,
    pub from: IVec2,
    pub to: IVec2,
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
