use crate::oceanfloor::point::Point;
use std::fmt::{Display, Formatter};

use super::rect::Rect;
use super::segment::Segment;

pub struct Map {
    boundaries: Rect,
    data: Vec<u64>,
}

impl Map {
    pub fn new(boundaries: &Rect) -> Map {
        // Initialize data
        let data = vec![0; (boundaries.width() * boundaries.height()) as usize];
        Map {
            boundaries: *boundaries,
            data,
        }
    }

    pub fn from(segments: &[Segment]) -> Map {
        let mut map = Map::new(&Map::region_enclosing(segments));
        for segment in segments {
            map.add_segment(segment);
        }
        map
    }

    fn region_enclosing(segments: &[Segment]) -> Rect {
        assert!(
            !segments.is_empty(),
            "Cannot create map from an empty Vec of Segments"
        );
        // Find boundaries
        let mut boundaries = segments[0].bbox();
        for segment in segments {
            boundaries = Rect::union(&boundaries, &segment.bbox());
        }
        // Enlarge boundaries to enclose the extreme points
        boundaries.b += Point::new(1, 1);
        boundaries
    }

    pub fn boundaries(&self) -> &Rect {
        &self.boundaries
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.boundaries.contains(point)
    }

    fn data(&self, p: &Point) -> &u64 {
        let local = *p - self.boundaries.a;
        &self.data[(local.x + local.y * (self.boundaries.width() as i64)) as usize]
    }

    fn data_mut(&mut self, p: &Point) -> &mut u64 {
        let local = *p - self.boundaries.a;
        &mut self.data[(local.x + local.y * (self.boundaries.width() as i64)) as usize]
    }

    pub fn add_segment(&mut self, segment: &Segment) {
        assert!(
            segment.is_horizontal() || segment.is_vertical() || segment.is_diagonal(),
            "Segment must be horizontal, vertical or diagonal"
        );
        for point in segment {
            assert!(self.contains(&point), "Map does not contain the point");
            *self.data_mut(&point) += 1;
        }
    }

    pub fn count_overlaps(&self, threshold: u64) -> u64 {
        let mut count = 0;
        for &value in &self.data {
            if value >= threshold {
                count += 1;
            }
        }
        count
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, &data) in self.data.iter().enumerate() {
            let i = i as u64;
            if (i > 0) && (i % self.boundaries.width() == 0) {
                writeln!(f)?;
            }
            if data == 0 {
                write!(f, ".")?;
            } else {
                write!(f, "{}", data)?;
            }
        }
        Ok(())
    }
}
