use super::prelude::*;

const DEFAULT_R0OM_SIZE_RATIO: f32 = 0.29;
const DEFAULT_ROOM_COUNT_RATIO: f32 = 0.005;

pub struct RoomsGenerator {
    pub room_size_ratio: f32,
    pub room_count_ratio: f32,
}
impl Default for RoomsGenerator {
    fn default() -> Self {
        Self {
            room_size_ratio: DEFAULT_R0OM_SIZE_RATIO,
            room_count_ratio: DEFAULT_ROOM_COUNT_RATIO,
        }
    }
}

impl MapGenerator for RoomsGenerator {
    fn gen(&self, rng: &mut StdRng, size: Vector2D) -> (Map, MapInfo) {
        let mut map = Map::filled_with(size, Tile::Wall);
        let room_size_max = size * self.room_size_ratio;
        let room_count = ((size.x() * size.y()) as f32 * self.room_count_ratio) as usize;

        let rooms = build_rooms(rng, size, room_size_max, room_count);
        for room in rooms.iter() {
            room.for_each(|pt| {
                if map.is_in_bounds(pt) {
                    let tile = &mut map[pt];
                    *tile = Tile::Floor;
                }
            });
        }
        carve_coriddors(&mut map, rooms.clone(), rng);

        let room_centers: Vec<Vector2D> = rooms.iter().map(|r| r.get_center()).collect();
        let info = MapInfo::new(
            room_centers[0],
            room_centers.clone(),
            room_centers[1..room_centers.len()].to_vec(),
        );
        (map, info)
    }
}

fn carve_coriddors(map: &mut Map, rooms: Vec<Rect>, rng: &mut StdRng) {
    let mut rooms = rooms.clone();
    rooms.sort_by(|a, b| a.get_center().y().cmp(&b.get_center().y()));
    for (i, room) in rooms.iter().enumerate().skip(1) {
        let prev = rooms[i - 1].get_center();
        let new = room.get_center();
        for pt in get_converging_carv_points(map, rng, prev, new) {
            let tile = &mut map[pt];
            *tile = Tile::Floor;
        }
    }
}

fn get_converging_carv_points(
    map: &Map,
    rng: &mut StdRng,
    c1: Vector2D,
    c2: Vector2D,
) -> Vec<Vector2D> {
    let mut path = Vec::new();
    let mut pt = c1;
    // even distribution of deltas
    let mut deltas = Vec::new();
    deltas.push(Vector2D::new(1, 0));
    deltas.push(Vector2D::new(-1, 0));
    deltas.push(Vector2D::new(0, 1));
    deltas.push(Vector2D::new(0, -1));

    let mut distance = pt.distance_pow2(c2);

    while pt != c2 {
        let distance_last = distance;
        let delta = deltas[rng.gen_range(0..deltas.len())];
        if !map.is_in_bounds(pt + delta)
            || map.is_edge(pt + delta)
            || (pt + delta).distance_pow2(c2) > distance_last
        {
            continue;
        }
        pt = pt + delta;
        distance = pt.distance_pow2(c2);
        path.push(pt);
    }
    path
}

fn build_rooms(
    rng: &mut StdRng,
    map_size: Vector2D,
    room_size_max: Vector2D,
    room_count: usize,
) -> Vec<Rect> {
    let mut rooms: Vec<Rect> = Vec::new();
    let mut i = 10000;
    while rooms.len() < room_count && i > 1 {
        i -= 1;
        let room_size = Vector2D::new(
            rng.gen_range(i32::max(room_size_max.x() / 4, 3)..room_size_max.x()),
            rng.gen_range(i32::max(room_size_max.y() / 4, 3)..room_size_max.y()),
        );
        let room = Rect::new(
            Vector2D::new(
                rng.gen_range(1..map_size.x() - room_size.x()),
                rng.gen_range(1..map_size.y() - room_size.y()),
            ),
            room_size,
        );
        let mut overlap = false;
        for r in rooms.iter() {
            if r.intersect_or_touch(room) {
                overlap = true;
            }
        }
        if !overlap {
            rooms.push(room);
        }
    }
    rooms
}
