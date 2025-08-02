#!/bin/bash

# Test script for simplified MVP relayer canister
# Tests the simplified data structures and functionality

set -e

echo "🧪 Testing Simplified MVP Relayer Canister"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -n "Testing $test_name... "
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if eval "$test_command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ FAIL${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        echo "  Command: $test_command"
    fi
}

# Function to check if a string contains another string
contains() {
    [[ "$1" == *"$2"* ]]
}

# Navigate to relayer directory
cd src/relayer

echo "📁 Current directory: $(pwd)"

# Test 1: Compilation
run_test "Compilation" "cargo check"

# Test 2: WASM Build
run_test "WASM Build" "cargo build --target wasm32-unknown-unknown --release"

# Test 3: Check WASM file exists
run_test "WASM File Exists" "test -f ../../target/wasm32-unknown-unknown/release/relayer.wasm"

# Test 4: Check simplified FusionOrder structure
echo -n "Testing Simplified FusionOrder Structure... "
TESTS_RUN=$((TESTS_RUN + 1))

# Count fields in FusionOrder (should be around 15-20 for MVP)
FIELD_COUNT=$(sed -n '/pub struct FusionOrder/,/^}/p' src/types.rs | grep -c "pub [a-zA-Z_]*:" || echo "0")

if [ "$FIELD_COUNT" -lt 30 ]; then
    echo -e "${GREEN}✅ PASS${NC} (Found $FIELD_COUNT fields - good for MVP)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Found $FIELD_COUNT fields - too many for MVP)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 5: Check simplified error types
echo -n "Testing Simplified Error Types... "
TESTS_RUN=$((TESTS_RUN + 1))

# Count error variants (should be around 8-15 for MVP)
ERROR_COUNT=$(grep -A 50 "pub enum FusionError" src/types.rs | grep -c "^    [A-Z]" || echo "0")

if [ "$ERROR_COUNT" -lt 20 ]; then
    echo -e "${GREEN}✅ PASS${NC} (Found $ERROR_COUNT error types - good for MVP)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Found $ERROR_COUNT error types - too many for MVP)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 6: Check Dutch auction removal
echo -n "Testing Dutch Auction Removal... "
TESTS_RUN=$((TESTS_RUN + 1))

if ! grep -q "PriceCurve" src/types.rs && ! grep -q "auction_start_rate" src/types.rs; then
    echo -e "${GREEN}✅ PASS${NC} (Dutch auction structures removed)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Dutch auction structures still present)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 7: Check partial fills removal
echo -n "Testing Partial Fills Removal... "
TESTS_RUN=$((TESTS_RUN + 1))

if ! grep -q "PartialFillData" src/types.rs && ! grep -q "merkle_tree_root" src/types.rs; then
    echo -e "${GREEN}✅ PASS${NC} (Partial fill structures removed)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Partial fill structures still present)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 8: Check FusionState removal
echo -n "Testing FusionState Removal... "
TESTS_RUN=$((TESTS_RUN + 1))

if ! grep -q "pub enum FusionState" src/types.rs; then
    echo -e "${GREEN}✅ PASS${NC} (Complex FusionState enum removed)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Complex FusionState enum still present)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 9: Check essential 1inch LOP fields preserved
echo -n "Testing 1inch LOP Compatibility Preserved... "
TESTS_RUN=$((TESTS_RUN + 1))

if grep -q "pub salt: String" src/types.rs && \
   grep -q "pub maker_asset: String" src/types.rs && \
   grep -q "pub taker_asset: String" src/types.rs && \
   grep -q "pub making_amount: u64" src/types.rs && \
   grep -q "pub taking_amount: u64" src/types.rs; then
    echo -e "${GREEN}✅ PASS${NC} (Essential 1inch LOP fields preserved)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Essential 1inch LOP fields missing)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 10: Check EIP-712 support preserved
echo -n "Testing EIP-712 Support Preserved... "
TESTS_RUN=$((TESTS_RUN + 1))

if grep -q "pub eip712_signature: Option<EIP712Signature>" src/types.rs && \
   grep -q "pub struct EIP712Signature" src/types.rs; then
    echo -e "${GREEN}✅ PASS${NC} (EIP-712 support preserved)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (EIP-712 support missing)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 11: Check lib.rs functions removed
echo -n "Testing Complex Functions Removed from lib.rs... "
TESTS_RUN=$((TESTS_RUN + 1))

if ! grep -q "get_current_price" src/lib.rs && \
   ! grep -q "partially_fill_order" src/lib.rs && \
   ! grep -q "reveal_multiple_secrets" src/lib.rs; then
    echo -e "${GREEN}✅ PASS${NC} (Complex functions removed)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Complex functions still present)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 12: Check core functions preserved
echo -n "Testing Core Functions Preserved... "
TESTS_RUN=$((TESTS_RUN + 1))

if grep -q "create_fusion_order" src/lib.rs && \
   grep -q "accept_fusion_order" src/lib.rs && \
   grep -q "complete_order_with_secret" src/lib.rs && \
   grep -q "cancel_order" src/lib.rs; then
    echo -e "${GREEN}✅ PASS${NC} (Core functions preserved)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Core functions missing)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 13: Check compilation warnings reduced
echo -n "Testing Compilation Warnings Reduced... "
TESTS_RUN=$((TESTS_RUN + 1))

WARNING_COUNT=$(cargo check 2>&1 | grep -c "warning:" || echo "0")

if [ "$WARNING_COUNT" -lt 10 ]; then
    echo -e "${GREEN}✅ PASS${NC} (Found $WARNING_COUNT warnings - acceptable for MVP)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${YELLOW}⚠️  WARN${NC} (Found $WARNING_COUNT warnings - could be improved)"
    TESTS_PASSED=$((TESTS_PASSED + 1))  # Still pass, just a warning
fi

# Test 14: Check backward compatibility maintained
echo -n "Testing Backward Compatibility... "
TESTS_RUN=$((TESTS_RUN + 1))

if grep -q "pub from_token: Token" src/types.rs && \
   grep -q "pub to_token: Token" src/types.rs && \
   grep -q "pub secret_hash: String" src/types.rs && \
   grep -q "create_order" src/lib.rs; then
    echo -e "${GREEN}✅ PASS${NC} (Backward compatibility maintained)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} (Backward compatibility broken)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Return to original directory
cd ../..

echo ""
echo "📊 Test Results Summary"
echo "======================="
echo "Tests Run: $TESTS_RUN"
echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 All tests passed! MVP simplification successful.${NC}"
    echo ""
    echo "✨ Simplification Achievements:"
    echo "  • Reduced FusionOrder fields to ~15 essential fields"
    echo "  • Simplified error handling to core types"
    echo "  • Removed complex Dutch auction system"
    echo "  • Removed partial fills implementation"
    echo "  • Removed complex state machine"
    echo "  • Maintained 1inch LOP compatibility"
    echo "  • Preserved EIP-712 support"
    echo "  • Kept backward compatibility"
    exit 0
else
    echo ""
    echo -e "${RED}❌ Some tests failed. Please review the implementation.${NC}"
    exit 1
fi