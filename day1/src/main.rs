use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_increases(fname: &str, wsize: usize) -> u64 {
    assert!(wsize > 0, "Window size must be greater than 0");
    let mut window: Vec<u32> = Vec::with_capacity(wsize);
    for _ in 0..wsize {
        window.push(0);
    }
    let mut prev_sum: Option<u32> = None;
    let mut iteration = 0;
    let mut n_increases = 0;
    let f = File::open(fname).expect("Could not open file");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let value: u32 = line
            .expect("Could not read line")
            .trim()
            .parse()
            .expect("Could not convert to u32");
        // Update window with new value
        for i in (1..wsize).rev() {
            window[i] = window[i - 1];
        }
        window[0] = value;

        iteration += 1;
        if iteration < wsize {
            continue;
        }
        // Compute the sum of the element in windows
        let mut sum = 0;
        for x in &window {
            sum += x;
        }
        // Check if sum has grown
        if sum > prev_sum.unwrap_or(sum) {
            n_increases += 1;
        }
        prev_sum = Some(sum);
    }
    n_increases
}

fn part1() {
    let n_increases = count_increases("data.txt", 1);
    println!("There were a total of {} increases", n_increases)
}

fn part2() {
    let n_increases = count_increases("data.txt", 3);
    println!("There were a total of {} increases", n_increases)
}

fn main() {
    println!("{:-<80}", "Part 1 ");
    part1();
    println!("{:-<80}", "Part 2 ");
    part2();
}
