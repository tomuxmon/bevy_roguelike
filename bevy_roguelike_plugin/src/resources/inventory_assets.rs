use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct InventoryAssets {
    pub slot: Handle<Image>,
    pub slot_equiped: Handle<Image>,
}
