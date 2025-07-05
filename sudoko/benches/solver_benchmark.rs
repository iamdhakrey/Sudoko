use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoko::{Sudoku, SudokuSolver};

fn benchmark_9x9_solve(c: &mut Criterion) {
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

    c.bench_function("solve 9x9 easy", |b| {
        b.iter(|| {
            let mut solver = SudokuSolver::new();
            solver.solve(black_box(puzzle.clone())).unwrap()
        })
    });
}

fn benchmark_4x4_solve(c: &mut Criterion) {
    let puzzle_str = "1.3..2.43.1..4.2";
    let puzzle = Sudoku::from_string(puzzle_str, 4).unwrap();

    c.bench_function("solve 4x4", |b| {
        b.iter(|| {
            let mut solver = SudokuSolver::new();
            solver.solve(black_box(puzzle.clone())).unwrap()
        })
    });
}

fn benchmark_validation(c: &mut Criterion) {
    let puzzle_str =
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

    c.bench_function("validate complete 9x9", |b| {
        b.iter(|| black_box(&puzzle).is_valid())
    });
}

fn benchmark_candidates(c: &mut Criterion) {
    let puzzle_str =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

    c.bench_function("get candidates", |b| {
        b.iter(|| black_box(&puzzle).get_candidates(0, 2))
    });
}

fn benchmark_puzzle_generation(c: &mut Criterion) {
    c.bench_function("generate 9x9 medium", |b| {
        b.iter(|| {
            let mut solver = SudokuSolver::new();
            solver
                .generate_puzzle(9, sudoko::Difficulty::Medium)
                .unwrap()
        })
    });
}

criterion_group!(
    benches,
    benchmark_9x9_solve,
    // benchmark_4x4_solve,
    benchmark_validation,
    benchmark_candidates,
    benchmark_puzzle_generation
);
criterion_main!(benches);
