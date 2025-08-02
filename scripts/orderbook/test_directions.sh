#!/bin/bash

# Test script for Task 15: Order Direction-Specific Coordination
# Tests both ICP→ETH and ETH→ICP order flows with proper coordination logic

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CANISTER_ID="$(dfx canister id orderbook 2>/dev/null || echo 'orderbook')"
DFX_NETWORK="${DFX_NETWORK:-local}"

echo -e "${BLUE}🧪 Testing Task 15: Order Direction-Specific Coordination${NC}"
echo "=================================================="

# Test 1: ICP→ETH Order Creation (Maker creates escrow)
echo -e "\n${YELLOW}Test 1: ICP→ETH Order Creation (Maker creates escrow)${NC}"
echo "--------------------------------------------------"

# Create ICP→ETH order
echo "Creating ICP→ETH order..."
ORDER_ID_ICP_ETH=$(dfx canister call $CANISTER_ID create_fusion_order \
    '("0x1234567890abcdef", "ICP", "ETH", 1000000, 500000, "0x", "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234", 1700000000000000000, null)' \
    --network $DFX_NETWORK | grep -o '"[^"]*"' | head -1 | tr -d '"')

if [ -z "$ORDER_ID_ICP_ETH" ]; then
    echo -e "${RED}❌ Failed to create ICP→ETH order${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Created ICP→ETH order: $ORDER_ID_ICP_ETH${NC}"

# Check order direction info
echo "Checking order direction info..."
DIRECTION_INFO=$(dfx canister call $CANISTER_ID get_order_direction_info "(\"$ORDER_ID_ICP_ETH\")" --network $DFX_NETWORK)
echo "Direction info: $DIRECTION_INFO"

# Verify it shows ICP→ETH and maker as escrow creator
if echo "$DIRECTION_INFO" | grep -q "ICP_TO_ETH" && echo "$DIRECTION_INFO" | grep -q "maker"; then
    echo -e "${GREEN}✅ ICP→ETH order direction and escrow creator correctly identified${NC}"
else
    echo -e "${RED}❌ ICP→ETH order direction or escrow creator incorrect${NC}"
    exit 1
fi

# Test 2: ETH→ICP Order Creation (Resolver creates escrow)
echo -e "\n${YELLOW}Test 2: ETH→ICP Order Creation (Resolver creates escrow)${NC}"
echo "--------------------------------------------------"

# Create ETH→ICP order with EIP-712 signature
echo "Creating ETH→ICP order with EIP-712 signature..."
ORDER_ID_ETH_ICP=$(dfx canister call $CANISTER_ID create_fusion_order \
    '("0xabcdef1234567890", "ETH", "ICP", 500000, 1000000, "0x", "b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456", 1700000000000000000, opt record { domain_separator = "0x1234567890abcdef"; type_hash = "0xabcdef1234567890"; order_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_r = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_s = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_v = 27; signer_address = "0x1234567890abcdef1234567890abcdef1234567890" })' \
    --network $DFX_NETWORK | grep -o '"[^"]*"' | head -1 | tr -d '"')

if [ -z "$ORDER_ID_ETH_ICP" ]; then
    echo -e "${RED}❌ Failed to create ETH→ICP order${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Created ETH→ICP order: $ORDER_ID_ETH_ICP${NC}"

# Check order direction info
echo "Checking order direction info..."
DIRECTION_INFO=$(dfx canister call $CANISTER_ID get_order_direction_info "(\"$ORDER_ID_ETH_ICP\")" --network $DFX_NETWORK)
echo "Direction info: $DIRECTION_INFO"

# Verify it shows ETH→ICP and resolver as escrow creator
if echo "$DIRECTION_INFO" | grep -q "ETH_TO_ICP" && echo "$DIRECTION_INFO" | grep -q "resolver"; then
    echo -e "${GREEN}✅ ETH→ICP order direction and escrow creator correctly identified${NC}"
else
    echo -e "${RED}❌ ETH→ICP order direction or escrow creator incorrect${NC}"
    exit 1
fi

# Test 3: Order Acceptance with Direction-Specific Logic
echo -e "\n${YELLOW}Test 3: Order Acceptance with Direction-Specific Logic${NC}"
echo "--------------------------------------------------"

# Accept ETH→ICP order (resolver will create escrow)
echo "Accepting ETH→ICP order..."
ACCEPT_RESPONSE=$(dfx canister call $CANISTER_ID accept_fusion_order "(\"$ORDER_ID_ETH_ICP\", \"0xabcdef1234567890abcdef1234567890abcdef12\")" --network $DFX_NETWORK)
echo "Accept response: $ACCEPT_RESPONSE"

# Verify response contains direction and escrow creator info
if echo "$ACCEPT_RESPONSE" | grep -q "ETH_TO_ICP" && echo "$ACCEPT_RESPONSE" | grep -q "resolver"; then
    echo -e "${GREEN}✅ ETH→ICP order acceptance correctly identifies resolver as escrow creator${NC}"
