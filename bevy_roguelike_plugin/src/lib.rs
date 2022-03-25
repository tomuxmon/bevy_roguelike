pub mod components;
pub mod map_generator;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use map_generator::{MapGenerator, RandomMapGenerator};
use rand::prelude::*;

use crate::components::Vector2D;

pub struct RoguelikePlugin;

impl Plugin for RoguelikePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_map);
        log::info!("Loaded Roguelike Plugin");
    }
}

impl RoguelikePlugin {
    pub fn create_map() {
        // max u64: 18_446_744_073_709_551_615
        let mut rng = StdRng::seed_from_u64(54155745465);
        let map_generator = RandomMapGenerator {};
        let (map, info) = map_generator.gen(&mut rng, Vector2D::new(95, 95));
        log::info!("{}", map.to_colorized_string());
        log::info!("{}", info.to_colorized_string());
    }
}
