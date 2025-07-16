use sudoko::{Sudoku, SudokuSolver};

fn main() {
    println!("Testing TUI scenario: place 3 at (8,1) then check validation");

    // Start with the original puzzle
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let mut puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

    println!("Original puzzle:");
    println!("{}", puzzle);

    // Get the correct solution for reference
    let mut solver = SudokuSolver::new();
    let solution = solver.solve(puzzle.clone()).unwrap();
    let correct_value = solution.get(8, 1).unwrap().value().unwrap();
    println!("Correct value at (8,1): {}", correct_value);

    // Test validation BEFORE placing the value (like TUI should do)
    println!("\n=== Testing validation BEFORE placing value ===");
    let test_value = 3;
    println!(
        "Testing placement of {} at (8,1) on original puzzle:",
        test_value
    );

    let basic_valid = puzzle.is_valid_placement(8, 1, test_value);
    let solution_correct = puzzle.is_correct_placement(8, 1, test_value);
    let enhanced_valid = puzzle.is_valid_and_correct_placement(8, 1, test_value);

    println!("Basic validation: {}", basic_valid);
    println!("Solution correctness: {}", solution_correct);
    println!("Enhanced validation: {}", enhanced_valid);

    // Now place the value (like TUI does)
    println!("\n=== Placing value in puzzle ===");
    puzzle.set(8, 1, test_value).unwrap();
    println!("Placed {} at (8,1)", test_value);

    // Test validation AFTER placing the value (like TUI rendering does)
    println!("\n=== Testing validation AFTER placing value ===");
    let cell = puzzle.get(8, 1).unwrap();
    let current_value = cell.value().unwrap();

    let basic_valid_after = puzzle.is_valid_placement(8, 1, current_value);
    let solution_correct_after = puzzle.is_correct_placement(8, 1, current_value);
    let enhanced_valid_after = puzzle.is_valid_and_correct_placement(8, 1, current_value);

    println!("Basic validation after: {}", basic_valid_after);
    println!("Solution correctness after: {}", solution_correct_after);
    println!("Enhanced validation after: {}", enhanced_valid_after);

    println!("\nCurrent puzzle state:");
    println!("{}", puzzle);
}
