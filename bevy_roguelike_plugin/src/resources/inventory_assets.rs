use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

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
