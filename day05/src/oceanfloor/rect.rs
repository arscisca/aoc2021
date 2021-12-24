use super::point::Point;
use std::cmp;

#[derive(Clone, Copy)]
pub struct Rect {
    pub a: Point,
    pub b: Point,
}

impl Rect {
    pub fn new(a: &Point, b: &Point) -> Rect {
        Rect {
            a: Point::new(cmp::min(a.x, b.x), cmp::min(a.y, b.y)),
            b: Point::new(cmp::max(a.x, b.x), cmp::max(a.y, b.y)),
        }
    }

    pub fn union(r1: &Rect, r2: &Rect) -> Rect {
        Rect {
            a: Point::new(cmp::min(r1.a.x, r2.a.x), cmp::min(r1.a.y, r2.a.y)),
            b: Point::new(cmp::max(r1.b.x, r2.b.x), cmp::max(r1.b.y, r2.b.y)),
        }
    }

    pub fn width(&self) -> u64 {
        (self.a.x - self.b.x).abs() as u64
    }

    pub fn height(&self) -> u64 {
        (self.a.y - self.b.y).abs() as u64
    }

    pub fn contains(&self, point: &Point) -> bool {
        (self.a.x <= point.x && point.x <= self.b.x) && (self.a.y <= point.y && point.y <= self.b.y)
    }
}
