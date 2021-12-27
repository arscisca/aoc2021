use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::display::{Digit, Display};

mod input;
mod display;

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let inputs = read_data("data.txt").expect("Could not read data");
    let mut count = 0;
    for input in &inputs {
        for output in &input.output {
            match output.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => (),
            }
        }
    }
    println!("Count: {}", count)
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let inputs = read_data("data.txt").expect("Could not read data");
    let mut sum = 0;
    for input in inputs {
        let patterns = input.patterns.map(|pattern| pattern.parse::<Digit>().expect("Could not parse digit"));
        let corrected_digits = correct_digits(patterns).expect("Could not correct digits");
        let mapping = create_mapping(&corrected_digits);
        // Create output display
        let mut display = Display::new();
        let outputs = input.output.map(|output| output.parse::<Digit>().expect("Could not parse output digit"));
        for (i, digit) in outputs.iter().enumerate() {
            display[3 - i] = *digit;
        }
        display.apply_digit_mapping(&mapping);
        sum += display.as_number().expect("Could not convert to number");
    }
    println!("Sum: {}", sum);
}

fn read_data(fname: &str) -> Result<Vec<input::InputLine>, Box<dyn Error>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let mut inputs = Vec::new();
    for line in reader.lines() {
        let iline = line?.parse::<input::InputLine>()?;
        inputs.push(iline);
    }
    Ok(inputs)
}

fn correct_digits(digits: [Digit; 10]) -> Result<[Digit; 10], String> {
    let mut corrected = [Digit::new(); 10];
    // Identify easy to find digits (1, 4, 7, 8) and classify others based on length
    let mut digits_len_5 = HashSet::new();
    let mut digits_len_6 = HashSet::new();
    for digit in &digits {
        match digit.count_active_segments() {
            2 => corrected[1] = *digit,
            3 => corrected[7] = *digit,
            4 => corrected[4] = *digit,
            5 => { digits_len_5.insert(*digit); },
            6 => { digits_len_6.insert(*digit); },
            7 => corrected[8] = *digit,
            _ => return Err(String::from("Invalid digit")),
        }
    }
    // Classify digits of length 5
    // Find 3
    for digit in &digits_len_5 {
        if (digit.clone() & !corrected[1].clone()).count_active_segments() == 3 {
            corrected[3] = *digit;
            break;
        }
    }
    if !digits_len_5.remove(&corrected[3]) {
        return Err(String::from("Digit 3 not recognized"));
    }
    // Find 2 and 5
    for digit in &digits_len_5 {
        let len = (digit.clone() & !corrected[4].clone()).count_active_segments();
        if len == 3 {
            corrected[2] = *digit;
        } else if len == 2 {
            corrected[5] = *digit;
        } else {
            return Err(String::from("Invalid digit"));
        }
    }
    digits_len_5.clear();
    // Classify digits of length 6
    let filter = corrected[3].clone() & !corrected[1].clone();
    for digit in &digits_len_6 {
        let len = (digit.clone() & filter.clone()).count_active_segments();
        if len == 2 {
            corrected[0] = *digit;
        }
    }
    if !digits_len_6.remove(&corrected[0]) {
        return Err(String::from("Digit 0 not recognized"));
    }
    for digit in &digits_len_6 {
        let len = (digit.clone() & corrected[1].clone()).count_active_segments();
        if len == 2 {
            corrected[9] = *digit;
        } else {
            corrected[6] = *digit;
        }
    }
    Ok(corrected)
}

fn create_mapping(digits: &[Digit; 10]) -> HashMap<Digit, Digit> {
    let mut mapping = HashMap::new();
    for (i, digit) in digits.iter().enumerate() {
        mapping.insert(*digit, Digit::from_number(i as u8));
    }
    mapping
}

fn main() {
    part1();
    part2();
}
