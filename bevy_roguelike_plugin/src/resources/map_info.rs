use crate::map_generator::*;
use bevy::math::IVec2;
use rand::prelude::*;

#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct MapInfo {
    pub player_start: IVec2,
    pub camera_focus: IVec2,
    pub monster_spawns: Vec<IVec2>,
    pub item_spawns: Vec<IVec2>,
}

impl MapInfo {
    pub fn from_map(map: &Map, rng: &mut StdRng) -> MapInfo {
        let floor: Vec<IVec2> = map
            .enumerate()
            .filter(|(_, t)| **t == Tile::Floor)
            .map(|(p, _)| p)
            .collect();

        let pidx = rng.gen_range(0..floor.len());
        let monster_count = floor.len() / 16;
        let item_count = floor.len() / 160;
        let player_start = floor[pidx];

        let mut monster_spawns = Vec::new();
        while monster_spawns.len() < monster_count {
            let midx = rng.gen_range(0..floor.len());
            let pt = floor[midx];
            if midx != pidx && !monster_spawns.contains(&pt) {
                monster_spawns.push(pt);
            }
        }

        let mut item_spawns = Vec::new();
        while item_spawns.len() < item_count {
            let iidx = rng.gen_range(0..floor.len());
            let pt = floor[iidx];
            item_spawns.push(pt);
        }

        MapInfo {
            player_start,
            camera_focus: player_start,
            monster_spawns,
            item_spawns,
        }
    }

    pub fn to_colorized_string(&self) -> String {
        format!(" player star: {}", self.player_start)
    }
}

impl Tile {
    #[cfg(feature = "debug")]
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

impl Map {
    #[cfg(feature = "debug")]
    pub(crate) fn to_colorized_string(&self) -> String {
        let mut buffer = format!("Map (w: {}, h: {})\n", self.size().x, self.size().y);
        let line: String = (0..(self.size().x + 2)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for y in (0..self.size().y).rev() {
            buffer = format!("{}|", buffer);
            for tile in self.get_slice(y) {
                buffer = format!("{}{}", buffer, tile.to_colorized_string());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }
}
