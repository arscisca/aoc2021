mod octopus;
mod map;

use std::fs::File;
use std::io::Read;
use map::Map;


fn part1() {
    const NSTEPS: usize = 100;
    const PRINT_INTERVAL: usize = 10;

    println!("{:-<80}", "Part 1 ");

    let mut f = File::open("data.txt").expect("Could not open file");
    let mut map = String::new();
    f.read_to_string(&mut map).expect("Could not read file");
    let mut map = map.parse::<Map>().expect("Could not parse map");
    println!("Initial step:");
    println!("{}", map);
    let mut flashes = 0;
    for i in 0..NSTEPS {
        flashes += map.update();
        if (i + 1) % PRINT_INTERVAL == 0 {
            println!("After step {}:\n{}", i + 1, map);
        }
    }
    println!("Total flashes: {}", flashes);
}

fn part2() {
    println!("{:-<80}", "Part 2 ");

    let mut f = File::open("data.txt").expect("Could not open file");
    let mut map = String::new();
    f.read_to_string(&mut map).expect("Could not read file");
    let mut map = map.parse::<Map>().expect("Could not parse map");
    let mut step = 0;
    while map.update() != (map.width() * map.height()) as u64 {
        step += 1;
        debug_assert!(step < 1000, "This is taking too long!");
    }
    println!("First simultaneous flash: {}", step + 1);
}


fn main() {
    part1();
    part2();
}
