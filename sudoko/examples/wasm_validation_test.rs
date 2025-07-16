use sudoko::WasmSudoku;

#[cfg(feature = "wasm")]
fn main() {
    println!("Testing WASM validation methods...");

    // Create a new 9x9 Sudoku
    let mut wasm_sudoku = WasmSudoku::new(9);

    // Test valid placement
    println!("Testing valid placement of 5 at (0,0):");
    let is_valid = wasm_sudoku.is_valid_placement(0, 0, 5);
    println!("Result: {}", is_valid);

    // Place the value
    wasm_sudoku.set_value(0, 0, 5).unwrap();
    println!("Placed 5 at (0,0)");

    // Test invalid placement (same row)
    println!("Testing invalid placement of 5 at (0,1) (same row):");
    let is_valid = wasm_sudoku.is_valid_placement(0, 1, 5);
    println!("Result: {}", is_valid);

    // Test get_candidates
    println!("Testing get_candidates for (0,1):");
    let candidates = wasm_sudoku.get_candidates(0, 1);
    println!("Candidates: {:?}", candidates);
    println!("Should not include 5: {}", !candidates.contains(&5));

    // Test with a real puzzle
    println!("\nTesting with a real puzzle:");
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let mut wasm_puzzle = WasmSudoku::from_string(puzzle_str, 9).unwrap();

    // Test invalid placement
    println!("Testing placement of 5 at (0,0) in real puzzle:");
    let is_valid = wasm_puzzle.is_valid_placement(0, 0, 5);
    println!(
        "Result: {} (should be false, conflicts with existing 5)",
        is_valid
    );

    // Test valid placement
    println!("Testing placement of 1 at (0,1) in real puzzle:");
    let is_valid = wasm_puzzle.is_valid_placement(0, 1, 1);
    println!("Result: {} (should be true)", is_valid);

    println!("\nWASM validation tests completed!");
}

#[cfg(not(feature = "wasm"))]
fn main() {
    println!("WASM feature not enabled. Run with: cargo run --example wasm_validation_test --features wasm");
}
