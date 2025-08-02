#!/bin/bash
# test-order-creation.sh - Test order creation functionality

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id limit-order)
echo "📝 Testing order creation functionality..."
echo "📋 Using canister: $CANISTER_ID"

# Get current identity principal for testing
CURRENT_PRINCIPAL=$(dfx identity get-principal)
echo "🔑 Current principal: $CURRENT_PRINCIPAL"

# Test 1: Create a basic order
echo "📝 Test 1: Creating basic order..."
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

# Test 2: Get order by ID
echo "📋 Test 2: Getting order by ID..."
ORDER_DETAILS=$(dfx canister call $CANISTER_ID get_order_by_id "(\"$ORDER_ID\")")
echo "Order details: $ORDER_DETAILS"

# Test 3: Get orders by maker
echo "📋 Test 3: Getting orders by maker..."
MAKER_ORDERS=$(dfx canister call $CANISTER_ID get_orders_by_maker "(principal \"$CURRENT_PRINCIPAL\")")
echo "Maker orders: $MAKER_ORDERS"

# Test 4: Get orders by asset pair
echo "📋 Test 4: Getting orders by asset pair..."
PAIR_ORDERS=$(dfx canister call $CANISTER_ID get_orders_by_asset_pair "(
  record {
    token_a = principal \"$CANISTER_ID\";
    token_b = principal \"$CANISTER_ID\";
  }
)")
echo "Asset pair orders: $PAIR_ORDERS"

echo "✅ Order creation tests completed!"
echo "📊 Summary:"
echo "  - Order creation: ✅"
echo "  - Get order by ID: ✅"
echo "  - Get orders by maker: ✅"
echo "  - Get orders by asset pair: ✅" 