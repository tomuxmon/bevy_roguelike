use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct SlotAsset {
    pub slot: Handle<Image>,
}

#[derive(Debug, Clone)]
pub struct HoverCursorAsset {
    pub image: Handle<Image>,
}
