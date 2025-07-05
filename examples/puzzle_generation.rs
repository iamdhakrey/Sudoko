use sudoko::{SudokuSolver, Difficulty};

fn main() {
    println!("=== Puzzle Generation Example ===\n");
    
    let mut solver = SudokuSolver::new();
    
    // Generate puzzles of different sizes and difficulties
    let test_cases = vec![
        (4, Difficulty::Easy),
        (9, Difficulty::Easy),
        (9, Difficulty::Medium),
        (9, Difficulty::Hard),
        (16, Difficulty::Medium),
    ];
    
    for (size, difficulty) in test_cases {
        println!("Generating {}x{} puzzle with {:?} difficulty:", size, size, difficulty);
        
        match solver.generate_puzzle(size, difficulty) {
            Ok(puzzle) => {
                println!("{}", puzzle);
                
                // Verify the generated puzzle is solvable
                println!("Verifying puzzle is solvable...");
                match solver.solve(puzzle.clone()) {
                    Ok(_) => println!("✓ Puzzle is solvable!\n"),
                    Err(_) => println!("✗ Generated puzzle is not solvable!\n"),
                }
            }
            Err(e) => {
                eprintln!("Failed to generate {}x{} puzzle: {}\n", size, size, e);
            }
        }
    }
    
    // Count solutions for a puzzle
    println!("=== Solution Counting Example ===\n");
    
    let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let puzzle = sudoko::Sudoku::from_string(puzzle_str, 9).unwrap();
    
    println!("Counting solutions for puzzle:");
    println!("{}", puzzle);
    
    let solution_count = solver.count_solutions(puzzle, 10); // Limit to 10 solutions max
    println!("Number of solutions found: {}", solution_count);
    
    if solution_count == 1 {
        println!("✓ Puzzle has a unique solution!");
    } else if solution_count > 1 {
        println!("! Puzzle has multiple solutions.");
    } else {
        println!("✗ Puzzle has no solutions.");
    }
}
