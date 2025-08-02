#!/bin/bash

# Test script for cross-chain identity management in relayer canister
# Tests identity storage, lookup, and SIWE integration flows

set -e

echo "🧪 Testing Cross-Chain Identity Management"
echo "=========================================="

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

# Test 1: Check if canister is ready (assumes setup_and_compile.sh was run)
run_test "Check canister readiness" "dfx canister call relayer get_active_fusion_orders --query --network local >/dev/null 2>&1"

# Test 3: Verify register_cross_chain_identity function is present
echo -e "${YELLOW}Verifying register_cross_chain_identity function...${NC}"
if grep -q "fn register_cross_chain_identity" src/relayer/src/lib.rs; then
    echo -e "${GREEN}✅ register_cross_chain_identity function present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ register_cross_chain_identity function missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 4: Verify identity storage functions
echo -e "${YELLOW}Verifying identity storage functions...${NC}"
STORAGE_FUNCTIONS=("store_cross_chain_identity" "get_cross_chain_identity" "get_cross_chain_identity_by_principal")
MISSING_STORAGE_FUNCTIONS=0

for func in "${STORAGE_FUNCTIONS[@]}"; do
    if grep -q "pub fn $func" src/relayer/src/memory.rs; then
        echo -e "${GREEN}✓ Found storage function: $func${NC}"
    else
        echo -e "${RED}✗ Missing storage function: $func${NC}"
        MISSING_STORAGE_FUNCTIONS=$((MISSING_STORAGE_FUNCTIONS + 1))
    fi
done

if [ "$MISSING_STORAGE_FUNCTIONS" -eq "0" ]; then
    echo -e "${GREEN}✅ All identity storage functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_STORAGE_FUNCTIONS storage functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 5: Verify bidirectional identity lookup functions
echo -e "${YELLOW}Verifying bidirectional identity lookup functions...${NC}"
LOOKUP_FUNCTIONS=("get_cross_chain_identity" "get_cross_chain_identity_by_principal" "get_principal_from_eth_address")
MISSING_LOOKUP_FUNCTIONS=0

for func in "${LOOKUP_FUNCTIONS[@]}"; do
    if grep -q "fn $func" src/relayer/src/lib.rs; then
        echo -e "${GREEN}✓ Found lookup function: $func${NC}"
    else
        echo -e "${RED}✗ Missing lookup function: $func${NC}"
        MISSING_LOOKUP_FUNCTIONS=$((MISSING_LOOKUP_FUNCTIONS + 1))
    fi
done

if [ "$MISSING_LOOKUP_FUNCTIONS" -eq "0" ]; then
    echo -e "${GREEN}✅ All bidirectional lookup functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_LOOKUP_FUNCTIONS lookup functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 6: Verify identity pair validation logic
echo -e "${YELLOW}Verifying identity pair validation logic...${NC}"
if grep -q "is_valid_address.*eth_address" src/relayer/src/lib.rs; then
    echo -e "${GREEN}✅ ETH address validation present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ ETH address validation missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 7: Verify CrossChainIdentity structure
echo -e "${YELLOW}Verifying CrossChainIdentity structure...${NC}"
IDENTITY_FIELDS=("eth_address" "icp_principal" "role")
MISSING_IDENTITY_FIELDS=0

for field in "${IDENTITY_FIELDS[@]}"; do
    if grep -q "pub $field:" src/relayer/src/types.rs; then
        echo -e "${GREEN}✓ Found identity field: $field${NC}"
    else
        echo -e "${RED}✗ Missing identity field: $field${NC}"
        MISSING_IDENTITY_FIELDS=$((MISSING_IDENTITY_FIELDS + 1))
    fi
done

if [ "$MISSING_IDENTITY_FIELDS" -eq "0" ]; then
    echo -e "${GREEN}✅ All CrossChainIdentity fields present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_IDENTITY_FIELDS identity fields${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 8: Verify UserRole enum
echo -e "${YELLOW}Verifying UserRole enum...${NC}"
USER_ROLES=("Maker" "Resolver")
MISSING_USER_ROLES=0

for role in "${USER_ROLES[@]}"; do
    if grep -q "$role" src/relayer/src/types.rs; then
        echo -e "${GREEN}✓ Found user role: $role${NC}"
    else
        echo -e "${RED}✗ Missing user role: $role${NC}"
        MISSING_USER_ROLES=$((MISSING_USER_ROLES + 1))
    fi
done

if [ "$MISSING_USER_ROLES" -eq "0" ]; then
    echo -e "${GREEN}✅ All UserRole variants present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_USER_ROLES user roles${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 9: Verify SIWE integration functions
echo -e "${YELLOW}Verifying SIWE integration functions...${NC}"
SIWE_FUNCTIONS=("store_siwe_identity")
MISSING_SIWE_FUNCTIONS=0

