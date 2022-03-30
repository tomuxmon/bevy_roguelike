use bevy::prelude::*;
use bevy_roguelike_plugin::{components::Vector2D, resources::MapOptions, RoguelikePlugin};
// TODO: only in debug
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "yeah right...!".to_string(),
        width: 1200.,
        height: 900.,
        ..Default::default()
    })
    .insert_resource(MapOptions {
        map_size: Vector2D::new(120, 90),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(RoguelikePlugin {})
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}
