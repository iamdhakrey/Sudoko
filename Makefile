# Makefile for Sudoku Solver Library

.PHONY: help build test run examples wasm clean doc

help: ## Show this help message
	@echo "Sudoku Solver Library"
	@echo "Available commands:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the library in release mode
	cargo build --release

test: ## Run all tests
	cargo test

run: ## Run the CLI application with example
	cargo run -- solve "530070000600195000098000060800060003400803001700020006060000280000419005000080079" 9

examples: ## Run example programs
	@echo "Running basic solving example..."
	cargo run --example basic_solving
	@echo "\nRunning puzzle generation example..."
	cargo run --example puzzle_generation
	@echo "\nRunning hints and validation example..."
	cargo run --example hints_and_validation

wasm: ## Build WASM packages for web
	./build-wasm.sh

wasm-dev: ## Build WASM for development (debug mode)
	wasm-pack build --dev --target web --out-dir pkg-web

doc: ## Generate documentation
	cargo doc --open

clean: ## Clean build artifacts
	cargo clean
	rm -rf pkg-*
	rm -rf web-example/pkg

benchmark: ## Run benchmarks (if implemented)
	cargo bench

check: ## Check code without building
	cargo check

fmt: ## Format code
	cargo fmt

clippy: ## Run clippy lints
	cargo clippy -- -D warnings

solve-easy: ## Solve an easy 9x9 puzzle
	cargo run -- solve "530070000600195000098000060800060003400803001700020006060000280000419005000080079" 9

solve-4x4: ## Solve a 4x4 puzzle
	cargo run -- solve "1.3..2.43.1..4.2" 4

generate-9x9: ## Generate a 9x9 puzzle
	cargo run -- generate 9 medium

validate: ## Validate a completed puzzle
	cargo run -- validate "534678912672195348198342567859761423426853791713924856961537284287419635345286179" 9

hint: ## Get a hint for a puzzle
	cargo run -- hint "530070000600195000098000060800060003400803001700020006060000280000419005000080079" 9

dev: build test examples ## Full development cycle

install-deps: ## Install development dependencies
	rustup target add wasm32-unknown-unknown
	@if ! command -v wasm-pack >/dev/null 2>&1; then \
		echo "Installing wasm-pack..."; \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
	fi
