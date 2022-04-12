use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct MapTile {
    pub is_passable: bool,
    // TODO: include cost of moving into a tile

    // TODO: should also describe the kind of tile (what sides are open)
}
