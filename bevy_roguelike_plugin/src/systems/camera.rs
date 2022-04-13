use crate::components::*;
use crate::events::CameraFocusEvent;
use crate::resources::*;
use bevy::prelude::*;
use bevy_easings::*;
use std::time::Duration;

pub fn camera_set_focus_player(
    players: Query<&Vector2D, With<Player>>,
    mut map_info: ResMut<MapInfo>,
    mut cmr_wrt: EventWriter<CameraFocusEvent>,
) {
    for pt in players.iter() {
        if map_info.camera_focus != *pt {
            map_info.camera_focus = *pt;
            cmr_wrt.send(CameraFocusEvent::new(*pt))
        }
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
pub fn camera_focus_smooth(
    mut cmd: Commands,
    mut cmr_rdr: EventReader<CameraFocusEvent>,
    cameras: Query<(Entity, &Transform), With<Camera>>,
    map_options: Res<MapOptions>,
) {
    for cfe in cmr_rdr.iter() {
        for (e, ct) in cameras.iter() {
            let z = ct.translation.z;
            cmd.entity(e).insert(
                ct.ease_to(
                    ct.clone()
                        .with_translation(map_options.to_world_position(cfe.position).extend(z)),
                    EaseFunction::QuinticOut,
                    EasingType::Once {
                        duration: Duration::from_millis(350),
                    },
                ),
            );
        }
    }
}
