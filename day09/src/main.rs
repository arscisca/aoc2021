use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

mod mapping;

fn read_map(fname: &str) -> Result<mapping::Map, ParseIntError> {
    let mut f = File::open(fname).expect("Could not open file");
    let mut map = String::new();
    f.read_to_string(&mut map).expect("Could not read file");
    map.parse::<mapping::Map>()
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let map = read_map("data.txt").expect("Could not parse map");
    let mut sum = 0u64;
    for point in map.low_points() {
        let score = point + 1;
        sum += score as u64;
    }
    println!("Total score: {}", sum);
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let map = read_map("data.txt").expect("Could not parse map");
    let mut basins = map.basins().collect::<Vec<mapping::Basin>>();
    basins.sort_by(|b1, b2| b1.size().cmp(&b2.size()));
    basins.reverse();
    println!("Basins:");
    for (i, basin) in basins.iter().enumerate() {
        println!("{}) size: {} | {:?}", i, basin.size(), basin.points());
    }
    assert!(basins.len() > 3, "Not enough basins for final result!");
    println!("Product: {}", basins[0].size() * basins[1].size() * basins[2].size());
}

fn main() {
    part1();
    part2();
}
