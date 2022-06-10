use bevy::{
    prelude::*,
    utils::{hashbrown::hash_map::Iter, HashMap},
};
use bevy_inventory::ItemType;
use serde::{Deserialize, Serialize};

pub use draggable_ui::ui_apply_drag_pos;
pub use draggable_ui::ui_drag_interaction;
pub use inventory_assets::InventoryAssets;
pub use systems::equipment_update;
pub use systems::inventory_update;
pub use systems::toggle_inventory_open;

mod draggable_ui;
mod inventory_assets;
mod systems;

/// Inventory display options. Must be used as a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryDisplayOptions {
    /// size of item
    pub tile_size: f32,
}

/// equipment display locations in 128 height x 256 width canvas
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EquipmentDisplay {
    pub items: HashMap<(ItemType, u8), Vec2>,
}
impl EquipmentDisplay {
    pub fn new(list: Vec<(ItemType, u8, Vec2)>) -> Self {
        let mut items = HashMap::default();
        for (t, i, r) in list {
            items.entry((t, i)).insert(r);
        }
        Self { items }
    }
    pub fn iter(&self) -> Iter<(ItemType, u8), Vec2> {
        self.items.iter()
    }
}
impl Default for EquipmentDisplay {
    fn default() -> Self {
        EquipmentDisplay::new(vec![(ItemType::MainHand, 0, Vec2::new(72., 58.))])
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
    pub id: Entity,
}


#[derive(Default, Debug, PartialEq, Eq, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct InventoryDisplaySlot {
    pub index: usize,
    pub item: Option<Entity>,
}

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct EquipmentDisplaySlot {
    pub index: (ItemType, u8),
    pub item: Option<Entity>,
    pub is_dummy_rendered: bool,
}

/// specifies how to render stuff if it is placed in the inventory disply or equipment disply
#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct UiRenderInfo {
    pub image: UiImage,
}

#[derive(Debug, Copy, Clone)]
pub struct InventoryDisplayToggleEvent {
    /// Entity ID of the actor wanting to toggle inventory display
    pub id: Entity,
}
