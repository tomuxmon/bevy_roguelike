use super::prelude::*;

const DEFAULT_WALK_RATIO: f32 = 0.015;
const DEFAULT_FLOOR_RATIO: f32 = 0.6;

pub struct DrunkardGenerator {
    pub walk_ratio: f32,
    pub floor_ratio: f32,
}

impl Default for DrunkardGenerator {
    fn default() -> Self {
        Self {
            walk_ratio: DEFAULT_WALK_RATIO,
            floor_ratio: DEFAULT_FLOOR_RATIO,
        }
    }
}

impl MapGenerator for DrunkardGenerator {
    fn gen(&self, rng: &mut StdRng, size: Vector2D) -> (Map, MapInfo) {
        let mut map = Map::filled_with(size, Tile::Wall);
        let mut room_centers = Vec::new();
        let walk_steps = ((size.x() * size.y()) as f32 * self.walk_ratio) as usize;
        let desired_floor = ((size.x() * size.y()) as f32 * self.floor_ratio) as usize;

        while map.iter().filter(|t| **t == Tile::Floor).count() < desired_floor {
            let mut from = Vector2D::new(
                rng.gen_range(1..map.size.x() - 1),
                rng.gen_range(1..map.size.y() - 1),
            );
            room_centers.push(from);
            let stagger_count = usize::max(walk_steps / 10, 5);
            for _ in 0..stagger_count {
                from = walk(from, walk_steps, rng, &mut map);
            }
        }

        let info = MapInfo {
            player_start: room_centers[rng.gen_range(0..room_centers.len())],
            room_centers,
        };

        (map, info)
    }
}

fn walk(from: Vector2D, max_step: usize, rng: &mut StdRng, map: &mut Map) -> Vector2D {
    let mut pt = from.clone();
    let mut last_valid_pt = from.clone();
    let mut step = 0;
    loop {
        // Carve it!
        let tile = &mut map[pt];
        *tile = Tile::Floor;

        if rng.gen_range(0..max_step) < max_step / 20 {
            last_valid_pt = pt.clone();
        }

        let deltas = Map::get_wasd_neighbor_deltas();
        let delta = deltas[rng.gen_range(0..deltas.len())];
        pt = pt + delta;

        step += 1;
        if map.is_edge(pt) || !map.is_in_bounds(pt) || step > max_step {
            return last_valid_pt;
        }
    }
}
