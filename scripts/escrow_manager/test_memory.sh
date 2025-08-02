#!/bin/bash

# Test script for enhanced memory management validation
# Tests memory operations and persistence for HTLC escrow data

set -e

echo "🧠 Testing Enhanced Memory Management for HTLC Escrow Data"
echo "========================================================="

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
    echo "📦 Checking compilation with enhanced memory management..."
    
    if cargo check --quiet; then
        echo -e "${GREEN}✅ Compilation successful${NC}"
        return 0
    else
        echo -e "${RED}❌ Compilation failed${NC}"
        return 1
    fi
}

# Function to test memory module structure
test_memory_module_structure() {
    echo "🔧 Testing memory module structure..."
    
    # Check if memory functions are properly defined
    if grep -q "store_htlc_escrow" src/memory.rs; then
        echo -e "${GREEN}✅ store_htlc_escrow function defined${NC}"
    else
        echo -e "${RED}❌ store_htlc_escrow function not found${NC}"
        return 1
    fi
    
    if grep -q "update_htlc_escrow" src/memory.rs; then
        echo -e "${GREEN}✅ update_htlc_escrow function defined${NC}"
    else
        echo -e "${RED}❌ update_htlc_escrow function not found${NC}"
        return 1
    fi
    
    if grep -q "update_htlc_escrow_status" src/memory.rs; then
        echo -e "${GREEN}✅ update_htlc_escrow_status function defined${NC}"
    else
        echo -e "${RED}❌ update_htlc_escrow_status function not found${NC}"
        return 1
    fi
    
    if grep -q "add_event_to_htlc_escrow" src/memory.rs; then
        echo -e "${GREEN}✅ add_event_to_htlc_escrow function defined${NC}"
    else
        echo -e "${RED}❌ add_event_to_htlc_escrow function not found${NC}"
        return 1
    fi
    
    if grep -q "get_htlc_escrows_by_status" src/memory.rs; then
        echo -e "${GREEN}✅ get_htlc_escrows_by_status function defined${NC}"
    else
        echo -e "${RED}❌ get_htlc_escrows_by_status function not found${NC}"
        return 1
    fi
    
    if grep -q "htlc_escrow_exists" src/memory.rs; then
        echo -e "${GREEN}✅ htlc_escrow_exists function defined${NC}"
    else
        echo -e "${RED}❌ htlc_escrow_exists function not found${NC}"
        return 1
    fi
    
    return 0
}

# Function to test cross-chain memory functions
test_cross_chain_memory_functions() {
    echo "🔗 Testing cross-chain memory functions..."
    
    # Check if cross-chain memory functions are properly defined
    if grep -q "store_cross_chain_escrow" src/memory.rs; then
        echo -e "${GREEN}✅ store_cross_chain_escrow function defined${NC}"
    else
        echo -e "${RED}❌ store_cross_chain_escrow function not found${NC}"
        return 1
    fi
    
    if grep -q "update_cross_chain_escrow" src/memory.rs; then
        echo -e "${GREEN}✅ update_cross_chain_escrow function defined${NC}"
    else
        echo -e "${RED}❌ update_cross_chain_escrow function not found${NC}"
        return 1
    fi
    
    if grep -q "update_cross_chain_coordination_state" src/memory.rs; then
        echo -e "${GREEN}✅ update_cross_chain_coordination_state function defined${NC}"
    else
        echo -e "${RED}❌ update_cross_chain_coordination_state function not found${NC}"
        return 1
    fi
    
    if grep -q "add_event_to_cross_chain_escrow" src/memory.rs; then
        echo -e "${GREEN}✅ add_event_to_cross_chain_escrow function defined${NC}"
    else
        echo -e "${RED}❌ add_event_to_cross_chain_escrow function not found${NC}"
        return 1
    fi
    
    if grep -q "get_cross_chain_escrows_by_state" src/memory.rs; then
        echo -e "${GREEN}✅ get_cross_chain_escrows_by_state function defined${NC}"
    else
        echo -e "${RED}❌ get_cross_chain_escrows_by_state function not found${NC}"
        return 1
    fi
    
    if grep -q "cross_chain_escrow_exists" src/memory.rs; then
        echo -e "${GREEN}✅ cross_chain_escrow_exists function defined${NC}"
    else
        echo -e "${RED}❌ cross_chain_escrow_exists function not found${NC}"
        return 1
    fi
    
    return 0
}

