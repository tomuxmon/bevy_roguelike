use super::ItemType;
use crate::components::Vector2D;
use bevy::{
    prelude::*,
    utils::{hashbrown::hash_map::Iter, HashMap},
};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Equipment {
    items: HashMap<(ItemType, u8), Option<Entity>>,
}
impl Equipment {
    pub fn list<T>(&self, t_items: &Query<&T, (With<ItemType>, Without<Vector2D>)>) -> Vec<T>
    where
        T: Component + Clone,
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
    pub fn iter<'s, T>(
        &'s self,
        t_items: &'s Query<'_, 's, &'s T, (With<ItemType>, Without<Vector2D>)>,
    ) -> impl Iterator<Item = &'s T> + 's
    where
        T: Component,
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

    pub fn iter_some<'a>(&'a self) -> impl Iterator<Item = ((ItemType, u8), Entity)> + 'a {
        self.items
            .iter()
            .filter(|(_, i)| i.is_some())
            .map(move |(a, i)| (*a, i.unwrap()))
    }
}
impl Default for Equipment {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}
impl From<&EquipmentDisplay> for Equipment {
    fn from(display: &EquipmentDisplay) -> Self {
        let mut items = HashMap::default();
        for (t, _) in display.items.iter() {
            items.entry(*t).insert(None);
        }
        Self { items }
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

/// equipment display locations in 128 height x 256 width canvas
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct EquipmentDisplay {
    items: HashMap<(ItemType, u8), Rect<Val>>,
}
impl EquipmentDisplay {
    pub fn new(list: Vec<(ItemType, u8, Rect<Val>)>) -> Self {
        let mut items = HashMap::default();
        for (t, i, r) in list {
            items.entry((t, i)).insert(r);
        }
        Self { items }
    }
    pub fn iter(&self) -> Iter<(ItemType, u8), Rect<Val>> {
        self.items.iter()
    }
}
impl Default for EquipmentDisplay {
    fn default() -> Self {
        EquipmentDisplay::new(vec![(
            ItemType::MainHand,
            0,
            Rect {
                top: Val::Px(58.),
                left: Val::Px(72.),
                ..default()
            },
        )])
    }
}

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
        self.items
            .iter()
            .filter(|i| i.is_some())
            .map(move |i| i.unwrap())
    }

    pub fn len(&self) -> usize {
        self.items.len()
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
