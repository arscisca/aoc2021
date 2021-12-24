use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::CostMode::{Linear, Quadratic};

fn read_data(fname: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let mut data = Vec::new();
    for line in reader.lines() {
        for number in line?.trim().split(",") {
            let number = number.parse::<i64>()?;
            data.push(number);
        }
    }
    Ok(data)
}

#[derive(Copy, Clone)]
enum CostMode {
    Linear,
    Quadratic,
}

fn fuel_cost(pos: i64, target: i64, mode: CostMode) -> i64 {
    let distance = (target - pos).abs();
    match mode {
        CostMode::Linear => distance,
        CostMode::Quadratic => distance * (distance + 1) / 2
    }
}

fn total_fuel_cost(positions: &Vec<i64>, target: i64, mode: CostMode) -> i64 {
    let mut cost = 0;
    for &pos in positions {
        cost += fuel_cost(pos, target, mode);
    }
    cost
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let positions = read_data("data.txt").expect("Could not read data");
    let pmin = *positions.iter().min().unwrap();
    let pmax = *positions.iter().max().unwrap();
    let mut costs = HashMap::with_capacity((pmax - pmin) as usize);
    for pos in pmin..=pmax {
        costs.insert(pos, total_fuel_cost(&positions, pos, Linear));
    }
    // Find minimum pair
    let min_pos_cost = costs.iter().reduce(|p1, p2| if p1.1 < p2.1 {p1} else {p2}).expect("Could not reduce");
    let min_pos  = *min_pos_cost.0;
    let min_cost = *min_pos_cost.1;
    println!("Minimum cost is {} at pos {}", min_cost, min_pos);
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let positions = read_data("data.txt").expect("Could not read data");
    let pmin = *positions.iter().min().unwrap();
    let pmax = *positions.iter().max().unwrap();
    let mut costs = HashMap::with_capacity((pmax - pmin) as usize);
    for pos in pmin..=pmax {
        costs.insert(pos, total_fuel_cost(&positions, pos, Quadratic));
    }
    // Find minimum pair
    let min_pos_cost = costs.iter().reduce(|p1, p2| if p1.1 < p2.1 {p1} else {p2}).expect("Could not reduce");
    let min_pos  = *min_pos_cost.0;
    let min_cost = *min_pos_cost.1;
    println!("Minimum cost is {} at pos {}", min_cost, min_pos);
}

fn main() {
    part1();
    part2();
}
