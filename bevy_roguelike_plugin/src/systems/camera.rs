use crate::components::*;
use crate::events::CameraFocusEvent;
use crate::resources::*;
use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use std::time::Duration;

pub fn camera_set_focus_player(
    players: Query<&Vector2D, With<MovingPlayer>>,
    mut map_info: ResMut<MapInfo>,
    mut cmr_wrt: EventWriter<CameraFocusEvent>,
) {
    for pt in players.iter() {
        if map_info.camera_focus != **pt {
            map_info.camera_focus = **pt;
            cmr_wrt.send(CameraFocusEvent::new(**pt))
        }
    }
}
pub fn camera_focus_smooth(
    mut cmd: Commands,
    mut cmr_rdr: EventReader<CameraFocusEvent>,
    cameras: Query<(Entity, &Transform), With<Camera2d>>,
    map_options: Res<MapOptions>,
) {
    for cfe in cmr_rdr.iter() {
        for (camera_entity, camera_transform) in cameras.iter() {
            cmd.entity(camera_entity).insert(Animator::new(Tween::new(
                EaseFunction::QuinticOut,
                Duration::from_millis(350),
                TransformPositionLens {
                    start: camera_transform.translation,
                    end: map_options
                        .to_world_position(cfe.position)
                        .extend(camera_transform.translation.z),
                },
            )));
        }
    }
}
