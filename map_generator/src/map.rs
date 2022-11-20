use glam::IVec2;
use rand::prelude::*;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

/// Enum describing a map tile
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

// TODO: inject Resource? have a trait for map instead
/// Flat tile map of tiles
#[derive(Debug, Clone)]
pub struct Map {
    size: IVec2,
    tiles: Vec<Tile>,
}

impl Map {
    pub(crate) fn filled_with(size: IVec2, tile: Tile) -> Self {
        Self {
            size,
            tiles: vec![tile; (size.x * size.y) as usize],
        }
    }

    pub(crate) fn random_noise(size: IVec2, rng: &mut StdRng) -> Self {
        let mut tiles = Vec::new();
        for _ in 0..size.x * size.y {
            let roll = rng.gen_range(0..=100);
            tiles.push(if roll > 55 { Tile::Floor } else { Tile::Wall })
        }
        Self { size, tiles }
    }

    pub fn is_in_bounds(&self, pt: IVec2) -> bool {
        self.size.x > pt.x && self.size.y > pt.y && pt.x >= 0 && pt.y >= 0
    }

    pub fn is_edge(&self, pt: IVec2) -> bool {
        pt.x == 0 || pt.y == 0 || pt.x == self.size.x - 1 || pt.y == self.size.y - 1
    }

    /// itterates over underlying tiles vector
    pub fn iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }
    /// enumerates tiles and positions of each tile
    pub fn enumerate(&self) -> impl Iterator<Item = (IVec2, &Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .map(move |(idx, t)| (self.get_point(idx), t))
    }

    pub fn get_edge(&self) -> Vec<IVec2> {
        let mut v = Vec::new();
        for x in 0..self.size.x {
            v.push(IVec2::new(x, 0));
            v.push(IVec2::new(x, self.size.y - 1));
        }
        for y in 1..self.size.y - 1 {
            v.push(IVec2::new(0, y));
            v.push(IVec2::new(self.size.x - 1, y));
        }
        v
    }

    /// returns slice at y level
    pub fn get_slice(&self, y: i32) -> &[Tile] {
        let start = self.get_index(IVec2::new(0, y));
        let end = start + self.size.x as usize;
        &self.tiles[start..end]
    }

    /// zero based indexing
    fn get_index(&self, pt: IVec2) -> usize {
        (pt.y * (self.size.x) + pt.x) as usize
    }
    fn get_point(&self, idx: usize) -> IVec2 {
        IVec2::new(
            (idx % self.size.x as usize) as i32,
            (idx / self.size.x as usize) as i32,
        )
    }

    /// returns neighboring points (deltas):
    ///
    /// |  |  |  |
    /// | --- | --- | --- |
    /// |(-1, 1)|(0, 1)|(1, 1)|
    /// |(-1, 0)|tile|(1, 0)|
    /// |(-1, -1)|(0, -1)|(1, -1)|
    pub fn get_neighbor_deltas() -> [IVec2; 8] {
        // TODO: should be static or const (not fn) but still be vectors :\
        [
            IVec2::new(-1, 1),
            IVec2::new(0, 1),
            IVec2::new(1, 1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(-1, -1),
            IVec2::new(0, -1),
            IVec2::new(1, -1),
        ]
    }

    pub fn get_wasd_neighbor_deltas() -> [IVec2; 4] {
        // TODO: should be static or const (not fn) but still be vectors :\
        [
            IVec2::new(0, 1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
        ]
    }
    pub fn size(&self) -> IVec2 {
        self.size
    }
}
impl Index<IVec2> for Map {
    type Output = Tile;

    fn index(&self, pt: IVec2) -> &Self::Output {
        let idx = self.get_index(pt);
        &self.tiles[idx]
    }
}
impl IndexMut<IVec2> for Map {
    fn index_mut(&mut self, pt: IVec2) -> &mut Self::Output {
        let idx = self.get_index(pt);
        &mut self.tiles[idx]
    }
}
