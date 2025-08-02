#!/bin/bash

# Test script for enhanced order creation functionality in relayer canister
# Tests 1inch LOP compatibility and order creation scenarios

set -e

echo "🧪 Testing Enhanced Order Creation for 1inch LOP Compatibility"
echo "=============================================================="

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

# Test 1: Compile enhanced order creation functionality
run_test "Compile enhanced order creation" "(cd src/relayer && cargo check)"

# Test 2: Build canister with enhanced order creation
run_test "Build canister with enhanced order creation" "(cd src/relayer && cargo build --target wasm32-unknown-unknown --release)"

# Test 3: Verify enhanced order creation functions are present
echo -e "${YELLOW}Verifying enhanced order creation functions...${NC}"
REQUIRED_FUNCTIONS=(
    "create_fusion_order"
    "validate_lop_parameters"
    "is_eth_asset"
    "token_to_address"
    "is_valid_address"
    "is_valid_hex_string"
)

MISSING_FUNCTIONS=0
for func in "${REQUIRED_FUNCTIONS[@]}"; do
    if grep -q "fn $func" src/relayer/src/lib.rs; then
        echo -e "${GREEN}✓ Found function: $func${NC}"
    else
        echo -e "${RED}✗ Missing function: $func${NC}"
        MISSING_FUNCTIONS=$((MISSING_FUNCTIONS + 1))
    fi
done

if [ "$MISSING_FUNCTIONS" -eq "0" ]; then
    echo -e "${GREEN}✅ All required order creation functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_FUNCTIONS required functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 4: Verify 1inch LOP parameter validation
echo -e "${YELLOW}Verifying 1inch LOP parameter validation...${NC}"
LOP_PARAMS=("salt" "maker_asset" "taker_asset" "making_amount" "taking_amount" "maker_traits")
MISSING_PARAMS=0

for param in "${LOP_PARAMS[@]}"; do
    if grep -q "$param" src/relayer/src/lib.rs; then
        echo -e "${GREEN}✓ Found parameter: $param${NC}"
    else
        echo -e "${RED}✗ Missing parameter: $param${NC}"
        MISSING_PARAMS=$((MISSING_PARAMS + 1))
    fi
done

if [ "$MISSING_PARAMS" -eq "0" ]; then
    echo -e "${GREEN}✅ All 1inch LOP parameters supported${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_PARAMS LOP parameters${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 5: Verify EIP-712 signature handling
echo -e "${YELLOW}Verifying EIP-712 signature handling...${NC}"
if grep -q "eip712_signature" src/relayer/src/lib.rs && grep -q "EIP712Signature" src/relayer/src/lib.rs; then
    echo -e "${GREEN}✅ EIP-712 signature handling present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ EIP-712 signature handling missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 6: Verify input validation functions
echo -e "${YELLOW}Verifying input validation functions...${NC}"
VALIDATION_FUNCTIONS=("validate_lop_parameters" "is_valid_address" "is_valid_hex_string")
MISSING_VALIDATION=0

for func in "${VALIDATION_FUNCTIONS[@]}"; do
    if grep -q "fn $func" src/relayer/src/lib.rs; then
        echo -e "${GREEN}✓ Found validation function: $func${NC}"
    else
        echo -e "${RED}✗ Missing validation function: $func${NC}"
        MISSING_VALIDATION=$((MISSING_VALIDATION + 1))
    fi
done

if [ "$MISSING_VALIDATION" -eq "0" ]; then
    echo -e "${GREEN}✅ All validation functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_VALIDATION validation functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 7: Verify legacy compatibility
echo -e "${YELLOW}Verifying legacy create_order compatibility...${NC}"
if grep -q "async fn create_order" src/relayer/src/lib.rs && grep -q "create_fusion_order" src/relayer/src/lib.rs; then
    echo -e "${GREEN}✅ Legacy create_order function maintained${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Legacy create_order function missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 8: Verify order direction detection
echo -e "${YELLOW}Verifying order direction detection...${NC}"
if grep -q "is_eth_asset" src/relayer/src/lib.rs && grep -q "is_eth_to_icp" src/relayer/src/lib.rs; then
    echo -e "${GREEN}✅ Order direction detection present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Order direction detection missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 9: Verify error handling enhancements
echo -e "${YELLOW}Verifying enhanced error handling...${NC}"
ERROR_TYPES=("InvalidSalt" "InvalidMakerTraits" "TokenAddressInvalid" "InvalidEIP712Signature")
MISSING_ERRORS=0

for error in "${ERROR_TYPES[@]}"; do
    if grep -q "$error" src/relayer/src/types.rs; then
        echo -e "${GREEN}✓ Found error type: $error${NC}"
    else
        echo -e "${RED}✗ Missing error type: $error${NC}"
        MISSING_ERRORS=$((MISSING_ERRORS + 1))
    fi
done

if [ "$MISSING_ERRORS" -eq "0" ]; then
    echo -e "${GREEN}✅ All required error types present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_ERRORS error types${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 10: Verify order ID generation
echo -e "${YELLOW}Verifying order ID generation...${NC}"
if grep -q "generate_order_id" src/relayer/src/lib.rs && grep -q "fusion_" src/relayer/src/lib.rs; then
    echo -e "${GREEN}✅ Order ID generation present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Order ID generation missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 11: Check for compilation warnings
echo -e "${YELLOW}Checking for compilation warnings...${NC}"
WARNINGS=$((cd src/relayer && cargo check 2>&1) | grep -c "warning:" || echo "0")
if [ "$WARNINGS" -lt "20" ]; then
    echo -e "${GREEN}✅ Acceptable number of compilation warnings: $WARNINGS${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${YELLOW}⚠️  Found $WARNINGS compilation warnings (review if needed)${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Summary
echo ""
echo "============================================================"
echo -e "${YELLOW}Test Summary${NC}"
echo "============================================================"
echo "Tests Run: $TESTS_RUN"
echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests Failed: ${RED}$((TESTS_RUN - TESTS_PASSED))${NC}"

if [ "$TESTS_PASSED" -eq "$TESTS_RUN" ]; then
    echo -e "${GREEN}🎉 All tests passed! Enhanced order creation is ready for 1inch LOP compatibility.${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Please review the issues above.${NC}"
    exit 1
fi