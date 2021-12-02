use std::fs::File;
use std::io::{BufRead, BufReader,};

fn main() {
    println!("{:-<32}", "Part 1 ");
    part1();
    println!("{:-<32}", "Part 2 ");
    part2();
}

struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Pos {
        Pos {x, y}
    }
}

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl Command {
    fn from_string(string: &str) -> Result<Command, &str> {
        let entries = string.split_once(' ').expect("Invalid command string");
        let cname = entries.0;
        let units: i64 = entries.1.parse().expect("Invalid units in command");
        match cname {
            "forward" => Ok(Command::Forward(units)),
            "down"    => Ok(Command::Down(units)),
            "up"      => Ok(Command::Up(units)),
            _         => Err("Invalid command name")
        }
    }
}

fn part1() {
    let mut pos = Pos::new(0, 0);

    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let command = line.expect("Could not read line");
        let command = Command::from_string(&command).expect("Invalid command string");
        match command {
            Command::Forward(units) => pos.x += units,
            Command::Down(units)    => pos.y += units,
            Command::Up(units)      => pos.y -= units,
        };
    }
    // Print final result
    println!("Final position ({}, {}); product = {}", pos.x, pos.y, pos.x * pos.y);
}

fn part2() {
    let mut pos = Pos::new(0, 0);
    let mut aim = 0;

    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let command = line.expect("Could not read line");
        let command = Command::from_string(&command).expect("Invalid command string");
        match command {
            Command::Forward(units) => {
                pos.x += units;
                pos.y += aim * units;
            },
            Command::Down(units) => aim += units,
            Command::Up(units) => aim -= units,
        }
    }
    // Print final result
    println!("Final position ({}, {}); product = {}", pos.x, pos.y, pos.x * pos.y);
}
