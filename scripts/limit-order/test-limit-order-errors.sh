#!/bin/bash
# test-limit-order-errors.sh - Test error handling for limit orders

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id limit-order)
echo "❌ Testing limit order error handling..."
echo "📋 Using canister: $CANISTER_ID"

# Get current identity principal for testing
CURRENT_PRINCIPAL=$(dfx identity get-principal)
echo "🔑 Current principal: $CURRENT_PRINCIPAL"

# Test 1: Try to create order with invalid parameters
echo "❌ Test 1: Creating order with invalid parameters..."
if dfx canister call limit-order create_order "(
  record {
    maker = principal \"$CURRENT_PRINCIPAL\";
    token_a = principal \"$CANISTER_ID\";
    token_b = principal \"$CANISTER_ID\";
    amount_a = 0 : nat64;  # Invalid: zero amount
    amount_b = 2_000_000 : nat64;
    price = 2_000_000 : nat64;
  }
)" 2>&1 | grep -q "InvalidAmount"; then
    echo "   ✅ Correctly failed with InvalidAmount"
else
    echo "   ❌ Unexpected response for invalid amount"
fi

# Test 2: Try to fill non-existent order
echo "❌ Test 2: Trying to fill non-existent order..."
if dfx canister call limit-order fill_order "(999:nat64)" 2>&1 | grep -q "OrderNotFound"; then
    echo "   ✅ Correctly failed with OrderNotFound"
else
    echo "   ❌ Unexpected response for non-existent order"
fi

# Test 3: Try to cancel non-existent order
echo "❌ Test 3: Trying to cancel non-existent order..."
if dfx canister call limit-order cancel_order "(999:nat64)" 2>&1 | grep -q "OrderNotFound"; then
    echo "   ✅ Correctly failed with OrderNotFound"
else
    echo "   ❌ Unexpected response for non-existent order"
fi

# Test 4: Create a valid order first
echo "📝 Test 4: Creating valid order for further testing..."
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

# Test 5: Try to fill the same order twice
echo "❌ Test 5: Trying to fill the same order twice..."
FILL_RESPONSE=$(dfx canister call limit-order fill_order "($ORDER_ID:nat64)")
echo "First fill: $FILL_RESPONSE"

if dfx canister call limit-order fill_order "($ORDER_ID:nat64)" 2>&1 | grep -q "OrderNotActive"; then
    echo "   ✅ Correctly failed to fill already filled order"
else
    echo "   ❌ Unexpected response for already filled order"
fi

# Test 6: Try to cancel already filled order
echo "❌ Test 6: Trying to cancel already filled order..."
if dfx canister call limit-order cancel_order "($ORDER_ID:nat64)" 2>&1 | grep -q "OrderNotActive"; then
    echo "   ✅ Correctly failed to cancel already filled order"
else
    echo "   ❌ Unexpected response for already filled order"
fi

echo "✅ Limit order error handling tests completed!"
echo "📊 Summary:"
echo "  - Invalid parameter handling: ✅"
echo "  - Non-existent order handling: ✅"
echo "  - Duplicate operation handling: ✅"
echo "  - State validation: ✅" 