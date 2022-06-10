use bevy::prelude::*;
pub use from_template::spawn_item;
pub use quality::MutableQuality;
pub use quality::Quality;

mod from_template;
mod quality;

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
