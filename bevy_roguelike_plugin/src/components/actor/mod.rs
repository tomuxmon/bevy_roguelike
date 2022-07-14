#![allow(clippy::forget_non_drop)] // https://github.com/bevyengine/bevy/issues/4601
use super::*;
use crate::resources::ActorTemplate;
use bevy::{prelude::*, reflect::FromReflect, utils::HashMap};
use bevy_inventory::{Equipment, Inventory, ItemType};
use bevy_inventory_ui::EquipmentDisplay;
use bevy_roguelike_combat::{stats_derived::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumIter;

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
    EnumIter,
)]
#[reflect(Component)]
#[reflect_value(PartialEq, Serialize, Deserialize)]
pub enum RogueAttributeType {
    #[default]
    Strength,
    Dexterity,
    Inteligence,
    Toughness,
    Perception,
    Willpower,
}
impl Display for RogueAttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RogueAttributeType::Strength => "str",
                RogueAttributeType::Dexterity => "dex",
                RogueAttributeType::Inteligence => "int",
                RogueAttributeType::Toughness => "tou",
                RogueAttributeType::Perception => "per",
                RogueAttributeType::Willpower => "wil",
            }
        )
    }
}
impl AttributeType for RogueAttributeType {}

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
    attributes: Attributes<RogueAttributeType>,
    ap: ActionPoints<RogueAttributeType>,
    hp: HitPoints<RogueAttributeType>,
    protection: Protection<RogueDamageKind, RogueAttributeType>,
    resistance: Resistance<RogueDamageKind>,
    evasion: Evasion<RogueAttributeType>,
    damage: DamageList<RogueDamageKind, RogueAttributeType>,
    fov: FieldOfView,
    position: Vector2D,
    render_info: RenderInfo,
    equipment_display: EquipmentDisplay<RogueItemType>,
    equipment: Equipment<RogueItemType>,
    inventory: Inventory,
    stats_computed: StatsComputed<RogueDamageKind, RogueAttributeType>,
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
        // TODO: ron asset for those formulas;
        let ap_increment_formula = LinearFormula::<RogueAttributeType>::new(vec![
            Multiplier::<RogueAttributeType> {
                multiplier: 70,
                attribute: RogueAttributeType::Dexterity,
            },
            Multiplier::<RogueAttributeType> {
                multiplier: 30,
                attribute: RogueAttributeType::Willpower,
            },
        ]);
        let hp_full_formula = LinearFormula::<RogueAttributeType>::new(vec![
            Multiplier::<RogueAttributeType> {
                multiplier: 70,
                attribute: RogueAttributeType::Toughness,
            },
            Multiplier::<RogueAttributeType> {
                multiplier: 20,
                attribute: RogueAttributeType::Strength,
            },
            Multiplier::<RogueAttributeType> {
                multiplier: 10,
                attribute: RogueAttributeType::Willpower,
            },
        ]);
        let hp_regen_increment_formula = LinearFormula::<RogueAttributeType>::new(vec![
            Multiplier::<RogueAttributeType> {
                multiplier: 80,
                attribute: RogueAttributeType::Toughness,
            },
            Multiplier::<RogueAttributeType> {
                multiplier: 10,
                attribute: RogueAttributeType::Strength,
            },
            Multiplier::<RogueAttributeType> {
                multiplier: 10,
                attribute: RogueAttributeType::Willpower,
            },
        ]);

        Self {
            name: Name::new(template.render.name.clone()),
            team: Team::new(team),
            state: TurnState::default(),
            attributes: template.attributes.clone(),
            ap: ActionPoints::<RogueAttributeType>::new(ap_increment_formula, &template.attributes),
            hp: HitPoints::<RogueAttributeType>::new(
                hp_full_formula,
                hp_regen_increment_formula,
                &template.attributes,
            ),
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
