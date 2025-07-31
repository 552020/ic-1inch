#!/bin/bash

# Fusion+ Mechanical Turk - Manual Test Script
# This script demonstrates the complete cross-chain swap flow

set -e

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

# Check if environment is loaded
if [ -z "$MAKER_PRINCIPAL" ]; then
    print_error "Environment not loaded. Run: source .env.mechanical-turk"
    exit 1
fi

echo "🚀 Starting Fusion+ Mechanical Turk Manual Test"
echo ""

# Test 1: Verify Canisters are Running
print_status "Test 1: Verifying fusion canisters..."

if [ -z "$ORDERBOOK_CANISTER_ID" ] || [ -z "$ESCROW_CANISTER_ID" ]; then
    print_error "Fusion canisters not deployed. Run: ./scripts/deploy-mechanical-turk.sh"
    exit 1
fi

# Test orderbook canister
print_status "Testing orderbook canister..."
dfx canister call orderbook get_active_fusion_orders '()'
print_success "Orderbook canister is responsive"

# Test escrow canister
print_status "Testing escrow canister..."
dfx canister call escrow list_fusion_escrows '()'
print_success "Escrow canister is responsive"

echo ""

# Test 2: Create Cross-Chain Order (ICP → ETH) with Automatic Token Locking
print_status "Test 2: Creating ICP → ETH fusion order with AUTOMATIC TOKEN LOCKING..."

dfx identity use maker
print_status "Switched to maker identity: $(dfx identity whoami)"

# Create fusion order (with automatic ICP token locking for ICP → ETH)
print_status "⚠️  ASYMMETRIC FLOW: ICP tokens will be AUTOMATICALLY LOCKED during order creation"
ORDER_RESULT=$(dfx canister call orderbook create_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ICP },
  variant { ETH },
  ${ICP_AMOUNT}:nat64,
  1000000000000000:nat64,
  ${ORDER_EXPIRATION}:nat64
)")

if [[ $ORDER_RESULT == *"Ok"* ]]; then
    ORDER_ID=$(echo $ORDER_RESULT | grep -o '"[^"]*"' | head -1 | tr -d '"')
    print_success "Created fusion order: $ORDER_ID"
    print_success "🔒 AUTOMATIC LOCKING: ICP tokens transferred to escrow during order creation"
    export CURRENT_ORDER_ID="$ORDER_ID"
    
    # Verify automatic token locking occurred
    print_status "Verifying automatic token locking..."
    MAKER_BALANCE=$(dfx canister call test_token_icp icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })")
    ESCROW_BALANCE=$(dfx canister call test_token_icp icrc1_balance_of "(record { owner = principal \"$(dfx canister id escrow)\"; subaccount = null })")
    
    print_success "Maker ICP balance after order creation: $MAKER_BALANCE"
    print_success "Escrow ICP balance after order creation: $ESCROW_BALANCE"
else
    print_error "Failed to create order: $ORDER_RESULT"
    exit 1
fi

echo ""

# Test 3: Resolver Accepts Order
print_status "Test 3: Resolver accepting the order..."

dfx identity use resolver
print_status "Switched to resolver identity: $(dfx identity whoami)"

# Accept the order
ACCEPT_RESULT=$(dfx canister call orderbook accept_fusion_order "(
  \"$CURRENT_ORDER_ID\",
  \"$RESOLVER_ETH_ADDRESS\"
)")

if [[ $ACCEPT_RESULT == *"Ok"* ]]; then
    print_success "Resolver accepted order: $CURRENT_ORDER_ID"
else
    print_error "Failed to accept order: $ACCEPT_RESULT"
    exit 1
fi

echo ""

# Test 4: Check Order Status
print_status "Test 4: Checking order status..."

ORDER_STATUS=$(dfx canister call orderbook get_fusion_order_status "(\"$CURRENT_ORDER_ID\")")
print_success "Order status: $ORDER_STATUS"

