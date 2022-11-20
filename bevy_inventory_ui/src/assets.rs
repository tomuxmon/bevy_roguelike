use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct InventoryUiAssets {
    pub slot: Handle<Image>,
    pub hover_cursor_image: Handle<Image>,
    pub font: Handle<Font>,
}
