use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Polymer = HashMap<(char, char), u64>;
type Rules = HashMap<(char, char), char>;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let (mut polymer, rules) = read_data(File::open("data.txt")
        .expect("Could not open file"))
        .expect("Could not read data");

    for _ in 0..10 {
        polymer = extend(polymer, &rules);
    }

    println!("Result: {}", get_result(&polymer));
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let (mut polymer, rules) = read_data(File::open("data.txt")
        .expect("Could not open file"))
        .expect("Could not read data");

    for _ in 0..40 {
        polymer = extend(polymer, &rules);
    }

    println!("Result: {}", get_result(&polymer));
}

fn extend(polymer: Polymer, rules: &Rules) -> Polymer {
    let mut result = Polymer::new();
    for (pair, quantity) in polymer.into_iter() {
        if let Some(element) = rules.get(&pair) {
            // Insert new element: XY -> XEY. The pairs AB break and new pairs AE, EB are born
            *result.entry((pair.0, *element)).or_insert(0) += quantity;
            *result.entry((*element, pair.1)).or_insert(0) += quantity;
        } else {
            *result.entry(pair).or_insert(0) += quantity;
        }
    }
    result
}

fn read_data(f: File) -> Result<(Polymer, Rules), Box<dyn Error>> {
    let mut polymer = Polymer::new();
    let mut rules = Rules::new();
    enum State {
        ReadingPolymer,
        ReadingSpacer,
        ReadingRules,
    }
    let reader = BufReader::new(f);
    let mut state = State::ReadingPolymer;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        match state {
            State::ReadingPolymer => {
                polymer = string_to_polymer(&line);
                state = State::ReadingSpacer;
            }
            State::ReadingSpacer => {
                state = State::ReadingRules;
            }
            State::ReadingRules => {
                let parts: Vec<&str> = line.split("->").collect();
                if parts.len() != 2 {
                    return Err(String::from("Invalid line formatting").into());
                }
                let pair: Vec<char> = parts[0].trim().chars().collect();
                let element = parts[1].trim().chars().collect::<Vec<char>>()[0];
                rules.insert((pair[0], pair[1]), element);
            }
        }
    }
    Ok((polymer, rules))
}

fn string_to_polymer(string: &str) -> Polymer {
    let mut polymer = Polymer::new();
    let elements: Vec<char> = string.chars().collect();
    for i in 1..elements.len() {
        let pair = (elements[i - 1], elements[i]);
        *polymer.entry(pair).or_insert(0) += 1;
    }
    polymer
}

fn get_result(polymer: &Polymer) -> u64 {
    let mut values = vec![0; ('Z' as u8 - 'A' as u8) as usize];
    for (pair, quantity) in polymer {
        // values[(pair.0 as u8 - 'A' as u8) as usize] += quantity;
        values[(pair.1 as u8 - 'A' as u8) as usize] += quantity;
    }
    values = values.into_iter().filter(|x| *x > 0).collect();
    values.sort_unstable();
    values.last().unwrap() - values.first().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn extend() {
        let mut polymer = string_to_polymer("AB");
        let rules = Rules::from([
            (('A', 'B'), 'B'),
            (('B', 'B'), 'A')
        ]);
        let expected = [
            "ABB",
            "ABBAB",
            "ABBABABB",
            "ABBABABBABBAB"
        ].map(|s| string_to_polymer(s));
        for (i, exp) in expected.into_iter().enumerate() {
            polymer = super::extend(polymer, &rules);
            assert_eq!(polymer, exp, "Invalid polymer at iteration {}", i);
        }
    }
}