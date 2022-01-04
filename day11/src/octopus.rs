use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: i8,
    pub y: i8,
}

impl Pos {
    pub fn new(x: i8, y: i8) -> Pos {
        Pos {x, y}
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

pub struct Octopus {
    energy: u8,
    flashed: bool,
    pos: Pos,
}

impl Octopus {
    const ENERGY_THRESHOLD: u8 = 9;

    pub fn new(x: i8, y: i8) -> Octopus {
        Octopus {
            energy: 0,
            flashed: false,
            pos: Pos::new(x, y),
        }
    }

    pub fn pos(&self) -> &Pos {
        &self.pos
    }

    fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }

    pub fn energy(&self) -> u8 {
        self.energy
    }

    pub fn set_energy(&mut self, energy: u8) {
        self.energy = energy;
    }

    pub fn increase_energy(&mut self, amount: u8) {
        self.energy += amount;
    }

    pub fn needs_to_flash(&self) -> bool {
        !self.flashed && self.energy > Self::ENERGY_THRESHOLD
    }

    pub fn flashed(&self) -> bool {
        self.flashed
    }

    pub fn reset_flash_status(&mut self) {
        self.flashed = false;
    }

    pub fn maybe_flash(&mut self) -> bool {
        if self.needs_to_flash() {
            self.flashed = true;
            self.energy = 0;
            true
        } else {
            false
        }
    }
}

/* TESTS **************************************************************************************************************/
mod test {
    use super::*;

    #[test]
    fn energy_threshold() {
        // Only flash with energy levels above 9
        for energy in 0..16 {
            let mut octopus = Octopus::new(0, 0);
            octopus.energy = energy;
            assert_eq!(octopus.needs_to_flash(), energy > 9);
        }
    }

    #[test]
    fn no_double_flash() {
        let mut octopus = Octopus::new(0, 0);
        octopus.increase_energy(10);
        assert_eq!(octopus.maybe_flash(), true);
        octopus.increase_energy(10);
        assert_eq!(octopus.maybe_flash(), false);
    }
}