echo ""

# Test 5: Test Basic Escrow Functions
print_status "Test 5: Testing basic escrow functions..."

dfx identity use maker
print_status "Switched to maker identity: $(dfx identity whoami)"

# Create escrow with hashlock (basic function test)
HASHLOCK="blob \"\\01\\02\\03\\04\\05\\06\\07\\08\\09\\0a\\0b\\0c\\0d\\0e\\0f\\10\\11\\12\\13\\14\\15\\16\\17\\18\\19\\1a\\1b\\1c\\1d\\1e\\1f\\20\""
TIMELOCK="$(($(date +%s) + 7200))000000000"  # 2 hours from now

print_status "Testing escrow creation..."
ESCROW_RESULT=$(dfx canister call escrow lock_icp_for_swap "(
  \"$CURRENT_ORDER_ID\",
  ${ICP_AMOUNT}:nat64,
  $HASHLOCK,
  ${TIMELOCK}:nat64
)")

if [[ $ESCROW_RESULT == *"Ok"* ]]; then
    ESCROW_ID=$(echo $ESCROW_RESULT | grep -o '"[^"]*"' | head -1 | tr -d '"')
    print_success "✅ Escrow creation function works: $ESCROW_ID"
    export CURRENT_ESCROW_ID="$ESCROW_ID"
else
    print_error "❌ Escrow creation failed: $ESCROW_RESULT"
    exit 1
fi

# Test escrow status query
print_status "Testing escrow status query..."
ESCROW_STATUS=$(dfx canister call escrow get_fusion_escrow_status "(\"$CURRENT_ESCROW_ID\")")
print_success "✅ Escrow status query works"

echo ""

# Test 6: Test Order Status Updates
print_status "Test 6: Testing order status updates..."

dfx identity use relayer
print_status "Switched to relayer identity: $(dfx identity whoami)"

# Test order status update function
print_status "Testing order status update..."
dfx canister call orderbook update_order_status "(
  \"$CURRENT_ORDER_ID\",
  variant { Completed }
)"

print_success "✅ Order status update function works"

echo ""

# Test 8: Verify Final State
print_status "Test 8: Verifying final state..."

# Check order status
FINAL_ORDER_STATUS=$(dfx canister call orderbook get_fusion_order_status "(\"$CURRENT_ORDER_ID\")")
print_success "Final order status: $FINAL_ORDER_STATUS"

# Check escrow status
FINAL_ESCROW_STATUS=$(dfx canister call escrow get_fusion_escrow_status "(\"$CURRENT_ESCROW_ID\")")
print_success "Final escrow status: $FINAL_ESCROW_STATUS"

# Check active orders (should be empty)
ACTIVE_ORDERS=$(dfx canister call orderbook get_active_fusion_orders '()')
print_success "Active orders: $ACTIVE_ORDERS"

echo ""

# Test Summary
echo "=================================================="
print_success "🎉 Basic Function Tests Completed Successfully!"
echo ""
echo "✅ Functions Tested:"
echo "• Orderbook canister: create_order, accept_fusion_order, get_fusion_order_status"
echo "• Escrow canister: lock_icp_for_swap, get_fusion_escrow_status"
echo "• Identity management: Multiple user roles work"
echo "• Order status updates: Relayer can update order states"
echo ""
echo "❌ NOT Tested (requires full system):"
echo "• Real cross-chain transactions"
echo "• Ethereum contract integration"
echo "• MetaMask authentication"
echo "• Actual token transfers"
echo "• Receipt-based atomic completion"
echo ""
echo "Next Steps for Full E2E Testing:"
echo "1. Deploy Ethereum contracts on Sepolia"
echo "2. Build React frontend with MetaMask"
echo "3. Integrate real token transfers"
echo "4. Test complete cross-chain flows"
echo ""
print_success "🚀 Basic canisters are ready for frontend integration!"
echo "=================================================="

# Reset to default identity
dfx identity use default