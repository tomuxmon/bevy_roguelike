use bevy::prelude::IVec2;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::ops::{Deref, DerefMut};

/// Vector of 2 Dimensions.
#[derive(
    Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Component, Deserialize, Serialize, Reflect,
)]
#[reflect(Component)]
pub struct Vector2D {
    internal: IVec2,
}
impl Deref for Vector2D {
    type Target = IVec2;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}
impl DerefMut for Vector2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}
impl From<IVec2> for Vector2D {
    fn from(internal: IVec2) -> Self {
        Self { internal }
    }
}
impl Display for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
