use super::RenderInfo;
use bevy::prelude::*;
use std::borrow::Cow;
use std::iter::Sum;

pub use inventory::Equipment;
pub use inventory::EquipmentDisplay;
pub use inventory::Inventory;
pub use inventory::InventoryDisplay;

mod inventory;

#[derive(Bundle)]
pub struct AttackItem {
    item_type: ItemType,
    name: Name,
    attack: AttackBoost,
    render_info: RenderInfo,
}

impl AttackItem {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        item_type: ItemType,
        attack: AttackBoost,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            item_type,
            name: Name::new(name),
            attack,
            render_info: RenderInfo { texture, z: 1. },
        }
    }
}

#[derive(Bundle)]
pub struct DefenseItem {
    item_type: ItemType,
    name: Name,
    attack: DefenseBoost,
    render_info: RenderInfo,
}
impl DefenseItem {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        item_type: ItemType,
        attack: DefenseBoost,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            item_type,
            name: Name::new(name),
            attack,
            render_info: RenderInfo { texture, z: 1. },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
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

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct AttackBoost {
    damage: i16,
    rate: i16,
    cost: i16,
}
impl AttackBoost {
    pub fn new(damage: i16, rate: i16, cost: i16) -> Self {
        Self { damage, rate, cost }
    }
    pub fn damage(&self) -> i16 {
        self.damage
    }
    pub fn rate(&self) -> i16 {
        self.rate
    }
    pub fn cost(&self) -> i16 {
        self.cost
    }
}
impl<'a> Sum<&'a Self> for AttackBoost {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(AttackBoost::new(0, 0, 0), |a, b| {
            Self::new(a.damage + b.damage, a.rate + b.rate, a.cost + b.cost)
        })
    }
}

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct DefenseBoost {
    absorb: i16,
    rate: i16,
    cost: i16,
}
impl DefenseBoost {
    pub fn new(absorb: i16, rate: i16, cost: i16) -> Self {
        Self { absorb, rate, cost }
    }
    pub fn absorb(&self) -> i16 {
        self.absorb
    }
    pub fn rate(&self) -> i16 {
        self.rate
    }
    pub fn cost(&self) -> i16 {
        self.cost
    }
}
impl<'a> Sum<&'a Self> for DefenseBoost {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(DefenseBoost::new(0, 0, 0), |a, b| {
            Self::new(a.absorb + b.absorb, a.rate + b.rate, a.cost + b.cost)
        })
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct ItemDisplaySlot {
    index: usize,
    pub item: Option<Entity>,
}

impl ItemDisplaySlot {
    pub fn new(index: usize) -> Self {
        Self { index, item: None }
    }
    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct ItemEquipSlot {
    index: (ItemType, u8),
    pub item: Option<Entity>,
    pub is_dummy_rendered: bool,
}
impl ItemEquipSlot {
    pub fn new(index: (ItemType, u8)) -> Self {
        Self {
            index,
            item: None,
            is_dummy_rendered: false,
        }
    }
    pub fn index(&self) -> (ItemType, u8) {
        self.index
    }
}
