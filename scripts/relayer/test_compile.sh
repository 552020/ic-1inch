#!/bin/bash

# Single compilation script for relayer canister
# This script compiles the canister once and can be sourced by other test scripts

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RELAYER_DIR="$SCRIPT_DIR/../../src/relayer"
BUILD_DIR="$SCRIPT_DIR/../../target/wasm32-unknown-unknown/release"

# Global variables for test scripts to use
COMPILATION_SUCCESS=false
COMPILATION_TIME=0
WASM_FILE=""
WARNINGS_COUNT=0

echo -e "${YELLOW}🔨 Compiling Relayer Canister${NC}"
echo "=================================="

# Function to compile the canister
compile_canister() {
    local start_time=$(date +%s.%N)
    
    echo -e "${YELLOW}Step 1: Checking compilation...${NC}"
    if (cd "$RELAYER_DIR" && cargo check); then
        echo -e "${GREEN}✅ Cargo check passed${NC}"
    else
        echo -e "${RED}❌ Cargo check failed${NC}"
        return 1
    fi
    
    echo -e "${YELLOW}Step 2: Building canister...${NC}"
    if (cd "$RELAYER_DIR" && cargo build --target wasm32-unknown-unknown --release); then
        echo -e "${GREEN}✅ Build successful${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        return 1
    fi
    
    local end_time=$(date +%s.%N)
    COMPILATION_TIME=$(echo "$end_time - $start_time" | bc)
    
    # Check if WASM file exists
    WASM_FILE="$BUILD_DIR/relayer.wasm"
    if [ -f "$WASM_FILE" ]; then
        echo -e "${GREEN}✅ WASM file created: $(basename "$WASM_FILE")${NC}"
        echo -e "${GREEN}📁 Location: $WASM_FILE${NC}"
        echo -e "${GREEN}📏 Size: $(ls -lh "$WASM_FILE" | awk '{print $5}')${NC}"
    else
        echo -e "${RED}❌ WASM file not found${NC}"
        return 1
    fi
    
    # Count warnings
    WARNINGS_COUNT=$((cd "$RELAYER_DIR" && cargo check 2>&1) | grep -c "warning:" || echo "0")
    echo -e "${YELLOW}⚠️  Compilation warnings: $WARNINGS_COUNT${NC}"
    
    COMPILATION_SUCCESS=true
    echo -e "${GREEN}✅ Compilation completed in ${COMPILATION_TIME}s${NC}"
    return 0
}

# Function to check if compilation is needed
is_compilation_needed() {
    # Check if WASM file exists and is newer than source files
    if [ ! -f "$WASM_FILE" ]; then
        return 0  # Compilation needed
    fi
    
    # Check if any source files are newer than WASM file
    local newest_source=$(find "$RELAYER_DIR/src" -name "*.rs" -newer "$WASM_FILE" 2>/dev/null | head -1)
    if [ -n "$newest_source" ]; then
        return 0  # Compilation needed
    fi
    
    return 1  # No compilation needed
}

# Main execution
if is_compilation_needed; then
    echo -e "${YELLOW}🔄 Compilation needed - building canister...${NC}"
    if compile_canister; then
        echo -e "${GREEN}✅ Compilation successful${NC}"
        exit 0
    else
        echo -e "${RED}❌ Compilation failed${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}✅ Compilation not needed - WASM file is up to date${NC}"
    echo -e "${GREEN}📁 Using existing: $WASM_FILE${NC}"
    COMPILATION_SUCCESS=true
    COMPILATION_TIME=0
    WARNINGS_COUNT=$((cd "$RELAYER_DIR" && cargo check 2>&1) | grep -c "warning:" || echo "0")
    echo -e "${YELLOW}⚠️  Compilation warnings: $WARNINGS_COUNT${NC}"
fi

# Export variables for other scripts to use
export COMPILATION_SUCCESS
export COMPILATION_TIME
export WASM_FILE
export WARNINGS_COUNT
export RELAYER_DIR 