use bevy::prelude::Resource;
use map_generator::Map;
use std::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct RogueMap(pub Map);

impl Deref for RogueMap {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RogueMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
