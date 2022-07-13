use crate::components::RogueDamageKind;
use bevy::reflect::TypeUuid;
use bevy_roguelike_combat::{stats_derived::*, *};
use serde::{Deserialize, Serialize};

// TODO: each item should define possible artifact bonuses
#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "5621d397-fbc8-4216-b1d8-3d90743338e8"]
pub enum ItemTemplate {
    Weapon(Weapon),
    Shield(Shield),
    Helm(Helm),
    Armor(Armor),
    Boots(Boots),
    Amulet(Amulet),
    Ring(Ring),
}
#[derive(Serialize, Deserialize)]
pub struct ItemRenderInfo {
    pub name: String,
    pub texture_path: String,
    pub texture_equiped_path: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ItemDefense {
    pub protection: Option<Protection<RogueDamageKind>>,
    pub resistance: Option<Resistance<RogueDamageKind>>,
}
#[derive(Serialize, Deserialize)]
pub struct ItemEnchantment {
    pub attributes: Option<Attributes>,
    // TODO: add various enchantment options.
    // evasion boost
    // speed bost
    // ...
}
#[derive(Serialize, Deserialize)]
pub struct Weapon {
    pub render: ItemRenderInfo,
    pub damage: Damage<RogueDamageKind>,
}
#[derive(Serialize, Deserialize)]
pub struct Shield {
    pub render: ItemRenderInfo,
    pub protection: Protection<RogueDamageKind>,
    pub block: Block<RogueDamageKind>,
}
#[derive(Serialize, Deserialize)]
pub struct Helm {
    pub render: ItemRenderInfo,
    pub defense: ItemDefense,
    pub enchantment: ItemEnchantment,
}
#[derive(Serialize, Deserialize)]
pub struct Armor {
    pub render: ItemRenderInfo,
    pub defense: ItemDefense,
    pub enchantment: ItemEnchantment,
}
#[derive(Serialize, Deserialize)]
pub struct Boots {
    pub render: ItemRenderInfo,
    pub defense: ItemDefense,
    pub enchantment: ItemEnchantment,
}
#[derive(Serialize, Deserialize)]
pub struct Amulet {
    pub render: ItemRenderInfo,
    pub defense: ItemDefense,
    pub enchantment: ItemEnchantment,
}
#[derive(Serialize, Deserialize)]
pub struct Ring {
    pub render: ItemRenderInfo,
    pub defense: ItemDefense,
    pub enchantment: ItemEnchantment,
}
