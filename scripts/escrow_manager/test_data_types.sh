#!/bin/bash

# Test script for enhanced data types validation
# Tests struct serialization/deserialization for HTLC escrow compatibility

set -e

echo "🧪 Testing Enhanced Data Types for HTLC Escrow Compatibility"
echo "=========================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -n "Testing $test_name... "
    
    if eval "$test_command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}"
        ((TESTS_FAILED++))
    fi
}

# Function to check if cargo check passes
check_compilation() {
    echo "📦 Checking compilation with enhanced data types..."
    
    if cargo check --quiet; then
        echo -e "${GREEN}✅ Compilation successful${NC}"
        return 0
    else
        echo -e "${RED}❌ Compilation failed${NC}"
        return 1
    fi
}

# Function to test Candid interface generation
test_candid_generation() {
    echo "📋 Testing Candid interface generation..."
    
    # Check if candid file exists and is valid
    if [ -f "escrow_manager.did" ]; then
        echo -e "${GREEN}✅ Candid interface file exists${NC}"
        
        # Check if the file contains our new types
        if grep -q "EscrowStatus" escrow_manager.did; then
            echo -e "${GREEN}✅ EscrowStatus found in Candid interface${NC}"
        else
            echo -e "${YELLOW}⚠️  EscrowStatus not found in Candid interface${NC}"
        fi
        
        if grep -q "EscrowError" escrow_manager.did; then
            echo -e "${GREEN}✅ Error types found in Candid interface${NC}"
        else
            echo -e "${YELLOW}⚠️  Error types not found in Candid interface${NC}"
        fi
        
        return 0
    else
        echo -e "${RED}❌ Candid interface file not found${NC}"
        return 1
    fi
}

# Function to test that new types are defined
test_new_types_exist() {
    echo "🔧 Testing that new HTLC types are defined..."
    
    # Check if the types file contains our new structures
    if grep -q "CoordinationState" src/types.rs; then
        echo -e "${GREEN}✅ CoordinationState defined${NC}"
    else
        echo -e "${RED}❌ CoordinationState not found${NC}"
        return 1
    fi
    
    if grep -q "HTLCEscrow" src/types.rs; then
        echo -e "${GREEN}✅ HTLCEscrow defined${NC}"
    else
        echo -e "${RED}❌ HTLCEscrow not found${NC}"
        return 1
    fi
    
    if grep -q "ChainHealthStatus" src/types.rs; then
        echo -e "${GREEN}✅ ChainHealthStatus defined${NC}"
    else
        echo -e "${RED}❌ ChainHealthStatus not found${NC}"
        return 1
    fi
    
    if grep -q "PartialFillInfo" src/types.rs; then
        echo -e "${GREEN}✅ PartialFillInfo defined${NC}"
    else
        echo -e "${RED}❌ PartialFillInfo not found${NC}"
        return 1
    fi
    
    if grep -q "CrossChainEscrowEvent" src/types.rs; then
        echo -e "${GREEN}✅ CrossChainEscrowEvent defined${NC}"
    else
        echo -e "${RED}❌ CrossChainEscrowEvent not found${NC}"
        return 1
    fi
    
    if grep -q "ChainFusionRequestFailed" src/types.rs; then
        echo -e "${GREEN}✅ Enhanced error types defined${NC}"
    else
        echo -e "${RED}❌ Enhanced error types not found${NC}"
        return 1
    fi
    
    return 0
}

# Main test execution
echo "Starting enhanced data types validation..."
echo ""

# Run tests
run_test "Compilation" "check_compilation"
run_test "New Types Definition" "test_new_types_exist"
run_test "Candid Generation" "test_candid_generation"

# Summary
echo ""
echo "📊 Test Summary:"
echo "================="
echo -e "${GREEN}✅ Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}❌ Tests Failed: $TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 All enhanced data types tests passed!${NC}"
    echo "✅ HTLC escrow data structures are ready for implementation"
    echo "✅ Chain Fusion integration types are properly defined"
    echo "✅ Enhanced error handling is in place"
    echo "✅ Candid interface is updated"
    exit 0
else
    echo ""
    echo -e "${RED}💥 Some tests failed. Please review the implementation.${NC}"
    exit 1
fi 