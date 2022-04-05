use crate::components::Vector2D;
use crate::resources::tile::Tile;
use rand::prelude::*;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

/// Flat tile map of tiles
#[derive(Debug, Clone)]
pub struct Map {
    // pub size: IVec2,
    pub size: Vector2D,
    tiles: Vec<Tile>,
}

#[derive(Debug, Clone)]
pub struct MapInfo {
    pub player_start: Vector2D,
    pub room_centers: Vec<Vector2D>,
    pub camera_focus: Vector2D,
}

impl Map {
    pub fn filled_with(size: Vector2D, tile: Tile) -> Self {
        Self {
            size,
            tiles: vec![tile; (size.x() * size.y()) as usize],
        }
    }

    pub fn random_noise(size: Vector2D, rng: &mut StdRng) -> Self {
        let mut tiles = Vec::new();
        for _ in 0..size.x() * size.y() {
            let roll = rng.gen_range(0..=100);
            tiles.push(if roll > 55 { Tile::Floor } else { Tile::Wall })
        }
        Self { size, tiles }
    }

    pub fn is_in_bounds(&self, pt: Vector2D) -> bool {
        self.size.x() - 1 >= pt.x() && self.size.y() - 1 >= pt.y()
    }

    pub fn is_edge(&self, pt: Vector2D) -> bool {
        pt.x() == 0 || pt.y() == 0 || pt.x() == self.size.x() - 1 || pt.y() == self.size.y() - 1
    }

    /// itterates over underlying tiles vector
    pub fn iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }
    /// enumerates tiles and positions of each tile
    pub fn enumerate(&self) -> impl Iterator<Item = (Vector2D, &Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .map(move |(idx, t)| (self.get_point(idx), t))
    }

    pub fn get_edge(&self) -> Vec<Vector2D> {
        let mut v = Vec::new();
        for x in 0..self.size.x() {
            v.push(Vector2D::new(x, 0));
            v.push(Vector2D::new(x, self.size.y() - 1));
        }
        for y in 1..self.size.y() - 1 {
            v.push(Vector2D::new(0, y));
            v.push(Vector2D::new(self.size.x() - 1, y));
        }
        v
    }

    /// returns slice at y level
    pub fn get_slice(&self, y: i32) -> &[Tile] {
        let start = self.get_index(Vector2D::new(0, y));
        let end = start + self.size.x() as usize;
        &self.tiles[start..end]
    }

    /// zero based indexing
    fn get_index(&self, pt: Vector2D) -> usize {
        (pt.y() * (self.size.x()) + pt.x()) as usize
    }
    fn get_point(&self, idx: usize) -> Vector2D {
        Vector2D::new(
            (idx % self.size.x() as usize) as i32,
            (idx / self.size.x() as usize) as i32,
        )
    }

    // TODO: only in debug : #[cfg(feature = "debug")]
    pub(crate) fn to_colorized_string(&self) -> String {
        // TODO: validate and manually test me
        let mut buffer = format!("Map (w: {}, h: {})\n", self.size.x(), self.size.y());
        let line: String = (0..(self.size.x() + 2)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for y in (0..self.size.y()).rev() {
            buffer = format!("{}|", buffer);
            for tile in self.get_slice(y) {
                buffer = format!("{}{}", buffer, tile.to_colorized_string());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    /// returns neighboring points (deltas):
    ///
    /// |  |  |  |
    /// | --- | --- | --- |
    /// |(-1, 1)|(0, 1)|(1, 1)|
    /// |(-1, 0)|tile|(1, 0)|
    /// |(-1, -1)|(0, -1)|(1, -1)|
    pub fn get_neighbor_deltas() -> [Vector2D; 8] {
        // TODO: should also include cost of moving there
        // TODO: should be static or const (not fn) but still be vectors :\
        [
            Vector2D::new(-1, 1),
            Vector2D::new(0, 1),
            Vector2D::new(1, 1),
            Vector2D::new(-1, 0),
            Vector2D::new(1, 0),
            Vector2D::new(-1, -1),
            Vector2D::new(0, -1),
            Vector2D::new(1, -1),
        ]
    }

    pub fn get_wasd_neighbor_deltas() -> [Vector2D; 4] {
        // TODO: should be static or const (not fn) but still be vectors :\
        [
            Vector2D::new(0, 1),
            Vector2D::new(-1, 0),
            Vector2D::new(1, 0),
            Vector2D::new(0, -1),
        ]
    }
}

impl Index<Vector2D> for Map {
    type Output = Tile;

    fn index(&self, pt: Vector2D) -> &Self::Output {
        let idx = self.get_index(pt);
        &self.tiles[idx]
    }
}

impl IndexMut<Vector2D> for Map {
    fn index_mut(&mut self, pt: Vector2D) -> &mut Self::Output {
        let idx = self.get_index(pt);
        &mut self.tiles[idx]
    }
}

impl MapInfo {
    pub fn new(player_start: Vector2D, room_centers: Vec<Vector2D>) -> Self {
        Self {
            player_start,
            room_centers,
            camera_focus: player_start,
        }
    }

    pub fn to_colorized_string(&self) -> String {
        format!(
            "room count: {}, player star: {}",
            self.room_centers.len(),
            self.player_start
        )
    }
}
