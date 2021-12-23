use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod oceanfloor {
    pub mod map;
    pub mod point;
    pub mod rect;
    pub mod segment;
}
use oceanfloor::map::Map;
use oceanfloor::segment::Segment;

fn read_segments<F>(fname: &str, filter: F) -> Result<Vec<Segment>, Box<dyn Error>>
where
    F: Fn(&Segment) -> bool,
{
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let mut segments = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let segment = line.trim().parse::<Segment>()?;
        if filter(&segment) {
            segments.push(segment);
        }
    }
    Ok(segments)
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let m = Map::from(
        &read_segments(&"data.txt", |&segment| {
            segment.is_horizontal() || segment.is_vertical()
        })
        .expect("Could not read segments"),
    );
    // println!("{}", m);
    println!(
        "Points where at least two lines overlap: {}",
        m.count_overlaps(2)
    );
}

fn part2() {
    println!("{:-<80}", "Part 1 ");
    let m = Map::from(
        &read_segments(&"data.txt", |&segment| {
            segment.is_horizontal() || segment.is_vertical() || segment.is_diagonal()
        })
        .expect("Could not read segments"),
    );
    println!(
        "Points where at least two lines overlap: {}",
        m.count_overlaps(2)
    );
}

fn main() {
    part1();
    part2();
}
