use std::error::Error;
use std::fs::{File, read};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::num::ParseIntError;
use std::fmt;
use std::fmt::{Binary, format};
use std::cmp::Ordering;
use std::collections::HashMap;

struct BinaryNumber {
    num: u64,
    len: u8,
}

impl BinaryNumber {
    fn new(len: u8) -> BinaryNumber {
        assert!(len < 64, "Maximum size is 64");
        BinaryNumber{num: 0, len}
    }

    fn from(num: u64, len: u8) -> BinaryNumber {
        BinaryNumber{num, len}
    }

    fn from_binary_str(string: &str) -> Result<BinaryNumber, ParseIntError> {
        assert!(string.len() < 64, "Maximum size is 64");
        let num = u64::from_str_radix(string, 2)?;
        Ok(BinaryNumber{num, len: string.len() as u8})
    }

    fn len(&self) -> u8 {
        self.len
    }

    fn bit(&self, pos: u8) -> u8 {
        assert!(pos < self.len, "Invalid bit position");
        ((self.num >> pos) & 0x1) as u8
    }

    fn set_bit(&mut self, pos: u8, val: u8) {
        assert!(pos < self.len,       "Invalid bit position");
        if val == 0 {
            self.num &= !(0x1 << pos) as u64;
        } else if val == 1 {
            self.num |= (0x1 << pos) as u64;
        } else {
            panic!("Invalid bit value");
        }
    }
}

impl fmt::Binary for BinaryNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>16b}", self.num)
    }
}
impl fmt::Display for BinaryNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

struct BitCounter {
    counts: Vec<u64>,
    entries: u64,
}

impl BitCounter {
    fn new(size: usize) -> BitCounter {
        assert!(size < 64, "Maximum size is 64");
        BitCounter {
            counts: vec![0; size],
            entries: 0
        }
    }

    fn from(values: &HashMap<u64, u64>, size: usize) -> BitCounter {
        let mut counter = BitCounter::new(size);
        for (value, count) in values {
            let binarynum = BinaryNumber::from(*value, size as u8);
            counter.count_multiple(binarynum, *count);
        }
        counter
    }

    fn count(&mut self, num: BinaryNumber) {
        self.count_multiple(num, 1);
    }

    fn count_multiple(&mut self, num: BinaryNumber, cnt: u64) {
        assert_eq!(num.len() as usize, self.counts.len(), "Size mismatch");
        for i in 0..num.len as u8{
            self.counts[i as usize] += (num.bit(i) as u64) * cnt;
        }
        self.entries += cnt;
    }

    fn most_common_bit(&self, pos: u8) -> Option<u8> {
        match self.counts[pos as usize].cmp(&((self.entries + 1) / 2)) {
            Ordering::Greater => Some(1),
            Ordering::Equal   => None,
            Ordering::Less    => Some(0),
        }
    }

    fn least_common_bit(&self, pos: u8) -> Option<u8> {
        match self.counts[pos as usize].cmp(&((self.entries + 1) / 2)) {
            Ordering::Greater => Some(0),
            Ordering::Equal   => None,
            Ordering::Less    => Some(1),
        }
    }
}

fn get_reader_and_entry_size(file: &File) -> (BufReader<&File>, usize) {
    let mut reader = BufReader::new(file);
    // Read first line
    let mut line = String::new();
    reader.read_line(&mut line).expect("Could not read line");
    let size = line.trim().len();
    // Reset reader
    reader.seek(SeekFrom::Start(0)).expect("Could not roll-back file reader");
    (reader, size)
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let f = File::open("data.txt").expect("Could not open file");
    let (reader, size) = get_reader_and_entry_size(&f);
    // Loop through lines
    let mut counter = BitCounter::new(size);
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let bnum = BinaryNumber::from_binary_str(&line).expect("Could not parse number");
        counter.count(bnum);
    }
    // Generate gamma
    let size = size as u8;
    let mut gamma = BinaryNumber::new(size);
    for i in 0..size {
        gamma.set_bit(i, counter.most_common_bit(i).unwrap_or(1));
    }
    // Generate epsilon
    let mut epsilon = BinaryNumber::new(size);
    for i in 0..size {
        epsilon.set_bit(i, counter.least_common_bit(i).unwrap_or(0));
    }
    println!("G = {}\tE = {}\tG*E = {}", gamma, epsilon, gamma.num * epsilon.num);
}

fn filter(mut data: HashMap<u64, u64>, pos: u8, val: u8) -> HashMap<u64, u64> {
    let mut numbers_to_remove = Vec::new();
    for number in data.keys() {
        let number = BinaryNumber::from(*number, 64);
        if number.bit(pos) != val {
            numbers_to_remove.push(number.num);
        }
    }
    for number_to_remove in &numbers_to_remove {
        data.remove(number_to_remove);
    }
    data
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let f = File::open("data.txt").expect("Could not open file");
    let (reader, size) = get_reader_and_entry_size(&f);
    let mut data = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let bnum = BinaryNumber::from_binary_str(&line).expect("Could not parse number");
        data.insert(bnum.num, data.get(&bnum.num).unwrap_or(&0) + 1);
    }
    // Make data immutable
    let data = data;
    // Filter data, starting from the MSB, to find the oxygen rating
    let size = size as u8;
    let mut oxygen_data = data.clone();
    let mut i = size - 1;
    while oxygen_data.len() > 1 {
        let counter = BitCounter::from(&oxygen_data, size as usize);
        let bit = counter.most_common_bit(i).unwrap_or(1);
        oxygen_data = filter(oxygen_data, i, bit);
        i = i.wrapping_sub(1);
    }
    let oxygen_rating = *oxygen_data.keys().collect::<Vec<&u64>>()[0];
    println!("O2: {}", oxygen_rating);
    // Filter data, starting from the MSB, to find the co2 rating
    let size = size as u8;
    let mut co2_data = data.clone();
    let mut i = size - 1;
    while co2_data.len() > 1 {
        let counter = BitCounter::from(&co2_data, size as usize);
        let bit = counter.least_common_bit(i).unwrap_or(0);
        co2_data = filter(co2_data, i, bit);
        i = i.wrapping_sub(1);
    }
    let co2_rating = *co2_data.keys().collect::<Vec<&u64>>()[0];
    println!("CO2: {}", co2_rating);
    println!("Product: {}", co2_rating * oxygen_rating);
}

fn main() {
    part1();
    part2();
}
