#!/bin/bash

# Deploy fusion+ mechanical turk canisters (orderbook and escrow)
# This script deploys only the canisters needed for the mechanical turk PoC

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Deploying Fusion+ Mechanical Turk Canisters${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo -e "${RED}❌ dfx.json not found. Please run this script from the project root.${NC}"
    exit 1
fi

# Check required tools
echo -e "${YELLOW}🔍 Checking required tools...${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ cargo not found. Please install Rust and Cargo.${NC}"
    exit 1
fi

if ! command -v candid-extractor &> /dev/null; then
    echo -e "${RED}❌ candid-extractor not found. Please install it with:${NC}"
    echo -e "${YELLOW}   cargo install candid-extractor${NC}"
    exit 1
fi

if ! command -v dfx &> /dev/null; then
    echo -e "${RED}❌ dfx not found. Please install the DFINITY SDK.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ All required tools found${NC}"

# Start dfx if not running
echo -e "${YELLOW}📡 Checking DFX status...${NC}"
if ! dfx ping >/dev/null 2>&1; then
    echo -e "${YELLOW}📡 Starting DFX...${NC}"
    dfx start --background --clean
else
    echo -e "${GREEN}✅ DFX is already running${NC}"
fi

echo ""
echo -e "${BLUE}📦 Building canisters (Step 1/3)...${NC}"

# Ensure WASM target is installed
echo -e "${YELLOW}🎯 Ensuring WASM target is installed...${NC}"
rustup target add wasm32-unknown-unknown

# Build orderbook canister to generate WASM
echo -e "${YELLOW}🔨 Building orderbook canister...${NC}"
if ! cargo build --target wasm32-unknown-unknown --release -p orderbook; then
    echo -e "${RED}❌ Failed to build orderbook canister${NC}"
    exit 1
fi

# Build escrow canister to generate WASM
echo -e "${YELLOW}🔨 Building escrow canister...${NC}"
if ! cargo build --target wasm32-unknown-unknown --release -p escrow; then
    echo -e "${RED}❌ Failed to build escrow canister${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}📝 Extracting Candid interfaces (Step 2/3)...${NC}"

# Extract .did files from WASM (solves chicken-and-egg problem)
echo -e "${YELLOW}📄 Extracting orderbook.did...${NC}"
if ! candid-extractor target/wasm32-unknown-unknown/release/orderbook.wasm > src/orderbook/orderbook.did; then
    echo -e "${RED}❌ Failed to extract orderbook.did${NC}"
    exit 1
fi

echo -e "${YELLOW}📄 Extracting escrow.did...${NC}"
if ! candid-extractor target/wasm32-unknown-unknown/release/escrow.wasm > src/escrow/escrow.did; then
    echo -e "${RED}❌ Failed to extract escrow.did${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}🚀 Deploying canisters (Step 3/3)...${NC}"

# Deploy orderbook canister (relayer-controlled order management)
echo -e "${YELLOW}🔄 Deploying orderbook canister...${NC}"
if ! dfx deploy orderbook; then
    echo -e "${RED}❌ Failed to deploy orderbook canister${NC}"
    exit 1
fi

# Deploy escrow canister (ICP token custody)
echo -e "${YELLOW}🔒 Deploying escrow canister...${NC}"
if ! dfx deploy escrow; then
    echo -e "${RED}❌ Failed to deploy escrow canister${NC}"
    exit 1
fi

# Generate declarations for fusion canisters only
echo -e "${YELLOW}📝 Generating fusion canister declarations...${NC}"
echo -e "${YELLOW}  • Generating orderbook declarations...${NC}"
dfx generate orderbook
echo -e "${YELLOW}  • Generating escrow declarations...${NC}"
dfx generate escrow

echo ""
echo -e "${GREEN}✅ Mechanical Turk deployment complete!${NC}"
echo ""
echo -e "${BLUE}📋 Deployed Canisters:${NC}"
echo -e "  • Orderbook: ${GREEN}$(dfx canister id orderbook)${NC}"
echo -e "  • Escrow:    ${GREEN}$(dfx canister id escrow)${NC}"
echo ""
echo -e "${BLUE}🔗 Candid Interfaces:${NC}"
echo -e "  • Orderbook: ${YELLOW}http://localhost:4943/?canisterId=$(dfx canister id __Candid_UI)&id=$(dfx canister id orderbook)${NC}"
echo -e "  • Escrow:    ${YELLOW}http://localhost:4943/?canisterId=$(dfx canister id __Candid_UI)&id=$(dfx canister id escrow)${NC}"
echo ""
echo -e "${BLUE}📚 Next Steps:${NC}"
echo -e "  1. Test orderbook functions via Candid UI"
echo -e "  2. Test escrow functions via Candid UI"
echo -e "  3. Deploy frontend with fusion components"
echo -e "  4. Set up Ethereum contracts on Sepolia testnet"
echo ""
echo -e "${GREEN}🎯 Ready for Mechanical Turk testing!${NC}"