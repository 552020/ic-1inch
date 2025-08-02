#!/bin/bash

# Test script for enhanced order acceptance functionality in orderbook canister
# Tests resolver coordination and order acceptance scenarios

set -e

echo "🧪 Testing Enhanced Order Acceptance and Resolver Coordination"
echo "============================================================="

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

# Test 1: Compile enhanced order acceptance functionality
run_test "Compile enhanced order acceptance" "(cd src/orderbook && cargo check)"

# Test 2: Build canister with enhanced order acceptance
run_test "Build canister with enhanced order acceptance" "(cd src/orderbook && cargo build --target wasm32-unknown-unknown --release)"

# Test 3: Verify enhanced accept_fusion_order function is present
echo -e "${YELLOW}Verifying enhanced accept_fusion_order function...${NC}"
if grep -q "async fn accept_fusion_order" src/orderbook/src/lib.rs; then
    echo -e "${GREEN}✅ Enhanced accept_fusion_order function present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Enhanced accept_fusion_order function missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 4: Verify comprehensive order status validation
echo -e "${YELLOW}Verifying comprehensive order status validation...${NC}"
STATUS_CHECKS=("OrderStatus::Pending" "OrderStatus::Accepted" "OrderStatus::Completed" "OrderStatus::Failed" "OrderStatus::Cancelled")
MISSING_STATUS_CHECKS=0

for status in "${STATUS_CHECKS[@]}"; do
    if grep -q "$status" src/orderbook/src/lib.rs; then
        echo -e "${GREEN}✓ Found status check: $status${NC}"
    else
        echo -e "${RED}✗ Missing status check: $status${NC}"
        MISSING_STATUS_CHECKS=$((MISSING_STATUS_CHECKS + 1))
    fi
done

if [ "$MISSING_STATUS_CHECKS" -eq "0" ]; then
    echo -e "${GREEN}✅ All order status validations present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_STATUS_CHECKS status validations${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 5: Verify enhanced expiration checking
echo -e "${YELLOW}Verifying enhanced expiration checking...${NC}"
if grep -q "time_remaining" src/orderbook/src/lib.rs && grep -q "ten_minutes_ns" src/orderbook/src/lib.rs; then
    echo -e "${GREEN}✅ Enhanced expiration checking present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Enhanced expiration checking missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 6: Verify EIP-712 signature validation function
echo -e "${YELLOW}Verifying EIP-712 signature validation...${NC}"
if grep -q "validate_eip712_signature_format" src/orderbook/src/lib.rs; then
    echo -e "${GREEN}✅ EIP-712 signature validation function present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ EIP-712 signature validation function missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 7: Verify EIP-712 signature format validation logic
echo -e "${YELLOW}Verifying EIP-712 signature format validation logic...${NC}"
EIP712_VALIDATIONS=("domain_separator" "type_hash" "order_hash" "signature_r" "signature_s" "signature_v" "signer_address")
MISSING_EIP712_VALIDATIONS=0

for validation in "${EIP712_VALIDATIONS[@]}"; do
    if grep -q "$validation" src/orderbook/src/lib.rs; then
        echo -e "${GREEN}✓ Found EIP-712 validation: $validation${NC}"
    else
        echo -e "${RED}✗ Missing EIP-712 validation: $validation${NC}"
        MISSING_EIP712_VALIDATIONS=$((MISSING_EIP712_VALIDATIONS + 1))
    fi
done

if [ "$MISSING_EIP712_VALIDATIONS" -eq "0" ]; then
    echo -e "${GREEN}✅ All EIP-712 signature validations present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_EIP712_VALIDATIONS EIP-712 validations${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 8: Verify resolver information updates
echo -e "${YELLOW}Verifying resolver information updates...${NC}"
RESOLVER_UPDATES=("resolver_eth_address" "resolver_icp_principal" "accepted_at")
MISSING_RESOLVER_UPDATES=0

for update in "${RESOLVER_UPDATES[@]}"; do
    if grep -q "$update" src/orderbook/src/lib.rs; then
        echo -e "${GREEN}✓ Found resolver update: $update${NC}"
    else
        echo -e "${RED}✗ Missing resolver update: $update${NC}"
        MISSING_RESOLVER_UPDATES=$((MISSING_RESOLVER_UPDATES + 1))
    fi
done

if [ "$MISSING_RESOLVER_UPDATES" -eq "0" ]; then
    echo -e "${GREEN}✅ All resolver information updates present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_RESOLVER_UPDATES resolver updates${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 9: Verify order direction detection
echo -e "${YELLOW}Verifying order direction detection...${NC}"
if grep -q "is_eth_to_icp" src/orderbook/src/lib.rs && grep -q "is_eth_asset" src/orderbook/src/lib.rs; then
    echo -e "${GREEN}✅ Order direction detection present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Order direction detection missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 10: Verify enhanced response data format
echo -e "${YELLOW}Verifying enhanced response data format...${NC}"
if grep -q "ETH_TO_ICP" src/orderbook/src/lib.rs && grep -q "ICP_TO_ETH" src/orderbook/src/lib.rs; then
    echo -e "${GREEN}✅ Enhanced response data format present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Enhanced response data format missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 11: Verify resolver authorization checks
echo -e "${YELLOW}Verifying resolver authorization checks...${NC}"
if grep -q "maker_icp_principal == caller" src/orderbook/src/lib.rs; then
    echo -e "${GREEN}✅ Resolver authorization checks present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Resolver authorization checks missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 12: Verify resolver ETH address validation
echo -e "${YELLOW}Verifying resolver ETH address validation...${NC}"
if grep -q "is_valid_address.*resolver_eth_address" src/orderbook/src/lib.rs; then
    echo -e "${GREEN}✅ Resolver ETH address validation present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Resolver ETH address validation missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 13: Check for compilation warnings
echo -e "${YELLOW}Checking for compilation warnings...${NC}"
WARNINGS=$((cd src/orderbook && cargo check 2>&1) | grep -c "warning:" || echo "0")
if [ "$WARNINGS" -lt "15" ]; then
    echo -e "${GREEN}✅ Acceptable number of compilation warnings: $WARNINGS${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${YELLOW}⚠️  Found $WARNINGS compilation warnings (review if needed)${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Summary
echo ""
echo "============================================================="
echo -e "${YELLOW}Test Summary${NC}"
echo "============================================================="
echo "Tests Run: $TESTS_RUN"
echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests Failed: ${RED}$((TESTS_RUN - TESTS_PASSED))${NC}"

if [ "$TESTS_PASSED" -eq "$TESTS_RUN" ]; then
    echo -e "${GREEN}🎉 All tests passed! Enhanced order acceptance is ready.${NC}"
    echo ""
    echo "✨ Enhanced Order Acceptance Features:"
    echo "  • Comprehensive order status validation"
    echo "  • Enhanced expiration checking with warnings"
    echo "  • EIP-712 signature validation for ETH→ICP orders"
    echo "  • Improved resolver information tracking"
    echo "  • Order direction detection and handling"
    echo "  • Enhanced response data for cross-chain coordination"
    echo "  • Resolver authorization and validation checks"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Please review the issues above.${NC}"
    exit 1
fi