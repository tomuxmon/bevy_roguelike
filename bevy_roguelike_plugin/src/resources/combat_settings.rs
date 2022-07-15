use crate::components::RogueAttributeType;
use bevy::reflect::TypeUuid;
use bevy_roguelike_combat::LinearFormula;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "f08a6321-17b4-4087-843b-75251ea7bde4"]
pub struct CombatSettings {
    pub ap_increment_formula: LinearFormula<RogueAttributeType>,
    pub hp_full_formula: LinearFormula<RogueAttributeType>,
    pub hp_regen_increment_formula: LinearFormula<RogueAttributeType>,
}
