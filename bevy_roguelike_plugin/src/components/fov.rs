use super::Attributes;
use bevy::{prelude::*, utils::HashSet};

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct VisibilityToggle;

#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct FieldOfView {
    pub radius: i32,
    pub tiles_visible: HashSet<IVec2>,
    pub tiles_revealed: HashSet<IVec2>,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub const MIN_RADIUS: i32 = 2;

    pub fn new(atr: &Attributes) -> Self {
        Self {
            radius: FieldOfView::MIN_RADIUS
                + (atr.perception as f32 / 3. + atr.inteligence as f32 / 10.) as i32,
            tiles_visible: HashSet::default(),
            tiles_revealed: HashSet::default(),
            is_dirty: true,
        }
    }
    pub fn update(&mut self, atr: &Attributes) {
        self.radius = FieldOfView::MIN_RADIUS
            + (atr.perception as f32 / 3. + atr.inteligence as f32 / 10.) as i32;
        self.is_dirty = true;
    }
}
