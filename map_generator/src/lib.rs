mod drunkard;
mod empty;
mod life;
mod map;
mod rect;
mod rooms;

mod prelude {
    pub use super::map::Map;
    pub use super::map::Tile;
    pub use super::rect::Rect;
    pub use super::MapGenerator;
    pub use glam::IVec2;
    pub use line_drawing::WalkGrid;
    pub use rand::prelude::*;
}

pub use drunkard::DrunkardGenerator;
pub use empty::EmptyGenerator;
pub use life::ConwayLifeGenerator;
pub use map::Map;
pub use map::Tile;
pub use rooms::RoomsGenerator;

use prelude::*;

pub trait MapGenerator {
    fn gen(&self, rng: &mut StdRng, size: IVec2) -> Map;
}

// TODO: implement possibility to do map generation composition (mix multiple generators)
// TODO: implement space straversal and room detector
// TODO: dijkstra plotting to determine distinct not connected rooms
// TODO: dijkstra plotting to place enemies, players at positions relative to wall sides
// TODO: dijkstra plotting to identify 1 tile wide tunnels

pub struct RandomMapGenerator {}

impl MapGenerator for RandomMapGenerator {
    fn gen(&self, rng: &mut StdRng, size: IVec2) -> Map {
        let generator: Box<dyn MapGenerator> = match rng.gen_range(1..4) {
            0 => Box::new(ConwayLifeGenerator::default()),
            1 => Box::new(DrunkardGenerator::default()),
            2 => Box::new(RoomsGenerator::default()),
            _ => Box::new(EmptyGenerator {}),
        };
        generator.gen(rng, size)
    }
}
