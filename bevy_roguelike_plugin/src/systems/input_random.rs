use crate::components::*;
use crate::events::MoveEvent;
use bevy::prelude::*;
use rand::prelude::StdRng;
use rand::Rng;

pub fn move_random(
    mut randomers: Query<(Entity, &Vector2D, &mut ActionPoints), With<MovingRandom>>,
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

    for (id, pt, mut ap) in randomers.iter_mut() {
        if ap.current >= 300 {
            let delta = deltas[rng.gen_range(0..deltas.len())];
            if delta != Vector2D::zero() {
                ap.current -= 300;
                move_writer.send(MoveEvent::new(id, *pt + delta));
            }
        }
    }
}
