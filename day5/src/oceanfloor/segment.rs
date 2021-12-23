use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use super::point::Point;
use super::rect::Rect;

#[derive(Copy, Clone)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Segment {
    pub fn new(a: &Point, b: &Point) -> Segment {
        Segment { a: *a, b: *b }
    }

    pub fn bbox(&self) -> Rect {
        Rect::new(&self.a, &self.b)
    }

    pub fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    pub fn is_diagonal(&self) -> bool {
        (self.a.x - self.b.x).abs() == (self.a.y - self.b.y).abs()
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", self.a, self.b)
    }
}

impl Debug for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl FromStr for Segment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.split("->").collect();
        let a = points[0].trim().parse::<Point>()?;
        let b = points[1].trim().parse::<Point>()?;
        Ok(Segment { a, b })
    }
}

pub struct SegmentIterator<'a> {
    segment: &'a Segment,
    current: Point,
    step: Point,
}

impl<'a> IntoIterator for &'a Segment {
    type Item = Point;
    type IntoIter = SegmentIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut step = self.b - self.a;
        for coord in [&mut step.x, &mut step.y] {
            if *coord != 0 {
                *coord /= coord.abs();
            }
        }
        SegmentIterator {
            segment: self,
            current: self.a,
            step,
        }
    }
}

impl Iterator for SegmentIterator<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        let new = current + self.step;
        let start = self.segment.a;
        let target = self.segment.b;
        // New point must be between current and target
        let in_range = Rect::new(&start, &target).contains(&current);
        if in_range {
            self.current = new;
            return Some(current);
        }
        None
    }
}
