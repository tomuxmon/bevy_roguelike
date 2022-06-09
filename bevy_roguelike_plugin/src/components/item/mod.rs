use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub use from_template::spawn_item;
pub use inventory::Equipment;
pub use inventory::EquipmentDisplay;
pub use inventory::Inventory;
pub use inventory::InventoryDisplay;
pub use quality::MutableQuality;
pub use quality::Quality;

mod from_template;
mod inventory;
mod quality;

#[derive(Reflect, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Component, Debug)]
#[reflect_value(PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
pub struct EquipedOwned {
    /// Entity id of the actor that has that item equiped
    pub id: Entity,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
pub struct EquipedRendition {
    // Entity id of the item rendition
    pub id: Entity,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
pub struct EquipedRenderedItem {
    /// Entity id of the item
    pub id: Entity,
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
