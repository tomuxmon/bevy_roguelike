use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct PlayerAssets {
    pub body: Handle<Image>,
    pub decals: Vec<Handle<Image>>,
}
