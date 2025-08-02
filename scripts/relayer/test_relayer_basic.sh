#!/bin/bash

# Basic test script for relayer canister functionality
# Tests basic canister operations after rename from orderbook to relayer

set -e

echo "🧪 Testing Relayer Canister Basic Functionality"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_RUN=0
TESTS_PASSED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${YELLOW}Testing: $test_name${NC}"
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if eval "$test_command"; then
        echo -e "${GREEN}✅ PASSED: $test_name${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ FAILED: $test_name${NC}"
    fi
    echo ""
}

# Test 1: Check if relayer canister is deployed
run_test "Check relayer canister deployment" "dfx canister status relayer"

# Test 2: Check if relayer canister is running
run_test "Check relayer canister is running" "dfx canister status relayer | grep -q 'Status: Running'"

# Test 3: Test basic function call
run_test "Test get_active_fusion_orders function" "dfx canister call relayer get_active_fusion_orders"

# Test 4: Test identity registration (should work even if no identities exist)
run_test "Test register_cross_chain_identity function" "dfx canister call relayer register_cross_chain_identity '(\"0x742d35Cc6431C8D0b6634CF0532B55c2d0C7Bfb8\", principal \"2vxsx-fae\", variant { Maker })'"

# Test 5: Test order creation (basic validation)
run_test "Test create_fusion_order function" "dfx canister call relayer create_fusion_order '(\"0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef\", \"ICP\", \"ETH\", 1000000, 1000000, \"0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890\", \"0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890\", 1754151549, null)'"

# Test 6: Check canister memory usage
run_test "Check relayer canister memory usage" "dfx canister status relayer | grep -q 'Memory Size:'"

# Test 7: Verify canister ID is correct
run_test "Verify relayer canister ID" "dfx canister id relayer"

# Test 8: Test canister info
run_test "Test canister info" "dfx canister info relayer"

echo "=============================================="
echo "📊 Test Results Summary"
echo "=============================================="
echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Tests Failed: $((TESTS_RUN - TESTS_PASSED))${NC}"
echo -e "${YELLOW}Total Tests: $TESTS_RUN${NC}"

if [ "$TESTS_PASSED" -eq "$TESTS_RUN" ]; then
    echo -e "${GREEN}🎉 All tests passed! Relayer canister is working correctly.${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Please check the relayer canister deployment.${NC}"
    exit 1
fi 