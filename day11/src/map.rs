use std::error::Error;
use std::fmt::Formatter;
use super::octopus::Octopus;

pub struct Map {
    width: usize,
    height: usize,
    octopi: Vec<Octopus>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut octopi = Vec::with_capacity(width * height);
        for y in 0..height as i8 {
            for x in 0..width as i8 {
                octopi.push(Octopus::new(x, y));
            }
        }
        Map {width, height, octopi}
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn push_row(&mut self, energies: Vec<u8>) -> Result<(), &'static str> {
        if self.width == 0 {
            self.width = energies.len();
        }
        if self.width != energies.len() {
            return Err("Invalid row size");
        }
        let y = self.height;
        for (x, &energy) in energies.iter().enumerate() {
            let mut octopus = Octopus::new(x as i8, y as i8);
            octopus.set_energy(energy);
            self.octopi.push(octopus);
        }
        self.height += 1;
        Ok(())
    }

    pub fn octopus(&self, x: i8, y: i8) -> Option<&Octopus> {
        if x < 0 || y < 0 {
            return None;
        }
        let index = x as usize + (y as usize * self.width);
        Some(&self.octopi[index])
    }

    pub fn octopus_mut(&mut self, x: i8, y: i8) -> Option<&mut Octopus> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        let index = x as usize + (y as usize * self.width);
        Some(&mut self.octopi[index])
    }

    pub fn octopi(&self) -> &Vec<Octopus> {
        &self.octopi
    }

    pub fn octopi_mut(&mut self) -> &mut Vec<Octopus> {
        &mut self.octopi
    }

    pub fn update(&mut self) -> u64 {
        let mut flashes_count = 0;
        let mut flashes = Vec::new();
        for octopus in self.octopi_mut() {
            octopus.increase_energy(1);
            if octopus.maybe_flash() {
                flashes.push(*octopus.pos());
                flashes_count += 1;
            }
        }

        while !flashes.is_empty() {
            let flash = flashes.pop().unwrap();
            for dy in -1..=1i8 {
                for dx in -1..=1i8 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let (x, y) = (flash.x + dx, flash.y + dy);
                    if let Some(neighbor) = self.octopus_mut(x, y) {
                        if neighbor.flashed() {
                            continue;
                        }
                        neighbor.increase_energy(1);
                        if neighbor.maybe_flash() {
                            flashes.push(*neighbor.pos());
                            flashes_count += 1;
                        }
                    }
                }
            }
        }

        for octopus in self.octopi_mut() {
            octopus.reset_flash_status();
        }

        flashes_count
    }
}

impl core::str::FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Map::new(0, 0);
        for line in s.lines() {
            let data_row: Vec<u8> = line.chars()
                .map(|c| c.to_digit(10).expect("Cannot convert digit") as u8)
                .collect();
            map.push_row(data_row)?;
        }
        Ok(map)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.octopi.len() / self.width {
            let line = &self.octopi[i * self.width .. i * self.width + self.width];
            for octopus in line {
                write!(f, "{}", octopus.energy())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

mod test {
    use crate::octopus::Pos;
    use super::*;

    fn create_map(width: usize, height: usize, data: Vec<u8>) -> Map {
        assert_eq!(data.len() % width, 0, "Incorrect map format");
        assert_eq!(data.len() / width, height, "Incorrect map format");
        let mut map = Map::new(width, 0);
        for i in 0..data.len() / width {
            map.push_row(Vec::from(&data[i * width .. (i + 1) * width]));
        }
        map
    }

    #[test]
    fn base_energy_increase() {
        let mut map = create_map(3, 3, vec![0; 3 * 3]);
        for i in 0..9 {
            for octopus in map.octopi() {
                assert_eq!(octopus.energy(), i);
            }
            map.update();
        }
    }

    #[test]
    fn flash_propagation() {
        let mut map = create_map(3, 3, vec![
            0, 0, 0,
            0, 10, 0,
            0, 0, 0,
        ]);
        map.update();
        for octopus in map.octopi() {
            if octopus.pos().x == 1 && octopus.pos().y == 1 {
                assert_eq!(octopus.energy(), 0, "Wrong energy level for octopus at pos {}", octopus.pos());
            } else {
                assert_eq!(octopus.energy(), 2, "Wrong energy level for octopus at pos {}", octopus.pos());
            }
        }
    }

    #[test]
    fn flash_induced_flash() {
        let mut map = create_map(2, 1, vec![8, 9]);
        map.update();
        println!("{}", map);
        assert_eq!(map.octopus(0, 0).unwrap().energy(), 0);
        assert_eq!(map.octopus(1, 0).unwrap().energy(), 1);
    }
}