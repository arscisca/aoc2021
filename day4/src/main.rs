use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::bingo::Board;

mod bingo;

fn read_generated_numbers(reader: &mut BufReader<File>) -> Vec<u8> {
    let mut line = String::new();
    reader.read_line(&mut line).expect("Could not read generated numbers");
    // Count number of inputs
    let mut nnumbers = 1;
    for c in line.chars() {
        if c == ',' {
            nnumbers += 1;
        }
    }
    let nnumbers = nnumbers;
    let mut numbers = Vec::with_capacity(nnumbers);
    for number in line.trim().split(",") {
        let number: u8 = number.parse().expect("Could not read number");
        numbers.push(number);
    }
    numbers
}

fn print_boards(boards: &Vec<Board>) {
    for (i, board) in boards.iter().enumerate() {
        println!("Board {}:\n{}\n", i, board);
    }
}

fn part1() {
    println!("{:-<80}", "Part 1 ");
    let f = File::open("data.txt").expect("Could not open file");
    let mut reader = BufReader::new(f);
    let numbers = read_generated_numbers(&mut reader);

    // Read all available boards
    let mut boards = Vec::new();
    loop {
        match Board::read(&mut reader) {
            Ok(board) => boards.push(board),
            Err(_) => break,
        }
    }
    // Print info about boards
    println!("Found {} boards", boards.len());
    // Find winner board
    let mut winner: Option<usize> = None;
    'extraction: for number in numbers {
        for board in &mut boards {
            board.mark_if_present(number);
        }
        for (i, board) in boards.iter().enumerate() {
            if board.wins() {
                winner = Some(i);
                break 'extraction;
            }
        }
    }
    if let Some(index) = winner {
        let winner = &boards[index];
        println!("Winner board:\n{}Score: {}", winner, winner.score()
                                                              .expect("Could not extract winner score"));
    } else {
        print!("There is no winner");
    }
}

fn part2() {
    println!("{:-<80}", "Part 2 ");
    let f = File::open("data.txt").expect("Could not open file");
    let mut reader = BufReader::new(f);
    let numbers = read_generated_numbers(&mut reader);

    // Read all available boards
    let mut boards = Vec::new();
    loop {
        match Board::read(&mut reader) {
            Ok(board) => boards.push(board),
            Err(_) => break,
        }
    }
    // Print info about boards
    println!("Found {} boards", boards.len());

    // Find last winner board
    for number in numbers {
        if boards.len() > 1 {
            for board in &mut boards {
                board.mark_if_present(number);
            }
            boards.retain(|board| !board.wins());
        } else {
            boards[0].mark_if_present(number);
            if boards[0].wins() {
                break;
            }
        }
    }
    let mut last_board = boards[0];
    boards.clear();
    if last_board.wins() {
        println!("Last winning board:\n{}Score: {}", last_board, last_board.score().expect("No winner score"));
    } else {
        println!("No last board won");
    }
}

fn main() {
    part1();
    part2();
}
