use crate::{components::*, events::*};
use bevy::{prelude::*, utils::HashMap};
use bevy_inventory::{Equipment, Inventory, ItemDropEvent, ItemPickUpEvent, ItemType};
use line_drawing::WalkGrid;
use map_generator::*;
use rand::prelude::*;

pub fn input_player<I: ItemType>(
    keys: Res<Input<KeyCode>>,
    players: Query<(Entity, &TurnState, &Inventory, &Equipment<I>), With<MovingPlayer>>,
    mut act_writer: EventWriter<ActEvent>,
    mut pick_up_writer: EventWriter<ItemPickUpEvent>,
    mut drop_writer: EventWriter<ItemDropEvent>,
) {
    for (id, _, inv, eqv) in players
        .iter()
        .filter(|(_, ts, _, _)| **ts == TurnState::Act)
    {
        let delta = if keys.just_pressed(KeyCode::Up) {
            IVec2::new(0, 1)
        } else if keys.just_pressed(KeyCode::Down) {
            IVec2::new(0, -1)
        } else if keys.just_pressed(KeyCode::Left) {
            IVec2::new(-1, 0)
        } else if keys.just_pressed(KeyCode::Right) {
            IVec2::new(1, 0)
        } else if keys.just_pressed(KeyCode::Space) || keys.pressed(KeyCode::Z) {
            IVec2::new(0, 0) // stay put - skip turn
        } else if keys.just_pressed(KeyCode::Comma) {
            pick_up_writer.send(ItemPickUpEvent { picker: id });
            IVec2::new(0, 0) // still stay put - skip turn
        } else if keys.just_pressed(KeyCode::D) {
            if let Some(ee) = inv.iter_some().last() {
                drop_writer.send(ItemDropEvent {
                    droper: id,
                    item: ee,
                });
            } else if let Some((_, ee)) = eqv.iter_some().last() {
                drop_writer.send(ItemDropEvent {
                    droper: id,
                    item: ee,
                });
            }
            return;
        } else {
            return;
        };
        act_writer.send(ActEvent { id, delta });
    }
}

pub fn input_fov_rand(
    mut rng: ResMut<StdRng>,
    actors: Query<(Entity, &Vector2D, &Team, &TurnState, &FieldOfView), With<MovingFovRandom>>,
    actors_all: Query<(&Vector2D, &Team)>,
    mut act_writer: EventWriter<ActEvent>,
    map: Res<Map>,
) {
    let team_pt: HashMap<_, _> = actors_all.iter().map(|(p, t)| (**p, *t)).collect();
    for (id, pt, team, _, fov) in actors
        .iter()
        .filter(|(_, _, _, ts, _)| **ts == TurnState::Act)
    {
        let deltas = vec![
            IVec2::new(0, 1),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(0, 0), // stay put - skip turn
            IVec2::new(0, 0), // stay put - skip turn
            IVec2::new(0, 0), // stay put - skip turn
            IVec2::new(0, 0), // stay put - skip turn
            IVec2::new(0, 0), // stay put - skip turn
        ];

        // NOTE: closest oposing team member search
        let mut distance_last = ((fov.radius + 1) * (fov.radius + 1)) as f32;
        let mut pt_move_target = None;
        for pt_visible in fov.tiles_visible.iter() {
            if let Some(other_team) = team_pt.get(pt_visible) {
                if *other_team != *team {
                    let distance = pt_visible.as_vec2().distance_squared(pt.as_vec2());
                    if distance < distance_last {
                        pt_move_target = Some(*pt_visible);
                        distance_last = distance;
                    }
                }
            }
        }

        let mut delta = IVec2::new(0, 0);
        if let Some(tgt) = pt_move_target {
            if let Some((x, y)) = WalkGrid::new((pt.x, pt.y), (tgt.x, tgt.y)).take(2).last() {
                let dest = IVec2::new(x, y);
                if map[dest] == Tile::Floor
                    && !(team_pt.get(&dest).is_some() && *team_pt.get(&dest).unwrap() == *team)
                {
                    delta = IVec2::new(x - pt.x, y - pt.y);
                }
            }
        } else {
            delta = deltas[rng.gen_range(0..deltas.len())];
        }
        act_writer.send(ActEvent { id, delta });
    }
}
