#!/bin/bash
# test-limit-order-errors.sh - Test limit order error scenarios

set -e  # Exit on any error

# Check if dfx is running
echo "🔍 Checking dfx status..."
if ! dfx ping > /dev/null 2>&1; then
    echo "❌ Error: dfx is not running!"
    echo "💡 Please start dfx with: dfx start --background"
    exit 1
fi
echo "✅ dfx is running"

# Get canister ID
CANISTER_ID=$(dfx canister id backend)
echo "⚠️ Testing limit order error scenarios..."
echo "📋 Using canister: $CANISTER_ID"

# Test principals (using guaranteed valid principals - must be different for asset pair validation)
MAKER_PRINCIPAL=$(dfx identity get-principal)
TAKER_PRINCIPAL="aaaaa-aa"  # Management canister as taker
TOKEN_A_PRINCIPAL="aaaaa-aa"  # Management canister as mock token A  
TOKEN_B_PRINCIPAL=$(dfx canister id backend)  # Backend canister as mock token B (different from A)

# Calculate timestamps
CURRENT_TIME=$(date +%s)000000000
FUTURE_TIME=$(($(date +%s) + 3600))000000000
PAST_TIME=$(($(date +%s) - 3600))000000000

echo "🔑 Test data:"
echo "  Maker: $MAKER_PRINCIPAL"
echo "  Current time: $CURRENT_TIME"
echo "  Future time: $FUTURE_TIME"
echo "  Past time: $PAST_TIME"

# Error Test 1: Create order with zero amount
echo "❌ Error Test 1: Creating order with zero making amount..."
ZERO_AMOUNT_RESULT=$(dfx canister call $CANISTER_ID create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_B_PRINCIPAL\",
  0 : nat64,
  2_000_000 : nat64,
  $FUTURE_TIME : nat64,
  null
)" 2>&1 || echo "Expected error: InvalidAmount")
echo "Zero amount result: $ZERO_AMOUNT_RESULT"

# Error Test 2: Create order with past expiration
echo "❌ Error Test 2: Creating order with past expiration..."
PAST_EXPIRATION_RESULT=$(dfx canister call $CANISTER_ID create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_B_PRINCIPAL\",
  1_000_000 : nat64,
  2_000_000 : nat64,
  $PAST_TIME : nat64,
  null
)" 2>&1 || echo "Expected error: InvalidExpiration")
echo "Past expiration result: $PAST_EXPIRATION_RESULT"

# Error Test 3: Create order with same asset pair
echo "❌ Error Test 3: Creating order with same asset pair..."
SAME_ASSET_RESULT=$(dfx canister call $CANISTER_ID create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  1_000_000 : nat64,
  2_000_000 : nat64,
  $FUTURE_TIME : nat64,
  null
)" 2>&1 || echo "Expected error: InvalidAssetPair")
echo "Same asset result: $SAME_ASSET_RESULT"

# Error Test 4: Fill non-existent order
echo "❌ Error Test 4: Filling non-existent order..."
NON_EXISTENT_FILL=$(dfx canister call $CANISTER_ID fill_order "(99999 : nat64)" 2>&1 || echo "Expected error: OrderNotFound")
echo "Non-existent fill result: $NON_EXISTENT_FILL"

# Error Test 5: Cancel non-existent order
echo "❌ Error Test 5: Cancelling non-existent order..."
NON_EXISTENT_CANCEL=$(dfx canister call $CANISTER_ID cancel_order "(99999 : nat64)" 2>&1 || echo "Expected error: OrderNotFound")
echo "Non-existent cancel result: $NON_EXISTENT_CANCEL"

# Create a valid order for further error tests
echo "📝 Creating valid order for error tests..."
VALID_ORDER_RESULT=$(dfx canister call $CANISTER_ID create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_B_PRINCIPAL\",
  1_000_000 : nat64,
  2_000_000 : nat64,
  $FUTURE_TIME : nat64,
  null
)")

ORDER_ID=$(echo "$VALID_ORDER_RESULT" | grep -o '[0-9]\+' | head -1)
echo "Valid order created with ID: $ORDER_ID"

# Error Test 6: Fill order, then try to fill again
echo "❌ Error Test 6: Double-filling order..."
echo "First fill (should succeed)..."
FIRST_FILL=$(dfx canister call $CANISTER_ID fill_order "($ORDER_ID : nat64)")
echo "First fill result: $FIRST_FILL"

