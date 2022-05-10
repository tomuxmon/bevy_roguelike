use bevy::prelude::*;

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub enum ItemType {
    MainHand,
    OffHand,
    Head,
    Neck,
    Body,
    Feet,
    Finger,
}
impl Default for ItemType {
    fn default() -> Self {
        Self::MainHand
    }
}
