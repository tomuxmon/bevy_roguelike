use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct InventoryAssets {
    pub slot: Handle<Image>,
    pub slot_head_wear: Handle<Image>,
    pub slot_body_wear: Handle<Image>,
    pub slot_main_hand_gear: Handle<Image>,
    pub slot_off_hand_gear: Handle<Image>,
    pub slot_finger_wear: Handle<Image>,
    pub slot_neck_wear: Handle<Image>,
    pub slot_feet_wear: Handle<Image>,
}
