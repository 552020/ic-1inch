#!/bin/bash

# ICP Limit Order Protocol - Automated Manual Testing Script
# This script automates the manual testing guide up to step 2.3 (filling the order)

set -e  # Exit on any error

echo "🚀 ICP Limit Order Protocol - Automated Manual Testing"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
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
print_status "Running setup script to get fresh environment variables..."
if [ -f "scripts/limit-order-manual-test-setup.sh" ]; then
    ./scripts/limit-order-manual-test-setup.sh
    print_success "Setup script completed"
else
    print_error "Setup script not found: scripts/limit-order-manual-test-setup.sh"
    exit 1
fi

# Ask user if they sourced the environment file
echo ""
print_warning "IMPORTANT: Make sure you have sourced the environment file!"
echo "Run: source .env.test"
echo ""
read -p "Have you sourced the .env.test file? (y/n): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_error "Please source the environment file first: source .env.test"
    exit 1
fi

print_success "Environment check passed!"

# ============================================================================
# SCENARIO 1: MAKER CREATES A LIMIT ORDER
# ============================================================================

echo ""
echo "=================================================="
print_status "SCENARIO 1: Maker Creates a Limit Order"
echo "=================================================="

## Step 1.1: Verify System is Ready
print_status "Step 1.1: Verifying system is ready..."

RESPONSE=$(dfx canister call backend greet '("Prime")')
if [[ $RESPONSE == *"Hello, Prime!"* ]]; then
    print_success "Backend connection verified"
else
    print_error "Backend connection failed: $RESPONSE"
    exit 1
fi

## Step 1.2: Create Your First Limit Order
print_status "Step 1.2: Creating first limit order..."

# Switch to maker identity
dfx identity use maker
print_success "Switched to maker identity: $(dfx identity whoami)"

# Create order: Sell 10 ICP for 0.001 TEST tokens
EXPIRATION=$(($(date +%s) + 3600))000000000
print_status "Creating order with expiration: $EXPIRATION"

ORDER_RESPONSE=$(dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_MAKER_ASSET\",
  principal \"$TEST_TAKER_ASSET\",
  1000000000:nat64,
  100000:nat64,
  $EXPIRATION:nat64
)")

if [[ $ORDER_RESPONSE == *"Ok = 1"* ]]; then
    print_success "Order 1 created successfully!"
    ORDER_ID=1
elif [[ $ORDER_RESPONSE == *"Ok = 2"* ]]; then
    print_success "Order 2 created successfully!"
    ORDER_ID=2
elif [[ $ORDER_RESPONSE == *"Ok = 3"* ]]; then
    print_success "Order 3 created successfully!"
    ORDER_ID=3
else
    print_error "Order creation failed: $ORDER_RESPONSE"
    exit 1
fi

## Step 1.3: Verify Order was Created
print_status "Step 1.3: Verifying order was created..."

ORDER_DETAILS=$(dfx canister call backend get_order_by_id "($ORDER_ID:nat64)")
if [[ $ORDER_DETAILS == *"id = $ORDER_ID"* ]]; then
    print_success "Order $ORDER_ID details retrieved successfully"
else
    print_error "Failed to get order details: $ORDER_DETAILS"
    exit 1
fi

## Step 1.4: View All Active Orders
print_status "Step 1.4: Viewing all active orders..."

ACTIVE_ORDERS=$(dfx canister call backend get_active_orders '()')
if [[ $ACTIVE_ORDERS == *"id = $ORDER_ID"* ]]; then
    print_success "Order $ORDER_ID appears in active orders list"
else
    print_error "Order $ORDER_ID not found in active orders: $ACTIVE_ORDERS"
    exit 1
fi

## Step 1.5: Create a Second Order
print_status "Step 1.5: Creating second order..."

EXPIRATION2=$(($(date +%s) + 7200))000000000
ORDER2_RESPONSE=$(dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_MAKER_ASSET\",
  principal \"$TEST_TAKER_ASSET\",
  500000000:nat64,
  50000:nat64,
  $EXPIRATION2:nat64
)")

