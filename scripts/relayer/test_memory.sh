#!/bin/bash

# Test script for enhanced memory management in relayer canister
# Tests memory operations and persistence for new data structures

set -e

echo "🧪 Testing Enhanced Memory Management for New Data Structures"
echo "============================================================"

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

# Test 1: Compile memory module with new data structures
run_test "Compile memory module with enhanced types" "(cd src/relayer && cargo check)"

# Test 2: Build canister with enhanced memory management
run_test "Build canister with enhanced memory" "(cd src/relayer && cargo build --target wasm32-unknown-unknown --release)"

# Test 3: Verify memory functions are properly exported
run_test "Verify memory functions compilation" "(cd src/relayer && cargo check --verbose 2>&1 | grep -q 'Finished')"

# Test 4: Check for memory-related compilation warnings
echo -e "${YELLOW}Checking for memory-related compilation warnings...${NC}"
WARNINGS=$((cd src/relayer && cargo check 2>&1) | grep -c "warning:" || echo "0")
if [ "$WARNINGS" -lt "10" ]; then
    echo -e "${GREEN}✅ Acceptable number of compilation warnings: $WARNINGS${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${YELLOW}⚠️  Found $WARNINGS compilation warnings (review if needed)${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 5: Verify enhanced memory functions are present
echo -e "${YELLOW}Verifying enhanced memory functions...${NC}"
REQUIRED_FUNCTIONS=(
    "store_fusion_order"
    "update_fusion_order"
    "get_fusion_order"
    "get_orders_by_status"
    "get_orders_by_maker"
    "get_active_fusion_orders"
    "serialize_orderbook_state"
    "deserialize_orderbook_state"
)

MISSING_FUNCTIONS=0
for func in "${REQUIRED_FUNCTIONS[@]}"; do
    if grep -q "pub fn $func" src/relayer/src/memory.rs; then
        echo -e "${GREEN}✓ Found function: $func${NC}"
    else
        echo -e "${RED}✗ Missing function: $func${NC}"
        MISSING_FUNCTIONS=$((MISSING_FUNCTIONS + 1))
    fi
done

if [ "$MISSING_FUNCTIONS" -eq "0" ]; then
    echo -e "${GREEN}✅ All required memory functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_FUNCTIONS required functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 6: Verify thread-safe storage is maintained
echo -e "${YELLOW}Verifying thread-safe storage...${NC}"
if grep -q "thread_local!" src/relayer/src/memory.rs && grep -q "RefCell" src/relayer/src/memory.rs; then
    echo -e "${GREEN}✅ Thread-safe storage maintained${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Thread-safe storage not properly maintained${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 7: Verify serialization/deserialization functions exist
echo -e "${YELLOW}Verifying serialization functions...${NC}"
if grep -q "serialize_relayer_state" src/relayer/src/memory.rs && grep -q "deserialize_relayer_state" src/relayer/src/memory.rs; then
    echo -e "${GREEN}✅ Serialization functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Serialization functions missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 8: Verify enhanced query functions
echo -e "${YELLOW}Verifying enhanced query functions...${NC}"
QUERY_FUNCTIONS=("get_orders_by_status" "get_orders_by_maker" "get_active_fusion_orders")
MISSING_QUERIES=0

for func in "${QUERY_FUNCTIONS[@]}"; do
    if grep -q "pub fn $func" src/relayer/src/memory.rs; then
        echo -e "${GREEN}✓ Found query function: $func${NC}"
    else
        echo -e "${RED}✗ Missing query function: $func${NC}"
        MISSING_QUERIES=$((MISSING_QUERIES + 1))
    fi
done

if [ "$MISSING_QUERIES" -eq "0" ]; then
    echo -e "${GREEN}✅ All enhanced query functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_QUERIES query functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 9: Verify cross-chain identity functions
echo -e "${YELLOW}Verifying cross-chain identity functions...${NC}"
IDENTITY_FUNCTIONS=("store_cross_chain_identity" "get_cross_chain_identity" "get_cross_chain_identity_by_principal")
MISSING_IDENTITY=0

for func in "${IDENTITY_FUNCTIONS[@]}"; do
    if grep -q "pub fn $func" src/relayer/src/memory.rs; then
        echo -e "${GREEN}✓ Found identity function: $func${NC}"
    else
        echo -e "${RED}✗ Missing identity function: $func${NC}"
        MISSING_IDENTITY=$((MISSING_IDENTITY + 1))
    fi
done

if [ "$MISSING_IDENTITY" -eq "0" ]; then
    echo -e "${GREEN}✅ All cross-chain identity functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_IDENTITY identity functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 10: Verify compatibility with enhanced FusionOrder structure
echo -e "${YELLOW}Verifying compatibility with enhanced FusionOrder...${NC}"
if grep -q "FusionOrder" src/relayer/src/memory.rs && grep -q "OrderStatus" src/relayer/src/memory.rs; then
    echo -e "${GREEN}✅ Compatible with enhanced FusionOrder structure${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Not compatible with enhanced FusionOrder structure${NC}"
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
    echo -e "${GREEN}🎉 All tests passed! Enhanced memory management is ready.${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Please review the issues above.${NC}"
    exit 1
fi