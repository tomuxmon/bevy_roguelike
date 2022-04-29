use super::prelude::*;

const DEFAULT_ITER_COUNT: usize = 9;

pub struct ConwayLifeGenerator {
    iter_count: usize,
}

impl Default for ConwayLifeGenerator {
    fn default() -> Self {
        Self {
            iter_count: DEFAULT_ITER_COUNT,
        }
    }
}

impl MapGenerator for ConwayLifeGenerator {
    fn gen(&self, rng: &mut StdRng, size: IVec2) -> Map {
        let mut map = Map::random_noise(size, rng);
        for _ in 0..=self.iter_count {
            iteration(&mut map);
        }
        for p in map.get_edge() {
            let t = &mut map[p];
            *t = Tile::Wall;
        }
        map
    }
}

fn iteration(map: &mut Map) {
    let map_clone = map.clone();
    for y in 1..map_clone.size().y - 1 {
        for x in 1..map_clone.size().x - 1 {
            let pt = IVec2::new(x, y);
            let neighbors = count_neighbors(&map_clone, pt);
            let t = &mut map[pt];
            *t = if neighbors > 4 || neighbors == 0 {
                Tile::Floor
            } else {
                Tile::Wall
            };
        }
    }
}

fn count_neighbors(map: &Map, pt: IVec2) -> usize {
    Map::get_neighbor_deltas()
        .map(|nb| pt + nb)
        .iter()
        .filter(|nb| map.is_in_bounds(**nb) && map[**nb] == Tile::Wall)
        .count()
}
