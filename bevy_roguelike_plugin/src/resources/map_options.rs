use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Map generation options. Must be used as a resource
// We use serde to allow saving option presets and loading them at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapOptions {
    /// Tile map size in dimentions or tile
    pub map_size: IVec2,
    /// Tile world size
    pub tile_size: f32,
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            map_size: IVec2::new(80, 50),
            tile_size: 32.0,
        }
    }
}

impl MapOptions {
    pub fn to_world_position(&self, pt: IVec2) -> Vec2 {
        // NOTE: When camera focus is in play we no longer need offset.
        // let x_offset = self.map_size.x as f32 * self.tile_size / -2.;
        // let y_offset = self.map_size.y as f32 * self.tile_size / -2.;
        Vec2::new(
            (pt.x as f32 * self.tile_size) + (self.tile_size / 2.),
            (pt.y as f32 * self.tile_size) + (self.tile_size / 2.),
        )
    }
    pub fn to_closest_map_position(&self, translation: Vec3) -> IVec2 {
        IVec2::new(
            ((translation.x - self.tile_size / 2.) / self.tile_size) as i32,
            ((translation.y - self.tile_size / 2.) / self.tile_size) as i32,
        )
    }
}
