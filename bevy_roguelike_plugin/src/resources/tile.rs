// TODO: only in debug
use colored::Colorize;

/// Enum describing a map tile
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
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
