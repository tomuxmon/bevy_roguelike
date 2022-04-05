use super::prelude::*;

const DEFAULT_ITER_COUNT: usize = 9;

pub struct AutomataGenerator {
    iter_count: usize,
}

impl Default for AutomataGenerator {
    fn default() -> Self {
        Self {
            iter_count: DEFAULT_ITER_COUNT,
        }
    }
}

impl MapGenerator for AutomataGenerator {
    fn gen(&self, rng: &mut StdRng, size: Vector2D) -> (Map, MapInfo) {
        let mut map = Map::random_noise(size, rng);

        for _ in 0..self.iter_count {
            iteration(&mut map);
        }
        let floor = iteration(&mut map);

        for p in map.get_edge() {
            let t = &mut map[p];
            *t = Tile::Wall;
        }

        let player_start = floor[rng.gen_range(0..floor.len())];
        // TODO: populate room centers
        let info = MapInfo::new(player_start, &Vec::new());

        (map, info)
    }
}

fn iteration(map: &mut Map) -> Vec<Vector2D> {
    let mut pts = Vec::new();
    let map_clone = map.clone();
    for y in 1..map_clone.size.y() - 1 {
        for x in 1..map_clone.size.x() - 1 {
            let pt = Vector2D::new(x, y);
            let neighbors = count_neighbors(&map_clone, pt);
            let tile = if neighbors > 4 || neighbors == 0 {
                pts.push(pt);
                Tile::Floor
            } else {
                Tile::Wall
            };
            let t = &mut map[pt];
            *t = tile;
        }
    }
    pts
}

fn count_neighbors(map: &Map, pt: Vector2D) -> usize {
    Map::get_neighbor_deltas()
        .map(|nb| pt + nb)
        .iter()
        .filter(|nb| map.is_in_bounds(**nb) && map[**nb] == Tile::Wall)
        .count()
}
