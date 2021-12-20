use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::fmt::{format, Formatter};
use std::fs::File;
use std::ptr::write;

const BOARD_ROWS: usize = 5;
const BOARD_COLS: usize = 5;

#[derive(Copy, Clone)]
pub struct Cell {
    pub number: u8,
    pub marked: bool,
}

impl Cell {
    pub fn new(number: u8) -> Cell {
        Cell {number, marked: false}
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    cells: [Cell; BOARD_ROWS * BOARD_COLS],
    winning_number: Option<u8>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::new(0); BOARD_ROWS * BOARD_COLS],
            winning_number: None,
        }
    }

    pub fn read(reader: &mut BufReader<File>) -> Result<Board, Error> {
        let mut board = Board::new();
        for i in 0..BOARD_ROWS {
            // Read line
            let mut line = String::new();
            while line.is_empty() {
                match reader.read_line(&mut line) {
                    Ok(bytes) => {
                        if bytes == 0 {
                            break;
                        }
                    },
                    Err(e) => return Err(e),
                }
                line = String::from(line.trim());
            }
            // Parse numbers
            let mut columns = 0;
            for (j, number) in line.trim().split_whitespace().enumerate() {
                let number: u8 = number.parse().expect("Could not parse number");
                board.get_cell_mut(i as u8, j as u8).number = number;
                columns = j + 1;
            }
            if columns < BOARD_COLS {
                return Err(Error::new(ErrorKind::InvalidInput, "Not enough input numbers"))
            }
        }
        Ok(board)
    }

    fn check_access(row: u8, col: u8) {
        assert!((row as usize) < BOARD_ROWS, "Invalid row {}", row);
        assert!((col as usize) < BOARD_ROWS, "Invalid column {}", col);
    }

    pub fn get_cell(&self, row: u8, col: u8) -> &Cell {
        Board::check_access(row, col);
        &self.cells[(row as usize) * BOARD_ROWS + col as usize]
    }

    pub fn get_cell_mut(&mut self, row: u8, col: u8) -> &mut Cell {
        Board::check_access(row, col);
        &mut self.cells[(row as usize) * BOARD_ROWS + col as usize]
    }

    pub fn find_number(&self, num: u8) -> Option<&Cell> {
        for cell in &self.cells {
            if cell.number == num {
                return Some(&cell);
            }
        }
        None
    }

    pub fn find_number_mut(&mut self, num: u8) -> Option<&mut Cell> {
        for cell in &mut self.cells {
            if cell.number == num {
                return Some(&mut *cell);
            }
        }
        None
    }

    pub fn mark_if_present(&mut self, num: u8) -> bool {
        if let Some(cell) = self.find_number_mut(num) {
            cell.marked = true;
            if self.check_winning() {
                self.winning_number = Some(num);
            }
            return true;
        }
        false
    }

    pub fn wins(&self) -> bool {
        self.winning_number.is_some()
    }

    pub fn winning_number(&self) -> Option<u8> {
        self.winning_number
    }

    fn check_winning(&self) -> bool {
        // Check if any row is complete
        for i in 0..BOARD_ROWS as u8 {
            let mut row_marked = true;
            for j in 0..BOARD_COLS as u8 {
                if !self.get_cell(i, j).marked {
                    row_marked = false;
                    break;
                }
            };
            if row_marked {
                return true;
            }
        }
        // Check if any column is complete
        for j in 0..BOARD_COLS as u8 {
            let mut column_marked = true;
            for i in 0..BOARD_ROWS as u8 {
                if !self.get_cell(i, j).marked {
                    column_marked = false;
                    break;
                }
            };
            if column_marked {
                return true;
            }
        }
        false
    }

    pub fn score(&self) -> Option<u64> {
        if let Some(winning_number) = self.winning_number {
            let mut score = 0u64;
            for cell in self.cells {
                if !cell.marked {
                    score += cell.number as u64;
                }
            }
            score *= winning_number as u64;
            Some(score)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for i in 0..BOARD_ROWS as u8 {
            for j in 0..BOARD_COLS as u8{
                let cell = self.get_cell(i, j);
                if cell.marked {
                    write!(f, "{:>6}", format!("({:>2})", cell.number))?;
                } else {
                    write!(f, "{:>6}", cell.number)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}