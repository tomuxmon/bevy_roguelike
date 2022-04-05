use crate::components::*;
use bevy::prelude::*;
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

impl MapOptions {
    pub fn to_world_position(&self, pt: Vector2D) -> Vec2 {
        let x_offset = self.map_size.x() as f32 * self.tile_size / -2.;
        let y_offset = self.map_size.y() as f32 * self.tile_size / -2.;
        Vec2::new(
            (pt.x() as f32 * self.tile_size) + (self.tile_size / 2.) + x_offset,
            (pt.y() as f32 * self.tile_size) + (self.tile_size / 2.) + y_offset,
        )
    }
}
