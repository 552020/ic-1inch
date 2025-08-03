#!/bin/bash

# Maker Script - Submit Order to Relayer
# This script submits a cross-chain order using the fusion_plus_relayer_submit endpoint
#
# USAGE: Run this from the project root directory (ic-1inch/)
# Example: ./scripts/relayer/maker-submit-order.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}📋 Maker - Submit Order to Relayer${NC}"
echo "============================================"

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

# Sample order data (following 1inch API format)
MAKER_ADDRESS="0x1234567890123456789012345678901234567890"
RECEIVER_ADDRESS="0x1234567890123456789012345678901234567890"
MAKER_ASSET="0x0000000000000000000000000000000000000001"  # ETH
TAKER_ASSET="0x0000000000000000000000000000000000000002"  # USDC
MAKING_AMOUNT="1000000000000000000"  # 1 ETH in wei
TAKING_AMOUNT="2000000000000000000"  # 2 tokens
SALT="42"
SIGNATURE="0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12"
EXTENSION="0x"
QUOTE_ID="test-quote-123"
SECRET_HASH="a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"

echo -e "${CYAN}📝 Order Details:${NC}"
echo "  Maker: $MAKER_ADDRESS"
echo "  Receiver: $RECEIVER_ADDRESS"
echo "  Selling: $MAKING_AMOUNT wei of $MAKER_ASSET"
echo "  Buying: $TAKING_AMOUNT wei of $TAKER_ASSET"
echo "  Salt: $SALT"
echo "  Quote ID: $QUOTE_ID"
echo ""

echo -e "${YELLOW}🚀 Submitting order to relayer...${NC}"

# Submit the order using dfx
RESULT=$(dfx canister call relayer fusion_plus_relayer_submit \
"(
  record {
    salt = \"$SALT\";
    maker = \"$MAKER_ADDRESS\";
    receiver = \"$RECEIVER_ADDRESS\";
    makerAsset = \"$MAKER_ASSET\";
    takerAsset = \"$TAKER_ASSET\";
    makingAmount = \"$MAKING_AMOUNT\";
    takingAmount = \"$TAKING_AMOUNT\";
    makerTraits = \"0x\";
  },
  1 : nat64,
  \"$SIGNATURE\",
  \"$EXTENSION\",
  \"$QUOTE_ID\",
  vec { \"$SECRET_HASH\" }
)" 2>&1)

# Check if the call was successful
if echo "$RESULT" | grep -q "Ok"; then
    ORDER_ID=$(echo "$RESULT" | grep -o '"0x[a-fA-F0-9]*"' | tr -d '"')
    echo -e "${GREEN}✅ Order submitted successfully!${NC}"
    echo -e "${GREEN}🆔 Order ID: $ORDER_ID${NC}"
    echo ""
    echo -e "${CYAN}📊 Next steps:${NC}"
    echo "  1. Use the taker script to fetch active orders"
    echo "  2. Check order status with: dfx canister call relayer fusion_plus_order_status '(\"$ORDER_ID\")'"
    echo "  3. Monitor order until taker accepts it"
else
    echo -e "${RED}❌ Order submission failed:${NC}"
    echo "$RESULT"
    exit 1
fi

echo ""
echo -e "${BLUE}🎯 Order successfully booked in the relayer!${NC}"