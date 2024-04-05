use bevy::{prelude::*, window::WindowResolution};
use bevy_inventory_ui::InventoryDisplayOptions;
use bevy_roguelike_plugin::{resources::*, RoguelikePlugin};
use bevy_roguelike_states::AppState;

fn main() {
    let mut app = App::new();
    app.add_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "rogue bevy".to_string(),
                        resolution: WindowResolution::new(1000., 600.),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RoguelikePlugin {})
        .add_startup_system(set_options);

    #[cfg(feature = "debug")]
    app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.run();
}

fn set_options(mut cmd: Commands) {
    cmd.insert_resource(MapOptions {
        map_size: IVec2::new(40, 25),
        tile_size: 32.0,
    });
    cmd.insert_resource(InventoryDisplayOptions { tile_size: 32.0 })
}
