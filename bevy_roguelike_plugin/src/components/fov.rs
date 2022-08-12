use super::RogueAttributeType;
use bevy::{prelude::*, utils::HashSet};
use bevy_roguelike_combat::Attributes;

// #[derive(Default, Component, Reflect)]
// #[reflect(Component)]
// pub struct VisibilityToggle;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct FieldOfViewDirty;

#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct FieldOfView {
    pub radius: i32,
    pub tiles_visible: HashSet<IVec2>,
    pub tiles_revealed: HashSet<IVec2>,
    // TODO: refactor as a separate dirty component (do not clash with FieldOfViewDirty use.. better name?)
    pub is_dirty: bool,
}

impl FieldOfView {
    pub const MIN_RADIUS: i32 = 2;

    // TODO: use formula instead like the rest of the attribute dependant derived attributes
    pub fn new(atr: &Attributes<RogueAttributeType>) -> Self {
        Self {
            radius: FieldOfView::MIN_RADIUS
                + (atr.get(&RogueAttributeType::Perception) as f32 / 3.
                    + atr.get(&RogueAttributeType::Inteligence) as f32 / 10.)
                    as i32,
            tiles_visible: HashSet::default(),
            tiles_revealed: HashSet::default(),
            is_dirty: true,
        }
    }
    pub fn update(&mut self, atr: &Attributes<RogueAttributeType>) {
        self.radius = FieldOfView::MIN_RADIUS
            + (atr.get(&RogueAttributeType::Perception) as f32 / 3.
                + atr.get(&RogueAttributeType::Inteligence) as f32 / 10.) as i32;
        self.is_dirty = true;
    }
}