if [[ $ORDER2_RESPONSE == *"Ok = 2"* ]]; then
    print_success "Order 2 created successfully!"
    ORDER2_ID=2
elif [[ $ORDER2_RESPONSE == *"Ok = 3"* ]]; then
    print_success "Order 3 created successfully!"
    ORDER2_ID=3
elif [[ $ORDER2_RESPONSE == *"Ok = 4"* ]]; then
    print_success "Order 4 created successfully!"
    ORDER2_ID=4
else
    print_error "Second order creation failed: $ORDER2_RESPONSE"
    exit 1
fi

## Step 1.6: Verify Both Orders are Active
print_status "Step 1.6: Verifying both orders are active..."

ACTIVE_ORDERS_FINAL=$(dfx canister call backend get_active_orders '()')
if [[ $ACTIVE_ORDERS_FINAL == *"id = $ORDER_ID"* ]] && [[ $ACTIVE_ORDERS_FINAL == *"id = $ORDER2_ID"* ]]; then
    print_success "Both orders appear in active orders list"
else
    print_error "Not all orders found in active list: $ACTIVE_ORDERS_FINAL"
    exit 1
fi

# ============================================================================
# SCENARIO 2: TAKER DISCOVERS AND FILLS ORDER
# ============================================================================

echo ""
echo "=================================================="
print_status "SCENARIO 2: Taker Discovers and Fills Order"
echo "=================================================="

## Step 2.1: Switch to Taker Identity
print_status "Step 2.1: Switching to taker identity..."

dfx identity use taker
TAKER_IDENTITY=$(dfx identity whoami)
if [[ $TAKER_IDENTITY == "taker" ]]; then
    print_success "Switched to taker identity: $TAKER_IDENTITY"
else
    print_error "Failed to switch to taker identity: $TAKER_IDENTITY"
    exit 1
fi

## Step 2.2: Discover Available Orders
print_status "Step 2.2: Discovering available orders..."

TAKER_ORDERS=$(dfx canister call backend get_active_orders '()')
if [[ $TAKER_ORDERS == *"id = $ORDER_ID"* ]]; then
    print_success "Taker can see order $ORDER_ID in active orders"
else
    print_error "Taker cannot see order $ORDER_ID: $TAKER_ORDERS"
    exit 1
fi

## Step 2.3: Fill the Order
print_status "Step 2.3: Filling order $ORDER_ID..."

FILL_RESPONSE=$(dfx canister call backend fill_order "($ORDER_ID:nat64)")
if [[ $FILL_RESPONSE == *"Ok"* ]]; then
    print_success "Order $ORDER_ID filled successfully!"
else
    print_warning "Order fill failed: $FILL_RESPONSE"
    print_warning "This might be due to insufficient balance in test_token"
    print_warning "You may need to fund the taker identity with TEST tokens"
fi

# ============================================================================
# FINAL VERIFICATION
# ============================================================================

echo ""
echo "=================================================="
print_status "FINAL VERIFICATION"
echo "=================================================="

## Check if order was actually filled
print_status "Verifying order fill status..."

FINAL_ACTIVE_ORDERS=$(dfx canister call backend get_active_orders '()')
if [[ $FINAL_ACTIVE_ORDERS != *"id = $ORDER_ID"* ]]; then
    print_success "Order $ORDER_ID no longer appears in active orders (filled successfully)"
else
    print_warning "Order $ORDER_ID still appears in active orders (may not have been filled)"
fi

## Get system statistics
print_status "Getting system statistics..."

STATS=$(dfx canister call backend get_system_stats '()')
print_success "System statistics: $STATS"

echo ""
echo "=================================================="
print_success "AUTOMATED TESTING COMPLETED!"
echo "=================================================="
echo ""
echo "Test Summary:"
echo "✅ Backend connection verified"
echo "✅ Order creation working"
echo "✅ Order discovery working"
echo "✅ Identity switching working"
echo "✅ Order filling attempted"
echo ""
echo "Next steps:"
echo "1. Check if order was actually filled"
echo "2. If not filled, fund taker identity with TEST tokens"
echo "3. Continue with manual testing guide from step 2.4"
echo "==================================================" 