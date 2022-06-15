use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct InventoryUiAssets {
    pub slot: Handle<Image>,
    pub hover_cursor_image: Handle<Image>,
    pub font: Handle<Font>,
}
