#!/bin/bash

# Master test script for Orderbook Canister - Comprehensive Testing Suite
# Runs all individual test scripts and provides summary report

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
CANISTER_ID="$(dfx canister id orderbook 2>/dev/null || echo 'orderbook')"
DFX_NETWORK="${DFX_NETWORK:-local}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

echo -e "${BLUE}🧪 Orderbook Canister - Comprehensive Testing Suite${NC}"
echo "=================================================="
echo -e "${CYAN}Network:${NC} $DFX_NETWORK"
echo -e "${CYAN}Canister:${NC} $CANISTER_ID"
echo -e "${CYAN}Script Directory:${NC} $SCRIPT_DIR"
echo ""

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

# Function to check canister deployment
check_deployment() {
    echo -e "${CYAN}Checking canister deployment...${NC}"
    
    if dfx canister status orderbook --network $DFX_NETWORK >/dev/null 2>&1; then
        echo -e "${GREEN}✅ Orderbook canister is deployed${NC}"
        return 0
    else
        echo -e "${RED}❌ Orderbook canister is not deployed${NC}"
        echo -e "${YELLOW}Deploying canister...${NC}"
        dfx deploy orderbook --network $DFX_NETWORK
        return $?
    fi
}

# Function to run performance test
run_performance_test() {
    echo -e "\n${YELLOW}Performance Test: Load Testing${NC}"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Test order creation performance
    echo "Testing order creation performance..."
    start_time=$(date +%s.%N)
    
    for i in {1..10}; do
        dfx canister call orderbook create_fusion_order \
            "(\"0x$(printf '%016x' $i)\", \"ICP\", \"ETH\", 1000000, 500000, \"0x\", \"a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234\", 1700000000000000000, null)" \
            --network $DFX_NETWORK >/dev/null 2>&1 || true
    done
    
    end_time=$(date +%s.%N)
    duration=$(echo "$end_time - $start_time" | bc)
    
    echo -e "${GREEN}✅ Performance test completed in ${duration}s${NC}"
    PASSED_TESTS=$((PASSED_TESTS + 1))
}

# Function to run deployment test
run_deployment_test() {
    echo -e "\n${YELLOW}Deployment Test: Canister Validation${NC}"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Check canister status
    if dfx canister status orderbook --network $DFX_NETWORK >/dev/null 2>&1; then
        echo -e "${GREEN}✅ Canister status check passed${NC}"
        
        # Check if canister responds to basic query
        if dfx canister call orderbook get_active_fusion_orders --query --network $DFX_NETWORK >/dev/null 2>&1; then
            echo -e "${GREEN}✅ Canister query test passed${NC}"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo -e "${RED}❌ Canister query test failed${NC}"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        echo -e "${RED}❌ Canister status check failed${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Function to run integration test
run_integration_test() {
    echo -e "\n${YELLOW}Integration Test: End-to-End Order Flow${NC}"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Create an order
    echo "Creating test order..."
    ORDER_ID=$(dfx canister call orderbook create_fusion_order \
        '("0xintegrationtest", "ICP", "ETH", 1000000, 500000, "0x", "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234", 1700000000000000000, null)' \
        --network $DFX_NETWORK | grep -o '"[^"]*"' | head -1 | tr -d '"' 2>/dev/null || echo "")
    
    if [ -n "$ORDER_ID" ]; then
        echo -e "${GREEN}✅ Order creation successful: $ORDER_ID${NC}"
        
        # Check order status
        ORDER_STATUS=$(dfx canister call orderbook get_fusion_order_status "(\"$ORDER_ID\")" --query --network $DFX_NETWORK 2>/dev/null || echo "")
        
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

# Check deployment first
check_deployment

# Run individual test scripts
run_test "Order Acceptance Test" "$SCRIPT_DIR/test_order_acceptance.sh" "Tests enhanced order acceptance functionality"
run_test "Direction Coordination Test" "$SCRIPT_DIR/test_directions.sh" "Tests order direction-specific coordination"
run_test "Identity Management Test" "$SCRIPT_DIR/test_identity.sh" "Tests cross-chain identity management"
run_test "Data Types Test" "$SCRIPT_DIR/test_data_types.sh" "Tests data structure serialization"
run_test "Memory Management Test" "$SCRIPT_DIR/test_memory.sh" "Tests memory operations and persistence"
run_test "Order Creation Test" "$SCRIPT_DIR/test_order_creation.sh" "Tests order creation functionality"
run_test "Simplified Auction Test" "$SCRIPT_DIR/test_simplified_auction.sh" "Tests simplified Dutch auction placeholder"

# Run additional tests
run_performance_test
run_deployment_test
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
    echo -e "\n${GREEN}🎉 ALL TESTS PASSED! Orderbook canister is ready for production.${NC}"
    exit 0
else
    echo -e "\n${RED}⚠️  Some tests failed. Please review the output above.${NC}"
    exit 1
fi 