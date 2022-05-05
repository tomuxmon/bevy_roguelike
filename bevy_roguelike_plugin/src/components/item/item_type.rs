use bevy::prelude::*;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct MainHandWeapon;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct OffHandShield;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BodyWear;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct FeetWear;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct HeadWear;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct NeckWear;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct FingerWear;
