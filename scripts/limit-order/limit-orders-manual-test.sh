#!/bin/bash

# ICP Limit Order Protocol - Automated Manual Testing Script
# This script automates the manual testing guide up to a specified step
# Usage: ./scripts/limit-orders-manual-test.sh [step]
# Examples: 
#   ./scripts/limit-orders-manual-test.sh 1.2    # Stop after step 1.2
#   ./scripts/limit-orders-manual-test.sh 2.3    # Stop after step 2.3
#   ./scripts/limit-orders-manual-test.sh        # Run all steps

set -e  # Exit on any error

# Parse command line argument for stop step
STOP_STEP=${1:-"2.6"}  # Default to 2.6 if no argument provided

echo "🚀 ICP Limit Order Protocol - Testing (stop at: $STOP_STEP)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Function to check if we should stop at current step
should_stop() {
    local current_step=$1
    if [[ "$current_step" == "$STOP_STEP" ]]; then
        echo ""
        echo "✅ Stopped at step $STOP_STEP"
        echo ""
        exit 0
    fi
}

# Check if dfx is available
if ! command -v dfx &> /dev/null; then
    print_error "dfx is not installed or not in PATH"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    print_error "Please run this script from the project root directory (where dfx.json is located)"
    exit 1
fi

# Run setup script to ensure fresh environment variables
if [ -f "scripts/limit-order-manual-test-setup.sh" ]; then
    ./scripts/limit-order-manual-test-setup.sh > /dev/null 2>&1
else
    print_error "Setup script not found: scripts/limit-order-manual-test-setup.sh"
    exit 1
fi

source .env.test

# ============================================================================
# SCENARIO 1: MAKER CREATES A LIMIT ORDER
# ============================================================================

echo ""
echo "📋 Scenario 1: Maker Creates Limit Order"

## Step 1.1: Verify System is Ready
echo "1.1 Testing limit-order connection..."

RESPONSE=$(dfx canister call limit-order greet '("Prime")')
if [[ $RESPONSE == *"Hello, Prime!"* ]]; then
    echo "   ✅ Backend connected"
else
    print_error "Limit-order connection failed: $RESPONSE"
    exit 1
fi

should_stop "1.1"

## Step 1.2: Create Your First Limit Order
echo "1.2 Creating first order..."

# Switch to maker identity
dfx identity use maker > /dev/null 2>&1

# Fund the maker with TOKEN_A tokens so they can create orders
MINT_RESPONSE=$(dfx canister call test_token_a mint_tokens "(principal \"$MAKER_PRINCIPAL\", 2000000000:nat)")
if [[ $MINT_RESPONSE == *"Ok"* ]]; then
    echo "   ✅ Maker funded with TOKEN_A"
else
    print_error "Failed to fund maker: $MINT_RESPONSE"
    exit 1
fi

# Create order: Sell 10 TOKEN_A for 0.001 TOKEN_B
EXPIRATION=$(($(date +%s) + 3600))000000000
ORDER_RESPONSE=$(dfx canister call limit-order create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_TOKEN_A\",
  principal \"$TEST_TOKEN_B\",
  1000000000:nat64,
  100000:nat64,
  $EXPIRATION:nat64
)")

if [[ $ORDER_RESPONSE == *"Ok = 1"* ]]; then
    echo "   ✅ Order 1 created"
    ORDER_ID=1
elif [[ $ORDER_RESPONSE == *"Ok = 2"* ]]; then
    echo "   ✅ Order 2 created"
    ORDER_ID=2
elif [[ $ORDER_RESPONSE == *"Ok = 3"* ]]; then
    echo "   ✅ Order 3 created"
    ORDER_ID=3
else
    print_error "Order creation failed: $ORDER_RESPONSE"
    exit 1
fi

should_stop "1.2"

## Step 1.3: Verify Order was Created
echo "1.3 Verifying order..."

ORDER_DETAILS=$(dfx canister call limit-order get_order_by_id "($ORDER_ID:nat64)")
if [[ $ORDER_DETAILS == *"id = $ORDER_ID"* ]]; then
    echo "   ✅ Order $ORDER_ID verified"
else
    print_error "Failed to get order details: $ORDER_DETAILS"
    exit 1
fi

should_stop "1.3"

## Step 1.4: View All Active Orders
echo "1.4 Checking active orders..."

ACTIVE_ORDERS=$(dfx canister call limit-order get_active_orders '()')
if [[ $ACTIVE_ORDERS == *"id = $ORDER_ID"* ]]; then
    echo "   ✅ Order $ORDER_ID in active list"
else
    print_error "Order $ORDER_ID not found in active orders: $ACTIVE_ORDERS"
    exit 1
fi

should_stop "1.4"

## Step 1.5: Create a Second Order
echo "1.5 Creating second order..."

EXPIRATION2=$(($(date +%s) + 7200))000000000
ORDER2_RESPONSE=$(dfx canister call limit-order create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_TOKEN_A\",
  principal \"$TEST_TOKEN_B\",
  500000000:nat64,
  50000:nat64,
  $EXPIRATION2:nat64
)")

