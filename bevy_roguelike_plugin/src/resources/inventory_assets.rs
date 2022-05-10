use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct InventoryAssets {
    pub slot: Handle<Image>,
    pub slot_head: Handle<Image>,
    pub slot_body: Handle<Image>,
    pub slot_main_hand: Handle<Image>,
    pub slot_off_hand: Handle<Image>,
    pub slot_finger: Handle<Image>,
    pub slot_neck: Handle<Image>,
    pub slot_feet: Handle<Image>,
}
