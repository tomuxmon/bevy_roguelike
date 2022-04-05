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
        if let Ok((mut world_pos, map_pos)) = movers.get_mut(m.id) {
            if map.is_in_bounds(m.destination) && map[m.destination] == Tile::Floor {
                let old_pos = world_pos.translation;
                let new_pos = map_options.to_world_position(m.destination);
                world_pos.translation = new_pos.extend(old_pos.z);
                *map_pos.into_inner() = m.destination;
            }
        } else {
            // the entity does not have the components from the query
            log::info!("no positions found");
        }
    }
}
