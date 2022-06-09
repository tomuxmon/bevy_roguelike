use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "ea186dbc-8eb4-48c5-b1c3-40b980b5da8a"]
pub struct InventoryTheme {
    pub slot: String,
    pub head_wear: String,
    pub body_wear: String,
    pub main_hand_gear: String,
    pub off_hand_gear: String,
    pub finger_wear: String,
    pub neck_wear: String,
    pub feet_wear: String,
}
