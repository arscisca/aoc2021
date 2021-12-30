use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::bracket::Bracket;
use crate::bracket::Mode::Open;
use crate::stack::Stack;

mod stack;

mod bracket {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Kind {
        Round,
        Square,
        Angle,
        Curly
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Mode {
        Open,
        Close
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Bracket {
        pub kind: Kind,
        pub mode: Mode,
    }

    impl Bracket {
        pub fn complementary(&self) -> Bracket {
            Bracket {
                kind: self.kind,
                mode: match self.mode {
                    Mode::Open => Mode::Close,
                    Mode::Close => Mode::Open,
                }
            }
        }
    }

    impl From<char> for Bracket {
        fn from(c: char) -> Self {
            match c {
                '(' => Bracket {kind: Kind::Round, mode: Mode::Open},
                ')' => Bracket {kind: Kind::Round, mode: Mode::Close},
                '[' => Bracket {kind: Kind::Square, mode: Mode::Open},
                ']' => Bracket {kind: Kind::Square, mode: Mode::Close},
                '{' => Bracket {kind: Kind::Curly, mode: Mode::Open},
                '}' => Bracket {kind: Kind::Curly, mode: Mode::Close},
                '<' => Bracket {kind: Kind::Angle, mode: Mode::Open},
                '>' => Bracket {kind: Kind::Angle, mode: Mode::Close},
                _ => panic!("Invalid character"),
            }
        }
    }
}

const fn error_score(bracket: &Bracket) -> u64 {
    match bracket.kind {
        bracket::Kind::Round => 3,
        bracket::Kind::Square => 57,
        bracket::Kind::Curly => 1197,
        bracket::Kind::Angle => 25137,
    }
}

const fn autocomplete_score(bracket: &Bracket) -> u64 {
    match bracket.kind {
        bracket::Kind::Round => 1,
        bracket::Kind::Square => 2,
        bracket::Kind::Curly => 3,
        bracket::Kind::Angle => 4,
    }
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let fname = "data.txt";
    let f = File::open(fname).expect("Could not open file");
    let reader = BufReader::new(f);
    let mut score = 0;
    for (i, line) in reader.lines().enumerate() {
        let mut stack: Stack<Bracket> = Stack::new();
        for (j, c) in line.expect("Could not read line").chars().enumerate() {
            let bracket = bracket::Bracket::from(c);
            if !stack.is_empty() && stack.top().unwrap().complementary() == bracket {
                // Bracket simplifies with the top of the stack
                stack.pop();
            } else {
                if bracket.mode == Open {
                    // New bracket has opened
                    stack.push(bracket);
                } else {
                    // Found an error!
                    println!("{}:{}:{}: Illegal bracket {:?}", fname, i + 1, j + 1, bracket);
                    score += error_score(&bracket);
                    break;
                }
            }
        }
    }
    println!("Score: {}", score);
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let fname = "data.txt";
    let f = File::open(fname).expect("Could not open file");
    let reader = BufReader::new(f);
    let mut scores = Vec::new();
    for line in reader.lines() {
        let mut stack: Stack<Bracket> = Stack::new();
        for c in line.expect("Could not read line").chars() {
            let bracket = bracket::Bracket::from(c);
            if !stack.is_empty() && stack.top().unwrap().complementary() == bracket {
                // Bracket simplifies with the top of the stack
                stack.pop();
            } else {
                if bracket.mode == Open {
                    // New bracket has opened
                    stack.push(bracket);
                } else {
                    // Found an error!
                    stack.clear();
                    break;
                }
            }
        }
        // Autocomplete lines
        if !stack.is_empty() {
            let mut score = 0;
            println!("{:?}", stack);
            while !stack.is_empty() {
                let current = stack.pop().expect("Invalid stack status");
                score = score * 5 + autocomplete_score(&current.complementary());
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("Middle score: {}", scores[scores.len() / 2]);
}

fn main() {
    part1();
    part2();
}
