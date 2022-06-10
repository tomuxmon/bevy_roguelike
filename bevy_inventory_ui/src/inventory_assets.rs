use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct InventoryAssets {
    pub slot: Handle<Image>,
    pub head_wear: Handle<Image>,
    pub body_wear: Handle<Image>,
    pub main_hand_gear: Handle<Image>,
    pub off_hand_gear: Handle<Image>,
    pub finger_wear: Handle<Image>,
    pub neck_wear: Handle<Image>,
    pub feet_wear: Handle<Image>,
}
