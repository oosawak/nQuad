#!/bin/bash
# Setup Emscripten SDK for Nantaraquad WASM builds
set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Emscripten SDK Setup for Nantaraquad              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Detect OS
OS=""
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    OS="windows"
else
    echo -e "${RED}Unsupported OS: $OSTYPE${NC}"
    exit 1
fi

echo -e "${BLUE}Detected OS: $OS${NC}"
echo

# 1. Check prerequisites
echo -e "${YELLOW}[1/4] Checking prerequisites...${NC}"

if ! command -v git &> /dev/null; then
    echo -e "${RED}✗ git not found. Install from https://git-scm.com/${NC}"
    exit 1
fi

if ! command -v python3 &> /dev/null; then
    echo -e "${RED}✗ python3 not found. Install Python 3.8+${NC}"
    exit 1
fi

echo -e "${GREEN}✓ git: $(git --version | head -1)${NC}"
echo -e "${GREEN}✓ python3: $(python3 --version)${NC}"
echo

# 2. Clone/Update Emscripten SDK
echo -e "${YELLOW}[2/4] Setting up Emscripten SDK...${NC}"

if [ "$OS" == "windows" ]; then
    EMSDK_DIR="C:/emsdk"
else
    EMSDK_DIR="$HOME/emsdk"
fi

if [ ! -d "$EMSDK_DIR" ]; then
    echo "Cloning Emscripten SDK to: $EMSDK_DIR"
    git clone https://github.com/emscripten-core/emsdk.git "$EMSDK_DIR"
else
    echo "Emscripten SDK already exists at: $EMSDK_DIR"
    cd "$EMSDK_DIR"
    git pull origin main --quiet
    cd -
fi

echo -e "${GREEN}✓ Emscripten SDK ready${NC}"
echo

# 3. Install latest Emscripten
echo -e "${YELLOW}[3/4] Installing latest Emscripten...${NC}"

cd "$EMSDK_DIR"

if [ "$OS" == "windows" ]; then
    ./emsdk.bat install latest
    ./emsdk.bat activate latest
else
    ./emsdk install latest
    ./emsdk activate latest
fi

echo -e "${GREEN}✓ Emscripten installed${NC}"
echo

# 4. Setup environment
echo -e "${YELLOW}[4/4] Setting up environment...${NC}"

if [ "$OS" == "windows" ]; then
    echo -e "${YELLOW}Manual steps for Windows:${NC}"
    echo "1. Open Environment Variables (Win+R → sysdm.cpl)"
    echo "2. Add new system variable:"
    echo "   Variable: EMSDK"
    echo "   Value: C:\\emsdk"
    echo "3. Update PATH to include: C:\\emsdk\\upstream\\emscripten"
    echo "4. Restart your terminal/IDE"
else
    SHELL_PROFILE=""
    if [ -f "$HOME/.bashrc" ]; then
        SHELL_PROFILE="$HOME/.bashrc"
    elif [ -f "$HOME/.zshrc" ]; then
        SHELL_PROFILE="$HOME/.zshrc"
    fi
    
    if [ -n "$SHELL_PROFILE" ]; then
        # Check if already added
        if ! grep -q "emsdk_env.sh" "$SHELL_PROFILE"; then
            echo "Adding Emscripten to $SHELL_PROFILE"
            echo "" >> "$SHELL_PROFILE"
            echo "# Emscripten SDK" >> "$SHELL_PROFILE"
            echo "source $EMSDK_DIR/emsdk_env.sh" >> "$SHELL_PROFILE"
            
            # Also source it now
            source "$EMSDK_DIR/emsdk_env.sh"
            echo -e "${GREEN}✓ Added to shell profile${NC}"
        else
            echo -e "${YELLOW}Already configured in $SHELL_PROFILE${NC}"
            source "$EMSDK_DIR/emsdk_env.sh"
        fi
    fi
fi

echo

# 5. Verify installation
echo -e "${YELLOW}Verifying Emscripten installation...${NC}"

if command -v emcc &> /dev/null; then
    echo -e "${GREEN}✓ emcc is accessible${NC}"
    echo "  Version: $(emcc --version | head -1)"
else
    echo -e "${YELLOW}⚠ emcc not yet in PATH${NC}"
    echo ""
    if [ "$OS" != "windows" ]; then
        echo "Please run:"
        echo "  source $EMSDK_DIR/emsdk_env.sh"
        echo ""
        echo "Or restart your terminal if you added it to your shell profile."
    fi
fi

echo

# 6. Install Rust wasm target
echo -e "${YELLOW}Installing Rust wasm32-unknown-emscripten target...${NC}"
if rustup target add wasm32-unknown-emscripten 2>&1 | tail -3; then
    echo -e "${GREEN}✓ Rust target installed${NC}"
else
    echo -e "${RED}✗ Failed to install Rust target${NC}"
    exit 1
fi

echo
echo "╔════════════════════════════════════════════════════════════╗"
echo -e "║              ${GREEN}✓ Setup Complete${NC}                           ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Next steps:"
echo "  1. Verify setup:"
echo "     emcc --version"
echo ""
echo "  2. Build Nantaraquad for WASM:"
echo "     cd /path/to/Nantaraquad"
echo "     ./scripts/build-wasm.sh --release"
echo ""
echo "  3. For detailed troubleshooting, see:"
echo "     EMSCRIPTEN_SETUP.md"
echo ""
