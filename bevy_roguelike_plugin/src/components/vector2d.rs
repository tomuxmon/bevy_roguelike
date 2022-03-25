use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::ops::Add;

/// Vector of 2 Dimensions.
#[derive(
    Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Deserialize, Serialize, Reflect,
)]
#[reflect(Component)]
pub struct Vector2D {
    x: i32,
    y: i32,
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
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

impl Display for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
