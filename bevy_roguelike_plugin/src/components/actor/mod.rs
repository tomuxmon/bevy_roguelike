use super::*;
use crate::resources::ActorTemplate;
use bevy::prelude::*;
use stats::*;

pub mod stats;

#[derive(Bundle)]
pub struct Actor {
    name: Name,
    team: Team,
    state: TurnState,
    attributes: Attributes,
    ap: ActionPoints,
    hp: HitPoints,
    protection: Protection,
    resistance: Resistance,
    evasion: Evasion,
    damage: DamageList,
    fov: FieldOfView,
    position: Vector2D,
    render_info: RenderInfo,
    equipment_display: EquipmentDisplay,
    equipment: Equipment,
    inventory: Inventory,
    stats_computed: StatsComputed,
    stats_computed_dirty: StatsComputedDirty,
}
impl Actor {
    /// Creates a new [`Actor`] using specified [`ActorTemplate`].
    pub fn new(
        asset_server: AssetServer,
        template: &ActorTemplate,
        team: u32,
        position: IVec2,
    ) -> Self {
        Self {
            name: Name::new(template.render.name.clone()),
            team: Team::new(team),
            state: TurnState::default(),
            attributes: template.attributes.clone(),
            ap: ActionPoints::new(&template.attributes),
            hp: HitPoints::new(&template.attributes),
            damage: template.damage.clone(),
            protection: template.protection.clone(),
            evasion: template.evasion.clone(),
            resistance: template.resistance.clone(),
            fov: FieldOfView::new(&template.attributes),
            inventory: Inventory::with_capacity(template.inventory_capacity),
            equipment_display: template.equipment_display.clone(),
            equipment: (&template.equipment_display).into(),
            position: Vector2D::from(position),
            render_info: RenderInfo {
                texture: asset_server.load(template.render.texture_path.as_str()),
                z: 2.,
            },
            stats_computed: StatsComputed::default(),
            stats_computed_dirty: StatsComputedDirty {},
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
pub struct HudHealthBar;

// NOTE: a clunky component to transfer damage
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ModifyHP {
    pub location: IVec2,
    pub amount: i16,
}
