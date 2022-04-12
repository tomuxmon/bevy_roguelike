use bevy::{prelude::*, utils::HashMap};
use bevy_roguelike_plugin::{components::*, events::ModifyHPEvent, resources::*, RoguelikePlugin};
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

pub fn input_all(
    keys: Res<Input<KeyCode>>,
    mut rng: ResMut<StdRng>,
    mut actors: Query<(
        Entity,
        &Team,
        &Behaviour,
        &mut TurnState,
        &mut Capability,
        &mut FieldOfView,
        &mut Vector2D,
        &mut Transform,
    )>,
    mut dmg_wr: EventWriter<ModifyHPEvent>,
    map_options: Res<MapOptions>,
    map: Res<Map>,
) {
    let ocupied = HashMap::from_iter(actors.iter().map(|(e, t, _, _, _, _, p, _)| (*p, (e, *t))));

    for (_, team, b, mut ts, mut cp, mut fov, mut pt, mut tr) in actors
        .iter_mut()
        .filter(|(_, _, _, ts, _, _, _, _)| **ts == TurnState::Act)
    {
        let deltas = vec![
            Vector2D::new(0, 1),
            Vector2D::new(0, -1),
            Vector2D::new(-1, 0),
            Vector2D::new(1, 0),
            Vector2D::new(0, 0), // stay put - skip turn
        ];
        let delta_costs = HashMap::from_iter(vec![
            (Vector2D::new(0, 1), 900),
            (Vector2D::new(0, -1), 900),
            (Vector2D::new(-1, 0), 900),
            (Vector2D::new(1, 0), 900),
            (Vector2D::new(0, 0), 451), // stay put - skip turn
        ]);

        let delta = match b {
            Behaviour::InputControlled => {
                if keys.just_pressed(KeyCode::Up) {
                    Vector2D::new(0, 1)
                } else if keys.just_pressed(KeyCode::Down) {
                    Vector2D::new(0, -1)
                } else if keys.just_pressed(KeyCode::Left) {
                    Vector2D::new(-1, 0)
                } else if keys.just_pressed(KeyCode::Right) {
                    Vector2D::new(1, 0)
                } else if keys.just_pressed(KeyCode::Space) {
                    Vector2D::new(0, 0) // stay put - skip turn
                } else {
                    Vector2D::minmin()
                }
            }
            Behaviour::RandomMove => deltas[rng.gen_range(0..deltas.len())],
        };

        if delta != Vector2D::minmin() {
            let mut cost = delta_costs[&delta];
            let dest = *pt + delta;
            if !map.is_in_bounds(dest) || map[dest] != Tile::Floor {
                continue;
            }
            let other = ocupied.get(&dest);
            // NOTE: can not move into a tile ocupied by a team mate
            if other.is_some() && other.unwrap().1 == *team && delta != Vector2D::new(0, 0) {
                continue;
            }
            // TODO: instead of 'delta != ..' check on is_same_id
            if other.is_some() && delta != Vector2D::new(0, 0) {
                cost = cp.attack_cost();
                dmg_wr.send(ModifyHPEvent::new(other.unwrap().0, -cp.attack_damage()));
            } else {
                if delta != Vector2D::new(0, 0) {
                    let old_pos = tr.translation;
                    let new_pos = map_options.to_world_position(dest);
                    tr.translation = new_pos.extend(old_pos.z);
                    *pt = dest;
                    fov.is_dirty = true;
                }
            }
            *ts = TurnState::End;
            cp.ap_current_minus(cost);
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_state(AppState::Out)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "rogue bevy".to_string(),
            width: 1200.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RoguelikePlugin {
            running_state: AppState::InGame,
        })
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(input_all))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(rogue_setup)
        .add_startup_system(camera_setup);

    app.run();
}

fn rogue_setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
) {
    cmd.insert_resource(MapOptions {
        map_size: Vector2D::new(31, 27),
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
