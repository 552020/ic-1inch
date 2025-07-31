#!/bin/bash
# run-all-tests.sh - Run complete test suite

set -e  # Exit on any error

echo "🧪 Running complete IC-1inch test suite..."
echo "🔄 HTLC Escrows + Limit Order Protocol"
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
    if ./scripts/$1; then
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

echo "🚀 Starting IC-1inch test suite..."
echo "📋 Test environment: Local network"
echo "📋 Components: HTLC Escrows + Limit Order Protocol"
echo "📋 Date: $(date)"

# Phase 1: Setup
echo -e "\n${YELLOW}Phase 1: Environment Setup${NC}"
run_test "setup-testing.sh"

# Phase 2: Basic Tests
echo -e "\n${YELLOW}Phase 2: Basic Functionality Tests${NC}"
run_test "test-basic.sh"

# Phase 3: Core Logic Tests
echo -e "\n${YELLOW}Phase 3: Core Logic Tests${NC}"
run_test "test-escrow-lifecycle.sh"

# Phase 4: Error Handling Tests
echo -e "\n${YELLOW}Phase 4: Error Handling Tests${NC}"
run_test "test-error-scenarios.sh"

# Phase 5: Limit Order Protocol Tests
echo -e "\n${YELLOW}Phase 5: Limit Order Protocol Tests${NC}"
run_test "test-limit-orders.sh"
run_test "test-limit-order-errors.sh"

# Final Summary
echo -e "\n${YELLOW}========================================"
echo "🎉 Test Suite Summary"
echo "========================================"
echo -e "${NC}"

echo "📊 Test Results:"
echo "  ✅ Environment setup"
echo "  ✅ Basic functionality"
echo "  ✅ Escrow lifecycle"
echo "  ✅ Error handling"
echo "  ✅ Limit order lifecycle"
echo "  ✅ Limit order error scenarios"

echo -e "\n${GREEN}🎯 All tests completed successfully!${NC}"
echo -e "${YELLOW}📋 Next steps:${NC}"
echo "  1. Review test output for any warnings"
echo "  2. Implement limit order core functions (A4, A5, A6)"
echo "  3. Deploy to testnet for integration testing"
echo "  4. Add ICRC-1 token transfers and ChainFusion+ features"

echo -e "\n${GREEN}🚀 IC-1inch platform is ready for MVP development!${NC}" 