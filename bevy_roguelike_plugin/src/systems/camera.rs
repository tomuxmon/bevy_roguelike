use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn camera_set_focus_player(
    players: Query<&Vector2D, With<Player>>,
    mut map_info: ResMut<MapInfo>,
) {
    for pt in players.iter() {
        map_info.camera_focus = *pt;
    }
}

pub fn camera_focus_immediate(
    mut camera: Query<&mut Transform, With<Camera>>,
    map_info: Res<MapInfo>,
    map_options: Res<MapOptions>,
) {
    for mut c in camera.iter_mut() {
        let old_pos = c.translation;
        let new_pos = map_options.to_world_position(map_info.camera_focus);
        c.translation = new_pos.extend(old_pos.z);
    }
}
// ! not implemented
pub fn camera_focus_smooth(
    mut camera: Query<&mut Transform, With<Camera>>,
    map_info: Res<MapInfo>,
    map_options: Res<MapOptions>,
) {
    for mut c in camera.iter_mut() {
        // TODO: smooth transition to other point
        let old_pos = c.translation;
        let new_pos = map_options.to_world_position(map_info.camera_focus);
        c.translation = new_pos.extend(old_pos.z);
    }
}
