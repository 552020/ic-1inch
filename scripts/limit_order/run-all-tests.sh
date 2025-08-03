#!/bin/bash
# run-all-tests.sh - Run complete LOP test suite

set -e  # Exit on any error

echo "🧪 Running complete LOP test suite..."
echo "🔄 Limit Order Protocol"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        exit 1
    fi
}

# Function to run test with status
run_test() {
    echo -e "\n${YELLOW}Running: $1${NC}"
    echo "----------------------------------------"
    if ./scripts/limit-order/$1; then
        print_status 0 "$1 completed successfully"
    else
        print_status 1 "$1 failed"
    fi
}

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo -e "${RED}❌ dfx.json not found. Please run this script from the project root.${NC}"
    exit 1
fi

# Check if dfx is installed
if ! command -v dfx &> /dev/null; then
    echo -e "${RED}❌ dfx not found. Please install dfx first.${NC}"
    exit 1
fi

# Check if dfx is running
echo "🔍 Checking dfx status..."
if ! dfx ping > /dev/null 2>&1; then
    echo -e "${RED}❌ dfx is not running!${NC}"
    echo -e "${YELLOW}💡 Please start dfx with: dfx start --background${NC}"
    exit 1
fi
echo -e "${GREEN}✅ dfx is running${NC}"

echo "🚀 Starting LOP test suite..."
echo "📋 Test environment: Local network"
echo "📋 Components: Limit Order Protocol"
echo "📋 Date: $(date)"

# Phase 1: Basic Tests
echo -e "\n${YELLOW}Phase 1: Basic Functionality Tests${NC}"
run_test "test-basic.sh"

# Phase 2: Order Creation Tests
echo -e "\n${YELLOW}Phase 2: Order Creation Tests${NC}"
run_test "test-order-creation.sh"

# Phase 3: Order Filling Tests
echo -e "\n${YELLOW}Phase 3: Order Filling Tests${NC}"
run_test "test-order-filling.sh"

# Phase 4: Order Cancellation Tests
echo -e "\n${YELLOW}Phase 4: Order Cancellation Tests${NC}"
run_test "test-order-cancellation.sh"

# Phase 5: Manual Tests (if available)
echo -e "\n${YELLOW}Phase 5: Manual Tests${NC}"
if [ -f "scripts/limit-order/test-limit-orders-manual.sh" ]; then
    run_test "test-limit-orders-manual.sh"
else
    echo -e "${YELLOW}⚠️  Manual test script not found, skipping...${NC}"
fi

# Final Summary
echo -e "\n${YELLOW}========================================"
echo "🎉 LOP Test Suite Summary"
echo "========================================"
echo -e "${NC}"

echo "📊 Test Results:"
echo "  ✅ Basic functionality"
echo "  ✅ Order creation"
echo "  ✅ Order filling"
echo "  ✅ Order cancellation"
echo "  ✅ Manual testing (if available)"

echo -e "\n${GREEN}🎯 All LOP tests completed successfully!${NC}"
echo -e "${YELLOW}📋 Next steps:${NC}"
echo "  1. Review test output for any warnings"
echo "  2. Test with real ICRC-1 tokens"
echo "  3. Deploy to testnet for integration testing"
echo "  4. Integrate with escrow_manager for cross-chain swaps"

echo -e "\n${GREEN}🚀 LOP is ready for MVP development!${NC}" 