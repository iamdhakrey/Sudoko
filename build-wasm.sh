#!/bin/bash

# Build script for Sudoku Solver WASM package

echo "Building Sudoku Solver for WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Build for different targets
echo "Building for web target..."
wasm-pack build --target web --out-dir pkg-web --release

echo "Building for Node.js target..."
wasm-pack build --target nodejs --out-dir pkg-nodejs --release

echo "Building for bundler target..."
wasm-pack build --target bundler --out-dir pkg-bundler --release

echo "WASM build complete!"

# Copy the web build to the web-example directory
if [ -d "pkg-web" ]; then
    echo "Copying WASM files to web-example..."
    rm -rf web-example/pkg
    cp -r pkg-web web-example/pkg
    echo "Web example ready! Open web-example/index.html in a browser."
    echo "Note: You may need to serve the files from a web server due to CORS restrictions."
    echo "Try: python3 -m http.server 8000 in the web-example directory"
fi

echo "Done!"
