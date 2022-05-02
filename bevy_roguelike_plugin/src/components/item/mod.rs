use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, utils::HashSet};

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Item;

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Equiped;




#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    items: HashSet<Entity>,
}
impl Deref for Inventory {
    type Target = HashSet<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl DerefMut for Inventory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
