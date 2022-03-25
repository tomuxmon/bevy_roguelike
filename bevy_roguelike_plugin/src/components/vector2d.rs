use bevy::prelude::Component;
use std::fmt::{self, Display, Formatter};
use std::ops::Add;

/// Vector of 2 Dimensions.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Component)]
// NOTE: only in debug
#[derive(bevy_inspector_egui::Inspectable)]
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
