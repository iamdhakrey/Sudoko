use sudoko::{Sudoku, SudokuSolver};

fn main() {
    println!("=== Hint System Example ===\n");
    
    // Start with a partially solved puzzle
    let puzzle_str = "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let mut puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();
    
    // Remove a few cells to create a puzzle that needs hints
    puzzle.set(0, 0, 0).unwrap(); // Remove the 5
    puzzle.set(1, 1, 0).unwrap(); // Remove the 7
    puzzle.set(2, 2, 0).unwrap(); // Remove the 8
    
    println!("Puzzle needing hints:");
    println!("{}", puzzle);
    
    let mut solver = SudokuSolver::new();
    
    // Get hints until puzzle is solved
    let mut hint_count = 0;
    
    while !puzzle.is_complete() && hint_count < 10 {
        match solver.get_hint(&mut puzzle) {
            Some((row, col, value)) => {
                hint_count += 1;
                println!("Hint {}: Place {} at position ({}, {})", hint_count, value, row + 1, col + 1);
                
                // Apply the hint
                puzzle.set(row, col, value).unwrap();
                
                println!("Puzzle after applying hint:");
                println!("{}", puzzle);
                
                if puzzle.is_complete() {
                    println!("✓ Puzzle completed!");
                    break;
                }
            }
            None => {
                println!("No more obvious hints available.");
                println!("You might need to use more advanced techniques or backtracking.");
                break;
            }
        }
    }
    
    // Validation example
    println!("\n=== Validation Example ===\n");
    
    let test_puzzles = vec![
        ("123456789456789123789123456234567891567891234891234567345678912678912345912345678", 9, "Valid complete solution"),
        ("123456789456789123789123456234567891567891234891234567345678912678912345912345677", 9, "Invalid solution (duplicate 7 in last row)"),
        ("530070000600195000098000060800060003400803001700020006060000280000419005000080079", 9, "Valid incomplete puzzle"),
        ("123412341234123412341234123412341234123412341234123412341234123412341234", 16, "Invalid 16x16 (too many duplicates)"),
    ];
    
    for (puzzle_str, size, description) in test_puzzles {
        println!("Testing: {}", description);
        
        match Sudoku::from_string(puzzle_str, size) {
            Ok(puzzle) => {
                println!("Puzzle:");
                println!("{}", puzzle);
                
                if puzzle.is_valid() {
                    println!("✓ Puzzle is valid");
                    
                    if puzzle.is_complete() {
                        println!("✓ Puzzle is complete and solved!");
                    } else {
                        println!("! Puzzle is valid but incomplete");
                    }
                } else {
                    println!("✗ Puzzle is invalid");
                    
                    if !puzzle.is_valid_rows() {
                        println!("  - Invalid rows detected");
                    }
                    if !puzzle.is_valid_cols() {
                        println!("  - Invalid columns detected");
                    }
                    if !puzzle.is_valid_boxes() {
                        println!("  - Invalid boxes detected");
                    }
                }
            }
            Err(e) => {
                println!("✗ Error parsing puzzle: {}", e);
            }
        }
        
        println!();
    }
}
