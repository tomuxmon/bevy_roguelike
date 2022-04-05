use super::prelude::*;

pub struct EmptyGenerator {}

impl MapGenerator for EmptyGenerator {
    fn gen(&self, rng: &mut StdRng, size: Vector2D) -> (Map, MapInfo) {
        (
            Map::filled_with(size, Tile::Floor),
            MapInfo::new(
                Vector2D::new(rng.gen_range(0..size.x()), rng.gen_range(0..size.y())),
                &vec![Vector2D::new(size.x() / 2, size.y() / 2)],
            ),
        )
    }
}
