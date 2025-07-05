mod solver;
mod strategies;
mod sudoku;
mod utils;

#[cfg(feature = "wasm")]
mod wasm;

pub use solver::{Difficulty, SudokuSolver};
pub use strategies::*;
pub use sudoku::{Cell, Sudoku};

#[cfg(feature = "wasm")]
pub use wasm::*;

use std::{fs, process};

pub fn solve_puzzle(puzzle_str: &str, size_str: &str) {
    let size = match size_str.parse::<usize>() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Invalid size: {}", size_str);
            process::exit(1);
        }
    };

    let puzzle = match Sudoku::from_string(puzzle_str, size) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            process::exit(1);
        }
    };

    println!("Original puzzle:");
    println!("{}", puzzle);

    let mut solver = SudokuSolver::new();

    match solver.solve_with_stats(puzzle) {
        Ok((solution, stats)) => {
            println!("Solution found!");
            println!("{}", solution);
            println!("\nSolver Statistics:");
            println!("Iterations: {}", stats.iterations);
            println!("Cells filled: {}", stats.cells_filled);
            println!("Backtrack steps: {}", stats.backtrack_steps);
            println!("Strategies used:");
            for (strategy, count) in stats.strategies_used {
                println!("  {}: {}", strategy, count);
            }
        }
        Err(e) => {
            eprintln!("Failed to solve puzzle: {}", e);
            process::exit(1);
        }
    }
}

pub fn solve_from_file(file_path: &str, size_str: &str) {
    let puzzle_str = match fs::read_to_string(file_path) {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    solve_puzzle(&puzzle_str, size_str);
}

pub fn generate_puzzle(size_str: &str, difficulty_str: &str) {
    let size = match size_str.parse::<usize>() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Invalid size: {}", size_str);
            process::exit(1);
        }
    };

    let difficulty = match difficulty_str.to_lowercase().as_str() {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        "hard" => Difficulty::Hard,
        "expert" => Difficulty::Expert,
        _ => {
            eprintln!(
                "Invalid difficulty: {}. Use easy, medium, hard, or expert",
                difficulty_str
            );
            process::exit(1);
        }
    };

    let mut solver = SudokuSolver::new();

    match solver.generate_puzzle(size, difficulty) {
        Ok(puzzle) => {
            println!("Generated {} puzzle ({}x{}):", difficulty_str, size, size);
            println!("{}", puzzle);
        }
        Err(e) => {
            eprintln!("Failed to generate puzzle: {}", e);
            process::exit(1);
        }
    }
}

pub fn validate_puzzle(puzzle_str: &str, size_str: &str) {
    let size = match size_str.parse::<usize>() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Invalid size: {}", size_str);
            process::exit(1);
        }
    };

    let puzzle = match Sudoku::from_string(puzzle_str, size) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            process::exit(1);
        }
    };

    println!("Puzzle:");
    println!("{}", puzzle);

    if puzzle.is_valid() {
        println!("✓ Puzzle is valid!");

        if puzzle.is_complete() {
            println!("✓ Puzzle is complete and solved!");
        } else {
            println!("! Puzzle is valid but not yet complete.");
        }
    } else {
        println!("✗ Puzzle is invalid!");

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

pub fn get_hint(puzzle_str: &str, size_str: &str) {
    let size = match size_str.parse::<usize>() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Invalid size: {}", size_str);
            process::exit(1);
        }
    };

    let mut puzzle = match Sudoku::from_string(puzzle_str, size) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            process::exit(1);
        }
    };

    println!("Current puzzle:");
    println!("{}", puzzle);

    let mut solver = SudokuSolver::new();

    match solver.get_hint(&mut puzzle) {
        Some((row, col, value)) => {
            println!(
                "Hint: Place {} at position ({}, {})",
                value,
                row + 1,
                col + 1
            );
            puzzle.set(row, col, value).unwrap();
            println!("\nPuzzle with hint applied:");
            println!("{}", puzzle);
        }
        None => {
            println!("No obvious hint available. You might need to use more advanced techniques.");
        }
    }
}