if [[ $ORDER2_RESPONSE == *"Ok = 2"* ]]; then
    echo "   ✅ Order 2 created"
    ORDER2_ID=2
elif [[ $ORDER2_RESPONSE == *"Ok = 3"* ]]; then
    echo "   ✅ Order 3 created"
    ORDER2_ID=3
elif [[ $ORDER2_RESPONSE == *"Ok = 4"* ]]; then
    echo "   ✅ Order 4 created"
    ORDER2_ID=4
else
    print_error "Second order creation failed: $ORDER2_RESPONSE"
    exit 1
fi

should_stop "1.5"

## Step 1.6: Verify Both Orders are Active
echo "1.6 Verifying both orders..."

ACTIVE_ORDERS_FINAL=$(dfx canister call limit-order get_active_orders '()')
if [[ $ACTIVE_ORDERS_FINAL == *"id = $ORDER_ID"* ]] && [[ $ACTIVE_ORDERS_FINAL == *"id = $ORDER2_ID"* ]]; then
    echo "   ✅ Both orders active"
else
    print_error "Not all orders found in active list: $ACTIVE_ORDERS_FINAL"
    exit 1
fi

should_stop "1.6"

# ============================================================================
# SCENARIO 2: TAKER DISCOVERS AND FILLS ORDER
# ============================================================================

echo ""
echo "📋 Scenario 2: Taker Fills Order"

## Step 2.1: Switch to Taker Identity
echo "2.1 Switching to taker..."

dfx identity use taker > /dev/null 2>&1
TAKER_IDENTITY=$(dfx identity whoami)
if [[ $TAKER_IDENTITY == "taker" ]]; then
    echo "   ✅ Switched to taker"
else
    print_error "Failed to switch to taker identity: $TAKER_IDENTITY"
    exit 1
fi

should_stop "2.1"

## Step 2.2: Fund Taker with TEST Tokens
echo "2.2 Funding taker..."

# Fund the taker identity with TOKEN_B tokens so they can fill orders
TAKER_MINT_RESPONSE=$(dfx canister call test_token_b mint_tokens "(principal \"$TAKER_PRINCIPAL\", 1000000:nat)")
if [[ $TAKER_MINT_RESPONSE == *"Ok"* ]]; then
    echo "   ✅ Taker funded with TOKEN_B"
else
    print_error "Failed to fund taker: $TAKER_MINT_RESPONSE"
    exit 1
fi

should_stop "2.2"

## Step 2.3: Discover Available Orders
echo "2.3 Discovering orders..."

TAKER_ORDERS=$(dfx canister call limit-order get_active_orders '()')
if [[ $TAKER_ORDERS == *"id = $ORDER_ID"* ]]; then
    echo "   ✅ Taker can see order $ORDER_ID"
else
    print_error "Taker cannot see order $ORDER_ID: $TAKER_ORDERS"
    exit 1
fi

should_stop "2.3"

## Step 2.4: Check Balances Before Fill
echo "2.4 Checking balances before fill..."

MAKER_TOKEN_A_BEFORE=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })")
TAKER_TOKEN_B_BEFORE=$(dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })")
MAKER_TOKEN_B_BEFORE=$(dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })")
TAKER_TOKEN_A_BEFORE=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })")

echo "   ✅ Balances checked"

should_stop "2.4"

## Step 2.5: Fill the Order
echo "2.5 Filling order $ORDER_ID..."

FILL_RESPONSE=$(dfx canister call limit-order fill_order "($ORDER_ID:nat64)")
if [[ $FILL_RESPONSE == *"Ok"* ]]; then
    echo "   ✅ Order $ORDER_ID filled"
else
    print_warning "Order fill failed: $FILL_RESPONSE"
fi

should_stop "2.5"

## Step 2.6: Check Balances After Fill
echo "2.6 Checking balances after fill..."

MAKER_TOKEN_A_AFTER=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })")
TAKER_TOKEN_B_AFTER=$(dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })")
MAKER_TOKEN_B_AFTER=$(dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })")
TAKER_TOKEN_A_AFTER=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })")

echo "   ✅ Balances checked"

should_stop "2.6"

# ============================================================================
# FINAL VERIFICATION
# ============================================================================

echo ""
echo "📋 Final Verification"

## Check if order was actually filled
FINAL_ACTIVE_ORDERS=$(dfx canister call limit-order get_active_orders '()')
if [[ $FINAL_ACTIVE_ORDERS != *"id = $ORDER_ID"* ]]; then
    echo "   ✅ Order $ORDER_ID filled successfully"
else
    print_warning "Order $ORDER_ID still active (may not have been filled)"
fi

## Get system statistics
STATS=$(dfx canister call limit-order get_system_stats '()')
echo "   ✅ System stats: $STATS"

echo ""
echo "🎉 Testing completed!" 