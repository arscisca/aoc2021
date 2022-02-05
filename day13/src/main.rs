mod paper;
use paper::Paper;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

enum Fold {
    Left(i32),
    Up(i32),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("fold along ") {
            if s.is_empty() {
                return Err(());
            }
            match s.chars().next().unwrap() {
                'x' => return Ok(Fold::Left(s[2..].parse().expect("Could not parse fold"))),
                'y' => return Ok(Fold::Up(s[2..].parse().expect("Could not parse fold"))),
                _   => return Err(()),
            }
        } else {
            Err(())
        }
    }
}

fn parse(f: File) -> (Paper, Vec<Fold>) {
    let mut paper = Paper::new();
    let mut folds = Vec::new();

    enum State {
        ReadingDots,
        ReadingFolds,
    }
    let mut state = State::ReadingDots;

    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        match state {
            State::ReadingDots => {
                if line.is_empty() {
                    state = State::ReadingFolds;
                    continue;
                }
                let parts: Vec<&str> = line.split(",").collect();
                let x: i32 = parts[0].parse().expect("Could not read number");
                let y: i32 = parts[1].parse().expect("Could not read number");
                paper.add_dot(x, y);
            }
            State::ReadingFolds => {
                let fold: Fold = line.parse().expect("Could not parse fold");
                folds.push(fold);
            }
        }
    }
    (paper, folds)
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let (mut paper, folds) = parse(File::open("data.txt").expect("Could not open file"));
    let fold = &folds[0];
    match *fold {
        Fold::Up(y) => paper.fold_y(y),
        Fold::Left(x) => paper.fold_x(x),
    };
    println!("Dots: {}", paper.num_dots());
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let (mut paper, folds) = parse(File::open("data.txt").expect("Could not open file"));
    for fold in folds {
        match fold {
            Fold::Up(y) => paper.fold_y(y),
            Fold::Left(x) => paper.fold_x(x),
        }
    }
    println!("{}", paper);
}

fn main() {
    part1();
    part2();
}
