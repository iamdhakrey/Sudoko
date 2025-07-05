use sudoko::{Sudoku, SudokuSolver, Difficulty};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    match args[1].as_str() {
        "solve" => {
            if args.len() < 4 {
                eprintln!("Usage: {} solve <puzzle_string> <size>", args[0]);
                process::exit(1);
            }
            solve_puzzle(&args[2], &args[3]);
        }
        "solve-file" => {
            if args.len() < 4 {
                eprintln!("Usage: {} solve-file <file_path> <size>", args[0]);
                process::exit(1);
            }
            solve_from_file(&args[2], &args[3]);
        }
        "generate" => {
            if args.len() < 3 {
                eprintln!("Usage: {} generate <size> [difficulty]", args[0]);
                process::exit(1);
            }
            let difficulty = args.get(3).map(|s| s.as_str()).unwrap_or("medium");
            generate_puzzle(&args[2], difficulty);
        }
        "validate" => {
            if args.len() < 4 {
                eprintln!("Usage: {} validate <puzzle_string> <size>", args[0]);
                process::exit(1);
            }
            validate_puzzle(&args[2], &args[3]);
        }
        "hint" => {
            if args.len() < 4 {
                eprintln!("Usage: {} hint <puzzle_string> <size>", args[0]);
                process::exit(1);
            }
            get_hint(&args[2], &args[3]);
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Sudoku Solver Library");
    println!("Usage:");
    println!("  solve <puzzle_string> <size>     - Solve a Sudoku puzzle");
    println!("  solve-file <file_path> <size>    - Solve a Sudoku from file");
    println!("  generate <size> [difficulty]     - Generate a new puzzle");
    println!("  validate <puzzle_string> <size>  - Validate a puzzle");
    println!("  hint <puzzle_string> <size>      - Get a hint for the next move");
    println!();
    println!("Sizes supported: 4 (2x2), 9 (3x3), 16 (4x4), 25 (5x5)");
    println!("Difficulties: easy, medium, hard, expert");
    println!();
    println!("Examples:");
    println!("  solve \"530070000600195000098000060800060003400803001700020006060000280000419005000080079\" 9");
    println!("  generate 9 hard");
    println!("  validate \"123456789456789123789123456234567891567891234891234567345678912678912345912345678\" 9");
}

fn solve_puzzle(puzzle_str: &str, size_str: &str) {
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

fn solve_from_file(file_path: &str, size_str: &str) {
    let puzzle_str = match fs::read_to_string(file_path) {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    solve_puzzle(&puzzle_str, size_str);
}

fn generate_puzzle(size_str: &str, difficulty_str: &str) {
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
            eprintln!("Invalid difficulty: {}. Use easy, medium, hard, or expert", difficulty_str);
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

fn validate_puzzle(puzzle_str: &str, size_str: &str) {
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

fn get_hint(puzzle_str: &str, size_str: &str) {
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
            println!("Hint: Place {} at position ({}, {})", value, row + 1, col + 1);
            puzzle.set(row, col, value).unwrap();
            println!("\nPuzzle with hint applied:");
            println!("{}", puzzle);
        }
        None => {
            println!("No obvious hint available. You might need to use more advanced techniques.");
        }
    }
}