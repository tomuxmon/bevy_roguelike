use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, utils::HashMap};

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum Behaviour {
    InputControlled,
    RandomMove,
}
impl Default for Behaviour {
    fn default() -> Self {
        Self::RandomMove
    }
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Team {
    id: u32,
}

impl Team {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct Attributes {
    inner: HashMap<String, i32>,
}
impl Attributes {
    pub fn new(attribs: impl IntoIterator<Item = (String, i32)>) -> Self {
        Self {
            inner: HashMap::from_iter(attribs),
        }
    }
}
impl Deref for Attributes {
    type Target = HashMap<String, i32>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Attributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}