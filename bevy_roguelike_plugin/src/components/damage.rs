use bevy::{prelude::*, reflect::FromReflect};
use bevy_roguelike_combat::DamageKind;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Type of damage that can be inflicted by actors or environment.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Component,
    Reflect,
    FromReflect,
    Serialize,
    Deserialize,
)]
#[reflect(Component)]
#[reflect_value(PartialEq, Serialize, Deserialize)]
pub enum RogueDamageKind {
    /// phisical crushing damage
    #[default]
    Blunt,
    /// phisical puncturing damage
    Pierce,
    /// phisical cut damage
    Slash,
    /// elemental heat damage
    Fire,
    /// elemental cold damage
    Cold,
    /// elemental electrical damage
    Lightning,
}
impl DamageKind for RogueDamageKind {}

impl Display for RogueDamageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
