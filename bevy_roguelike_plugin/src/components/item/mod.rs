use bevy::prelude::*;
use bevy_inventory::ItemType;
pub use from_template::spawn_item;
pub use quality::MutableQuality;
pub use quality::Quality;
use serde::Deserialize;
use serde::Serialize;

mod from_template;
mod quality;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Component)]
pub enum RogueItemType {
    MainHand,
    OffHand,
    Head,
    Neck,
    Body,
    Feet,
    Finger,
}
impl Default for RogueItemType {
    fn default() -> Self {
        Self::MainHand
    }
}
impl ItemType for RogueItemType {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
pub struct EquipedRendition {
    // Entity id of the item rendition
    pub item_render_entity: Entity,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
pub struct EquipedRenderedItem {
    /// Entity id of the item
    pub item: Entity,
}

#[derive(Debug, Component)]
pub struct ItemEquipedOwned {
    /// Entity id of the actor that has that item equiped
    pub actor: Entity,
}
