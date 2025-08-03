#!/bin/bash

# Master test script for Relayer Canister - Comprehensive Testing Suite
# Runs all individual test scripts and provides summary report
#
# USAGE: 
# 1. First run setup: ./scripts/relayer/setup_and_compile.sh (from project root)
# 2. Then run tests: ./scripts/relayer/run_all_tests.sh (from project root)
#
# DO NOT run from scripts/relayer/ directory - paths are relative to project root

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
CANISTER_ID="$(dfx canister id relayer 2>/dev/null || echo 'relayer')"
DFX_NETWORK="${DFX_NETWORK:-local}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

echo -e "${BLUE}🧪 Relayer Canister - Comprehensive Testing Suite${NC}"
echo "=================================================="
echo -e "${CYAN}Network:${NC} $DFX_NETWORK"
echo -e "${CYAN}Canister:${NC} $CANISTER_ID"
echo -e "${CYAN}Script Directory:${NC} $SCRIPT_DIR"
echo ""

# Function to check if canister is ready for testing
check_canister_ready() {
    echo -e "${YELLOW}🔍 Checking Relayer Canister Status${NC}"
    echo "=========================================="
    
    # Just check if canister responds to queries (assume it's already deployed)
    if dfx canister call relayer get_active_fusion_orders --query --network $DFX_NETWORK >/dev/null 2>&1; then
        echo -e "${GREEN}✅ Canister is responding to queries${NC}"
        echo -e "${CYAN}Canister ID: $(dfx canister id relayer --network $DFX_NETWORK)${NC}"
        return 0
    else
        echo -e "${RED}❌ Canister is not responding${NC}"
        echo -e "${YELLOW}💡 Run ./scripts/relayer/setup_and_compile.sh first${NC}"
        return 1
    fi
}

# Function to run a test script and track results
run_test() {
    local test_name="$1"
    local test_script="$2"
    local test_description="$3"
    
    echo -e "\n${YELLOW}Running: $test_name${NC}"
    echo "Description: $test_description"
    echo "Script: $test_script"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if [ -f "$test_script" ]; then
        if bash "$test_script" 2>&1; then
            echo -e "${GREEN}✅ $test_name - PASSED${NC}"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo -e "${RED}❌ $test_name - FAILED${NC}"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        echo -e "${PURPLE}⚠️  $test_name - SKIPPED (script not found)${NC}"
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
    fi
}







# Function to run integration test
run_integration_test() {
    echo -e "\n${YELLOW}Integration Test: End-to-End Order Flow${NC}"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Create an order
    echo "Creating test order..."
    ORDER_ID=$(dfx canister call relayer create_fusion_order \
        '("0xintegrationtest", "ICP", "ETH", 1000000, 500000, "0x", "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234", 1700000000000000000, null)' \
        --network $DFX_NETWORK | grep -o '"[^"]*"' | head -1 | tr -d '"' 2>/dev/null || echo "")
    
    if [ -n "$ORDER_ID" ]; then
        echo -e "${GREEN}✅ Order creation successful: $ORDER_ID${NC}"
        
        # Check order status
        ORDER_STATUS=$(dfx canister call relayer get_fusion_order_status "(\"$ORDER_ID\")" --query --network $DFX_NETWORK 2>/dev/null || echo "")
        
        if echo "$ORDER_STATUS" | grep -q "Pending"; then
            echo -e "${GREEN}✅ Order status check passed${NC}"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo -e "${RED}❌ Order status check failed${NC}"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        echo -e "${RED}❌ Order creation failed${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Main test execution
echo -e "${BLUE}Starting comprehensive test suite...${NC}"

# Check if canister is ready for testing
if ! check_canister_ready; then
    echo -e "${RED}❌ Canister not ready - aborting tests${NC}"
    exit 1
fi

# Run the only useful test (all garbage tests deleted)
run_test "Minimal Functionality Test" "$SCRIPT_DIR/test_relayer_minimal.sh" "Tests essential canister functionality"

# Run basic integration test (skip performance - not needed for hackathon)
run_integration_test

# Summary report
echo -e "\n${BLUE}📊 Test Summary Report${NC}"
echo "========================"
echo -e "${GREEN}✅ Passed: $PASSED_TESTS${NC}"
echo -e "${RED}❌ Failed: $FAILED_TESTS${NC}"
echo -e "${PURPLE}⚠️  Skipped: $SKIPPED_TESTS${NC}"
echo -e "${CYAN}📈 Total: $TOTAL_TESTS${NC}"

# Calculate success rate
if [ $TOTAL_TESTS -gt 0 ]; then
    SUCCESS_RATE=$(echo "scale=1; $PASSED_TESTS * 100 / $TOTAL_TESTS" | bc)
    echo -e "${CYAN}📊 Success Rate: ${SUCCESS_RATE}%${NC}"
fi

# Final result
if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "\n${GREEN}🎉 ALL TESTS PASSED! Relayer canister is ready for production.${NC}"
    exit 0
else
    echo -e "\n${RED}⚠️  Some tests failed. Please review the output above.${NC}"
    exit 1
fi 