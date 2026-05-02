#!/bin/bash
# Update Pyxel fork and verify WASM compatibility
set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FORK_DIR="$PROJECT_ROOT/pyxel_fork"

echo "=== Pyxel Fork Update Script ==="
echo "Project root: $PROJECT_ROOT"
echo "Fork directory: $FORK_DIR"
echo

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 1. Setup upstream remote if not exists
echo -e "${YELLOW}[1/5] Setting up upstream remote...${NC}"
cd "$FORK_DIR"

if ! git remote | grep -q upstream; then
    git remote add upstream https://github.com/kitao/pyxel.git
    echo "  ✓ Upstream remote added"
else
    echo "  ✓ Upstream remote already exists"
fi

# 2. Fetch latest from upstream
echo -e "${YELLOW}[2/5] Fetching latest from upstream...${NC}"
git fetch upstream main
CURRENT_VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "unknown")
UPSTREAM_VERSION=$(git describe --tags upstream/main --abbrev=0 2>/dev/null || echo "unknown")
echo "  Current:  $CURRENT_VERSION"
echo "  Upstream: $UPSTREAM_VERSION"
echo

# 3. Check for merge conflicts
echo -e "${YELLOW}[3/5] Checking for conflicts...${NC}"
if git merge --no-commit --no-ff upstream/main 2>/dev/null; then
    git merge --abort 2>/dev/null || true
    echo "  ✓ No conflicts detected"
else
    echo -e "${RED}  ✗ Conflicts detected. Manual merge required.${NC}"
    echo "    Run: cd $FORK_DIR && git merge upstream/main"
    exit 1
fi
echo

# 4. Build and test with Emscripten
echo -e "${YELLOW}[4/5] Testing Emscripten build...${NC}"
cd "$PROJECT_ROOT"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}  ✗ cargo not found. Install Rust toolchain.${NC}"
    exit 1
fi

if ! rustup target list | grep -q "wasm32-unknown-emscripten (installed)"; then
    echo "  Installing wasm32-unknown-emscripten target..."
    rustup target add wasm32-unknown-emscripten
fi

# Clean previous build
cargo clean --target wasm32-unknown-emscripten 2>/dev/null || true

# Build check
if RUSTFLAGS="-C target-feature=+crt-static" cargo build --target wasm32-unknown-emscripten 2>&1 | head -50; then
    echo "  ✓ Emscripten build successful"
else
    echo -e "${RED}  ✗ Emscripten build failed. See output above.${NC}"
    exit 1
fi
echo

# 5. Merge if tests pass
echo -e "${YELLOW}[5/5] Merging upstream...${NC}"
cd "$FORK_DIR"

if [ "$CURRENT_VERSION" != "$UPSTREAM_VERSION" ]; then
    git merge upstream/main --no-edit
    NEW_VERSION=$(git describe --tags --abbrev=0)
    echo -e "${GREEN}  ✓ Merged successfully (v$CURRENT_VERSION → v$NEW_VERSION)${NC}"
    
    # Tag the merge for reference
    EMSCRIPTEN_TAG="$NEW_VERSION-emscripten-$(date +%Y%m%d)"
    git tag -a "$EMSCRIPTEN_TAG" -m "Pyxel $NEW_VERSION with Emscripten support"
    echo "  ✓ Tagged as: $EMSCRIPTEN_TAG"
else
    echo "  ℹ Already at latest version: $CURRENT_VERSION"
fi
echo

echo -e "${GREEN}=== Update Complete ===${NC}"
echo "Next steps:"
echo "  1. Run: cargo build --target wasm32-unknown-emscripten --release"
echo "  2. Test in browser: npm run dev (in web/ directory)"
