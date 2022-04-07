use bevy::prelude::*;
use bevy_roguelike_plugin::{components::*, resources::*, RoguelikePlugin};
use rand::prelude::*;

// TODO: only in debug
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Reseting,
    Pause,
    Out,
}

pub fn input_player(
    keys: Res<Input<KeyCode>>,
    mut players: Query<
        (
            &mut Transform,
            &mut Vector2D,
            &mut ActionPoints,
            &mut TurnState,
        ),
        With<Player>,
    >,
    mut map_info: ResMut<MapInfo>,
    map_options: Res<MapOptions>,
    map: Res<Map>,
) {
    for (mut tr, mut pt, mut ap, mut ts) in players
        .iter_mut()
        .filter(|(_, _, _, ts)| **ts == TurnState::Act)
    {
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
        if delta != Vector2D::zero() {
            let cost = 900;
            let dest = *pt + delta;
            // NOTE: immediately setting camera focus so it does update on the same frame
            map_info.camera_focus = dest;

            if map.is_in_bounds(dest) && map[dest] == Tile::Floor {
                let old_pos = tr.translation;
                let new_pos = map_options.to_world_position(dest);
                tr.translation = new_pos.extend(old_pos.z);
                *pt = dest;
                *ts = TurnState::End;
                ap.current_minus(cost);
            }
        }
    }
}

pub fn input_random(
    mut rng: ResMut<StdRng>,
    mut randomers: Query<
        (
            &mut Transform,
            &mut Vector2D,
            &mut ActionPoints,
            &mut TurnState,
        ),
        With<MovingRandom>,
    >,
    map_options: Res<MapOptions>,
    map: Res<Map>,
) {
    for (mut tr, mut pt, mut ap, mut ts) in randomers
        .iter_mut()
        .filter(|(_, _, _, ts)| **ts == TurnState::Act)
    {
        let deltas = vec![
            Vector2D::new(0, 1),
            Vector2D::new(0, -1),
            Vector2D::new(-1, 0),
            Vector2D::new(1, 0),
            Vector2D::zero(),
        ];
        let delta = deltas[rng.gen_range(0..deltas.len())];
        if delta != Vector2D::zero() {
            let cost = 900;
            let dest = *pt + delta;

            if map.is_in_bounds(dest) && map[dest] == Tile::Floor {
                let old_pos = tr.translation;
                let new_pos = map_options.to_world_position(dest);
                tr.translation = new_pos.extend(old_pos.z);
                *pt = dest;
                *ts = TurnState::End;
                ap.current_minus(cost);
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_state(AppState::Out)
        .insert_resource(WindowDescriptor {
            title: "rogue bevy".to_string(),
            width: 900.,
            height: 900.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RoguelikePlugin {
            running_state: AppState::InGame,
        })
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(input_player)
                .with_system(input_random),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(rogue_setup)
        .add_startup_system(camera_setup);

    // TODO: add local systems..

    app.run();
}

fn rogue_setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
) {
    cmd.insert_resource(MapOptions {
        map_size: Vector2D::new(49, 32),
        tile_size: 32.0,
    });
    cmd.insert_resource(MapAssets {
        floor: vec![
            asset_server.load("sprites/floor/pebble_brown_0.png"),
            asset_server.load("sprites/floor/pebble_brown_1.png"),
            asset_server.load("sprites/floor/pebble_brown_2.png"),
            asset_server.load("sprites/floor/pebble_brown_3.png"),
            asset_server.load("sprites/floor/pebble_brown_4.png"),
            asset_server.load("sprites/floor/pebble_brown_5.png"),
            asset_server.load("sprites/floor/pebble_brown_6.png"),
            asset_server.load("sprites/floor/pebble_brown_7.png"),
            asset_server.load("sprites/floor/pebble_brown_8.png"),
        ],
        wall: vec![
            asset_server.load("sprites/walls/brick_brown_0.png"),
            asset_server.load("sprites/walls/brick_brown_1.png"),
            asset_server.load("sprites/walls/brick_brown_2.png"),
            asset_server.load("sprites/walls/brick_brown_3.png"),
            asset_server.load("sprites/walls/brick_brown_4.png"),
            asset_server.load("sprites/walls/brick_brown_5.png"),
            asset_server.load("sprites/walls/brick_brown_6.png"),
            asset_server.load("sprites/walls/brick_brown_7.png"),
        ],
    });
    cmd.insert_resource(PlayerAssets {
        body: asset_server.load("sprites/player/human_male.png"),
        wear: vec![
            asset_server.load("sprites/player/jacket_2.png"),
            asset_server.load("sprites/player/pants_black.png"),
        ],
    });
    cmd.insert_resource(EnemyAssets {
        skins: vec![
            asset_server.load("sprites/enemy/cyclops.png"),
            asset_server.load("sprites/enemy/ettin.png"),
            asset_server.load("sprites/enemy/frost_giant.png"),
            asset_server.load("sprites/enemy/gnoll.png"),
            asset_server.load("sprites/enemy/goblin.png"),
            asset_server.load("sprites/enemy/hobgoblin.png"),
            asset_server.load("sprites/enemy/kobold.png"),
            asset_server.load("sprites/enemy/orc.png"),
            asset_server.load("sprites/enemy/stone_giant.png"),
        ],
    });

    state.set(AppState::InGame).unwrap();
}

fn camera_setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}
