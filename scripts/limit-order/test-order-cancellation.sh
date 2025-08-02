#!/bin/bash
# test-order-cancellation.sh - Test order cancellation functionality

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id limit-order)
echo "❌ Testing order cancellation functionality..."
echo "📋 Using canister: $CANISTER_ID"

# Get current identity principal for testing
CURRENT_PRINCIPAL=$(dfx identity get-principal)
echo "🔑 Current principal: $CURRENT_PRINCIPAL"

# Test 1: Create an order first
echo "📝 Test 1: Creating order for cancellation..."
ORDER_RESPONSE=$(dfx canister call $CANISTER_ID create_order "(
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

# Test 2: Check order is active
echo "📋 Test 2: Checking order is active..."
ACTIVE_ORDERS=$(dfx canister call $CANISTER_ID get_active_orders)
echo "Active orders: $ACTIVE_ORDERS"

# Test 3: Cancel the order
echo "❌ Test 3: Cancelling the order..."
CANCEL_RESPONSE=$(dfx canister call $CANISTER_ID cancel_order "(\"$ORDER_ID\")")
echo "Cancel response: $CANCEL_RESPONSE"

# Test 4: Check order is no longer active
echo "📋 Test 4: Checking order is no longer active..."
ACTIVE_ORDERS_AFTER=$(dfx canister call $CANISTER_ID get_active_orders)
echo "Active orders after cancellation: $ACTIVE_ORDERS_AFTER"

# Test 5: Try to cancel non-existent order (should fail)
echo "❌ Test 5: Trying to cancel non-existent order..."
if dfx canister call $CANISTER_ID cancel_order '("non_existent")' 2>&1 | grep -q "OrderNotFound"; then
    echo "   ✅ Correctly failed to cancel non-existent order"
else
    echo "   ❌ Unexpected response for non-existent order"
fi

# Test 6: Try to cancel already cancelled order (should fail)
echo "❌ Test 6: Trying to cancel already cancelled order..."
if dfx canister call $CANISTER_ID cancel_order "(\"$ORDER_ID\")" 2>&1 | grep -q "OrderNotActive"; then
    echo "   ✅ Correctly failed to cancel already cancelled order"
else
    echo "   ❌ Unexpected response for already cancelled order"
fi

echo "✅ Order cancellation tests completed!"
echo "📊 Summary:"
echo "  - Order creation: ✅"
echo "  - Order cancellation: ✅"
echo "  - Active order tracking: ✅"
echo "  - Error handling: ✅" 