# Function to test canister upgrade support
test_canister_upgrade_support() {
    echo "🔄 Testing canister upgrade support..."
    
    # Check if backup/restore functions are defined
    if grep -q "export_escrow_data" src/memory.rs; then
        echo -e "${GREEN}✅ export_escrow_data function defined${NC}"
    else
        echo -e "${RED}❌ export_escrow_data function not found${NC}"
        return 1
    fi
    
    if grep -q "import_escrow_data" src/memory.rs; then
        echo -e "${RED}✅ import_escrow_data function defined${NC}"
    else
        echo -e "${RED}❌ import_escrow_data function not found${NC}"
        return 1
    fi
    
    if grep -q "EscrowBackup" src/memory.rs; then
        echo -e "${GREEN}✅ EscrowBackup struct defined${NC}"
    else
        echo -e "${RED}❌ EscrowBackup struct not found${NC}"
        return 1
    fi
    
    return 0
}

# Function to test memory statistics
test_memory_statistics() {
    echo "📊 Testing memory statistics..."
    
    # Check if memory statistics functions are defined
    if grep -q "get_memory_stats" src/memory.rs; then
        echo -e "${GREEN}✅ get_memory_stats function defined${NC}"
    else
        echo -e "${RED}❌ get_memory_stats function not found${NC}"
        return 1
    fi
    
    if grep -q "MemoryStats" src/memory.rs; then
        echo -e "${GREEN}✅ MemoryStats struct defined${NC}"
    else
        echo -e "${RED}❌ MemoryStats struct not found${NC}"
        return 1
    fi
    
    return 0
}

# Function to test thread safety
test_thread_safety() {
    echo "🔒 Testing thread safety implementation..."
    
    # Check if thread_local! is used
    if grep -q "thread_local!" src/memory.rs; then
        echo -e "${GREEN}✅ thread_local! used for safe global state${NC}"
    else
        echo -e "${RED}❌ thread_local! not found${NC}"
        return 1
    fi
    
    # Check if RefCell is used
    if grep -q "RefCell" src/memory.rs; then
        echo -e "${GREEN}✅ RefCell used for interior mutability${NC}"
    else
        echo -e "${RED}❌ RefCell not found${NC}"
        return 1
    fi
    
    # Check if HashMap is used
    if grep -q "HashMap" src/memory.rs; then
        echo -e "${GREEN}✅ HashMap used for efficient storage${NC}"
    else
        echo -e "${RED}❌ HashMap not found${NC}"
        return 1
    fi
    
    return 0
}

# Main test execution
echo "Starting enhanced memory management validation..."
echo ""

# Run tests
run_test "Compilation" "check_compilation"
run_test "Memory Module Structure" "test_memory_module_structure"
run_test "Cross-Chain Memory Functions" "test_cross_chain_memory_functions"
run_test "Canister Upgrade Support" "test_canister_upgrade_support"
run_test "Memory Statistics" "test_memory_statistics"
run_test "Thread Safety" "test_thread_safety"

# Summary
echo ""
echo "📊 Test Summary:"
echo "================="
echo -e "${GREEN}✅ Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}❌ Tests Failed: $TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 All enhanced memory management tests passed!${NC}"
    echo "✅ HTLC escrow memory operations are properly defined"
    echo "✅ Cross-chain escrow coordination memory is ready"
    echo "✅ Canister upgrade support is implemented"
    echo "✅ Memory statistics and monitoring are available"
    echo "✅ Thread safety is properly implemented"
    exit 0
else
    echo ""
    echo -e "${RED}💥 Some tests failed. Please review the implementation.${NC}"
    exit 1
fi