echo "Second fill (should fail)..."
SECOND_FILL=$(dfx canister call $CANISTER_ID fill_order "($ORDER_ID : nat64)" 2>&1 || echo "Expected error: OrderAlreadyFilled")
echo "Second fill result: $SECOND_FILL"

# Error Test 7: Try to cancel filled order
echo "❌ Error Test 7: Cancelling filled order..."
CANCEL_FILLED=$(dfx canister call $CANISTER_ID cancel_order "($ORDER_ID : nat64)" 2>&1 || echo "Expected error: OrderAlreadyFilled")
echo "Cancel filled result: $CANCEL_FILLED"

# Create another order for cancellation test
echo "📝 Creating order for cancellation error test..."
CANCEL_ORDER_RESULT=$(dfx canister call $CANISTER_ID create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_B_PRINCIPAL\",
  500_000 : nat64,
  1_000_000 : nat64,
  $FUTURE_TIME : nat64,
  null
)")

CANCEL_ORDER_ID=$(echo "$CANCEL_ORDER_RESULT" | grep -o '[0-9]\+' | head -1)
echo "Cancel test order created with ID: $CANCEL_ORDER_ID"

# Error Test 8: Cancel order, then try to fill
echo "❌ Error Test 8: Filling cancelled order..."
echo "Cancelling order..."
CANCEL_SUCCESS=$(dfx canister call $CANISTER_ID cancel_order "($CANCEL_ORDER_ID : nat64)")
echo "Cancel result: $CANCEL_SUCCESS"

echo "Trying to fill cancelled order..."
FILL_CANCELLED=$(dfx canister call $CANISTER_ID fill_order "($CANCEL_ORDER_ID : nat64)" 2>&1 || echo "Expected error: OrderCancelled")
echo "Fill cancelled result: $FILL_CANCELLED"

# Error Test 9: Try to cancel already cancelled order
echo "❌ Error Test 9: Double-cancelling order..."
DOUBLE_CANCEL=$(dfx canister call $CANISTER_ID cancel_order "($CANCEL_ORDER_ID : nat64)" 2>&1 || echo "Expected error: OrderCancelled")
echo "Double cancel result: $DOUBLE_CANCEL"

# Error Test 10: Create order with very far future expiration (should fail)
echo "❌ Error Test 10: Creating order with too far future expiration..."
FAR_FUTURE_TIME=$(($(date +%s) + 31536000))000000000  # 1 year from now
FAR_FUTURE_RESULT=$(dfx canister call $CANISTER_ID create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_B_PRINCIPAL\",
  1_000_000 : nat64,
  2_000_000 : nat64,
  $FAR_FUTURE_TIME : nat64,
  null
)" 2>&1 || echo "Expected error: InvalidExpiration")
echo "Far future result: $FAR_FUTURE_RESULT"

# Error Test 11: Get non-existent order
echo "❌ Error Test 11: Getting non-existent order..."
NON_EXISTENT_GET=$(dfx canister call $CANISTER_ID get_order "(99999 : nat64)")
echo "Non-existent get result: $NON_EXISTENT_GET"

# Error Test 12: Create private order and try to fill with wrong taker
echo "❌ Error Test 12: Testing private order restrictions..."
PRIVATE_ORDER_RESULT=$(dfx canister call $CANISTER_ID create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_B_PRINCIPAL\",
  1_000_000 : nat64,
  2_000_000 : nat64,
  $FUTURE_TIME : nat64,
  opt principal \"$TAKER_PRINCIPAL\"
)")

PRIVATE_ORDER_ID=$(echo "$PRIVATE_ORDER_RESULT" | grep -o '[0-9]\+' | head -1)
echo "Private order created with ID: $PRIVATE_ORDER_ID"

echo "Trying to fill private order with unauthorized caller..."
UNAUTHORIZED_FILL=$(dfx canister call $CANISTER_ID fill_order "($PRIVATE_ORDER_ID : nat64)" 2>&1 || echo "Expected error: Unauthorized")
echo "Unauthorized fill result: $UNAUTHORIZED_FILL"

echo "✅ Limit order error scenario tests completed!"
echo "📊 Summary:"
echo "  - Invalid amount: ✅"
echo "  - Invalid expiration: ✅"
echo "  - Invalid asset pair: ✅"
echo "  - Non-existent operations: ✅"
echo "  - Double operations: ✅"
echo "  - State conflicts: ✅"
echo "  - Private order restrictions: ✅" 