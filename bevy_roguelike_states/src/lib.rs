use bevy::prelude::States;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    Setup,
    AssetLoad,
    Construct,
    InGame,
    Pause,
    Reseting,
}
