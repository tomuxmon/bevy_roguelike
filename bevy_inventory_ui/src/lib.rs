use bevy::{
    prelude::*,
    utils::{hashbrown::hash_map::Iter, HashMap}, ecs::system::Resource,
};
use bevy_inventory::{Equipment, Inventory, ItemDropEvent, ItemType};
use serde::{Deserialize, Serialize};

pub use assets::SlotAsset;
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

#[derive(Default, Debug, Clone, Component)]
pub struct HoverTip;

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

pub trait ItemTypeUiImage<I: ItemType> : Resource {
    fn get_image(&self, item_type: I) -> UiImage;
}

// pub(crate) fn ui_hovertip_interaction(mut interactive_hovertip: Query<(&Interaction, &HoverTip)>) {
//     for (i, _) in interactive_hovertip.iter_mut() {
//         //
//         if *i == Interaction::Hovered {
//             // TODO: draw gui/tooltip/cursor.png
//         }
//         // else undraw
//     }
// }

pub(crate) fn ui_click_item_equip<I: ItemType>(
    interactive_equipables: Query<(&Interaction, &Equipable)>,
    mut actors: Query<(&mut Inventory, &mut Equipment<I>)>,
    items: Query<&I>,
) {
    for (interaction, equipable) in interactive_equipables.iter() {
        if *interaction == Interaction::Clicked {
            if let Ok((mut inventory, mut equipment)) = actors.get_mut(equipable.actor) {
                let item_type = if let Ok(item_type) = items.get(equipable.item) {
                    item_type
                } else {
                    bevy::log::error!("item with no type");
                    continue;
                };
                if inventory.take(equipable.item) {
                    if !equipment.add(equipable.item, item_type) {
                        inventory.add(equipable.item);
                        bevy::log::info!("could not equip item placing back into inventory");
                    }
                } else {
                    bevy::log::error!("Equipable Item not in inventory.");
                }
            }
        }
    }
}

pub(crate) fn ui_click_item_unequip<I: ItemType>(
    interactive_unequipables: Query<(&Interaction, &Unequipable)>,
    mut actors: Query<(&mut Inventory, &mut Equipment<I>)>,
    mut drop_writer: EventWriter<ItemDropEvent>,
) {
    for (interaction, unequipable) in interactive_unequipables.iter() {
        if *interaction == Interaction::Clicked {
            if let Ok((mut inventory, mut equipment)) = actors.get_mut(unequipable.actor) {
                if equipment.take(unequipable.item) {
                    if !inventory.add(unequipable.item) {
                        drop_writer.send(ItemDropEvent {
                            droper: unequipable.actor,
                            item: unequipable.item,
                        });
                        bevy::log::info!(
                            "could not place unequiped item into inventory. dropping it."
                        );
                    }
                } else {
                    bevy::log::error!("Unequipable Item not in Equipment.");
                }
            }
        }
    }
}
