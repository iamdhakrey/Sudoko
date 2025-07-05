use sudoko::{Sudoku, SudokuSolver};

fn main() {
    // Example 9x9 Sudoku puzzle
    let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    
    println!("=== Basic Sudoku Solving Example ===\n");
    
    // Parse the puzzle
    let puzzle = match Sudoku::from_string(puzzle_str, 9) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            return;
        }
    };
    
    println!("Original puzzle:");
    println!("{}", puzzle);
    
    // Create solver and solve with statistics
    let mut solver = SudokuSolver::new();
    
    match solver.solve_with_stats(puzzle) {
        Ok((solution, stats)) => {
            println!("Solution found!");
            println!("{}", solution);
            
            println!("\n=== Solver Statistics ===");
            println!("Iterations: {}", stats.iterations);
            println!("Cells filled: {}", stats.cells_filled);
            println!("Backtrack steps: {}", stats.backtrack_steps);
            
            if !stats.strategies_used.is_empty() {
                println!("\nStrategies used:");
                for (strategy, count) in stats.strategies_used {
                    println!("  {}: {} times", strategy, count);
                }
            }
            
            // Validate the solution
            if solver.validate_solution(&solution) {
                println!("\n✓ Solution is valid and complete!");
            } else {
                println!("\n✗ Solution is invalid!");
            }
        }
        Err(e) => {
            eprintln!("Failed to solve puzzle: {}", e);
        }
    }
    
    // Example with different sizes
    println!("\n=== 4x4 Sudoku Example ===\n");
    
    let small_puzzle_str = "1.3..2.43.1..4.2";
    let small_puzzle = Sudoku::from_string(small_puzzle_str, 4).unwrap();
    
    println!("4x4 puzzle:");
    println!("{}", small_puzzle);
    
    match solver.solve(small_puzzle) {
        Ok(solution) => {
            println!("4x4 solution:");
            println!("{}", solution);
        }
        Err(e) => {
            eprintln!("Failed to solve 4x4 puzzle: {}", e);
        }
    }
}
