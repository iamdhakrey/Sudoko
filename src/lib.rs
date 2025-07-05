mod solver;
mod strategies;
mod sudoku;
mod utils;

pub use solver::{Difficulty, SudokuSolver};
pub use strategies::*;
pub use sudoku::{Cell, Sudoku};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, sudoko!");
}

// Export the solver for WASM
#[wasm_bindgen]
pub struct WasmSudokuSolver {
    solver: SudokuSolver,
}

#[wasm_bindgen]
impl WasmSudokuSolver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmSudokuSolver {
        utils::set_panic_hook();
        WasmSudokuSolver {
            solver: SudokuSolver::new(),
        }
    }

    #[wasm_bindgen]
    pub fn solve_from_string(&mut self, puzzle_str: &str, size: usize) -> String {
        match Sudoku::from_string(puzzle_str, size) {
            Ok(puzzle) => match self.solver.solve(puzzle) {
                Ok(solution) => {
                    // Convert solution to string format that's easy to parse in JS
                    let mut result = String::new();
                    for row in 0..solution.size {
                        for col in 0..solution.size {
                            if let Some(value) = solution.grid[row][col].value() {
                                if value <= 9 {
                                    result.push_str(&value.to_string());
                                } else {
                                    result.push((b'A' + value - 10) as char);
                                }
                            } else {
                                result.push('0');
                            }
                        }
                    }
                    result
                }
                Err(e) => format!("Error solving puzzle: {}", e),
            },
            Err(e) => format!("Error parsing puzzle: {}", e),
        }
    }

    #[wasm_bindgen]
    pub fn get_hint(&mut self, puzzle_str: &str, size: usize) -> String {
        match Sudoku::from_string(puzzle_str, size) {
            Ok(mut puzzle) => match self.solver.get_hint(&mut puzzle) {
                Some((row, col, value)) => format!("{}:{}:{}", row, col, value),
                None => "No hint available".to_string(),
            },
            Err(e) => format!("Error parsing puzzle: {}", e),
        }
    }

    #[wasm_bindgen]
    pub fn validate(&self, puzzle_str: &str, size: usize) -> bool {
        match Sudoku::from_string(puzzle_str, size) {
            Ok(puzzle) => puzzle.is_valid(),
            Err(_) => false,
        }
    }

    #[wasm_bindgen]
    pub fn is_complete(&self, puzzle_str: &str, size: usize) -> bool {
        match Sudoku::from_string(puzzle_str, size) {
            Ok(puzzle) => puzzle.is_complete(),
            Err(_) => false,
        }
    }

    #[wasm_bindgen]
    pub fn get_candidates(&self, puzzle_str: &str, size: usize, row: usize, col: usize) -> String {
        match Sudoku::from_string(puzzle_str, size) {
            Ok(puzzle) => {
                let candidates = puzzle.get_candidates(row, col);
                let mut result = Vec::new();
                for candidate in candidates.iter() {
                    result.push(candidate.to_string());
                }
                result.join(",")
            }
            Err(_) => String::new(),
        }
    }

    #[wasm_bindgen]
    pub fn set_cell(
        &self,
        puzzle_str: &str,
        size: usize,
        row: usize,
        col: usize,
        value: u8,
    ) -> String {
        match Sudoku::from_string(puzzle_str, size) {
            Ok(mut puzzle) => {
                match puzzle.set(row, col, value) {
                    Ok(()) => {
                        // Return the updated puzzle string
                        let mut result = String::new();
                        for r in 0..puzzle.size {
                            for c in 0..puzzle.size {
                                if let Some(v) = puzzle.grid[r][c].value() {
                                    if v <= 9 {
                                        result.push_str(&v.to_string());
                                    } else {
                                        result.push((b'A' + v - 10) as char);
                                    }
                                } else {
                                    result.push('0');
                                }
                            }
                        }
                        result
                    }
                    Err(e) => format!("Error: {}", e),
                }
            }
            Err(e) => format!("Error parsing puzzle: {}", e),
        }
    }

    #[wasm_bindgen]
    pub fn solve_step(&mut self, puzzle_str: &str, size: usize) -> String {
        match Sudoku::from_string(puzzle_str, size) {
            Ok(mut puzzle) => {
                // Try one iteration of logical strategies
                if self.solver.solve_step(&mut puzzle) {
                    // Return the updated puzzle string
                    let mut result = String::new();
                    for row in 0..puzzle.size {
                        for col in 0..puzzle.size {
                            if let Some(value) = puzzle.grid[row][col].value() {
                                if value <= 9 {
                                    result.push_str(&value.to_string());
                                } else {
                                    result.push((b'A' + value - 10) as char);
                                }
                            } else {
                                result.push('0');
                            }
                        }
                    }
                    result
                } else {
                    // No progress made
                    puzzle_str.to_string()
                }
            }
            Err(e) => format!("Error parsing puzzle: {}", e),
        }
    }
}
