use super::Attributes;
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct VisibilityInfo {
    pub is_revealed: bool,
    pub is_visible: bool,
    pub is_ambient: bool,
    pub hp_percent: f32,
}
impl VisibilityInfo {
    pub fn new(is_revealed: bool, is_visible: bool, is_ambient: bool, hp_percent: f32) -> Self {
        Self {
            is_revealed,
            is_visible,
            is_ambient,
            hp_percent,
        }
    }
    pub fn is_damaged(&self) -> bool {
        self.hp_percent != 1.
    }
}

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct VisibilityToggle {
    inner: HashMap<Entity, VisibilityInfo>,
}
impl Deref for VisibilityToggle {
    type Target = HashMap<Entity, VisibilityInfo>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for VisibilityToggle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

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
