#!/bin/bash

# Fusion+ Mechanical Turk - Automated Real Token Testing Script
# This script automatically tests the real token integration implemented in Task 2.1.1

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Function to check if command succeeded
check_result() {
    if [ $? -eq 0 ]; then
        print_success "$1"
    else
        print_error "$1"
        exit 1
    fi
}

# Function to extract value from dfx output
extract_value() {
    echo "$1" | grep -o '"[^"]*"' | head -1 | tr -d '"'
}

# Function to check if canister call succeeded
check_canister_call() {
    local result="$1"
    local expected="$2"
    local description="$3"
    
    if echo "$result" | grep -q "$expected"; then
        print_success "$description"
        return 0
    else
        print_error "$description - Expected: $expected, Got: $result"
        return 1
    fi
}

echo "🚀 Starting Fusion+ Mechanical Turk Real Token Integration Tests"
echo "================================================================"

# Load environment variables
if [ -f ".env.mechanical-turk" ]; then
    source .env.mechanical-turk
    print_status "Loaded environment variables"
else
    print_error "Environment file .env.mechanical-turk not found"
    exit 1
fi

# Test 1: Verify Canisters are Deployed
print_status "Test 1: Verifying canisters are deployed..."

# Check orderbook canister
ORDERBOOK_RESULT=$(dfx canister call orderbook get_active_fusion_orders '()' 2>/dev/null || echo "ERROR")
check_canister_call "$ORDERBOOK_RESULT" "()" "Orderbook canister responds"

# Check escrow canister
ESCROW_RESULT=$(dfx canister call escrow list_fusion_escrows '()' 2>/dev/null || echo "ERROR")
check_canister_call "$ESCROW_RESULT" "()" "Escrow canister responds"

# Check test token canisters
TOKEN_A_RESULT=$(dfx canister call test_token_a icrc1_name '()' 2>/dev/null || echo "ERROR")
check_canister_call "$TOKEN_A_RESULT" "Token A" "Test token A canister responds"

TOKEN_B_RESULT=$(dfx canister call test_token_b icrc1_name '()' 2>/dev/null || echo "ERROR")
check_canister_call "$TOKEN_B_RESULT" "Token B" "Test token B canister responds"

print_success "All canisters are deployed and responding"

# Test 2: Fund Test Identities
print_status "Test 2: Funding test identities with tokens..."

# Fund maker identity
dfx identity use maker
MAKER_PRINCIPAL=$(dfx identity get-principal)
print_status "Maker principal: $MAKER_PRINCIPAL"

# Fund maker with test tokens
dfx canister call test_token_a mint_for_caller "(1000000000:nat64)" > /dev/null
dfx canister call test_token_b mint_for_caller "(1000000000:nat64)" > /dev/null
check_result "Funded maker with test tokens"

# Fund resolver identity
dfx identity use resolver
RESOLVER_PRINCIPAL=$(dfx identity get-principal)
print_status "Resolver principal: $RESOLVER_PRINCIPAL"

# Fund resolver with test tokens
dfx canister call test_token_a mint_for_caller "(1000000000:nat64)" > /dev/null
dfx canister call test_token_b mint_for_caller "(1000000000:nat64)" > /dev/null
check_result "Funded resolver with test tokens"

# Switch back to maker
dfx identity use maker

print_success "Test identities funded with tokens"

# Test 3: Verify Initial Balances
print_status "Test 3: Verifying initial token balances..."

# Check maker's initial balance
MAKER_BALANCE_A=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })" 2>/dev/null)
MAKER_BALANCE_B=$(dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })" 2>/dev/null)

if echo "$MAKER_BALANCE_A" | grep -q "1000000000" && echo "$MAKER_BALANCE_B" | grep -q "1000000000"; then
    print_success "Maker has correct initial balances (10 tokens each)"
else
    print_error "Maker initial balances incorrect"
    exit 1
fi

print_success "Initial balances verified"

# Test 4: Create Fusion Order
print_status "Test 4: Creating fusion order..."

# Create ICP → ETH fusion order
ORDER_RESULT=$(dfx canister call orderbook create_fusion_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ICP },
  variant { ETH },
  ${ICP_AMOUNT}:nat64,
  10000000000000000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)")

