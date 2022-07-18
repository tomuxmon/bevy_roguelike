use bevy::prelude::*;
use bevy::utils::HashMap;
use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Index, IndexMut},
};

pub use events::*;

mod events;

pub trait ItemType: Component + Copy + Clone + Eq + Hash + Debug + Default {}

#[derive(Debug, Default, Clone, Component)]
pub struct Equipment<I: ItemType> {
    pub items: HashMap<(I, u8), Option<Entity>>,
}

impl<I: ItemType> Equipment<I> {
    pub fn list<T, V>(&self, t_items: &Query<&T, (With<I>, Without<V>)>) -> Vec<T>
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
    pub fn add(&mut self, item: Entity, item_type: &I) -> bool {
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

    pub fn iter_some(&'_ self) -> impl Iterator<Item = ((I, u8), Entity)> + '_ {
        // TODO: use filter_map instead
        self.items
            .iter()
            .filter(|(_, i)| i.is_some())
            .map(move |(a, i)| (*a, i.unwrap()))
    }
}

impl<I: ItemType> Index<(I, u8)> for Equipment<I> {
    type Output = Option<Entity>;

    fn index(&self, index: (I, u8)) -> &Self::Output {
        if let Some(item) = self.items.get(&index) {
            return item;
        }
        &None
    }
}

impl<I: ItemType> IndexMut<(I, u8)> for Equipment<I> {
    fn index_mut(&mut self, index: (I, u8)) -> &mut Self::Output {
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

    pub fn is_full(&self) -> bool {
        self.items.iter().all(|i| i.is_some())
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
