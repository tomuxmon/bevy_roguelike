use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

pub use events::*;

mod events;

// TODO: make ItemType generic (ability to define item type outside of lib scope. in user code)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Component)]
pub enum ItemType {
    MainHand,
    OffHand,
    Head,
    Neck,
    Body,
    Feet,
    Finger,
}
impl Default for ItemType {
    fn default() -> Self {
        Self::MainHand
    }
}

#[derive(Debug, Default, Clone, Component)]
pub struct Equipment {
    pub items: HashMap<(ItemType, u8), Option<Entity>>,
}
impl Equipment {
    pub fn list<T, V>(&self, t_items: &Query<&T, (With<ItemType>, Without<V>)>) -> Vec<T>
    where
        T: Component + Clone,
        V: Component,
    {
        self.iter_some()
            .filter_map(|(_, e)| {
                if let Ok(t) = t_items.get(e) {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    /// OMFG what a signature :| also virtually imposible to use since lifetime polution goes up the call stack
    pub fn iter<'s, T, V>(
        &'s self,
        t_items: &'s Query<'_, 's, &'s T, (With<ItemType>, Without<V>)>,
    ) -> impl Iterator<Item = &'s T> + 's
    where
        T: Component,
        V: Component,
    {
        self.iter_some().filter_map(|(_, e)| {
            if let Ok(t) = t_items.get(e) {
                Some(t)
            } else {
                None
            }
        })
    }
    // move seems to reduce the lifetime polution problem
    pub fn enumerate_fn<'a, T, F>(&'a self, mut get_component: F) -> impl Iterator<Item = T> + 'a
    where
        T: Component,
        F: FnMut(Entity) -> Option<T> + 'a,
    {
        self.iter_some().filter_map(move |(_, e)| get_component(e))
    }

    pub fn add(&mut self, item: Entity, item_type: &ItemType) -> bool {
        if let Some((_, item_slot)) = self
            .items
            .iter_mut()
            .find(|((t, _), b)| t == item_type && b.is_none())
        {
            *item_slot = Some(item);
            true
        } else {
            false
        }
    }
    // TODO: implement replace add item and return existing one

    pub fn take(&mut self, item: Entity) -> bool {
        if let Some((_, e)) = self
            .items
            .iter_mut()
            .find(|(_, b)| b.is_some() && b.unwrap() == item)
        {
            *e = None;
            true
        } else {
            false
        }
    }

    pub fn iter_some(&'_ self) -> impl Iterator<Item = ((ItemType, u8), Entity)> + '_ {
        // TODO: use filter_map instead
        self.items
            .iter()
            .filter(|(_, i)| i.is_some())
            .map(move |(a, i)| (*a, i.unwrap()))
    }
}

impl Index<(ItemType, u8)> for Equipment {
    type Output = Option<Entity>;

    fn index(&self, index: (ItemType, u8)) -> &Self::Output {
        if let Some(item) = self.items.get(&index) {
            return item;
        }
        &None
    }
}

impl IndexMut<(ItemType, u8)> for Equipment {
    fn index_mut(&mut self, index: (ItemType, u8)) -> &mut Self::Output {
        if let Some(ee) = self.items.get_mut(&index) {
            return ee;
        }
        panic!("No item with index {:?}", index);
    }
}

#[derive(Debug, Clone, Component)]
pub struct Inventory {
    items: Vec<Option<Entity>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::with_capacity(Inventory::DEFAULT_CAPACITY)
    }
}

impl Inventory {
    pub const DEFAULT_CAPACITY: usize = 32;

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: vec![None; cap],
        }
    }

    pub fn add(&mut self, item: Entity) -> bool {
        if let Some((_, e)) = self.items.iter_mut().enumerate().find(|(_, b)| b.is_none()) {
            *e = Some(item);
            true
        } else {
            false
        }
    }

    pub fn take(&mut self, item: Entity) -> bool {
        if let Some((_, e)) = self
            .items
            .iter_mut()
            .enumerate()
            .find(|(_, b)| b.is_some() && b.unwrap() == item)
        {
            *e = None;
            true
        } else {
            false
        }
    }
    pub fn iter_some(&self) -> impl Iterator<Item = Entity> + '_ {
        self.items.iter().filter_map(|i| *i)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<usize> for Inventory {
    type Output = Option<Entity>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
