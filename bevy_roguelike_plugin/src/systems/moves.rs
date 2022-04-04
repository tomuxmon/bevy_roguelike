use crate::components::*;
use crate::events::MoveEvent;
use crate::resources::map::Map;
use crate::resources::tile::Tile;
use crate::resources::MapOptions;
use bevy::log;
use bevy::prelude::*;

pub fn moves(
    map: Res<Map>,
    map_options: Res<MapOptions>,
    mut movers: Query<(&mut Transform, &mut Vector2D)>,
    mut move_reader: EventReader<MoveEvent>,
) {
    for m in move_reader.iter() {
        log::info!("id: {:?}, destination: {:?}", m.id, m.destination);
        if let Ok((mut world_pos, map_pos)) = movers.get_mut(m.id) {
            log::info!(
                "world position : {:?}, map position: {:?}",
                world_pos,
                map_pos
            );
            if map.is_in_bounds(m.destination) && map[m.destination] == Tile::Floor {
                let new_pos = to_world_position(m.destination, map.size, map_options.tile_size);
                world_pos.translation = new_pos;
                *map_pos.into_inner() = m.destination;
            }
        } else {
            // the entity does not have the components from the query
            log::info!("no positions found");
        }
    }
}

pub fn to_world_position(map_pos: Vector2D, map_size: Vector2D, tile_size: f32) -> Vec3 {
    let x_offset = map_size.x() as f32 * tile_size / -2.;
    let y_offset = map_size.y() as f32 * tile_size / -2.;
    Vec3::new(
        (map_pos.x() as f32 * tile_size) + (tile_size / 2.) + x_offset,
        (map_pos.y() as f32 * tile_size) + (tile_size / 2.) + y_offset,
        0.,
    )
}
