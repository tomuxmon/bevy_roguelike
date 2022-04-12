use super::Vector2D;
use bevy::{prelude::*, utils::HashSet};

#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct VisibilityFOV;

#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct FieldOfView {
    pub radius: i32,
    pub visible_tiles: HashSet<Vector2D>,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            radius,
            visible_tiles: HashSet::default(),
            is_dirty: true,
        }
    }
}
