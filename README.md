# Sudoku Solver - Modular Rust Workspace

A comprehensive, modular Rust library and application suite for solving advanced Sudoku puzzles of various sizes (3x3, 4x4, 5x5, and more) using multiple solving strategies, with WebAssembly (WASM) and terminal UI support.

## Features

- **Multiple Grid Sizes**: 4x4 (2x2), 9x9 (3x3), 16x16 (4x4), 25x25 (5x5), and more
- **Advanced Solving Strategies**: Naked/Hidden Singles, Pairs, X-Wing, Swordfish, and more
- **Backtracking Algorithm**: For hard puzzles
- **WebAssembly Support**: Use the library in web browsers
- **Puzzle Generation**: Create new puzzles with difficulty levels
- **Validation & Hints**: Check solutions and get next-move suggestions
- **Statistics**: Track solving performance and strategy usage
- **Simple Text UIs**: Both TUI and WASM use lightweight text rendering

---

## Project Structure

This workspace consists of three main crates:

### 📚 `sudoko` - Core Library
- Located in `sudoko/`
- Provides all puzzle logic, solving, generation, validation, and CLI
- **Public API:**
  - `Sudoku`, `SudokuSolver`, `Cell`, `Difficulty`, and utilities

### 🖥️ `sudoko-tui` - Terminal User Interface
- Located in `sudoko-tui/`
- Simple interactive terminal UI (WASD navigation, number input, solve, etc.)
- Uses only the public API from the core library
- No external UI dependencies

### 🌐 `sudoko-wasm` - WebAssembly Interface
- Located in `sudoko-wasm/`
- WASM-compatible API for web integration
- Text-based rendering for simple web UIs
- All core functionality accessible from JavaScript

---

## Building and Running

### Prerequisites
- Rust 1.70+ with Cargo

### Build All Crates
```bash
cargo build
```

### Build Individual Crates
```bash
# Core library
cargo build -p sudoko

# Terminal UI
cargo build -p sudoko-tui

# WebAssembly
cargo build -p sudoko-wasm
```

### Running Applications

#### CLI Interface
```bash
# Solve a puzzle
cargo run -p sudoko --bin sudoko-cli -- solve "530070000600195000..." 9

# Generate a puzzle
cargo run -p sudoko --bin sudoko-cli -- generate 9 hard

# Get help
cargo run -p sudoko --bin sudoko-cli -- --help
```

#### Terminal UI
```bash
cargo run -p sudoko-tui
```

#### WebAssembly (WASM)
Use the provided `build-wasm.sh` script to build for web, node, and bundler targets:
```bash
./build-wasm.sh
```
See `web-example/` for a browser demo.

---

## Usage Examples

### Core Library (Rust)
```rust
use sudoko::{Sudoku, SudokuSolver, Difficulty};

// Create a new puzzle
let mut puzzle = Sudoku::new(9);

// Load from string
let puzzle = Sudoku::from_string("530070000...", 9)?;

// Solve the puzzle
let mut solver = SudokuSolver::new();
let solution = solver.solve(puzzle)?;

// Generate a new puzzle
let puzzle = solver.generate_puzzle(9, Difficulty::Hard)?;
```

### WASM (JavaScript)
```javascript
import { WasmSudoku } from './pkg/sudoko_wasm.js';

// Create a new puzzle
const sudoku = new WasmSudoku(9);

// Load an example puzzle
const example = create_example_puzzle();

// Solve it
example.solve();

// Render as text
console.log(example.render_text());
```

---

## Architecture Benefits

1. **Separation of Concerns**: Core logic, UI, and web interface are completely separate
2. **Reusability**: The core library can be used in any Rust project
3. **Web Ready**: WASM crate enables easy web integration
4. **Simple UIs**: Both TUI and WASM use simple text rendering (no heavy dependencies)
5. **Public API**: Clean, well-defined public interfaces for all functionality

## Development Notes

- All crates use only the public API from the core library, ensuring clean boundaries and maintainable code.
- The implementations avoid heavy dependencies, making them lightweight and easy to integrate into different environments.
- See `ARCHITECTURE.md` for more technical details (if present).

## License

MIT
