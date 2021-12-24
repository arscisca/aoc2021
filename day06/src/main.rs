use std::fs::File;
use std::io::{BufRead, BufReader};

mod lanternfish {
    const RESET_COUNTDOWN: u8 = 6;
    const NEW_FISH_COUNTOWN: u8 = 8;
    const COUNTDOWN_MAP_SIZE: usize = NEW_FISH_COUNTOWN as usize + 1;

    pub struct School {
        pub fish: [u64; COUNTDOWN_MAP_SIZE],
    }

    impl School {
        pub fn new() -> Self {
            Self { fish: [0; COUNTDOWN_MAP_SIZE] }
        }

        pub fn advance_day(&mut self) {
            let mut next_day_fish = self.fish.clone();
            for (countdown, &curr_day_fish) in self.fish.iter().enumerate() {
                // Move fish to new countdown
                next_day_fish[countdown] -= curr_day_fish;
                if countdown > 0 {
                    next_day_fish[countdown - 1] += curr_day_fish;
                } else {
                    next_day_fish[RESET_COUNTDOWN as usize] += curr_day_fish;
                    next_day_fish[NEW_FISH_COUNTOWN as usize] += curr_day_fish;
                }
            }
            self.fish = next_day_fish;
        }

        pub fn add_fish(&mut self, countdown: u8, nfish: u64) {
            self.fish[countdown as usize] += nfish;
        }

        pub fn size(&self) -> usize {
            let mut count = 0usize;
            for &nfish in &self.fish {
                count += nfish as usize;
            }
            count
        }
    }
}

use crate::lanternfish::School;

fn simulate(school: &mut School, n: u64) -> usize {
    for _ in 0..n {
        school.advance_day();
    }
    school.size()
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    // Read data from file
    let f = File::open("data.txt").expect("Could not open file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut school = lanternfish::School::new();
    reader.read_line(&mut line).expect("Could not read line");
    for countdown in line.trim().split(',') {
        let countdown = countdown.parse::<u8>().expect("Could not parse number of fish");
        school.add_fish(countdown, 1);
    }
    // Simulate 80 days
    const N: u64 = 80;
    println!("School size after {} days: {}", N, simulate(&mut school, N));
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    // Read data from file
    let f = File::open("data.txt").expect("Could not open file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut school = lanternfish::School::new();
    reader.read_line(&mut line).expect("Could not read line");
    for countdown in line.trim().split(',') {
        let countdown = countdown.parse::<u8>().expect("Could not parse number of fish");
        school.add_fish(countdown, 1);
    }

    // Simulate 256 days
    const N: u64 = 256;
    println!("School size after {} days: {}", N, simulate(&mut school, N));
}

fn main() {
    part1();
    part2();
}
