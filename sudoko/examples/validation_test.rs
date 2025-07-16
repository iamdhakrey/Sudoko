use sudoko::{Difficulty, Sudoku, SudokuSolver};

fn main() {
    println!("Testing validation methods...");

    // Create a new 9x9 Sudoku
    let mut sudoku = Sudoku::new(9);

    // Test valid placement
    println!("Testing valid placement of 5 at (0,0):");
    let is_valid = sudoku.is_valid_placement(0, 0, 5);
    println!("Result: {}", is_valid);

    // Place the value
    sudoku.set(0, 0, 5).unwrap();
    println!("Placed 5 at (0,0)");

    // Test invalid placement (same row)
    println!("Testing invalid placement of 5 at (0,1) (same row):");
    let is_valid = sudoku.is_valid_placement(0, 1, 5);
    println!("Result: {}", is_valid);

    // Test invalid placement (same column)
    println!("Testing invalid placement of 5 at (1,0) (same column):");
    let is_valid = sudoku.is_valid_placement(1, 0, 5);
    println!("Result: {}", is_valid);

    // Test invalid placement (same box)
    println!("Testing invalid placement of 5 at (1,1) (same box):");
    let is_valid = sudoku.is_valid_placement(1, 1, 5);
    println!("Result: {}", is_valid);

    // Test valid placement in different box
    println!("Testing valid placement of 5 at (3,3) (different box):");
    let is_valid = sudoku.is_valid_placement(3, 3, 5);
    println!("Result: {}", is_valid);

    // Test get_candidates
    println!("Testing get_candidates for (0,1):");
    let candidates = sudoku.get_candidates(0, 1);
    println!("Candidates: {:?}", candidates);
    println!("Should not include 5: {}", !candidates.contains(&5));

    // Test with a real puzzle
    println!("\nTesting with a real puzzle:");
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let mut puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

    // Test invalid placement
    println!("Testing placement of 5 at (0,0) in real puzzle:");
    let is_valid = puzzle.is_valid_placement(0, 0, 5);
    println!(
        "Result: {} (should be false, conflicts with existing 5)",
        is_valid
    );

    // Test valid placement
    println!("Testing placement of 1 at (0,1) in real puzzle:");
    let is_valid = puzzle.is_valid_placement(0, 1, 1);
    println!("Result: {} (should be true)", is_valid);

    println!("\nValidation tests completed!");
}
