use bevy::prelude::*;
use bevy_roguelike_plugin::{
    components::*,
    resources::{map_assets::MapAssets, player_assets::PlayerAssets, MapOptions},
    RoguelikePlugin,
};
// TODO: only in debug
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Reseting,
    Pause,
    Out,
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
        map_size: Vector2D::new(70, 70),
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
    state.set(AppState::InGame).unwrap();
}

fn camera_setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}
