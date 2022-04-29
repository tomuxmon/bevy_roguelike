use super::prelude::*;

pub struct EmptyGenerator {}

impl MapGenerator for EmptyGenerator {
    fn gen(&self, _rng: &mut StdRng, size: IVec2) -> Map {
        let mut map = Map::filled_with(size, Tile::Floor);
        for p in map.get_edge() {
            let t = &mut map[p];
            *t = Tile::Wall;
        }
        map
    }
}
