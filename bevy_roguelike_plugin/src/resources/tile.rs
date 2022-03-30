use std::fmt::*;
// TODO: only in debug
use colored::Colorize;

/// Enum describing a map tile
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    // TODO: should also describe the kind of wall (what sides are open)
    // could be read from environment?
    Wall,
    Floor,
}

impl Tile {
    pub const fn is_wall(&self) -> bool {
        matches!(self, Self::Wall)
    }

    // TODO: only in debug #[cfg(feature = "debug")]
    pub fn to_colorized_string(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Wall => "#".bright_red(),
                Tile::Floor => ".".bright_green(),
            }
        )
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
