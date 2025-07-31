#!/bin/bash

# ICP Limit Order Protocol - Manual Test Script
# Run this after deployment to test the system

set -e

# Check if environment variables are set
if [ -z "$MAKER_PRINCIPAL" ] || [ -z "$TAKER_PRINCIPAL" ]; then
    echo "❌ Environment variables not set. Run limit-order-manual-test-setup.sh first"
    echo "   Then run this test script in the same shell session"
    exit 1
fi

echo "✅ Using environment variables:"
echo "   MAKER_PRINCIPAL: $MAKER_PRINCIPAL"
echo "   TAKER_PRINCIPAL: $TAKER_PRINCIPAL"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🧪 Starting ICP Limit Order Protocol Manual Tests${NC}"
echo "=================================================="

# Test 1: Create order as maker
echo -e "\n${YELLOW}Test 1: Creating order as maker${NC}"
switch-to-maker
echo "Using maker identity: $(dfx identity whoami)"
echo "Maker principal: $MAKER_PRINCIPAL"

# Create order
echo "Creating test order..."
dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_MAKER_ASSET\",
  principal \"$TEST_TAKER_ASSET\",
  $TEST_MAKING_AMOUNT:nat64,
  $TEST_TAKING_AMOUNT:nat64,
  $(date -d "+$TEST_EXPIRATION_HOURS hour" +%s)000000000:nat64
)"

echo -e "${GREEN}✅ Order created successfully${NC}"

# Test 2: List orders
echo -e "\n${YELLOW}Test 2: Listing active orders${NC}"
dfx canister call backend get_active_orders_list '()'

# Test 3: Fill order as taker
echo -e "\n${YELLOW}Test 3: Filling order as taker${NC}"
switch-to-taker
echo "Using taker identity: $(dfx identity whoami)"
echo "Taker principal: $TAKER_PRINCIPAL"

echo "Filling order ID 1..."
dfx canister call backend fill_order '(1:nat64)'
echo -e "${GREEN}✅ Order filled successfully${NC}"

# Test 4: Verify order is no longer active
echo -e "\n${YELLOW}Test 4: Verifying order is filled${NC}"
dfx canister call backend get_active_orders_list '()'

# Test 5: Check system statistics
echo -e "\n${YELLOW}Test 5: System statistics${NC}"
dfx canister call backend get_system_statistics '()'

echo -e "\n${GREEN}🎉 All tests completed successfully!${NC}"
echo "==================================================" 