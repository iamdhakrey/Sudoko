//! Terminal User Interface for the Sudoku solver using a simple text interface

use std::io::{self, Write};
use sudoko::{Sudoku, SudokuSolver};

struct SudokuApp {
    sudoku: Sudoku,
    solver: SudokuSolver,
    selected: (usize, usize),
    message: String,
}

impl SudokuApp {
    fn new() -> Self {
        Self {
            sudoku: Sudoku::new(9),
            solver: SudokuSolver::new(),
            selected: (0, 0),
            message: "Use WASD to move, 1-9 to fill, S to solve, Q to quit.".to_string(),
        }
    }

    fn handle_input(&mut self, input: char) -> bool {
        match input.to_ascii_lowercase() {
            'q' => return false, // quit
            'w' => {
                if self.selected.0 > 0 {
                    self.selected.0 -= 1;
                }
            }
            's' => {
                if input == 's' {
                    // Solve puzzle
                    match self.solver.solve(self.sudoku.clone()) {
                        Ok(solution) => {
                            self.sudoku = solution;
                            self.message = "Puzzle solved!".to_string();
                        }
                        Err(_) => {
                            self.message = "No solution found.".to_string();
                        }
                    }
                } else if self.selected.0 < self.sudoku.size - 1 {
                    self.selected.0 += 1;
                }
            }
            'a' => {
                if self.selected.1 > 0 {
                    self.selected.1 -= 1;
                }
            }
            'd' => {
                if self.selected.1 < self.sudoku.size - 1 {
                    self.selected.1 += 1;
                }
            }
            '1'..='9' => {
                let n = input.to_digit(10).unwrap() as u8;
                if n <= self.sudoku.size as u8 {
                    if let Err(e) = self.sudoku.set(self.selected.0, self.selected.1, n) {
                        self.message = format!("Error: {}", e);
                    } else {
                        self.message = format!(
                            "Set {} at ({}, {})",
                            n,
                            self.selected.0 + 1,
                            self.selected.1 + 1
                        );
                    }
                }
            }
            '0' => {
                if let Err(e) = self.sudoku.set(self.selected.0, self.selected.1, 0) {
                    self.message = format!("Error: {}", e);
                } else {
                    self.message = "Cleared cell".to_string();
                }
            }
            _ => {}
        }
        true
    }

    fn render_simple(&self) {
        println!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top-left

        println!("{}", self.message);
        println!();

        for row in 0..self.sudoku.size {
            if row > 0 && row % self.sudoku.box_size == 0 {
                println!(
                    "{}",
                    "-".repeat(self.sudoku.size * 3 + self.sudoku.box_size - 1)
                );
            }

            for col in 0..self.sudoku.size {
                if col > 0 && col % self.sudoku.box_size == 0 {
                    print!("|");
                }

                let cell_value = self
                    .sudoku
                    .get(row, col)
                    .and_then(|cell| cell.value())
                    .unwrap_or(0);

                let ch = if cell_value == 0 {
                    '.'
                } else if cell_value <= 9 {
                    char::from_digit(cell_value as u32, 10).unwrap()
                } else {
                    // For values > 9, use letters A, B, C, etc.
                    char::from(b'A' + cell_value - 10)
                };

                if (row, col) == self.selected {
                    print!("[{}]", ch);
                } else {
                    print!(" {} ", ch);
                }
            }
            println!();
        }

        println!();
        println!(
            "Selected: ({}, {})",
            self.selected.0 + 1,
            self.selected.1 + 1
        );
        print!("Enter command: ");
        io::stdout().flush().unwrap();
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut app = SudokuApp::new();

    // Load example puzzle
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    if let Ok(puzzle) = Sudoku::from_string(puzzle_str, 9) {
        app.sudoku = puzzle;
        app.message =
            "Example puzzle loaded. Use WASD to move, 1-9 to fill, S to solve, Q to quit."
                .to_string();
    }

    loop {
        app.render_simple();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Some(ch) = input.trim().chars().next() {
            if !app.handle_input(ch) {
                break;
            }
        }
    }

    println!("Thanks for playing!");
    Ok(())
}
