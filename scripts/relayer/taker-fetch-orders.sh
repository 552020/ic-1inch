#!/bin/bash

# Taker Script - Fetch Active Orders from Relayer
# This script fetches all active orders using the fusion_plus_orders_active endpoint
#
# USAGE: Run this from the project root directory (ic-1inch/)
# Example: ./scripts/relayer/taker-fetch-orders.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Taker - Fetch Active Orders from Relayer${NC}"
echo "==============================================="

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo -e "${RED}❌ Error: Please run this script from the project root directory (ic-1inch/)${NC}"
    exit 1
fi

# Check if DFX is running
if ! dfx ping > /dev/null 2>&1; then
    echo -e "${RED}❌ Error: DFX is not running. Please start dfx with 'dfx start --clean' first.${NC}"
    exit 1
fi

# Get relayer canister ID
RELAYER_CANISTER_ID=$(dfx canister id relayer 2>/dev/null || echo "")
if [ -z "$RELAYER_CANISTER_ID" ]; then
    echo -e "${RED}❌ Error: Relayer canister not found. Please run setup_and_compile.sh first.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Relayer canister ID: ${RELAYER_CANISTER_ID}${NC}"
echo ""

echo -e "${YELLOW}🔍 Fetching active orders...${NC}"

# Fetch active orders
RESULT=$(dfx canister call relayer fusion_plus_orders_active '()' 2>&1)

# Check if the call was successful
if echo "$RESULT" | grep -q "vec {"; then
    echo -e "${GREEN}✅ Successfully fetched active orders!${NC}"
    echo ""
    
    # Count the number of orders
    ORDER_COUNT=$(echo "$RESULT" | grep -o "record {" | wc -l | xargs)
    
    if [ "$ORDER_COUNT" -eq 0 ]; then
        echo -e "${YELLOW}📭 No active orders found${NC}"
        echo ""
        echo -e "${CYAN}💡 Tip: Run the maker script to submit an order first:${NC}"
        echo "  ./scripts/relayer/maker-submit-order.sh"
    else
        echo -e "${GREEN}📋 Found $ORDER_COUNT active order(s):${NC}"
        echo ""
        echo -e "${CYAN}📊 Order Details:${NC}"
        echo "$RESULT" | sed 's/;/;\n  /g' | sed 's/record {/\n🔸 Order:\n  /g' | sed 's/}/\n/g'
        echo ""
        
        # Extract order IDs for easy reference
        echo -e "${CYAN}🆔 Order IDs:${NC}"
        echo "$RESULT" | grep -o 'id = "[^"]*"' | sed 's/id = "//g' | sed 's/"//g' | while read -r order_id; do
            echo "  📌 $order_id"
        done
        echo ""
        
        echo -e "${CYAN}🎯 Next steps for takers:${NC}"
        echo "  1. Choose an order ID from the list above"
        echo "  2. Check order details with: dfx canister call relayer fusion_plus_order_status '(\"ORDER_ID\")'"
        echo "  3. Check order escrow with: dfx canister call relayer fusion_plus_order_escrow '(\"ORDER_ID\", 1)'"
        echo "  4. Get order secrets with: dfx canister call relayer fusion_plus_order_secrets '(\"ORDER_ID\")'"
        echo "  5. Check if ready for fills: dfx canister call relayer fusion_plus_order_ready_to_accept_secret_fills '(\"ORDER_ID\")'"
    fi
else
    echo -e "${RED}❌ Failed to fetch active orders:${NC}"
    echo "$RESULT"
    exit 1
fi

echo ""
echo -e "${BLUE}🎯 Active orders successfully retrieved!${NC}"

# Optional: Show additional commands
echo ""
echo -e "${CYAN}🛠️  Additional Commands:${NC}"
echo "  🔍 Get specific order status:"
echo "    dfx canister call relayer fusion_plus_order_status '(\"ORDER_ID\")'"
echo ""
echo "  🔐 Get order secrets:"
echo "    dfx canister call relayer fusion_plus_order_secrets '(\"ORDER_ID\")'"
echo ""
echo "  ⚡ Check if order is ready for secret fills:"
echo "    dfx canister call relayer fusion_plus_order_ready_to_accept_secret_fills '(\"ORDER_ID\")'"