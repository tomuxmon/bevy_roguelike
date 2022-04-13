mod automata;
mod drunkard;
mod empty;
mod rect;
mod rooms;

mod prelude {
    pub use super::rect::Rect;
    pub use super::MapGenerator;
    pub use crate::components::Vector2D;
    pub use crate::resources::{Map, MapInfo, Tile};
    pub use bevy::log;
    pub use rand::prelude::*;
}

use crate::{
    components::Vector2D,
    resources::{Map, MapInfo},
};
pub use automata::AutomataGenerator;
pub use drunkard::DrunkardGenerator;
use empty::EmptyGenerator;
use rand::prelude::*;
pub use rooms::RoomsGenerator;

pub trait MapGenerator {
    fn gen(&self, rng: &mut StdRng, size: Vector2D) -> (Map, MapInfo);
}

// TODO: implement possibility to do map generation composition (mix multiple generators)
// TODO: implement space straversal and room detector
// TODO: dijkstra plotting to determine distinct not connected rooms
// TODO: dijkstra plotting to place enemies, players at positions relative to wall sides
// TODO: dijkstra plotting to identify 1 tile wide tunnels

pub struct RandomMapGenerator {}

impl MapGenerator for RandomMapGenerator {
    fn gen(&self, rng: &mut StdRng, size: Vector2D) -> (Map, MapInfo) {
        let generator: Box<dyn MapGenerator> = match rng.gen_range(0..3) {
            0 => Box::new(AutomataGenerator::default()),
            1 => Box::new(DrunkardGenerator::default()),
            2 => Box::new(RoomsGenerator::default()),
            _ => Box::new(EmptyGenerator {}),
        };
        generator.gen(rng, size)
    }
}
