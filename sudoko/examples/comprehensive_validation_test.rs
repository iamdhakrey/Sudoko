use sudoko::{Sudoku, SudokuSolver};

fn main() {
    println!("Testing comprehensive validation scenarios...");

    // Test case 1: Original puzzle with various invalid placements
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let mut puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

    // Get correct solution
    let mut solver = SudokuSolver::new();
    let solution = solver.solve(puzzle.clone()).unwrap();

    println!("Original puzzle:");
    println!("{}", puzzle);
    println!("\nCorrect solution:");
    println!("{}", solution);

    // Test various positions
    let test_cases = vec![
        (0, 2, 4, "Position (0,2) - should be correct"),
        (0, 2, 1, "Position (0,2) - wrong but valid by basic rules"),
        (
            0,
            2,
            5,
            "Position (0,2) - violates basic rules (5 exists in row)",
        ),
        (8, 1, 4, "Position (8,1) - should be correct"),
        (8, 1, 3, "Position (8,1) - wrong and violates basic rules"),
        (8, 1, 2, "Position (8,1) - wrong but valid by basic rules"),
    ];

    for (row, col, value, description) in test_cases {
        println!("\n=== {} ===", description);
        let correct_value = solution.get(row, col).unwrap().value().unwrap();

        // Test on original puzzle
        let basic_valid = puzzle.is_valid_placement(row, col, value);
        let solution_correct = puzzle.is_correct_placement(row, col, value);
        let enhanced_valid = puzzle.is_valid_and_correct_placement(row, col, value);

        println!("Correct value: {}, Testing value: {}", correct_value, value);
        println!("Basic validation: {}", basic_valid);
        println!("Solution correctness: {}", solution_correct);
        println!("Enhanced validation: {}", enhanced_valid);

        // Determine expected color in TUI
        let expected_color = if enhanced_valid { "WHITE" } else { "RED" };
        println!("Expected TUI color: {}", expected_color);

        // Test with value already placed
        let mut test_puzzle = puzzle.clone();
        test_puzzle.set(row, col, value).unwrap();

        let basic_valid_after = test_puzzle.is_valid_placement(row, col, value);
        let solution_correct_after = test_puzzle.is_correct_placement(row, col, value);
        let enhanced_valid_after = test_puzzle.is_valid_and_correct_placement(row, col, value);

        println!(
            "After placing - Basic: {}, Solution: {}, Enhanced: {}",
            basic_valid_after, solution_correct_after, enhanced_valid_after
        );

        let expected_color_after = if enhanced_valid_after { "WHITE" } else { "RED" };
        println!("Expected TUI color after placing: {}", expected_color_after);
    }

    println!("\n=== Summary ===");
    println!("In the TUI:");
    println!("- Values that match the solution AND follow basic rules → WHITE");
    println!("- Values that don't match the solution OR violate basic rules → RED");
    println!("- Given/preset values → CYAN");
    println!("- Current cursor position → YELLOW background");
}