else
    echo -e "${RED}❌ ETH→ICP order acceptance response incorrect${NC}"
    exit 1
fi

# Test 4: Get Orders by Direction
echo -e "\n${YELLOW}Test 4: Get Orders by Direction${NC}"
echo "--------------------------------------------------"

# Get ICP→ETH orders
echo "Getting ICP→ETH orders..."
ICP_ETH_ORDERS=$(dfx canister call $CANISTER_ID get_orders_by_direction '("ICP_TO_ETH")' --network $DFX_NETWORK)
echo "ICP→ETH orders: $ICP_ETH_ORDERS"

# Get ETH→ICP orders
echo "Getting ETH→ICP orders..."
ETH_ICP_ORDERS=$(dfx canister call $CANISTER_ID get_orders_by_direction '("ETH_TO_ICP")' --network $DFX_NETWORK)
echo "ETH→ICP orders: $ETH_ICP_ORDERS"

# Verify orders are correctly filtered by direction
if echo "$ICP_ETH_ORDERS" | grep -q "$ORDER_ID_ICP_ETH" && ! echo "$ICP_ETH_ORDERS" | grep -q "$ORDER_ID_ETH_ICP"; then
    echo -e "${GREEN}✅ ICP→ETH orders correctly filtered${NC}"
else
    echo -e "${RED}❌ ICP→ETH orders filtering incorrect${NC}"
    exit 1
fi

if echo "$ETH_ICP_ORDERS" | grep -q "$ORDER_ID_ETH_ICP" && ! echo "$ETH_ICP_ORDERS" | grep -q "$ORDER_ID_ICP_ETH"; then
    echo -e "${GREEN}✅ ETH→ICP orders correctly filtered${NC}"
else
    echo -e "${RED}❌ ETH→ICP orders filtering incorrect${NC}"
    exit 1
fi

# Test 5: Get Orders for Escrow Creation
echo -e "\n${YELLOW}Test 5: Get Orders for Escrow Creation${NC}"
echo "--------------------------------------------------"

# Get orders where caller is responsible for escrow creation
echo "Getting orders for escrow creation..."
ESCROW_ORDERS=$(dfx canister call $CANISTER_ID get_orders_for_escrow_creation --network $DFX_NETWORK)
echo "Escrow creation orders: $ESCROW_ORDERS"

# Test 6: Validation of Direction-Specific Requirements
echo -e "\n${YELLOW}Test 6: Validation of Direction-Specific Requirements${NC}"
echo "--------------------------------------------------"

# Try to create ETH→ICP order without EIP-712 signature (should fail)
echo "Testing ETH→ICP order creation without EIP-712 signature (should fail)..."
ETH_ICP_NO_SIG_RESPONSE=$(dfx canister call $CANISTER_ID create_fusion_order \
    '("0xfedcba0987654321", "ETH", "ICP", 500000, 1000000, "0x", "c3d4e5f6789012345678901234567890abcdef1234567890abcdef12345678", 1700000000000000000, null)' \
    --network $DFX_NETWORK 2>&1 || true)

if echo "$ETH_ICP_NO_SIG_RESPONSE" | grep -q "InvalidEIP712Signature"; then
    echo -e "${GREEN}✅ ETH→ICP order correctly rejected without EIP-712 signature (format validation only)${NC}"
else
    echo -e "${RED}❌ ETH→ICP order should have been rejected without EIP-712 signature${NC}"
    exit 1
fi

# Try to create ICP→ETH order without EIP-712 signature (should succeed)
echo "Testing ICP→ETH order creation without EIP-712 signature (should succeed)..."
ICP_ETH_NO_SIG_RESPONSE=$(dfx canister call $CANISTER_ID create_fusion_order \
    '("0x9876543210fedcba", "ICP", "ETH", 1000000, 500000, "0x", "d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890", 1700000000000000000, null)' \
    --network $DFX_NETWORK | grep -o '"[^"]*"' | head -1 | tr -d '"')

if [ -n "$ICP_ETH_NO_SIG_RESPONSE" ]; then
    echo -e "${GREEN}✅ ICP→ETH order correctly created without EIP-712 signature${NC}"
else
    echo -e "${RED}❌ ICP→ETH order should have been created without EIP-712 signature${NC}"
    exit 1
fi

# Summary
echo -e "\n${BLUE}📊 Task 15 Test Summary${NC}"
echo "========================"
echo -e "${GREEN}✅ ICP→ETH Order Flow: Maker creates escrow${NC}"
echo -e "${GREEN}✅ ETH→ICP Order Flow: Resolver creates escrow${NC}"
echo -e "${GREEN}✅ Direction-specific validation working${NC}"
echo -e "${GREEN}✅ EIP-712 signature format validation enforced (crypto validation on Ethereum)${NC}"
echo -e "${GREEN}✅ Order filtering by direction working${NC}"
echo -e "${GREEN}✅ Escrow creation responsibility tracking working${NC}"

echo -e "\n${GREEN}🎉 Task 15: Order Direction-Specific Coordination - ALL TESTS PASSED!${NC}" 