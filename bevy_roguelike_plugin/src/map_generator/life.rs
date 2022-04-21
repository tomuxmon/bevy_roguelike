use super::prelude::*;
use bevy::math::IVec2;

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
    fn gen(&self, rng: &mut StdRng, size: IVec2) -> (Map, MapInfo) {
        let mut map = Map::random_noise(size, rng);

        for _ in 0..self.iter_count {
            iteration(&mut map);
        }
        let floor = iteration(&mut map);

        for p in map.get_edge() {
            let t = &mut map[p];
            *t = Tile::Wall;
        }

        let pidx = rng.gen_range(0..floor.len());
        let monster_count = floor.len() / 16;
        let player_start = floor[pidx];
        let mut monster_spawns = Vec::new();
        while monster_spawns.len() < monster_count {
            let midx = rng.gen_range(0..floor.len());
            let pt = floor[midx];
            if midx != pidx && !monster_spawns.contains(&pt) {
                monster_spawns.push(pt);
            }
        }
        // TODO: populate room centers
        let info = MapInfo::new(player_start, Vec::new(), monster_spawns);

        (map, info)
    }
}

fn iteration(map: &mut Map) -> Vec<IVec2> {
    let mut pts = Vec::new();
    let map_clone = map.clone();
    for y in 1..map_clone.size().y - 1 {
        for x in 1..map_clone.size().x - 1 {
            let pt = IVec2::new(x, y);
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

fn count_neighbors(map: &Map, pt: IVec2) -> usize {
    Map::get_neighbor_deltas()
        .map(|nb| pt + nb)
        .iter()
        .filter(|nb| map.is_in_bounds(**nb) && map[**nb] == Tile::Wall)
        .count()
}
