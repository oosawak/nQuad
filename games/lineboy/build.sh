#!/bin/bash
# Build Lineboy WASM game

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"
WEB_DIR="$PROJECT_DIR/../../web/lineboy"
BUILDS_DIR="$PROJECT_DIR/../../wasm-builds/lineboy"

echo "🔨 Building Lineboy WASM..."
cd "$PROJECT_DIR"

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Build WASM
echo "📦 Compiling to WebAssembly..."
wasm-pack build --target web --release --out-dir ../../wasm-builds/lineboy --no-opt

echo "✅ Build complete!"
echo ""
echo "📂 Output files:"
ls -lh "$BUILDS_DIR"/lineboy*

echo ""
echo "📋 Copying to web directory..."
mkdir -p "$WEB_DIR"
cp -v "$BUILDS_DIR"/* "$WEB_DIR/"

echo ""
echo "✨ Ready to run!"
echo "   cd ../.."
echo "   node server.js"
echo "   # Open http://localhost:8000/lineboy_test.html in browser"

