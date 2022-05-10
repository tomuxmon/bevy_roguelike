use crate::components::Team;
use map_generator::Map;
use bevy::prelude::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct TeamMap {
    size: IVec2,
    map: Vec<Option<Team>>,
}

impl TeamMap {
    pub fn empty(size: IVec2) -> Self {
        Self {
            size,
            map: vec![None; (size.x * size.y) as usize],
        }
    }
    fn get_index(&self, pt: IVec2) -> usize {
        (pt.y * (self.size.x) + pt.x) as usize
    }
    fn get_point(&self, idx: usize) -> IVec2 {
        IVec2::new(
            (idx % self.size.x as usize) as i32,
            (idx / self.size.x as usize) as i32,
        )
    }
    pub fn enumerate(&self) -> impl Iterator<Item = (IVec2, &Option<Team>)> {
        self.map
            .iter()
            .enumerate()
            .map(move |(idx, t)| (self.get_point(idx), t))
    }
}

impl FromWorld for TeamMap {
    fn from_world(world: &mut World) -> Self {
        Self::empty(world.resource::<Map>().size())
    }
}
impl Index<IVec2> for TeamMap {
    type Output = Option<Team>;

    fn index(&self, pt: IVec2) -> &Self::Output {
        let idx = self.get_index(pt);
        &self.map[idx]
    }
}
impl IndexMut<IVec2> for TeamMap {
    fn index_mut(&mut self, pt: IVec2) -> &mut Self::Output {
        let idx = self.get_index(pt);
        &mut self.map[idx]
    }
}
