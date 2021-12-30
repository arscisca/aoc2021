use std::num::ParseIntError;
use std::fmt;
use std::fmt::Formatter;
use std::str;

pub struct Map {
    width: usize,
    height: usize,
    points: Vec<u8>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        Map {
            width,
            height,
            points: vec![0; width * height],
        }
    }

    fn index_to_xy(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn location_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn point(&self, x: usize, y: usize) -> &u8 {
        &self.points[x + y * self.width]
    }

    pub fn point_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.points[x + y * self.width]
    }

    pub fn low_points(&self) -> LowPoints {
        // Find first low point
        LowPoints {
            map: &self,
            i: 0,
        }
    }

    pub fn basins(&self) -> Basins {
        Basins {
            map: &self,
            low_points: self.low_points(),
            visited: vec![false; self.points.len()]
        }
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let point = self.point(x, y);
        // Check vertically
        if let Some(y_minus_one) = y.checked_sub(1) {
            if *point >= *self.point(x, y_minus_one) {
                return false;
            }
        }
        if y + 1 < self.height {
            if *point >= *self.point(x, y + 1) {
                return false;
            }
        }
        // Check horizontally
        if let Some(x_minus_one) = x.checked_sub(1) {
            if *point >= *self.point(x_minus_one, y) {
                return false;
            }
        }
        if x + 1 < self.width {
            if *point >= *self.point(x + 1, y) {
                return false;
            }
        }
        true
    }

    fn collect_basin(&self, x: usize, y: usize, visited: &mut Vec<bool>, basin: &mut Vec<(usize, usize)>) {
        let point = self.point(x, y);
        if *point == 9 || visited[x + y * self.width] {
            return;
        }
        visited[x + y * self.width] = true;
        // Check vertically
        if let Some(y_minus_one) = y.checked_sub(1) {
            if *point < *self.point(x, y_minus_one) {
                self.collect_basin(x, y_minus_one, visited, basin);
            }
        }
        if y + 1 < self.height {
            if *point < *self.point(x, y + 1) {
                self.collect_basin(x, y + 1, visited, basin);
            }
        }
        // Check horizontally
        if let Some(x_minus_one) = x.checked_sub(1) {
            if *point < *self.point(x_minus_one, y) {
                self.collect_basin(x_minus_one, y, visited, basin);
            }
        }
        if x + 1 < self.width {
            if *point < *self.point(x + 1, y) {
                self.collect_basin(x + 1, y, visited, basin);
            }
        }
        basin.push((x, y));
    }

    fn push_data_row(&mut self, mut row: Vec<u8>) {
        if self.width == 0 {
            self.width = row.len();
        } else if row.len() != self.width {
            panic!("Row size incompatible with map size")
        }
        self.points.append(&mut row);
        self.height += 1;
    }
}

impl str::FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Map::new(0, 0);
        for line in s.lines() {
            let data_row = line.chars()
                .map(|c| c.to_digit(10).expect("Cannot convert digit") as u8)
                .collect::<Vec<u8>>();
            map.push_data_row(data_row);
        }
        Ok(map)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.points.len() / self.width {
            let line = &self.points[i * self.width .. i * self.width + self.width];
            for num in line {
                write!(f, "{}", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct LowPoints<'a> {
    map: &'a Map,
    i: usize,
}

impl<'a> Iterator for LowPoints<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Find next low point
        while self.i < self.map.points.len() {
            let xy = self.map.index_to_xy(self.i);
            self.i += 1;
            if self.map.is_low_point(xy.0, xy.1) {
                return Some(self.map.point(xy.0, xy.1));
            }
        }
        None
    }
}

pub struct Basin {
    points: Vec<(usize, usize)>,
}

impl Basin {
    pub fn new() -> Basin {
        Basin {points: Vec::new()}
    }

    pub fn points(&self) -> &Vec<(usize, usize)> {
        &self.points
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }
}

pub struct Basins<'a> {
    map: &'a Map,
    low_points: LowPoints<'a>,
    visited: Vec<bool>,
}

impl<'a> Iterator for Basins<'a> {
    type Item = Basin;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.low_points.next() {
            let mut basin = Basin::new();
            let xy = self.map.index_to_xy(self.low_points.i - 1);
            println!("Checking out point {:?} (height = {})", xy, self.map.point(xy.0, xy.1));
            self.map.collect_basin(xy.0, xy.1, &mut self.visited, &mut basin.points);
            Some(basin)
        } else {
            None
        }
    }
}