//! WebAssembly interface for the Sudoku solver with simple text-based UI

use crate::{Difficulty, Sudoku, SudokuSolver};
use wasm_bindgen::prelude::*;

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
//
// For more details see
// https://github.com/rustwasm/console_error_panic_hook#readme
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct WasmSudoku {
    sudoku: Sudoku,
    solver: SudokuSolver,
}

#[wasm_bindgen]
impl WasmSudoku {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> WasmSudoku {
        #[cfg(feature = "console_error_panic_hook")]
        set_panic_hook();

        WasmSudoku {
            sudoku: Sudoku::new(size),
            solver: SudokuSolver::new(),
        }
    }

    #[wasm_bindgen]
    pub fn from_string(puzzle_str: &str, size: usize) -> Result<WasmSudoku, String> {
        #[cfg(feature = "console_error_panic_hook")]
        set_panic_hook();

        match Sudoku::from_string(puzzle_str, size) {
            Ok(sudoku) => Ok(WasmSudoku {
                sudoku,
                solver: SudokuSolver::new(),
            }),
            Err(e) => Err(format!("Failed to parse sudoku: {}", e)),
        }
    }

    #[wasm_bindgen]
    pub fn get_size(&self) -> usize {
        self.sudoku.size
    }

    #[wasm_bindgen]
    pub fn get_value(&self, row: usize, col: usize) -> u8 {
        self.sudoku
            .get(row, col)
            .and_then(|cell| cell.value())
            .unwrap_or(0)
    }

    #[wasm_bindgen]
    pub fn set_value(&mut self, row: usize, col: usize, value: u8) -> Result<(), String> {
        self.sudoku
            .set(row, col, value)
            .map_err(|e| format!("Failed to set value: {}", e))
    }

    #[wasm_bindgen]
    pub fn is_valid(&self) -> bool {
        self.sudoku.is_valid()
    }

    #[wasm_bindgen]
    pub fn is_complete(&self) -> bool {
        self.sudoku.is_complete()
    }

    #[wasm_bindgen]
    pub fn solve(&mut self) -> Result<(), String> {
        match self.solver.solve(self.sudoku.clone()) {
            Ok(solution) => {
                self.sudoku = solution;
                Ok(())
            }
            Err(e) => Err(format!("Failed to solve: {}", e)),
        }
    }

    #[wasm_bindgen]
    pub fn get_hint(&mut self) -> Option<String> {
        match self.solver.get_hint(&mut self.sudoku.clone()) {
            Some((row, col, value)) => Some(format!("{}:{}:{}", row, col, value)),
            None => None,
        }
    }

    #[wasm_bindgen]
    pub fn generate_puzzle(&mut self, difficulty: &str) -> Result<(), String> {
        let diff = match difficulty.to_lowercase().as_str() {
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            "expert" => Difficulty::Expert,
            _ => return Err("Invalid difficulty. Use easy, medium, hard, or expert".to_string()),
        };

        match self.solver.generate_puzzle(self.sudoku.size, diff) {
            Ok(puzzle) => {
                self.sudoku = puzzle;
                Ok(())
            }
            Err(e) => Err(format!("Failed to generate puzzle: {}", e)),
        }
    }

    /// Render the sudoku grid as a simple text representation
    #[wasm_bindgen]
    pub fn render_text(&self) -> String {
        let mut result = String::new();

        for row in 0..self.sudoku.size {
            if row > 0 && row % self.sudoku.box_size == 0 {
                result.push_str(&"-".repeat(self.sudoku.size * 3 + self.sudoku.box_size - 1));
                result.push('\n');
            }

            for col in 0..self.sudoku.size {
                if col > 0 && col % self.sudoku.box_size == 0 {
                    result.push('|');
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

                result.push_str(&format!(" {} ", ch));
            }
            result.push('\n');
        }

        result
    }

    /// Get the puzzle as a string (for saving/loading)
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.sudoku.to_string()
    }
}

// Additional utility functions for WASM
#[wasm_bindgen]
pub fn create_example_puzzle() -> WasmSudoku {
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    WasmSudoku::from_string(puzzle_str, 9).unwrap()
}
