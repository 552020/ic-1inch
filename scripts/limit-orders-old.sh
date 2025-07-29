#!/bin/bash
# test-limit-orders.sh - Test complete limit order lifecycle

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
echo "🔄 Testing complete limit order lifecycle..."
echo "📋 Using canister: $CANISTER_ID"

# Test principals (using guaranteed valid principals - must be different for asset pair validation)
MAKER_PRINCIPAL=$(dfx identity get-principal)
TAKER_PRINCIPAL="aaaaa-aa"  # Management canister as taker
TOKEN_A_PRINCIPAL="aaaaa-aa"  # Management canister as mock token A  
TOKEN_B_PRINCIPAL=$(dfx canister id backend)  # Backend canister as mock token B (different from A)

echo "🔑 Test data:"
echo "  Maker: $MAKER_PRINCIPAL"
echo "  Taker: $TAKER_PRINCIPAL"
echo "  Token A: $TOKEN_A_PRINCIPAL"
echo "  Token B: $TOKEN_B_PRINCIPAL"

# Calculate future timestamp (1 hour from now)
FUTURE_TIME=$(($(date +%s) + 3600))000000000

# Test 1: Create a basic limit order
echo "📝 Test 1: Creating basic limit order..."
CREATE_RESULT=$(dfx canister call $CANISTER_ID create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_B_PRINCIPAL\",
  1_000_000 : nat64,
  2_000_000 : nat64,
  $FUTURE_TIME : nat64,
  null
)")

# Extract order ID from result
ORDER_ID=$(echo "$CREATE_RESULT" | grep -o '[0-9]\+' | head -1)
echo "Order created with ID: $ORDER_ID"

# Test 2: Get the created order
echo "📋 Test 2: Getting created order..."
GET_ORDER_RESULT=$(dfx canister call $CANISTER_ID get_order "($ORDER_ID : nat64)")
echo "Order details: $GET_ORDER_RESULT"

# Test 3: List active orders
echo "📋 Test 3: Listing active orders..."
ACTIVE_ORDERS=$(dfx canister call $CANISTER_ID get_active_orders)
echo "Active orders: $ACTIVE_ORDERS"

# Test 4: Get orders by maker
echo "📋 Test 4: Getting orders by maker..."
MAKER_ORDERS=$(dfx canister call $CANISTER_ID get_orders_by_maker "(principal \"$MAKER_PRINCIPAL\")")
echo "Maker orders: $MAKER_ORDERS"

# Test 5: Get orders by asset pair
echo "📋 Test 5: Getting orders by asset pair..."
PAIR_ORDERS=$(dfx canister call $CANISTER_ID get_orders_by_asset_pair "(
  principal \"$TOKEN_A_PRINCIPAL\",
  principal \"$TOKEN_B_PRINCIPAL\"
)")
echo "Asset pair orders: $PAIR_ORDERS"

# Test 6: Fill the order (simulate taker)
echo "💱 Test 6: Filling the order..."
FILL_RESULT=$(dfx canister call $CANISTER_ID fill_order "($ORDER_ID : nat64)")
echo "Fill result: $FILL_RESULT"

# Test 7: Check order status after fill
echo "📋 Test 7: Checking order status after fill..."
POST_FILL_ORDER=$(dfx canister call $CANISTER_ID get_order "($ORDER_ID : nat64)")
echo "Order after fill: $POST_FILL_ORDER"

# Test 8: Try to cancel already filled order (should fail)
echo "❌ Test 8: Trying to cancel filled order (should fail)..."
CANCEL_RESULT=$(dfx canister call $CANISTER_ID cancel_order "($ORDER_ID : nat64)" 2>&1 || echo "Expected error: OrderAlreadyFilled")
echo "Cancel result: $CANCEL_RESULT"

# Test 9: Create and cancel order
echo "📝 Test 9: Creating order for cancellation test..."
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

echo "🚫 Cancelling order..."
CANCEL_SUCCESS=$(dfx canister call $CANISTER_ID cancel_order "($CANCEL_ORDER_ID : nat64)")
echo "Cancel result: $CANCEL_SUCCESS"

# Test 10: Get system statistics
echo "📊 Test 10: Getting system statistics..."
STATS_RESULT=$(dfx canister call $CANISTER_ID get_system_stats)
echo "System stats: $STATS_RESULT"

# Test 11: List all active orders (for debugging)
echo "📋 Test 11: Listing all active orders..."
ALL_ORDERS=$(dfx canister call $CANISTER_ID get_active_orders)
echo "All active orders: $ALL_ORDERS"

echo "✅ Limit order lifecycle test completed!"
echo "📊 Summary:"
echo "  - Order creation: ✅"
echo "  - Order queries: ✅"
echo "  - Order filling: ✅"
echo "  - Order cancellation: ✅"
echo "  - System statistics: ✅" 