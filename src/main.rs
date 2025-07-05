use std::env;
use std::process;
use sudoko::generate_puzzle;
use sudoko::get_hint;
use sudoko::solve_from_file;
use sudoko::solve_puzzle;
use sudoko::validate_puzzle;

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
