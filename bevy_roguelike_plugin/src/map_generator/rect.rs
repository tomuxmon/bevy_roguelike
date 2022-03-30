use super::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    pub start: Vector2D,
    pub size: Vector2D,
}

impl Rect {
    pub fn new(start: Vector2D, size: Vector2D) -> Self {
        Self { start, size }
    }

    pub fn get_center(&self) -> Vector2D {
        Vector2D::new(
            self.start.x() + self.size.x() / 2,
            self.start.y() + self.size.y() / 2,
        )
    }

    pub fn intersect_or_touch(&self, rhs: Rect) -> bool {
        self.start.x() <= rhs.start.x() + rhs.size.x()
            && self.start.x() + self.size.x() >= rhs.start.x()
            && self.start.y() <= rhs.start.y() + rhs.size.y()
            && self.start.y() + self.size.y() >= rhs.start.y()
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Vector2D),
    {
        for y in self.start.y()..self.start.y() + self.size.y() {
            for x in self.start.x()..self.start.x() + self.size.x() {
                f(Vector2D::new(x, y));
            }
        }
    }
}