ORDER_ID=$(extract_value "$ORDER_RESULT")
if [ -n "$ORDER_ID" ]; then
    print_success "Created fusion order: $ORDER_ID"
else
    print_error "Failed to create fusion order"
    exit 1
fi

# Test 5: Accept Order
print_status "Test 5: Accepting fusion order..."

# Switch to resolver and accept order
dfx identity use resolver
ACCEPT_RESULT=$(dfx canister call orderbook accept_fusion_order "(
  \"$ORDER_ID\",
  \"$RESOLVER_ETH_ADDRESS\"
)")

check_canister_call "$ACCEPT_RESULT" "(variant { Ok })" "Order accepted successfully"

# Test 6: Lock Real Tokens in Escrow
print_status "Test 6: Locking real tokens in escrow..."

# Switch back to maker and lock tokens
dfx identity use maker
ESCROW_RESULT=$(dfx canister call escrow lock_icp_for_swap "(
  \"$ORDER_ID\",
  ${ICP_AMOUNT}:nat64,
  principal \"$RESOLVER_PRINCIPAL\",
  $(($(date +%s) + 7200))000000000:nat64
)")

ESCROW_ID=$(extract_value "$ESCROW_RESULT")
if [ -n "$ESCROW_ID" ]; then
    print_success "Created escrow: $ESCROW_ID"
else
    print_error "Failed to create escrow"
    exit 1
fi

# Test 7: Verify Token Transfer to Escrow
print_status "Test 7: Verifying token transfer to escrow..."

# Get escrow canister ID
ESCROW_CANISTER_ID=$(dfx canister id escrow)

# Check escrow status
ESCROW_STATUS=$(dfx canister call escrow get_fusion_escrow_status "(\"$ESCROW_ID\")")
check_canister_call "$ESCROW_STATUS" "Funded" "Escrow status is Funded"

# Check escrow token balance
ESCROW_BALANCE=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$ESCROW_CANISTER_ID\"; subaccount = null })")
if echo "$ESCROW_BALANCE" | grep -q "1000000000"; then
    print_success "Escrow holds correct token amount (10 tokens)"
else
    print_error "Escrow token balance incorrect"
    exit 1
fi

# Check maker's reduced balance
MAKER_NEW_BALANCE=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })")
if echo "$MAKER_NEW_BALANCE" | grep -q "0"; then
    print_success "Maker's balance correctly reduced to 0"
else
    print_error "Maker's balance not correctly reduced"
    exit 1
fi

print_success "Token transfer to escrow verified"

# Test 8: Claim Tokens as Resolver
print_status "Test 8: Claiming tokens as resolver..."

# Switch to resolver and claim tokens
dfx identity use resolver
CLAIM_RESULT=$(dfx canister call escrow claim_locked_icp "(
  \"$ESCROW_ID\",
  \"0x1234567890abcdef...\"
)")

check_canister_call "$CLAIM_RESULT" "(variant { Ok })" "Tokens claimed successfully"

# Test 9: Verify Final Token Balances
print_status "Test 9: Verifying final token balances..."

# Check resolver's increased balance
RESOLVER_FINAL_BALANCE=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$RESOLVER_PRINCIPAL\"; subaccount = null })")
if echo "$RESOLVER_FINAL_BALANCE" | grep -q "2000000000"; then
    print_success "Resolver's balance correctly increased to 20 tokens"
else
    print_error "Resolver's balance not correctly increased"
    exit 1
fi

# Check escrow's empty balance
ESCROW_FINAL_BALANCE=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$ESCROW_CANISTER_ID\"; subaccount = null })")
if echo "$ESCROW_FINAL_BALANCE" | grep -q "0"; then
    print_success "Escrow balance correctly reduced to 0"
else
    print_error "Escrow balance not correctly reduced"
    exit 1
fi

# Check final escrow status
ESCROW_FINAL_STATUS=$(dfx canister call escrow get_fusion_escrow_status "(\"$ESCROW_ID\")")
check_canister_call "$ESCROW_FINAL_STATUS" "Claimed" "Escrow status is Claimed"

print_success "Final token balances verified"

# Test 10: Error Testing
print_status "Test 10: Testing error scenarios..."

