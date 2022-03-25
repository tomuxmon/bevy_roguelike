mod automata;
mod drunkard;
mod empty;

mod prelude {
    pub use super::MapGenerator;
    pub use crate::components::Vector2D;
    pub use crate::resources::map::{Map, MapInfo};
    pub use crate::resources::tile::Tile;
    pub use bevy::log;
    pub use rand::prelude::*;
}

use crate::{
    components::Vector2D,
    resources::map::{Map, MapInfo},
};
use automata::AutomataGenerator;
use drunkard::DrunkardGenerator;
use empty::EmptyGenerator;
use rand::prelude::*;

pub trait MapGenerator {
    fn gen(&self, rng: &mut StdRng, size: Vector2D) -> (Map, MapInfo);
}

pub struct RandomMapGenerator {}

impl MapGenerator for RandomMapGenerator {
    fn gen(&self, rng: &mut StdRng, size: Vector2D) -> (Map, MapInfo) {
        let generator: Box<dyn MapGenerator> = match rng.gen_range(0..2) {
            0 => Box::new(AutomataGenerator {}),
            1 => Box::new(DrunkardGenerator::default()),
            // TODO: rooms
            _ => Box::new(EmptyGenerator {}),
        };
        generator.gen(rng, size)
    }
}
