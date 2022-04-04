use crate::components::*;
use crate::events::MoveEvent;
use bevy::prelude::*;

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &Vector2D), With<Player>>,
    mut move_writer: EventWriter<MoveEvent>,
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
        for (id, pt) in player.iter_mut() {
            move_writer.send(MoveEvent::new(id, *pt + delta));
        }
    }

    // TODO: handle combos [keys.pressed] + .. + [keys.pressed] +  [keys.just_pressed]
}
