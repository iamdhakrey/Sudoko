use crate::strategies::{get_all_strategies, SolvingStrategy};
use crate::sudoku::{Cell, Sudoku};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SolverStats {
    pub strategies_used: HashMap<String, usize>,
    pub cells_filled: usize,
    pub iterations: usize,
    pub backtrack_steps: usize,
}

impl SolverStats {
    pub fn new() -> Self {
        Self {
            strategies_used: HashMap::new(),
            cells_filled: 0,
            iterations: 0,
            backtrack_steps: 0,
        }
    }
}

pub struct SudokuSolver {
    strategies: Vec<Box<dyn SolvingStrategy>>,
    max_iterations: usize,
    use_backtracking: bool,
}

impl SudokuSolver {
    pub fn new() -> Self {
        Self {
            strategies: get_all_strategies(),
            max_iterations: 1000,
            use_backtracking: true,
        }
    }

    pub fn new_with_strategies(strategies: Vec<Box<dyn SolvingStrategy>>) -> Self {
        Self {
            strategies,
            max_iterations: 1000,
            use_backtracking: true,
        }
    }

    pub fn set_max_iterations(&mut self, max_iterations: usize) {
        self.max_iterations = max_iterations;
    }

    pub fn set_use_backtracking(&mut self, use_backtracking: bool) {
        self.use_backtracking = use_backtracking;
    }

    pub fn solve(&mut self, mut sudoku: Sudoku) -> Result<Sudoku, String> {
        if !sudoku.is_valid() {
            return Err("Invalid initial state".to_string());
        }

        let mut stats = SolverStats::new();

        // First, try logical strategies
        if self.solve_with_strategies(&mut sudoku, &mut stats) {
            return Ok(sudoku);
        }

        // If logical strategies aren't enough, use backtracking
        if self.use_backtracking {
            if self.solve_with_backtracking(&mut sudoku, &mut stats) {
                return Ok(sudoku);
            }
        }

        if sudoku.is_complete() && sudoku.is_valid() {
            Ok(sudoku)
        } else {
            Err("No solution found".to_string())
        }
    }

    pub fn solve_with_stats(
        &mut self,
        mut sudoku: Sudoku,
    ) -> Result<(Sudoku, SolverStats), String> {
        if !sudoku.is_valid() {
            return Err("Invalid initial state".to_string());
        }

        let mut stats = SolverStats::new();

        // First, try logical strategies
        if self.solve_with_strategies(&mut sudoku, &mut stats) {
            return Ok((sudoku, stats));
        }

        // If logical strategies aren't enough, use backtracking
        if self.use_backtracking {
            if self.solve_with_backtracking(&mut sudoku, &mut stats) {
                return Ok((sudoku, stats));
            }
        }

        if sudoku.is_complete() && sudoku.is_valid() {
            Ok((sudoku, stats))
        } else {
            Err("No solution found".to_string())
        }
    }

    fn solve_with_strategies(&self, sudoku: &mut Sudoku, stats: &mut SolverStats) -> bool {
        let mut progress = true;

        while progress && !sudoku.is_complete() && stats.iterations < self.max_iterations {
            progress = false;
            stats.iterations += 1;

            for strategy in &self.strategies {
                let initial_empty_count = self.count_empty_cells(sudoku);

                if strategy.apply(sudoku) {
                    let final_empty_count = self.count_empty_cells(sudoku);
                    let cells_filled = initial_empty_count - final_empty_count;

                    stats.cells_filled += cells_filled;
                    *stats
                        .strategies_used
                        .entry(strategy.name().to_string())
                        .or_insert(0) += 1;

                    progress = true;

                    if !sudoku.is_valid() {
                        return false;
                    }
                }
            }
        }

        sudoku.is_complete() && sudoku.is_valid()
    }

    fn solve_with_backtracking(&self, sudoku: &mut Sudoku, stats: &mut SolverStats) -> bool {
        if sudoku.is_complete() {
            return sudoku.is_valid();
        }

        // Find the empty cell with the fewest candidates (MRV heuristic)
        let (row, col) = match self.find_best_empty_cell(sudoku) {
            Some(pos) => pos,
            None => return sudoku.is_valid(),
        };

        let candidates = sudoku.get_candidates(row, col);

        for &value in &candidates {
            if sudoku.set(row, col, value).is_ok() {
                stats.backtrack_steps += 1;

                if sudoku.is_valid() {
                    if self.solve_with_backtracking(sudoku, stats) {
                        return true;
                    }
                }

                // Backtrack
                sudoku.set(row, col, 0).unwrap();
            }
        }

        false
    }

