use sudoko::{Sudoku, SudokuSolver};

fn main() {
    println!("Testing specific case: position (8,1) with value 3");

    // Use the exact same puzzle from the TUI
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

    println!("Original puzzle:");
    println!("{}", puzzle);

    // Solve the puzzle to see the correct solution
    let mut solver = SudokuSolver::new();
    let solution = solver.solve(puzzle.clone()).unwrap();

    println!("Correct solution:");
    println!("{}", solution);

    // Check what should be at position (8,1) in the solution
    let correct_value = solution.get(8, 1).unwrap().value().unwrap();
    println!("Correct value at (8,1): {}", correct_value);

    // Test validation for placing 3 at (8,1)
    let test_value = 3;
    println!("\nTesting placement of {} at (8,1):", test_value);

    // Basic validation
    let basic_valid = puzzle.is_valid_placement(8, 1, test_value);
    println!("Basic validation (rules only): {}", basic_valid);

    // Solution correctness
    let solution_correct = puzzle.is_correct_placement(8, 1, test_value);
    println!("Solution correctness: {}", solution_correct);

    // Enhanced validation
    let enhanced_valid = puzzle.is_valid_and_correct_placement(8, 1, test_value);
    println!("Enhanced validation (rules + solution): {}", enhanced_valid);

    // Test with the correct value
    println!(
        "\nTesting placement of {} at (8,1) (correct value):",
        correct_value
    );
    let correct_basic = puzzle.is_valid_placement(8, 1, correct_value);
    let correct_solution = puzzle.is_correct_placement(8, 1, correct_value);
    let correct_enhanced = puzzle.is_valid_and_correct_placement(8, 1, correct_value);

    println!("Basic validation: {}", correct_basic);
    println!("Solution correctness: {}", correct_solution);
    println!("Enhanced validation: {}", correct_enhanced);
}
