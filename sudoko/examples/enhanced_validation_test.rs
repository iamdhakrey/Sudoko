use sudoko::{Difficulty, Sudoku, SudokuSolver};

fn main() {
    println!("Testing enhanced validation (rules + solution correctness)...");

    // Create a simple puzzle with a known solution
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

    println!("Original puzzle:");
    println!("{}", puzzle);

    // First, solve the puzzle to know the correct solution
    let mut solver = SudokuSolver::new();
    let solution = solver.solve(puzzle.clone()).unwrap();

    println!("Correct solution:");
    println!("{}", solution);

    // Test 1: Valid placement that matches solution
    println!("\n=== Test 1: Valid placement that matches solution ===");
    let pos = (0, 1); // Position (0, 1) should be 4 in the solution
    let correct_value = solution.get(pos.0, pos.1).unwrap().value().unwrap();
    println!(
        "Testing placement of {} at ({}, {}) - should be correct",
        correct_value, pos.0, pos.1
    );

    println!(
        "Basic validation: {}",
        puzzle.is_valid_placement(pos.0, pos.1, correct_value)
    );
    println!(
        "Solution correctness: {}",
        puzzle.is_correct_placement(pos.0, pos.1, correct_value)
    );
    println!(
        "Enhanced validation: {}",
        puzzle.is_valid_and_correct_placement(pos.0, pos.1, correct_value)
    );

    // Test 2: Valid placement that doesn't match solution
    println!("\n=== Test 2: Valid placement that doesn't match solution ===");
    let wrong_value = if correct_value == 1 { 2 } else { 1 }; // Pick a different valid number
    println!(
        "Testing placement of {} at ({}, {}) - should be invalid (wrong solution)",
        wrong_value, pos.0, pos.1
    );

    println!(
        "Basic validation: {}",
        puzzle.is_valid_placement(pos.0, pos.1, wrong_value)
    );
    println!(
        "Solution correctness: {}",
        puzzle.is_correct_placement(pos.0, pos.1, wrong_value)
    );
    println!(
        "Enhanced validation: {}",
        puzzle.is_valid_and_correct_placement(pos.0, pos.1, wrong_value)
    );

    // Test 3: Invalid placement (violates basic rules)
    println!("\n=== Test 3: Invalid placement (violates basic rules) ===");
    let invalid_value = 5; // 5 already exists in the first row
    println!(
        "Testing placement of {} at ({}, {}) - should be invalid (rule violation)",
        invalid_value, pos.0, pos.1
    );

    println!(
        "Basic validation: {}",
        puzzle.is_valid_placement(pos.0, pos.1, invalid_value)
    );
    println!(
        "Solution correctness: {}",
        puzzle.is_correct_placement(pos.0, pos.1, invalid_value)
    );
    println!(
        "Enhanced validation: {}",
        puzzle.is_valid_and_correct_placement(pos.0, pos.1, invalid_value)
    );

    // Test 4: Show what values are valid vs correct
    println!("\n=== Test 4: Candidates vs Correct Value ===");
    let candidates = puzzle.get_candidates(pos.0, pos.1);
    println!(
        "Valid candidates for ({}, {}): {:?}",
        pos.0, pos.1, candidates
    );
    println!(
        "Correct value for ({}, {}): {}",
        pos.0, pos.1, correct_value
    );

    for &candidate in &candidates {
        let is_correct = puzzle.is_correct_placement(pos.0, pos.1, candidate);
        println!(
            "  {} -> {}",
            candidate,
            if is_correct {
                "✓ CORRECT"
            } else {
                "✗ wrong"
            }
        );
    }

    println!("\nEnhanced validation tests completed!");
    println!("Now when you enter a number that doesn't match the solution, it will show in RED!");
}
