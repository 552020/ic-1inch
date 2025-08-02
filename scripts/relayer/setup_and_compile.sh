#!/bin/bash

# Manual Setup and Compilation Script for Relayer Canister
# Run this script once to compile and deploy the canister before running tests
#
# USAGE: Run this script from the project root directory (ic-1inch/)
# Example: ./scripts/relayer/setup_and_compile.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Relayer Canister Setup and Compilation${NC}"
echo "================================================"

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo -e "${RED}❌ Error: Please run this script from the project root directory (ic-1inch/)${NC}"
    echo -e "${YELLOW}Current directory: $(pwd)${NC}"
    echo -e "${YELLOW}Expected: dfx.json should be in current directory${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Running from project root directory${NC}"

# Step 1: Start local replica if not running
echo -e "\n${YELLOW}Step 1: Starting local replica...${NC}"

# Check if dfx is already running
if dfx ping local >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Local replica is already running and accessible${NC}"
else
    echo -e "${YELLOW}🔄 Starting local replica...${NC}"
    # Try to start dfx, but handle if it's already running
    if ! dfx start --clean --background 2>/dev/null; then
        echo -e "${YELLOW}⚠️  dfx is already running, checking if it's accessible...${NC}"
        sleep 3
        if dfx ping local >/dev/null 2>&1; then
            echo -e "${GREEN}✅ Local replica is now accessible${NC}"
        else
            echo -e "${RED}❌ dfx is running but not responding to ping${NC}"
            echo -e "${YELLOW}💡 You may need to restart dfx manually: pkill -f 'dfx start' && dfx start --clean --background${NC}"
            exit 1
        fi
    else
        echo -e "${GREEN}✅ Local replica started successfully${NC}"
        sleep 5  # Wait for replica to start
    fi
fi

# Step 2: Compile the canister
echo -e "\n${YELLOW}Step 2: Compiling Relayer Canister...${NC}"
start_time=$(date +%s.%N)

# Check if compilation is needed
wasm_file="target/wasm32-unknown-unknown/release/relayer.wasm"
relayer_dir="src/relayer"

if [ -f "$wasm_file" ]; then
    # Check if source files are newer than WASM
    newest_source=$(find "$relayer_dir/src" -name "*.rs" -newer "$wasm_file" 2>/dev/null | head -1)
    if [ -z "$newest_source" ]; then
        echo -e "${GREEN}✅ Using existing WASM file (no recompilation needed)${NC}"
    else
        echo -e "${YELLOW}🔄 Source files changed - recompiling...${NC}"
        (cd "$relayer_dir" && cargo build --target wasm32-unknown-unknown --release)
    fi
else
    echo -e "${YELLOW}🔄 Compilation needed - building canister...${NC}"
    (cd "$relayer_dir" && cargo build --target wasm32-unknown-unknown --release)
fi

    end_time=$(date +%s.%N)
    duration=$(echo "$end_time - $start_time" | bc)
echo -e "${GREEN}✅ Compilation completed in ${duration}s${NC}"
echo -e "${GREEN}📁 WASM file: $wasm_file${NC}"
echo -e "${GREEN}📏 Size: $(ls -lh "$wasm_file" | awk '{print $5}')${NC}"

# Step 3: Deploy the canister
echo -e "\n${YELLOW}Step 3: Deploying Relayer Canister...${NC}"
if dfx canister status relayer --network local >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Relayer canister is already deployed${NC}"
    echo -e "${CYAN}Canister ID: $(dfx canister id relayer --network local)${NC}"
else
    echo -e "${YELLOW}🔄 Deploying canister...${NC}"
    dfx deploy relayer --network local
    echo -e "${GREEN}✅ Relayer canister deployed successfully${NC}"
    echo -e "${CYAN}Canister ID: $(dfx canister id relayer --network local)${NC}"
fi

# Step 4: Verify deployment
echo -e "\n${YELLOW}Step 4: Verifying deployment...${NC}"
if dfx canister call relayer get_active_fusion_orders --query --network local >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Canister is responding to queries${NC}"
else
    echo -e "${RED}❌ Canister is not responding to queries${NC}"
    exit 1
fi

echo -e "\n${GREEN}🎉 Setup completed successfully!${NC}"
echo -e "${CYAN}You can now run tests with: ./scripts/relayer/run_all_tests.sh${NC}"
echo -e "${CYAN}Or run individual tests from the scripts/relayer/ directory${NC}" 