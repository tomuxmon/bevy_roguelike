use crate::components::Vector2D;
use serde::{Deserialize, Serialize};

/// Map generation options. Must be used as a resource
// We use serde to allow saving option presets and loading them at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapOptions {
    /// Tile map size in dimentions or tile 
    pub map_size: Vector2D,
    /// Tile world size
    pub tile_size: f32,
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            map_size: Vector2D::new(80, 50),
            tile_size: 32.0,
        }
    }
}
