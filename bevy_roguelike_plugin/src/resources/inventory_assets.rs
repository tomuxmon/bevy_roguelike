use crate::components::RogueItemType;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_inventory_ui::ItemTypeUiImage;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, TypeUuid)]
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

#[derive(Resource, Debug, Clone)]
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

impl ItemTypeUiImage<RogueItemType> for InventoryAssets {
    fn get_image(&self, item_type: RogueItemType) -> UiImage {
        match item_type {
            RogueItemType::MainHand => self.main_hand_gear.clone(),
            RogueItemType::OffHand => self.off_hand_gear.clone(),
            RogueItemType::Head => self.head_wear.clone(),
            RogueItemType::Neck => self.neck_wear.clone(),
            RogueItemType::Body => self.body_wear.clone(),
            RogueItemType::Feet => self.feet_wear.clone(),
            RogueItemType::Finger => self.finger_wear.clone(),
        }
        .into()
    }
}
