use crate::components::*;
use crate::events::MoveEvent;
use bevy::prelude::*;
use rand::prelude::StdRng;
use rand::Rng;

pub fn move_random(
    mut randomers: Query<(Entity, &Vector2D), With<MovingRandom>>,
    mut move_writer: EventWriter<MoveEvent>,
    mut rng: ResMut<StdRng>,
) {
    let deltas = vec![
        Vector2D::new(0, 1),
        Vector2D::new(0, -1),
        Vector2D::new(-1, 0),
        Vector2D::new(1, 0),
        Vector2D::zero(),
    ];
    let delta = deltas[rng.gen_range(0..deltas.len())]; // rng..
    if delta != Vector2D::zero() {
        for (id, pt) in randomers.iter_mut() {
            move_writer.send(MoveEvent::new(id, *pt + delta));
        }
    }
}
