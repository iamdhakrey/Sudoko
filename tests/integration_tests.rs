use sudoko::{Sudoku, SudokuSolver};

#[test]
fn test_9x9_basic_solve() {
    let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();
    
    let mut solver = SudokuSolver::new();
    let solution = solver.solve(puzzle).unwrap();
    
    assert!(solution.is_complete());
    assert!(solution.is_valid());
}

#[test]
fn test_4x4_solve() {
    // Test creating and validating a complete 4x4 solution (1,2,3,4 in each row/col/box)
    let complete_str = "1234341221434321";
    let complete = Sudoku::from_string(complete_str, 4).unwrap();
    
    assert!(complete.is_complete());
    assert!(complete.is_valid());
    
    // Test basic 4x4 structure
    let puzzle = Sudoku::new(4);
    assert_eq!(puzzle.size, 4);
    assert_eq!(puzzle.box_size, 2);
    assert!(!puzzle.is_complete());
}

#[test]
fn test_validation() {
    // Valid complete solution
    let valid_str = "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let valid_puzzle = Sudoku::from_string(valid_str, 9).unwrap();
    assert!(valid_puzzle.is_valid());
    assert!(valid_puzzle.is_complete());
    
    // Invalid solution (duplicate in row)
    let invalid_str = "534678912672195348198342567859761423426853791713924856961537284287419635345286177";
    let invalid_puzzle = Sudoku::from_string(invalid_str, 9).unwrap();
    assert!(!invalid_puzzle.is_valid());
}

#[test]
fn test_candidates() {
    let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();
    
    // Test candidates for position (0, 2) which should be empty
    let candidates = puzzle.get_candidates(0, 2);
    assert!(!candidates.is_empty());
    assert!(!candidates.contains(&5)); // 5 is already in the row
    assert!(!candidates.contains(&3)); // 3 is already in the row
}

#[test]
fn test_hint_system() {
    let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let mut puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();
    
    let mut solver = SudokuSolver::new();
    let hint = solver.get_hint(&mut puzzle);
    
    assert!(hint.is_some());
    if let Some((row, col, value)) = hint {
        assert!(row < 9);
        assert!(col < 9);
        assert!(value >= 1 && value <= 9);
        
        // Verify the hint is valid
        let candidates = puzzle.get_candidates(row, col);
        assert!(candidates.contains(&value));
    }
}

#[test]
fn test_empty_puzzle_creation() {
    let sudoku = Sudoku::new(9);
    assert_eq!(sudoku.size, 9);
    assert_eq!(sudoku.box_size, 3);
    assert!(!sudoku.is_complete());
    
    // All cells should be empty
    for row in 0..9 {
        for col in 0..9 {
            assert!(sudoku.grid[row][col].is_empty());
        }
    }
}

#[test]
fn test_invalid_size() {
    // Test with invalid size (not a perfect square)
    std::panic::catch_unwind(|| {
        Sudoku::new(10);
    }).unwrap_err();
}

#[test]
fn test_from_string_errors() {
    // Test with wrong length
    let result = Sudoku::from_string("123", 9);
    assert!(result.is_err());
    
    // Test with invalid characters
    let result = Sudoku::from_string("53007000060019500009800006080006000340080300170002000606000028000041900500008007X", 9);
    assert!(result.is_err());
}

#[test]
fn test_solver_stats() {
    let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();
    
    let mut solver = SudokuSolver::new();
    let (solution, stats) = solver.solve_with_stats(puzzle).unwrap();
    
    assert!(solution.is_complete());
    assert!(solution.is_valid());
    assert!(stats.iterations > 0);
    assert!(stats.cells_filled > 0);
}
