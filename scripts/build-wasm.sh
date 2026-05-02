#!/bin/bash
# Build Nantaraquad for Emscripten (WASM) - Updated Strategy
set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "=== Emscripten WASM Build ==="
echo "Project: $PROJECT_ROOT"
echo

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Parse arguments
BUILD_MODE="debug"
RELEASE_FLAG=""
CLEAN_FLAG=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            BUILD_MODE="release"
            RELEASE_FLAG="--release"
            ;;
        --clean)
            CLEAN_FLAG="yes"
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --release    Build optimized release binary"
            echo "  --clean      Clean before building"
            echo "  --help       Show this help message"
            exit 0
            ;;
    esac
    shift
done

# 1. Check dependencies
echo -e "${YELLOW}[1/4] Checking dependencies...${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ cargo not found. Install Rust: https://rustup.rs/${NC}"
    exit 1
fi

if ! command -v emcc &> /dev/null; then
    echo -e "${RED}✗ emcc not found. Install Emscripten: https://emscripten.org/docs/getting_started/${NC}"
    exit 1
fi

# Ensure Emscripten target is installed
if ! rustup target list | grep -q "wasm32-unknown-emscripten (installed)"; then
    echo "Installing wasm32-unknown-emscripten target..."
    rustup target add wasm32-unknown-emscripten
fi

echo -e "${GREEN}✓ Dependencies OK${NC}"
echo "  Rust: $(rustc --version)"
echo "  Cargo: $(cargo --version)"
echo "  Emcc: $(emcc --version | head -1)"
echo

# 2. Clean (optional)
if [ "$CLEAN_FLAG" == "yes" ]; then
    echo -e "${YELLOW}[2/4] Cleaning...${NC}"
    cargo clean --target wasm32-unknown-emscripten
    echo -e "${GREEN}✓ Cleaned${NC}"
    echo
fi

# 3. Build
echo -e "${YELLOW}[3/4] Building ($BUILD_MODE)...${NC}"
cd "$PROJECT_ROOT"

# Set Rust flags for Emscripten
export RUSTFLAGS="-C target-feature=+crt-static"

# Run build
if cargo build --target wasm32-unknown-emscripten $RELEASE_FLAG 2>&1 | tee /tmp/nquad_build.log; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    tail -20 /tmp/nquad_build.log
    exit 1
fi
echo

# 4. Show output
echo -e "${YELLOW}[4/4] Build artifacts:${NC}"

if [ "$BUILD_MODE" == "release" ]; then
    WASM_FILE="target/wasm32-unknown-emscripten/release/nquad.wasm"
else
    WASM_FILE="target/wasm32-unknown-emscripten/debug/nquad.wasm"
fi

if [ -f "$WASM_FILE" ]; then
    SIZE=$(du -h "$WASM_FILE" | cut -f1)
    echo -e "${GREEN}✓ WASM Binary${NC}"
    echo "  Path: $WASM_FILE"
    echo "  Size: $SIZE"
else
    echo -e "${YELLOW}ℹ No WASM binary (library build)${NC}"
fi

echo

# 5. Show next steps
echo -e "${GREEN}=== Build Complete ===${NC}"
echo ""
echo "Next steps:"
echo "  1. Integrate WASM into web project:"
echo "     cp -r web/ <output-dir>"
echo ""
echo "  2. Copy WASM binary (if needed):"
echo "     cp $WASM_FILE web/public/nquad.wasm"
echo ""
echo "  3. Serve and test:"
echo "     cd web && npm run dev"
echo ""

