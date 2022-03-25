use crate::components::Vector2D;
use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    /// Fixed tile size
    Fixed(f32),
    /// Window adaptative tile size
    Adaptive { min: f32, max: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapPosition {
    /// Centered board
    Centered { offset: Vec3 },
    /// Custom position
    Custom(Vec3),
}

/// Map generation options. Must be used as a resource
// We use serde to allow saving option presets and loading them at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapOptions {
    /// Tile map size
    pub map_size: Vector2D,
    /// Map world position
    // TODO: should use follow camera instead
    pub position: MapPosition,
    /// Tile world size
    pub tile_size: TileSize,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for MapPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            map_size: Vector2D::new(80, 50),
            position: Default::default(),
            tile_size: Default::default(),
        }
    }
}
