use bevy::prelude::*;
use bevy_inventory_ui::InventoryDisplayOptions;
use bevy_roguelike_plugin::{resources::*, RoguelikePlugin, StateNext};

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Setup,
    AssetLoad,
    Construct,
    InGame,
    Pause,
    Reseting,
}

impl StateNext for AppState {
    fn next(&self) -> Option<Self> {
        match self {
            AppState::Setup => Some(AppState::AssetLoad),
            AppState::AssetLoad => Some(AppState::Construct),
            AppState::Construct => Some(AppState::InGame),
            AppState::InGame => Some(AppState::Pause),
            AppState::Pause => Some(AppState::InGame),
            AppState::Reseting => Some(AppState::Construct),
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_state_to_stage(CoreStage::First, AppState::Setup)
        .add_state_to_stage(CoreStage::PreUpdate, AppState::Setup)
        .add_state_to_stage(CoreStage::Update, AppState::Setup)
        .add_state_to_stage(CoreStage::PostUpdate, AppState::Setup)
        .add_state_to_stage(CoreStage::Last, AppState::Setup)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "rogue bevy".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins {})
        .add_plugin(RoguelikePlugin {
            state_asset_load: AppState::AssetLoad,
            state_construct: AppState::Construct,
            state_running: AppState::InGame,
        })
        .add_startup_system(set_options);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn set_options(mut cmd: Commands) {
    cmd.insert_resource(MapOptions {
        map_size: IVec2::new(25, 25),
        tile_size: 32.0,
    });
    cmd.insert_resource(InventoryDisplayOptions { tile_size: 32.0 })
}