    fn find_best_empty_cell(&self, sudoku: &Sudoku) -> Option<(usize, usize)> {
        let mut best_cell = None;
        let mut min_candidates = usize::MAX;

        for row in 0..sudoku.size {
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.len() < min_candidates {
                        min_candidates = candidates.len();
                        best_cell = Some((row, col));

                        // If we find a cell with no candidates, return immediately
                        if min_candidates == 0 {
                            return best_cell;
                        }
                    }
                }
            }
        }

        best_cell
    }

    fn count_empty_cells(&self, sudoku: &Sudoku) -> usize {
        let mut count = 0;
        for row in 0..sudoku.size {
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn get_hint(&mut self, sudoku: &mut Sudoku) -> Option<(usize, usize, u8)> {
        // Try to find a cell that can be filled using logical strategies
        for row in 0..sudoku.size {
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.len() == 1 {
                        let value = *candidates.iter().next().unwrap();
                        return Some((row, col, value));
                    }
                }
            }
        }

        // If no naked singles, try hidden singles
        for strategy in &self.strategies {
            if strategy.name() == "Hidden Singles" {
                let mut temp_sudoku = sudoku.clone();
                if strategy.apply(&mut temp_sudoku) {
                    // Find the difference
                    for row in 0..sudoku.size {
                        for col in 0..sudoku.size {
                            if sudoku.grid[row][col] != temp_sudoku.grid[row][col] {
                                if let Some(value) = temp_sudoku.grid[row][col].value() {
                                    return Some((row, col, value));
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn validate_solution(&self, sudoku: &Sudoku) -> bool {
        sudoku.is_complete() && sudoku.is_valid()
    }

    pub fn count_solutions(&mut self, mut sudoku: Sudoku, max_solutions: usize) -> usize {
        let mut count = 0;
        self.count_solutions_recursive(&mut sudoku, &mut count, max_solutions);
        count
    }

    fn count_solutions_recursive(
        &self,
        sudoku: &mut Sudoku,
        count: &mut usize,
        max_solutions: usize,
    ) {
        if *count >= max_solutions {
            return;
        }

        if sudoku.is_complete() {
            if sudoku.is_valid() {
                *count += 1;
            }
            return;
        }

        let (row, col) = match sudoku.find_empty_cell() {
            Some(pos) => pos,
            None => return,
        };

        let candidates = sudoku.get_candidates(row, col);

        for &value in &candidates {
            if sudoku.set(row, col, value).is_ok() {
                if sudoku.is_valid() {
                    self.count_solutions_recursive(sudoku, count, max_solutions);
                }
                sudoku.set(row, col, 0).unwrap();
            }
        }
    }

    pub fn generate_puzzle(
        &mut self,
        size: usize,
        difficulty: Difficulty,
    ) -> Result<Sudoku, String> {
        let mut sudoku = Sudoku::new(size);

        // Fill the diagonal boxes first (they don't interfere with each other)
        self.fill_diagonal_boxes(&mut sudoku)?;

        // Solve the complete puzzle
        let full_solution = self.solve(sudoku.clone())?;

        // Remove cells based on difficulty
        let cells_to_remove = match difficulty {
            Difficulty::Easy => size * size * 40 / 100, // Remove 40%
            Difficulty::Medium => size * size * 50 / 100, // Remove 50%
            Difficulty::Hard => size * size * 60 / 100, // Remove 60%
            Difficulty::Expert => size * size * 70 / 100, // Remove 70%
        };

        self.remove_cells_symmetrically(full_solution, cells_to_remove)
    }

    fn fill_diagonal_boxes(&self, sudoku: &mut Sudoku) -> Result<(), String> {
        for i in 0..sudoku.box_size {
            self.fill_box(sudoku, i * sudoku.box_size, i * sudoku.box_size)?;
        }
        Ok(())
    }

    fn fill_box(
        &self,
        sudoku: &mut Sudoku,
        start_row: usize,
        start_col: usize,
    ) -> Result<(), String> {
        let mut values: Vec<u8> = (1..=sudoku.size as u8).collect();

        // Shuffle values (simplified random shuffle)
        for i in 0..values.len() {
            let j = (i * 7 + 3) % values.len(); // Simple pseudo-random
            values.swap(i, j);
        }

        let mut idx = 0;
        for row in start_row..start_row + sudoku.box_size {
            for col in start_col..start_col + sudoku.box_size {
                sudoku.set(row, col, values[idx])?;
                idx += 1;
            }
        }

        Ok(())
    }

    fn remove_cells_symmetrically(
        &self,
        mut sudoku: Sudoku,
        cells_to_remove: usize,
    ) -> Result<Sudoku, String> {
        let mut removed = 0;
        let size = sudoku.size;

        // Simple cell removal (in a real implementation, you'd ensure unique solution)
        for i in 0..size {
            for j in 0..size {
                if removed >= cells_to_remove {
                    break;
                }

                // Remove cells in a pattern
                if (i + j) % 3 == 0 && removed < cells_to_remove {
                    sudoku.grid[i][j] = Cell::Empty;
                    removed += 1;

                    // Also remove symmetric cell
                    let sym_i = size - 1 - i;
                    let sym_j = size - 1 - j;
                    if sym_i != i || sym_j != j {
                        if removed < cells_to_remove {
                            sudoku.grid[sym_i][sym_j] = Cell::Empty;
                            removed += 1;
                        }
                    }
                }
            }
            if removed >= cells_to_remove {
                break;
            }
        }

        Ok(sudoku)
    }

    pub fn solve_step(&self, sudoku: &mut Sudoku) -> bool {
        for strategy in &self.strategies {
            if strategy.apply(sudoku) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Default for SudokuSolver {
    fn default() -> Self {
        Self::new()
    }
}
