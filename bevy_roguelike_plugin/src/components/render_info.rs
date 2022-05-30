use bevy::prelude::*;

/// specifies how to render stuff if it is placed on a map
#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct RenderInfo {
    pub texture: Handle<Image>,    
    pub z: f32,
}
/// specifies how to render stuff if it is equiped by an actor
#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct RenderInfoEquiped {
    pub texture: Handle<Image>,    
    pub z: f32,
}