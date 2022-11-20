use bevy::prelude::Resource;
use rand::rngs::StdRng;
use std::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct RogueRng(pub StdRng);

impl Deref for RogueRng {
    type Target = StdRng;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RogueRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