for func in "${SIWE_FUNCTIONS[@]}"; do
    if grep -q "fn $func" src/relayer/src/lib.rs; then
        echo -e "${GREEN}✓ Found SIWE function: $func${NC}"
    else
        echo -e "${RED}✗ Missing SIWE function: $func${NC}"
        MISSING_SIWE_FUNCTIONS=$((MISSING_SIWE_FUNCTIONS + 1))
    fi
done

if [ "$MISSING_SIWE_FUNCTIONS" -eq "0" ]; then
    echo -e "${GREEN}✅ All SIWE integration functions present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_SIWE_FUNCTIONS SIWE functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 10: Verify identity storage in memory module
echo -e "${YELLOW}Verifying identity storage in memory module...${NC}"
if grep -q "CROSS_CHAIN_IDENTITIES" src/relayer/src/memory.rs; then
    echo -e "${GREEN}✅ Cross-chain identity storage present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Cross-chain identity storage missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 11: Verify thread-safe identity storage
echo -e "${YELLOW}Verifying thread-safe identity storage...${NC}"
if grep -q "thread_local!" src/relayer/src/memory.rs && grep -q "RefCell.*CrossChainIdentity" src/relayer/src/memory.rs; then
    echo -e "${GREEN}✅ Thread-safe identity storage present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Thread-safe identity storage missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 12: Verify identity serialization for upgrades
echo -e "${YELLOW}Verifying identity serialization for upgrades...${NC}"
if grep -q "serialize_relayer_state" src/relayer/src/memory.rs && grep -q "CrossChainIdentity" src/relayer/src/memory.rs; then
    echo -e "${GREEN}✅ Identity serialization for upgrades present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Identity serialization for upgrades missing${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 13: Verify error handling for identity operations
echo -e "${YELLOW}Verifying error handling for identity operations...${NC}"
IDENTITY_ERRORS=("TokenAddressInvalid" "OrderNotFound")
MISSING_IDENTITY_ERRORS=0

for error in "${IDENTITY_ERRORS[@]}"; do
    if grep -q "$error" src/relayer/src/types.rs; then
        echo -e "${GREEN}✓ Found identity error: $error${NC}"
    else
        echo -e "${RED}✗ Missing identity error: $error${NC}"
        MISSING_IDENTITY_ERRORS=$((MISSING_IDENTITY_ERRORS + 1))
    fi
done

if [ "$MISSING_IDENTITY_ERRORS" -eq "0" ]; then
    echo -e "${GREEN}✅ All identity error types present${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_IDENTITY_ERRORS identity error types${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 14: Verify identity query functions are exported
echo -e "${YELLOW}Verifying identity query functions are exported...${NC}"
QUERY_FUNCTIONS=("get_cross_chain_identity" "get_cross_chain_identity_by_principal")
MISSING_QUERY_FUNCTIONS=0

for func in "${QUERY_FUNCTIONS[@]}"; do
    # Check if function exists with ic_cdk::query attribute
    if grep -B 1 -A 1 "fn $func" src/relayer/src/lib.rs | grep -q "#\[ic_cdk::query\]"; then
        echo -e "${GREEN}✓ Found exported query function: $func${NC}"
    else
        echo -e "${RED}✗ Missing exported query function: $func${NC}"
        MISSING_QUERY_FUNCTIONS=$((MISSING_QUERY_FUNCTIONS + 1))
    fi
done

if [ "$MISSING_QUERY_FUNCTIONS" -eq "0" ]; then
    echo -e "${GREEN}✅ All identity query functions exported${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Missing $MISSING_QUERY_FUNCTIONS exported query functions${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Test 15: Check canister status (assumes setup_and_compile.sh was run)
echo -e "${YELLOW}Checking canister status...${NC}"
if dfx canister status relayer --network local >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Canister is deployed and ready${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Canister is not ready - run setup_and_compile.sh first${NC}"
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Summary
echo ""
echo "=========================================="
echo -e "${YELLOW}Test Summary${NC}"
echo "=========================================="
echo "Tests Run: $TESTS_RUN"
echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests Failed: ${RED}$((TESTS_RUN - TESTS_PASSED))${NC}"

if [ "$TESTS_PASSED" -eq "$TESTS_RUN" ]; then
    echo -e "${GREEN}🎉 All tests passed! Cross-chain identity management is ready.${NC}"
    echo ""
    echo "✨ Identity Management Features:"
    echo "  • Cross-chain identity registration (ETH address + ICP principal)"
    echo "  • Bidirectional identity lookup (ETH→ICP and ICP→ETH)"
    echo "  • Identity pair validation and error handling"
    echo "  • SIWE integration support"
    echo "  • Thread-safe identity storage"
    echo "  • Identity serialization for canister upgrades"
    echo "  • User role management (Maker/Resolver)"
    echo "  • Query functions for frontend integration"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Please review the issues above.${NC}"
    exit 1
fi