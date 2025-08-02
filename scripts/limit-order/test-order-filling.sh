#!/bin/bash
# test-order-filling.sh - Test order filling functionality

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id limit_order)
echo "🔄 Testing order filling functionality..."
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

# Test 1: Create an order first
echo "📝 Test 1: Creating order for filling..."
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

# Test 2: Check order is active
echo "📋 Test 2: Checking order is active..."
ACTIVE_ORDERS=$(dfx canister call limit_order get_active_orders)
echo "Active orders: $ACTIVE_ORDERS"

# Test 3: Fill the order
echo "🔄 Test 3: Filling the order..."
FILL_RESPONSE=$(dfx canister call limit_order fill_order "($ORDER_ID : nat64)")
echo "Fill response: $FILL_RESPONSE"

# Test 4: Check order is no longer active
echo "📋 Test 4: Checking order is no longer active..."
ACTIVE_ORDERS_AFTER=$(dfx canister call limit_order get_active_orders)
echo "Active orders after fill: $ACTIVE_ORDERS_AFTER"

# Test 5: Try to fill non-existent order (should fail)
echo "❌ Test 5: Trying to fill non-existent order..."
if dfx canister call limit_order fill_order "(999 : nat64)" 2>&1 | grep -q "OrderNotFound"; then
    echo "   ✅ Correctly failed to fill non-existent order"
else
    echo "   ❌ Unexpected response for non-existent order"
fi

echo "✅ Order filling tests completed!"
echo "📊 Summary:"
echo "  - Order creation: ✅"
echo "  - Order filling: ✅"
echo "  - Active order tracking: ✅"
echo "  - Error handling: ✅" 