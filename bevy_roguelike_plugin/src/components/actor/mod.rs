#![allow(clippy::forget_non_drop)] // https://github.com/bevyengine/bevy/issues/4601
use super::*;
use crate::resources::ActorTemplate;
use bevy::{prelude::*, utils::HashMap};
use bevy_inventory::{Equipment, Inventory, ItemType};
use bevy_inventory_ui::EquipmentDisplay;
use bevy_roguelike_combat::{stats_derived::*, *};

fn from_display<I: ItemType>(display: &EquipmentDisplay<I>) -> Equipment<I> {
    let mut items = HashMap::default();
    for (t, _) in display.items.iter() {
        items.entry(*t).insert(None);
    }
    Equipment { items }
}

#[derive(Bundle)]
pub struct Actor {
    name: Name,
    team: Team,
    state: TurnState,
    attributes: Attributes,
    ap: ActionPoints,
    hp: HitPoints,
    protection: Protection<RogueDamageKind>,
    resistance: Resistance<RogueDamageKind>,
    evasion: Evasion,
    damage: DamageList<RogueDamageKind>,
    fov: FieldOfView,
    position: Vector2D,
    render_info: RenderInfo,
    equipment_display: EquipmentDisplay<RogueItemType>,
    equipment: Equipment<RogueItemType>,
    inventory: Inventory,
    stats_computed: StatsComputed<RogueDamageKind>,
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
            fov: FieldOfView::new(&template.attributes),
            damage: template.damage.clone(),
            protection: template.protection.clone(),
            evasion: template.evasion.clone(),
            resistance: template.resistance.clone(),
            inventory: Inventory::with_capacity(template.inventory_capacity),
            equipment_display: template.equipment_display.clone(),
            equipment: from_display(&template.equipment_display),
            position: Vector2D::from(position),
            render_info: RenderInfo {
                texture: asset_server.load(template.render.texture_path.as_str()),
                cosmetic_textures: template
                    .render
                    .texture_path_cosmetics
                    .iter()
                    .map(|p| asset_server.load(p.as_str()))
                    .collect(),
                z: 2.,
            },
            stats_computed_dirty: StatsComputedDirty {},
            stats_computed: StatsComputed::default(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TurnState {
    #[default]
    Collect,
    Act,
    End,
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
    pub fn id(&self) -> u32 {
        self.id
    }
}

// TODO: fix lousy name
#[derive(Default, Debug, Clone, Eq, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct HudHealthBar;
