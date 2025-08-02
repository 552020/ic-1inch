#!/bin/bash
# test-order-creation.sh - Test order creation functionality

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id limit_order)
echo "📝 Testing order creation functionality..."
echo "📋 Using canister: $CANISTER_ID"

# Get current identity principal for testing
CURRENT_PRINCIPAL=$(dfx identity get-principal)
echo "🔑 Current principal: $CURRENT_PRINCIPAL"

# Get test token canister IDs
TOKEN_A_ID=$(dfx canister id test_token_icp)
TOKEN_B_ID=$(dfx canister id test_token_eth)
echo "🔑 Test tokens:"
echo "  Token A (ICP): $TOKEN_A_ID"
echo "  Token B (ETH): $TOKEN_B_ID"

# Test 1: Create a basic order
echo "📝 Test 1: Creating basic order..."
# Calculate expiration time (1 hour from now)
EXPIRATION=$(($(date +%s) + 3600))000000000
ORDER_RESPONSE=$(dfx canister call limit_order create_order "(
  principal \"$CURRENT_PRINCIPAL\",
  principal \"$TOKEN_A_ID\",
  principal \"$TOKEN_B_ID\",
  1_000_000 : nat64,
  2_000_000 : nat64,
  $EXPIRATION : nat64
)")

# Extract order ID from response
ORDER_ID=$(echo "$ORDER_RESPONSE" | grep -o '[0-9]\+' | head -1)
echo "Order created: $ORDER_ID"

# Test 2: Get order by ID
echo "📋 Test 2: Getting order by ID..."
ORDER_DETAILS=$(dfx canister call limit_order get_order_by_id "($ORDER_ID : nat64)")
echo "Order details: $ORDER_DETAILS"

# Test 3: Get orders by maker
echo "📋 Test 3: Getting orders by maker..."
MAKER_ORDERS=$(dfx canister call limit_order get_orders_by_maker "(principal \"$CURRENT_PRINCIPAL\")")
echo "Maker orders: $MAKER_ORDERS"

# Test 4: Get orders by asset pair
echo "📋 Test 4: Getting orders by asset pair..."
PAIR_ORDERS=$(dfx canister call limit_order get_orders_by_asset_pair "(
  principal \"$TOKEN_A_ID\",
  principal \"$TOKEN_B_ID\"
)")
echo "Asset pair orders: $PAIR_ORDERS"

echo "✅ Order creation tests completed!"
echo "📊 Summary:"
echo "  - Order creation: ✅"
echo "  - Get order by ID: ✅"
echo "  - Get orders by maker: ✅"
echo "  - Get orders by asset pair: ✅" 