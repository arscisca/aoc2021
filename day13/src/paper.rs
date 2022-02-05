use std::cmp;
use std::cmp::Ordering;
use std::fmt::Formatter;

struct Dot {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Dot {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Dot {
    fn clone(&self) -> Self {
        Dot { x: self.x, y: self.y }
    }
}

pub struct Paper {
    dots: Vec<Dot>
}

impl Paper {
    pub fn new() -> Paper {
        Paper {
            dots: Vec::new()
        }
    }

    pub fn num_dots(&self) -> usize {
        self.dots.len()
    }

    fn dot_order(d1: &Dot, d2: &Dot) -> Ordering {
        // Compare x first, if x1 == x2 compare y
        match d1.x.cmp(&d2.x) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match d1.y.cmp(&d2.y) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            },
        }
    }

    pub fn add_dot(&mut self, x: i32, y: i32) {
        if let Err(index) = self.index_dot(x, y) {
            self.dots.insert(index, Dot {x, y})
        }
    }

    fn index_dot(&self, x: i32, y: i32) -> Result<usize, usize> {
        let reference = Dot {x, y};
        self.dots.binary_search_by(|other| Paper::dot_order(&reference, &other))
    }

    pub fn has_dot(&self, x: i32, y: i32) -> bool {
        self.index_dot(x, y).is_ok()
    }

    pub fn remove_dot(&mut self, x: i32, y: i32) -> bool {
        if let Ok(index) = self.index_dot(x, y) {
            self.dots.remove(index);
            true
        } else {
            false
        }
    }

    pub fn fold_x(&mut self, x: i32) {
        let original = self.dots.clone();
        self.dots.clear();
        for dot in original {
            let (mut newx, newy) = (dot.x, dot.y);
            if dot.x > x {
                newx = x - (dot.x - x);
            }
            self.add_dot(newx, newy);
        }
    }

    pub fn fold_y(&mut self, y: i32) {
        let original = self.dots.clone();
        self.dots.clear();
        for dot in original {
            let (newx, mut newy) = (dot.x, dot.y);
            if dot.y > y {
                newy = y - (dot.y - y);
            }
            self.add_dot(newx, newy);
        }
    }
}


impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.dots.is_empty() {
            return Ok(());
        }
        let mut clone = self.dots.clone();
        let Dot {x: xmin, y: ymin} = clone.into_iter()
            .reduce(|dot1, dot2| Dot {
                x: cmp::min(dot1.x, dot2.x),
                y: cmp::min(dot1.y, dot2.y)})
            .unwrap();
        clone = self.dots.clone();
        let Dot {x: xmax, y: ymax} = clone.into_iter()
            .reduce(|dot1, dot2| Dot {
                x: cmp::max(dot1.x, dot2.x),
                y: cmp::max(dot1.y, dot2.y)})
            .unwrap();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if self.has_dot(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
