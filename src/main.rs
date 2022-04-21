use bevy::prelude::*;
use bevy_roguelike_plugin::{
    components::*, events::*, resources::*, systems::turns::gather_action_points, RoguelikePlugin,
};
use rand::prelude::*;

#[cfg(feature = "debug")]
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
    mut actors: Query<(Entity, &Behaviour, &TurnState)>,
    mut act_writer: EventWriter<ActEvent>,
) {
    for (id, b, _) in actors
        .iter_mut()
        .filter(|(_, _, ts)| **ts == TurnState::Act)
    {
        let deltas = vec![
            IVec2::new(0, 1),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(0, 0), // stay put - skip turn
        ];
        let delta = match b {
            Behaviour::InputControlled => {
                if keys.just_pressed(KeyCode::Up) {
                    IVec2::new(0, 1)
                } else if keys.just_pressed(KeyCode::Down) {
                    IVec2::new(0, -1)
                } else if keys.just_pressed(KeyCode::Left) {
                    IVec2::new(-1, 0)
                } else if keys.just_pressed(KeyCode::Right) {
                    IVec2::new(1, 0)
                } else if keys.pressed(KeyCode::Space) {
                    IVec2::new(0, 0) // stay put - skip turn
                } else {
                    return;
                }
            }
            Behaviour::RandomMove => deltas[rng.gen_range(0..deltas.len())],
        };
        act_writer.send(ActEvent::new(id, delta));
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
        .add_plugins(DefaultPlugins {})
        .add_plugin(RoguelikePlugin {
            running_state: AppState::InGame,
        })
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(input_all.after(gather_action_points)),
        )
        .add_startup_system(rogue_setup);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    // #[cfg(feature = "debug")]
    // app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    //     .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());

    app.run();
}

fn rogue_setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
) {
    cmd.insert_resource(MapOptions {
        map_size: IVec2::new(79, 61),
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
