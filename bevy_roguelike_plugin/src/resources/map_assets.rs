use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct MapAssets {
    pub floor: Vec<Handle<Image>>,
    pub wall: Vec<Handle<Image>>,
}
