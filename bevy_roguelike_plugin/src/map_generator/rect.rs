use bevy::math::IVec2;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    pub start: IVec2,
    pub size: IVec2,
}

impl Rect {
    pub fn new(start: IVec2, size: IVec2) -> Self {
        Self { start, size }
    }

    pub fn get_center(&self) -> IVec2 {
        IVec2::new(
            self.start.x + self.size.x / 2,
            self.start.y + self.size.y / 2,
        )
    }

    pub fn intersect_or_touch(&self, rhs: Rect) -> bool {
        self.start.x <= rhs.start.x + rhs.size.x
            && self.start.x + self.size.x >= rhs.start.x
            && self.start.y <= rhs.start.y + rhs.size.y
            && self.start.y + self.size.y >= rhs.start.y
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(IVec2),
    {
        for y in self.start.y..self.start.y + self.size.y {
            for x in self.start.x..self.start.x + self.size.x {
                f(IVec2::new(x, y));
            }
        }
    }
}
