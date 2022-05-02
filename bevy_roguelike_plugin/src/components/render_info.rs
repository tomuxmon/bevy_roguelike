use bevy::prelude::*;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct RenderInfo {
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub z: f32,
}
