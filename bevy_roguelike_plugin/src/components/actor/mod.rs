use super::FieldOfView;
use super::Vector2D;
use super::VisibilityToggle;
use crate::resources::MapOptions;
use bevy::prelude::*;
use std::borrow::Cow;

pub use stats::ActionPoints;
pub use stats::AttackStats;
pub use stats::Attributes;
pub use stats::HitPoints;

pub mod stats;

#[derive(Bundle)]
pub struct Actor {
    name: Name,
    team: Team,
    state: TurnState,
    attributes: Attributes,
    ap: ActionPoints,
    hp: HitPoints,
    atack: AttackStats,
    // TODO: DefenceStats
    fov: FieldOfView,
    toggle: VisibilityToggle,
    position: Vector2D,
    #[bundle]
    transform: TransformBundle,
}
impl Actor {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        team: u32,
        attributes: Attributes,
        position: IVec2,
        options: &MapOptions,
    ) -> Self {
        Self {
            name: Name::new(name),
            team: Team::new(team),
            state: TurnState::default(),
            attributes,
            ap: ActionPoints::new(&attributes),
            hp: HitPoints::new(&attributes),
            atack: AttackStats::new(&attributes),
            fov: FieldOfView::new(&attributes),
            toggle: VisibilityToggle::default(),
            position: Vector2D::from(position),
            transform: TransformBundle {
                local: Transform::from_translation(options.to_world_position(position).extend(2.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TurnState {
    Collect,
    Act,
    End,
}
impl Default for TurnState {
    fn default() -> Self {
        TurnState::Collect
    }
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct MovingPlayer;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct MovingRandom;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct MovingFovRandom;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Team {
    id: u32,
}

impl Team {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

// TODO: fix lousy name
#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct OnTopHud;

// NOTE: a clunky component to transfer damage
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ModifyHP {
    pub location: IVec2,
    pub amount: i16,
}

impl ModifyHP {
    pub fn new(location: IVec2, amount: i16) -> Self {
        Self { location, amount }
    }
}