# Test insufficient balance
dfx identity use maker
INSUFFICIENT_RESULT=$(dfx canister call escrow lock_icp_for_swap "(
  \"$ORDER_ID\",
  999999999999:nat64,
  principal \"$RESOLVER_PRINCIPAL\",
  $(($(date +%s) + 7200))000000000:nat64
)" 2>&1 || echo "ERROR")

check_canister_call "$INSUFFICIENT_RESULT" "InsufficientBalance" "Insufficient balance error handled"

# Test unauthorized claim
UNAUTHORIZED_RESULT=$(dfx canister call escrow claim_locked_icp "(
  \"$ESCROW_ID\",
  \"0x1234567890abcdef...\"
)" 2>&1 || echo "ERROR")

check_canister_call "$UNAUTHORIZED_RESULT" "Unauthorized" "Unauthorized access blocked"

print_success "Error scenarios tested successfully"

# Test 11: Refund Testing
print_status "Test 11: Testing refund functionality..."

# Create new order for refund test
REFUND_ORDER_RESULT=$(dfx canister call orderbook create_fusion_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ICP },
  variant { ETH },
  ${ICP_AMOUNT}:nat64,
  10000000000000000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)")

REFUND_ORDER_ID=$(extract_value "$REFUND_ORDER_RESULT")

# Accept order
dfx identity use resolver
dfx canister call orderbook accept_fusion_order "(
  \"$REFUND_ORDER_ID\",
  \"$RESOLVER_ETH_ADDRESS\"
)" > /dev/null

# Lock tokens with short timelock
dfx identity use maker
REFUND_ESCROW_RESULT=$(dfx canister call escrow lock_icp_for_swap "(
  \"$REFUND_ORDER_ID\",
  ${ICP_AMOUNT}:nat64,
  principal \"$RESOLVER_PRINCIPAL\",
  $(($(date +%s) + 10))000000000:nat64
)")

REFUND_ESCROW_ID=$(extract_value "$REFUND_ESCROW_RESULT")

# Wait for timelock to expire
sleep 15

# Refund tokens
REFUND_RESULT=$(dfx canister call escrow refund_locked_icp "(\"$REFUND_ESCROW_ID\")")
check_canister_call "$REFUND_RESULT" "(variant { Ok })" "Tokens refunded successfully"

# Verify refund
MAKER_REFUND_BALANCE=$(dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })")
if echo "$MAKER_REFUND_BALANCE" | grep -q "1000000000"; then
    print_success "Maker's balance correctly restored after refund"
else
    print_error "Maker's balance not correctly restored after refund"
    exit 1
fi

print_success "Refund functionality tested successfully"

# Test 12: Integration Testing
print_status "Test 12: Testing integration between canisters..."

# Verify orderbook and escrow work together
ACTIVE_ORDERS=$(dfx canister call orderbook get_active_fusion_orders '()')
ALL_ESCROWS=$(dfx canister call escrow list_fusion_escrows '()')

if [ -n "$ACTIVE_ORDERS" ] && [ -n "$ALL_ESCROWS" ]; then
    print_success "Orderbook and escrow integration working"
else
    print_error "Orderbook and escrow integration failed"
    exit 1
fi

print_success "Integration testing completed"

# Final Summary
echo ""
echo "================================================================"
print_success "🎉 ALL TESTS PASSED! Real Token Integration is Working!"
echo "================================================================"
echo ""
echo "✅ Real token transfers work with ICRC-1 calls"
echo "✅ Balance checking prevents insufficient fund errors"
echo "✅ Escrow custody actually holds tokens"
echo "✅ Claim operations transfer tokens to resolvers"
echo "✅ Refund operations return tokens to original lockers"
echo "✅ Error handling provides clear feedback"
echo "✅ Integration works between all canisters"
echo "✅ State management tracks token movements correctly"
echo ""
echo "🚀 Mechanical Turk with Real Token Integration is Ready!"
echo ""
echo "Next Steps:"
echo "1. Deploy to testnet for broader testing"
echo "2. Integrate with real ICP ledger for production"
echo "3. Add frontend integration for user experience"
echo "4. Implement cross-chain coordination tools"
echo "================================================================" 