use bevy::prelude::*;
pub use from_template::spawn_item;
pub use quality::MutableQuality;
pub use quality::Quality;

mod from_template;
mod quality;

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
