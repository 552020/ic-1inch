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

# Step 1: Check if DFX is running
echo -e "\n${YELLOW}Step 1: Starting local replica...${NC}"
if dfx ping > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Local replica is already running and accessible${NC}"
else
    echo -e "${YELLOW}🔄 Starting local replica...${NC}"
    dfx start --background --clean
    echo -e "${GREEN}✅ Local replica started successfully${NC}"
fi

# Step 2: Compile the relayer canister
echo -e "\n${YELLOW}Step 2: Compiling Relayer Canister...${NC}"
wasm_file="target/wasm32-unknown-unknown/release/relayer.wasm"

if [ -f "$wasm_file" ]; then
    echo -e "${YELLOW}🔄 Source files changed - recompiling...${NC}"
fi

# Build the canister
start_time=$(date +%s.%N)
cargo build --target wasm32-unknown-unknown --release -p relayer
end_time=$(date +%s.%N)

# Calculate duration (requires bc)
if command -v bc > /dev/null 2>&1; then
    duration=$(echo "$end_time - $start_time" | bc)
else
    duration="N/A"
fi

echo -e "${GREEN}✅ Compilation completed in ${duration}s${NC}"
echo -e "${GREEN}📁 WASM file: $wasm_file${NC}"
echo -e "${GREEN}📏 Size: $(ls -lh "$wasm_file" | awk '{print $5}')${NC}"

# Step 3: Deploy the canister
echo -e "\n${YELLOW}Step 3: Deploying Relayer Canister...${NC}"
echo -e "${YELLOW}🔄 Force redeploying to get latest clean code...${NC}"
printf "yes\nyes\n" | dfx deploy relayer --mode reinstall --network local
echo -e "${GREEN}✅ Relayer canister deployed successfully${NC}"
echo -e "${CYAN}Canister ID: $(dfx canister id relayer --network local)${NC}"

# Step 4: Generate DID files
echo -e "\n${YELLOW}Step 4: Generating Candid interface...${NC}"
generate-did relayer
echo -e "${GREEN}✅ Candid interface generated${NC}"

# Step 5: Generate TypeScript declarations
echo -e "\n${YELLOW}Step 5: Generating TypeScript declarations...${NC}"
dfx generate relayer
echo -e "${GREEN}✅ TypeScript declarations generated${NC}"

# Step 6: Verify deployment
echo -e "\n${YELLOW}Step 6: Verifying deployment...${NC}"
if dfx canister call relayer fusion_plus_orders_active --query --network local >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Canister is responding to queries${NC}"
else
    echo -e "${RED}❌ Canister is not responding to queries${NC}"
    exit 1
fi

echo -e "\n${GREEN}🎉 Setup completed successfully!${NC}"
echo -e "${CYAN}You can now test the relayer with:${NC}"
echo -e "${CYAN}  1. Maker: ./scripts/relayer/maker-submit-order.sh${NC}"
echo -e "${CYAN}  2. Taker: ./scripts/relayer/taker-fetch-orders.sh${NC}"