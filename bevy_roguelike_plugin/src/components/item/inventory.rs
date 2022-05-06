use bevy::prelude::*;
use std::ops::Index;

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    items: Vec<Option<Entity>>,
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![None; Inventory::DEFAULT_CAPACITY],
        }
    }
}
impl Inventory {
    pub const DEFAULT_CAPACITY: usize = 32;

    pub fn add(&mut self, item: Entity) -> bool {
        let search =
            if let Some((idx, _)) = self.items.iter().enumerate().find(|(_, b)| b.is_none()) {
                Some(idx)
            } else {
                None
            };

        if let Some(idx) = search {
            self.items[idx] = Some(item);
            true
        } else {
            false
        }
    }

    pub fn take(&mut self, item: Entity) -> Option<Entity> {
        let search = if let Some((idx, item)) = self
            .items
            .iter()
            .enumerate()
            .find(|(_, b)| b.is_some() && b.unwrap() == item)
        {
            Some((idx, *item))
        } else {
            None
        };
        if let Some((idx, item)) = search {
            self.items[idx] = None;
            item
        } else {
            None
        }
    }
    pub fn iter_some(&self) -> impl Iterator<Item = Entity> + '_ {
        self.items
            .iter()
            .filter(|i| i.is_some())
            .map(move |i| i.unwrap())
    }
}
impl Index<usize> for Inventory {
    type Output = Option<Entity>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct InventoryDisplay;
