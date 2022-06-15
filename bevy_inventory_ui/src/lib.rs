use bevy::{
    ecs::system::Resource,
    prelude::*,
    utils::{hashbrown::hash_map::Iter, HashMap},
};
use bevy_inventory::ItemType;
use serde::{Deserialize, Serialize};

pub use assets::InventoryUiAssets;
pub use plugin::InventoryUiPlugin;

mod assets;
mod draggable_ui;
mod plugin;
mod systems;

/// Inventory display options. Must be used as a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryDisplayOptions {
    /// size of item
    pub tile_size: f32,
}

/// equipment display locations in 128 height x 256 width canvas
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct EquipmentDisplay<I: ItemType> {
    pub items: HashMap<(I, u8), Vec2>,
}
impl<I: ItemType> EquipmentDisplay<I> {
    pub fn new(list: Vec<(I, u8, Vec2)>) -> Self {
        let mut items = HashMap::default();
        for (t, i, r) in list {
            items.entry((t, i)).insert(r);
        }
        Self { items }
    }
    pub fn iter(&self) -> Iter<(I, u8), Vec2> {
        self.items.iter()
    }
}
impl<I: ItemType> Default for EquipmentDisplay<I> {
    fn default() -> Self {
        EquipmentDisplay::new(vec![(I::default(), 0, Vec2::new(72., 58.))])
    }
}

/// specifies the owner of the inventory and equipment UI
#[derive(Debug, Clone, Component)]
pub struct InventoryDisplayOwner {
    pub id: Entity,
}

/// Specifies the node containing children of InventoryDisplaySlot
#[derive(Debug, Clone, Component)]
pub struct InventoryDisplayNode {
    /// Entity id of the actor having this inventory
    pub id: Entity,
}
/// Specifies the node containing children of EquipmentDisplaySlot
#[derive(Debug, Clone, Component)]
pub struct EquipmentDisplayNode {
    /// Entity id of the actor having this Equipment
    pub actor: Entity,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Component)]
pub struct InventoryDisplaySlot {
    pub index: usize,
    pub item: Option<Entity>,
}

#[derive(Default, Debug, Clone, Component)]
pub struct EquipmentDisplaySlot<I: ItemType> {
    pub index: (I, u8),
    pub item: Option<Entity>,
    pub is_dummy_rendered: bool,
}

/// specifies how to render stuff if it is placed in the inventory disply or equipment disply
#[derive(Default, Debug, Clone, Component)]
pub struct UiRenderInfo {
    pub image: UiImage,
}

#[derive(Debug, Copy, Clone)]
pub struct InventoryDisplayToggleEvent {
    /// Entity ID of the actor wanting to toggle inventory display
    pub actor: Entity,
}

#[derive(Debug, Clone, Component)]
pub struct HoverTip {
    pub(crate) hovered: bool,
    pub(crate) tooltip_shown: bool,
    item_entity: Entity,
}
impl HoverTip {
    pub fn new(item_entity: Entity) -> Self {
        Self {
            hovered: false,
            tooltip_shown: false,
            item_entity,
        }
    }
    pub fn item_entity(&self) -> Entity {
        self.item_entity
    }
}
#[derive(Debug, Clone, Component)]
pub struct ItemUiTextInfo {
    pub name: String,
    pub infos: HashMap<String, String>,
}

// TODO: move to bevy_inventory lib
#[derive(Debug, Clone, Component)]
pub struct Equipable {
    actor: Entity,
    item: Entity,
}
// TODO: move to bevy_inventory lib
#[derive(Debug, Clone, Component)]
pub struct Unequipable {
    actor: Entity,
    item: Entity,
}

pub trait ItemTypeUiImage<I: ItemType>: Resource {
    fn get_image(&self, item_type: I) -> UiImage;
}
