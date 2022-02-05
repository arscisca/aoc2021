use std::fs::File;
use std::io::{BufRead, BufReader};

mod caves;

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let f = File::open("data.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let mut cavesystem = caves::CaveSystem::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('-').collect();
        assert_eq!(parts.len(), 2);
        for part in &parts {
            if !cavesystem.contains(part) {
                cavesystem.insert(part);
            }
        }
        cavesystem.connect(parts[0], parts[1]).expect("Could not connect caves");
    }
    let paths = cavesystem.find_paths("start", "end", 0).expect("Could not find paths");
    println!("Paths: {}", paths.len());
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let f = File::open("data.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let mut cavesystem = caves::CaveSystem::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('-').collect();
        assert_eq!(parts.len(), 2);
        for part in &parts {
            if !cavesystem.contains(part) {
                cavesystem.insert(part);
            }
        }
        cavesystem.connect(parts[0], parts[1]).expect("Could not connect caves");
    }
    let paths = cavesystem.find_paths("start", "end", 1).expect("Could not find paths");
    println!("Paths: {}", paths.len());
}

fn main() {
    part1();
    part2();
}
