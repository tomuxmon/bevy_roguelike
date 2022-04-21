use std::ops::{Deref, DerefMut};

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct VisibilityInfo {
    pub is_revealed: bool,
    pub is_visible: bool,
    pub is_ambient: bool,
}
impl VisibilityInfo {
    pub fn new(is_revealed: bool, is_visible: bool, is_ambient: bool) -> Self {
        Self {
            is_revealed,
            is_visible,
            is_ambient,
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
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
    pub fn new(radius: i32) -> Self {
        Self {
            radius,
            tiles_visible: HashSet::default(),
            tiles_revealed: HashSet::default(),
            is_dirty: true,
        }
    }
}
