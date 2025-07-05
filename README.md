# Sudoku Solver Library

A comprehensive Rust library for solving advanced Sudoku puzzles of various sizes (3x3, 4x4, 5x5, and more) using multiple solving strategies, with WebAssembly (WASM) support.

## Features

- **Multiple Grid Sizes**: Support for 4x4 (2x2 boxes), 9x9 (3x3 boxes), 16x16 (4x4 boxes), 25x25 (5x5 boxes), and more
- **Advanced Solving Strategies**:
  - Naked Singles
  - Hidden Singles
  - Naked Pairs (framework ready)
  - Pointing Pairs (framework ready)
  - Box/Line Reduction (framework ready)
  - X-Wing (framework ready)
  - Swordfish (framework ready)
- **Backtracking Algorithm**: For puzzles that require brute force
- **WebAssembly Support**: Use the library in web browsers
- **Puzzle Generation**: Create new puzzles with different difficulty levels
- **Validation**: Check if puzzles and solutions are valid
- **Hints System**: Get suggestions for the next move
- **Statistics**: Track solving performance and strategy usage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sudoko = "0.1.0"
```

## Usage

### Command Line Interface

```bash
# Solve a 9x9 Sudoku puzzle
cargo run -- solve "530070000600195000098000060800060003400803001700020006060000280000419005000080079" 9

# Generate a new puzzle
cargo run -- generate 9 hard

# Validate a solution
cargo run -- validate "123456789456789123789123456234567891567891234891234567345678912678912345912345678" 9

# Get a hint
cargo run -- hint "530070000600195000098000060800060003400803001700020006060000280000419005000080079" 9
```

### Library Usage

```rust
use sudoko::{Sudoku, SudokuSolver};

// Create a puzzle from string
let puzzle = Sudoku::from_string(
    "530070000600195000098000060800060003400803001700020006060000280000419005000080079",
    9
).unwrap();

// Create solver and solve
let mut solver = SudokuSolver::new();
let solution = solver.solve(puzzle).unwrap();

println!("Solution: {}", solution);
```

### WebAssembly Usage

Build for WASM:

```bash
wasm-pack build --target web
```

Use in JavaScript:

```javascript
import init, { WasmSudokuSolver } from './pkg/sudoko.js';

async function solveSudoku() {
    await init();
    
    const solver = new WasmSudokuSolver();
    const puzzle = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    const solution = solver.solve_from_string(puzzle, 9);
    
    console.log("Solution:", solution);
}
```

## Supported Puzzle Formats

### Input Format
- Use digits 1-9 for standard 9x9 Sudokus
- Use digits 1-9 and letters A-F for 16x16 Sudokus
- Use 0, '.', or ' ' for empty cells

### Examples

**4x4 Sudoku:**
```
1.3.
.2.4
3.1.
.4.2
```

**9x9 Sudoku:**
```
530070000
600195000
098000060
800060003
400803001
700020006
060000280
000419005
000080079
```

**16x16 Sudoku:**
```
1.3...7.9.B.D.F.
.2.4.6.8.A.C.E.0
...
```

## Difficulty Levels

- **Easy**: ~40% of cells removed
- **Medium**: ~50% of cells removed  
- **Hard**: ~60% of cells removed
- **Expert**: ~70% of cells removed

## Building

### Native Build
```bash
cargo build --release
```

### WASM Build
```bash
# Install wasm-pack first
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for web
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers
wasm-pack build --target bundler
```

## Testing

```bash
cargo test
```

## Performance

The library uses optimized algorithms:
- **MRV (Minimum Remaining Values) heuristic** for backtracking
- **Constraint propagation** through logical strategies
- **Early termination** when solutions are found

## Contributing

Contributions are welcome! Areas for improvement:
- Implementation of advanced strategies (Naked Pairs, X-Wing, etc.)
- Better puzzle generation algorithms
- Performance optimizations
- Additional puzzle formats

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Examples

See the `examples/` directory for more usage examples:
- Basic solving
- Custom strategies
- WASM integration
- Puzzle generation
