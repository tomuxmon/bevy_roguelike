use crate::components::*;
use bevy::reflect::TypeUuid;
use bevy_inventory_ui::EquipmentDisplay;
use bevy_roguelike_combat::{stats_derived::*, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "0c475ce2-2031-4259-9e22-6a8f3b94a1d1"]
pub struct ActorTemplate {
    pub render: ActorRenderInfo,
    pub attributes: Attributes,
    pub protection: Protection,
    pub resistance: Resistance,
    pub evasion: Evasion,
    pub damage: DamageList,
    pub equipment_display: EquipmentDisplay<RogueItemType>,
    pub inventory_capacity: usize,
    // TODO: initial equipment
    // TODO: initial inventory
}

#[derive(Serialize, Deserialize)]
pub struct ActorRenderInfo {
    pub name: String,
    pub texture_path: String,
    pub texture_path_cosmetics: Vec<String>,
}
