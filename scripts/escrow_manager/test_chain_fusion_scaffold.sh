#!/bin/bash

# Test script for Chain Fusion scaffold implementation
# Task 5: Scaffold Chain Fusion integration for EVM escrow creation

set -e

echo "🚀 Testing Chain Fusion Scaffold Implementation..."
echo "================================================="

# Configuration
CANISTER_NAME="escrow_manager"
NETWORK="local"
IDENTITY="default"

# Test data
ORDER_HASH="0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
HASHLOCK="0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
MAKER="0x742d35Cc6431C8D0b6634CF0532B55c2d0C7Bfb8"
TAKER="0x8ba1f109551bD432803012645Hac136c4C00A567"
TOKEN="0xA0b86a33E6C24b6C78F0D0A1F0f6c2e4B9fC32b1"
AMOUNT=1000000
SAFETY_DEPOSIT=100000
TIMELOCK=$(($(date +%s) + 3600)) # 1 hour from now
SRC_CHAIN_ID=1
DST_CHAIN_ID=84532
SRC_TOKEN="ICP"
DST_TOKEN="ETH"
SRC_AMOUNT=500000
DST_AMOUNT=1000000

echo "📋 Test Configuration:"
echo "  Network: $NETWORK"
echo "  Canister: $CANISTER_NAME"
echo "  Order Hash: $ORDER_HASH"
echo "  Chain IDs: $SRC_CHAIN_ID → $DST_CHAIN_ID"
echo ""

# Function to test canister call
test_canister_call() {
    local method=$1
    local args=$2
    local description=$3
    
    echo "🔧 Testing: $description"
    echo "   Method: $method"
    echo "   Args: $args"
    
    # Use dfx canister call with error handling
    if output=$(dfx canister call $CANISTER_NAME $method "$args" --network $NETWORK 2>&1); then
        echo "   ✅ Success: $output"
        return 0
    else
        echo "   ❌ Failed: $output"
        return 1
    fi
}

# Function to check if canister is deployed
check_canister_deployed() {
    echo "🔍 Checking if canister is deployed..."
    
    if dfx canister status $CANISTER_NAME --network $NETWORK >/dev/null 2>&1; then
        echo "   ✅ Canister is deployed"
        return 0
    else
        echo "   ❌ Canister is not deployed"
        echo "   💡 Please deploy the canister first: dfx deploy $CANISTER_NAME --network $NETWORK"
        return 1
    fi
}

# Test 1: Check canister deployment
echo "🏁 Test 1: Canister Deployment Check"
echo "-----------------------------------"
if ! check_canister_deployed; then
    echo "❌ Cannot proceed with tests - canister not deployed"
    exit 1
fi
echo ""

# Test 2: Check threshold ECDSA health
echo "🏁 Test 2: Threshold ECDSA Health Check"
echo "---------------------------------------"
if test_canister_call "check_threshold_ecdsa_health" "()" "Check threshold ECDSA health"; then
    ECDSA_TEST_PASSED=true
else
    ECDSA_TEST_PASSED=false
fi
echo ""

# Test 3: Derive deterministic EVM address
echo "🏁 Test 3: Deterministic EVM Address Derivation"
echo "-----------------------------------------------"
if test_canister_call "derive_deterministic_evm_address" "(\"$ORDER_HASH\")" "Derive EVM address from order hash"; then
    ADDRESS_TEST_PASSED=true
else
    ADDRESS_TEST_PASSED=false
fi
echo ""

# Test 4: Get Chain Fusion configuration
echo "🏁 Test 4: Chain Fusion Configuration"
echo "-------------------------------------"
if test_canister_call "get_chain_fusion_config" "()" "Get Chain Fusion configuration"; then
    CONFIG_TEST_PASSED=true
else
    CONFIG_TEST_PASSED=false
fi
echo ""

# Test 5: Create EVM escrow via Chain Fusion (scaffold)
echo "🏁 Test 5: EVM Escrow Creation (Scaffold)"
echo "-----------------------------------------"
evm_escrow_args="(
    \"$ORDER_HASH\",
    \"$HASHLOCK\",
    \"$MAKER\",
    \"$TAKER\",
    \"$TOKEN\",
    $AMOUNT,
    $SAFETY_DEPOSIT,
    $TIMELOCK,
    $SRC_CHAIN_ID,
    $DST_CHAIN_ID,
    \"$SRC_TOKEN\",
    \"$DST_TOKEN\",
    $SRC_AMOUNT,
    $DST_AMOUNT
)"

if test_canister_call "create_evm_escrow_via_chain_fusion" "$evm_escrow_args" "Create EVM escrow via Chain Fusion"; then
    ESCROW_CREATE_TEST_PASSED=true
else
    ESCROW_CREATE_TEST_PASSED=false
fi
echo ""

# Test 6: Verify EVM escrow state
echo "🏁 Test 6: EVM Escrow State Verification"
echo "----------------------------------------"
# Use a placeholder address for testing
TEST_ESCROW_ADDRESS="0x1234567890123456789012345678901234567890"
if test_canister_call "verify_evm_escrow_state" "(\"$TEST_ESCROW_ADDRESS\")" "Verify EVM escrow state"; then
    VERIFY_TEST_PASSED=true
else
    VERIFY_TEST_PASSED=false
fi
echo ""

# Test Results Summary
echo "📊 Test Results Summary"
echo "======================="
echo "Test 1 - Canister Deployment:        ✅ PASSED"
echo "Test 2 - Threshold ECDSA Health:     $([ "$ECDSA_TEST_PASSED" = true ] && echo "✅ PASSED" || echo "❌ FAILED")"
echo "Test 3 - EVM Address Derivation:     $([ "$ADDRESS_TEST_PASSED" = true ] && echo "✅ PASSED" || echo "❌ FAILED")"
echo "Test 4 - Chain Fusion Config:        $([ "$CONFIG_TEST_PASSED" = true ] && echo "✅ PASSED" || echo "❌ FAILED")"
echo "Test 5 - EVM Escrow Creation:        $([ "$ESCROW_CREATE_TEST_PASSED" = true ] && echo "✅ PASSED" || echo "❌ FAILED")"
echo "Test 6 - EVM State Verification:     $([ "$VERIFY_TEST_PASSED" = true ] && echo "✅ PASSED" || echo "❌ FAILED")"
echo ""

# Overall result
if [ "$ECDSA_TEST_PASSED" = true ] && [ "$ADDRESS_TEST_PASSED" = true ] && [ "$CONFIG_TEST_PASSED" = true ] && [ "$ESCROW_CREATE_TEST_PASSED" = true ] && [ "$VERIFY_TEST_PASSED" = true ]; then
    echo "🎉 All Chain Fusion scaffold tests PASSED!"
    echo "✅ Chain Fusion integration scaffold is working correctly"
    exit 0
else
    echo "❌ Some Chain Fusion scaffold tests FAILED"
    echo "💡 Please check the implementation and try again"
    exit 1
fi