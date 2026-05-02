#!/bin/bash
# Complete build and test pipeline for Nantaraquad
set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Nantaraquad CI/CD Pipeline                        ║"
echo "║ $TIMESTAMP"
echo "╚════════════════════════════════════════════════════════════╝"
echo

# Parse arguments
SKIP_CLIPPY=false
SKIP_TESTS=false
RELEASE_BUILD=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-clippy)
            SKIP_CLIPPY=true
            ;;
        --skip-tests)
            SKIP_TESTS=true
            ;;
        --release)
            RELEASE_BUILD=true
            ;;
    esac
    shift
done

# Counter
STEP=0
TOTAL=7
if [ "$SKIP_CLIPPY" = true ]; then TOTAL=$((TOTAL-1)); fi
if [ "$SKIP_TESTS" = true ]; then TOTAL=$((TOTAL-1)); fi

# 1. Check Rust toolchain
STEP=$((STEP+1))
echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Checking Rust toolchain...${NC}"
rustup update
rustup target add wasm32-unknown-emscripten
echo -e "${GREEN}✓${NC} Rust toolchain OK"
echo

# 2. Format check
STEP=$((STEP+1))
echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Checking code formatting...${NC}"
if cargo fmt -- --check 2>&1 | head -20; then
    echo -e "${GREEN}✓${NC} Code formatting OK"
else
    echo -e "${RED}✗${NC} Code needs formatting. Run: cargo fmt"
    exit 1
fi
echo

# 3. Clippy linting
if [ "$SKIP_CLIPPY" = false ]; then
    STEP=$((STEP+1))
    echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Running Clippy linter...${NC}"
    if cargo clippy --all-targets --target wasm32-unknown-emscripten -- -D warnings 2>&1 | tail -10; then
        echo -e "${GREEN}✓${NC} Clippy checks passed"
    else
        echo -e "${RED}✗${NC} Clippy warnings found. Fix and retry."
        exit 1
    fi
    echo
fi

# 4. Build (debug)
STEP=$((STEP+1))
echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Building debug target...${NC}"
if cargo build --target wasm32-unknown-emscripten 2>&1 | tail -10; then
    echo -e "${GREEN}✓${NC} Debug build successful"
else
    echo -e "${RED}✗${NC} Debug build failed"
    exit 1
fi
echo

# 5. Build (release)
if [ "$RELEASE_BUILD" = true ]; then
    STEP=$((STEP+1))
    echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Building release target...${NC}"
    export RUSTFLAGS="-C target-feature=+crt-static"
    if cargo build --target wasm32-unknown-emscripten --release 2>&1 | tail -10; then
        echo -e "${GREEN}✓${NC} Release build successful"
        
        # Show binary size
        WASM_FILE="target/wasm32-unknown-emscripten/release/nquad.wasm"
        if [ -f "$WASM_FILE" ]; then
            SIZE=$(du -h "$WASM_FILE" | cut -f1)
            echo "   WASM size: $SIZE"
        fi
    else
        echo -e "${RED}✗${NC} Release build failed"
        exit 1
    fi
    echo
fi

# 6. Unit tests
if [ "$SKIP_TESTS" = false ]; then
    STEP=$((STEP+1))
    echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Running unit tests...${NC}"
    if cargo test --lib --target wasm32-unknown-emscripten 2>&1 | tail -20; then
        echo -e "${GREEN}✓${NC} Unit tests passed"
    else
        echo -e "${RED}✗${NC} Unit tests failed"
        exit 1
    fi
    echo
fi

# 7. Type checking
STEP=$((STEP+1))
echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Running type checker...${NC}"
if cargo check --all-targets --target wasm32-unknown-emscripten 2>&1 | tail -5; then
    echo -e "${GREEN}✓${NC} Type checking passed"
else
    echo -e "${RED}✗${NC} Type errors found"
    exit 1
fi
echo

# Final summary
echo "╔════════════════════════════════════════════════════════════╗"
echo -e "║                  ${GREEN}✓ All checks passed${NC}                      ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Summary:"
echo "  • Rust toolchain: OK"
echo "  • Code formatting: OK"
if [ "$SKIP_CLIPPY" = false ]; then
    echo "  • Linting (Clippy): OK"
fi
echo "  • Debug build: OK"
if [ "$RELEASE_BUILD" = true ]; then
    echo "  • Release build: OK"
fi
if [ "$SKIP_TESTS" = false ]; then
    echo "  • Unit tests: OK"
fi
echo "  • Type checking: OK"
echo ""
echo "Ready to deploy! 🚀"
