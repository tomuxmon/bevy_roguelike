use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "272889fd-4633-4b9c-8d33-ff19d3f2ecef"]
pub struct MapTheme {
    pub floor: Vec<String>,
    pub wall: Vec<String>,
}
