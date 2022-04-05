use crate::components::*;
use crate::events::MoveEvent;
use crate::resources::map::MapInfo;
use bevy::prelude::*;

pub fn player_input_read(
    keys: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &Vector2D, &mut ActionPoints), With<Player>>,
    mut move_writer: EventWriter<MoveEvent>,
    mut map_info: ResMut<MapInfo>,
) {
    let delta = if keys.just_pressed(KeyCode::Up) {
        Vector2D::new(0, 1)
    } else if keys.just_pressed(KeyCode::Down) {
        Vector2D::new(0, -1)
    } else if keys.just_pressed(KeyCode::Left) {
        Vector2D::new(-1, 0)
    } else if keys.just_pressed(KeyCode::Right) {
        Vector2D::new(1, 0)
    } else {
        Vector2D::zero()
    };

    // TODO: implement key mappings

    if delta != Vector2D::zero() {
        for (id, pt, mut ap) in player.iter_mut() {
            if ap.current >= 300 {
                ap.current -= 300;
                move_writer.send(MoveEvent::new(id, *pt + delta));
                // NOTE: immediately setting camera focus so it does update on the same frame
                map_info.camera_focus = *pt + delta;
            }
        }
    }

    // TODO: handle combos [keys.pressed] + .. + [keys.pressed] +  [keys.just_pressed]
}
