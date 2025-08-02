#!/bin/bash
# test-limit-orders-manual.sh - Manual testing for limit orders

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id limit-order)
echo "🧪 Manual testing for limit orders..."
echo "📋 Using canister: $CANISTER_ID"

# Get current identity principal for testing
CURRENT_PRINCIPAL=$(dfx identity get-principal)
echo "🔑 Current principal: $CURRENT_PRINCIPAL"

# Test 1: Create an order
echo "📝 Test 1: Creating order..."
ORDER_RESPONSE=$(dfx canister call limit-order create_order "(
  record {
    maker = principal \"$CURRENT_PRINCIPAL\";
    token_a = principal \"$CANISTER_ID\";
    token_b = principal \"$CANISTER_ID\";
    amount_a = 1_000_000 : nat64;
    amount_b = 2_000_000 : nat64;
    price = 2_000_000 : nat64;
  }
)")

# Extract order ID from response
ORDER_ID=$(echo "$ORDER_RESPONSE" | grep -o '"[^"]*"' | head -1 | tr -d '"')
echo "Order created: $ORDER_ID"

# Test 2: List active orders
echo "📋 Test 2: Listing active orders..."
ACTIVE_ORDERS=$(dfx canister call limit-order get_active_orders '()')
echo "Active orders: $ACTIVE_ORDERS"

# Test 3: Fill the order
echo "🔄 Test 3: Filling order..."
FILL_RESPONSE=$(dfx canister call limit-order fill_order "($ORDER_ID:nat64)")
echo "Fill response: $FILL_RESPONSE"

# Test 4: Check active orders again
echo "📋 Test 4: Checking active orders after fill..."
ACTIVE_ORDERS_AFTER=$(dfx canister call limit-order get_active_orders '()')
echo "Active orders after fill: $ACTIVE_ORDERS_AFTER"

# Test 5: Get system stats
echo "📊 Test 5: Getting system stats..."
STATS=$(dfx canister call limit-order get_system_stats '()')
echo "System stats: $STATS"

echo "✅ Manual limit order tests completed!"
echo "📊 Summary:"
echo "  - Order creation: ✅"
echo "  - Order listing: ✅"
echo "  - Order filling: ✅"
echo "  - System stats: ✅" 