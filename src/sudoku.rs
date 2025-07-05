use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Given(u8),
    Filled(u8),
}

impl Cell {
    pub fn value(&self) -> Option<u8> {
        match self {
            Cell::Empty => None,
            Cell::Given(v) | Cell::Filled(v) => Some(*v),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    pub fn is_given(&self) -> bool {
        matches!(self, Cell::Given(_))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sudoku {
    pub grid: Vec<Vec<Cell>>,
    pub size: usize,
    pub box_size: usize,
}

impl Sudoku {
    pub fn new(size: usize) -> Self {
        let box_size = (size as f64).sqrt() as usize;
        if box_size * box_size != size {
            panic!("Invalid Sudoku size: {} is not a perfect square", size);
        }

        Self {
            grid: vec![vec![Cell::Empty; size]; size],
            size,
            box_size,
        }
    }

    pub fn from_string(s: &str, size: usize) -> Result<Self, String> {
        let mut sudoku = Self::new(size);
        let chars: Vec<char> = s.chars().collect();

        if chars.len() != size * size {
            return Err(format!(
                "Invalid input length: expected {}, got {}",
                size * size,
                chars.len()
            ));
        }

        for (i, &ch) in chars.iter().enumerate() {
            let row = i / size;
            let col = i % size;

            match ch {
                '0' | '.' | ' ' => sudoku.grid[row][col] = Cell::Empty,
                _ => {
                    if let Some(digit) = ch.to_digit(10) {
                        let value = digit as u8;
                        if value >= 1 && value <= size as u8 {
                            sudoku.grid[row][col] = Cell::Given(value);
                        } else {
                            return Err(format!(
                                "Invalid digit {} at position ({}, {})",
                                value, row, col
                            ));
                        }
                    } else if ch.is_ascii_uppercase() {
                        // Support hexadecimal for larger Sudokus
                        let value = ch as u8 - b'A' + 10;
                        if value >= 10 && value <= size as u8 {
                            sudoku.grid[row][col] = Cell::Given(value);
                        } else {
                            return Err(format!(
                                "Invalid character {} at position ({}, {})",
                                ch, row, col
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Invalid character {} at position ({}, {})",
                            ch, row, col
                        ));
                    }
                }
            }
        }

        Ok(sudoku)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        if row < self.size && col < self.size {
            Some(self.grid[row][col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) -> Result<(), String> {
        if row >= self.size || col >= self.size {
            return Err("Invalid position".to_string());
        }

        if value == 0 {
            self.grid[row][col] = Cell::Empty;
        } else if value > self.size as u8 {
            return Err(format!(
                "Value {} is too large for {}x{} Sudoku",
                value, self.size, self.size
            ));
        } else {
            self.grid[row][col] = Cell::Filled(value);
        }

        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_rows() && self.is_valid_cols() && self.is_valid_boxes()
    }

    pub fn is_valid_rows(&self) -> bool {
        for row in 0..self.size {
            let mut seen = HashSet::new();
            for col in 0..self.size {
                if let Some(value) = self.grid[row][col].value() {
                    if !seen.insert(value) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn is_valid_cols(&self) -> bool {
        for col in 0..self.size {
            let mut seen = HashSet::new();
            for row in 0..self.size {
                if let Some(value) = self.grid[row][col].value() {
                    if !seen.insert(value) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn is_valid_boxes(&self) -> bool {
        for box_row in 0..self.box_size {
            for box_col in 0..self.box_size {
                let mut seen = HashSet::new();
                for row in box_row * self.box_size..(box_row + 1) * self.box_size {
                    for col in box_col * self.box_size..(box_col + 1) * self.box_size {
                        if let Some(value) = self.grid[row][col].value() {
                            if !seen.insert(value) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn is_complete(&self) -> bool {
        for row in 0..self.size {
            for col in 0..self.size {
                if self.grid[row][col].is_empty() {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_candidates(&self, row: usize, col: usize) -> HashSet<u8> {
        if !self.grid[row][col].is_empty() {
            return HashSet::new();
        }

        let mut candidates: HashSet<u8> = (1..=self.size as u8).collect();

        // Remove values in the same row
        for c in 0..self.size {
            if let Some(value) = self.grid[row][c].value() {
                candidates.remove(&value);
            }
        }

        // Remove values in the same column
        for r in 0..self.size {
            if let Some(value) = self.grid[r][col].value() {
                candidates.remove(&value);
            }
        }

        // Remove values in the same box
        let box_row = row / self.box_size;
        let box_col = col / self.box_size;
        for r in box_row * self.box_size..(box_row + 1) * self.box_size {
            for c in box_col * self.box_size..(box_col + 1) * self.box_size {
                if let Some(value) = self.grid[r][c].value() {
                    candidates.remove(&value);
                }
            }
        }

        candidates
    }

    pub fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for row in 0..self.size {
            for col in 0..self.size {
                if self.grid[row][col].is_empty() {
                    return Some((row, col));
                }
            }
        }
        None
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.size {
            if row > 0 && row % self.box_size == 0 {
                writeln!(f, "{}", "-".repeat(self.size * 2 + self.box_size - 1))?;
            }

            for col in 0..self.size {
                if col > 0 && col % self.box_size == 0 {
                    write!(f, "|")?;
                }

                match self.grid[row][col] {
                    Cell::Empty => write!(f, ". ")?,
                    Cell::Given(v) | Cell::Filled(v) => {
                        if v <= 9 {
                            write!(f, "{} ", v)?;
                        } else {
                            write!(f, "{} ", (b'A' + v - 10) as char)?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
