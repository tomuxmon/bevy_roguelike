use super::prelude::*;
use bevy::math::IVec2;

pub struct EmptyGenerator {}

impl MapGenerator for EmptyGenerator {
    fn gen(&self, rng: &mut StdRng, size: IVec2) -> (Map, MapInfo) {
        (
            Map::filled_with(size, Tile::Floor),
            MapInfo::new(
                IVec2::new(rng.gen_range(0..size.x), rng.gen_range(0..size.y)),
                vec![IVec2::new(size.x / 2, size.y / 2)],
                vec![],
            ),
        )
    }
}
