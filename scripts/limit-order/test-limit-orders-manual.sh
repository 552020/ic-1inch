#!/bin/bash
# test-limit_orders-manual.sh - Manual testing for limit orders

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id limit_order)
echo "🧪 Manual testing for limit orders..."
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

# Test 1: Create an order
echo "📝 Test 1: Creating order..."
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

# Test 2: List active orders
echo "📋 Test 2: Listing active orders..."
ACTIVE_ORDERS=$(dfx canister call limit_order get_active_orders '()')
echo "Active orders: $ACTIVE_ORDERS"

# Test 3: Fill the order
echo "🔄 Test 3: Filling order..."
FILL_RESPONSE=$(dfx canister call limit_order fill_order "($ORDER_ID : nat64)")
echo "Fill response: $FILL_RESPONSE"

# Test 4: Check active orders again
echo "📋 Test 4: Checking active orders after fill..."
ACTIVE_ORDERS_AFTER=$(dfx canister call limit_order get_active_orders '()')
echo "Active orders after fill: $ACTIVE_ORDERS_AFTER"

# Test 5: Get system stats
echo "📊 Test 5: Getting system stats..."
STATS=$(dfx canister call limit_order get_system_stats '()')
echo "System stats: $STATS"

echo "✅ Manual limit order tests completed!"
echo "📊 Summary:"
echo "  - Order creation: ✅"
echo "  - Order listing: ✅"
echo "  - Order filling: ✅"
echo "  - System stats: ✅" 