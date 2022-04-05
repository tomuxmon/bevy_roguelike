use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Mul};

/// Vector of 2 Dimensions.
#[derive(
    Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Deserialize, Serialize, Reflect,
)]
#[reflect(Component)]
pub struct Vector2D {
    // TODO: consider using bevy::prelude::IVec2 internally instead of implementin all from scratch.
    x: i32,
    y: i32,
}

impl Vector2D {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn distance_pow2(&self, other: Self) -> i32 {
        i32::pow(i32::abs(self.x - other.x), 2) + i32::pow(i32::abs(self.y - other.y), 2)
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Self;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: (self.x as f32 * rhs) as i32,
            y: (self.y as f32 * rhs) as i32,
        }
    }
}

impl Display for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
