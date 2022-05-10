use super::prelude::*;

const DEFAULT_R0OM_SIZE_RATIO: f32 = 0.19;
const DEFAULT_ROOM_COUNT_RATIO: f32 = 0.017;

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
    fn gen(&self, rng: &mut StdRng, size: IVec2) -> Map {
        let mut map = Map::filled_with(size, Tile::Wall);
        let room_size_max = (size.as_vec2() * self.room_size_ratio).as_ivec2();
        let room_count = ((size.x * size.y) as f32 * self.room_count_ratio) as usize;

        let rooms = build_rooms(rng, size, room_size_max, room_count);
        for room in rooms.iter() {
            room.for_each(|pt| {
                if map.is_in_bounds(pt) {
                    let tile = &mut map[pt];
                    *tile = Tile::Floor;
                }
            });
        }
        carve_coriddors(&mut map, rooms.clone());

        map
    }
}

fn carve_coriddors(map: &mut Map, rooms: Vec<Rect>) {
    let mut rooms = rooms.clone();
    rooms.sort_by(|a, b| a.get_center().y.cmp(&b.get_center().y));
    for (i, room) in rooms.iter().enumerate().skip(1) {
        let prev = rooms[i - 1].get_center();
        let new = room.get_center();
        for pt in WalkGrid::new((prev.x, prev.y), (new.x, new.y)).map(|(x, y)| IVec2::new(x, y)) {
            let tile = &mut map[pt];
            *tile = Tile::Floor;
        }
    }
}

fn build_rooms(
    rng: &mut StdRng,
    map_size: IVec2,
    room_size_max: IVec2,
    room_count: usize,
) -> Vec<Rect> {
    let mut rooms: Vec<Rect> = Vec::new();
    let mut i = 10000;
    while rooms.len() < room_count && i > 1 {
        i -= 1;
        let room_size = IVec2::new(
            rng.gen_range(i32::max(room_size_max.x / 4, 3)..i32::max(room_size_max.x, 5)),
            rng.gen_range(i32::max(room_size_max.y / 4, 3)..i32::max(room_size_max.y, 5)),
        );
        let room = Rect::new(
            IVec2::new(
                rng.gen_range(1..map_size.x - room_size.x),
                rng.gen_range(1..map_size.y - room_size.y),
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
