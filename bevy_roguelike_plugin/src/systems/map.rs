use crate::components::*;
use crate::resources::MapOptions;
use bevy::prelude::*;

pub fn apply_position_to_transform(
    mut actors: Query<(&Vector2D, &mut Transform, Changed<Vector2D>)>,
    map_options: Res<MapOptions>,
) {
    for (pt, mut tr, _) in actors.iter_mut() {
        let z = tr.translation.z;
        tr.translation = map_options.to_world_position(**pt).extend(z);
    }
}
