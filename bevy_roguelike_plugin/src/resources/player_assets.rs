use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct PlayerAssets {
    pub body: Handle<Image>,
    pub wear: Vec<Handle<Image>>,
}
