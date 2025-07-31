#!/bin/bash

# Stop and clean up fusion+ mechanical turk deployment
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🛑 Stopping Fusion+ Mechanical Turk Deployment${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo -e "${RED}❌ dfx.json not found. Please run this script from the project root.${NC}"
    exit 1
fi

# Stop specific canisters
echo -e "${YELLOW}🔄 Stopping mechanical turk canisters...${NC}"

if dfx canister status orderbook >/dev/null 2>&1; then
    echo -e "  • Stopping orderbook canister..."
    dfx canister stop orderbook
fi

if dfx canister status escrow >/dev/null 2>&1; then
    echo -e "  • Stopping escrow canister..."
    dfx canister stop escrow
fi

echo ""
echo -e "${BLUE}🗑️  Optional: Clean up (removes all canister data)${NC}"
echo -e "${YELLOW}Run the following commands if you want to completely reset:${NC}"
echo -e "  dfx canister delete orderbook"
echo -e "  dfx canister delete escrow"
echo -e "  dfx stop"
echo ""
echo -e "${GREEN}✅ Mechanical Turk canisters stopped!${NC}"