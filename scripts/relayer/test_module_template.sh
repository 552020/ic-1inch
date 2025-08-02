#!/bin/bash

# Test module template for relayer canister tests
# This template can be sourced by other test scripts to avoid compilation duplication

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

# Function to check if compilation was successful
check_compilation() {
    if [ "$COMPILATION_SUCCESS" != "true" ]; then
        echo -e "${RED}❌ Compilation failed - cannot run tests${NC}"
        return 1
    fi
    return 0
}

# Function to get compilation info
get_compilation_info() {
    echo -e "${BLUE}📊 Compilation Info:${NC}"
    echo -e "  • Success: $COMPILATION_SUCCESS"
    echo -e "  • Time: ${COMPILATION_TIME}s"
    echo -e "  • Warnings: $WARNINGS_COUNT"
    echo -e "  • WASM: $WASM_FILE"
    echo ""
}

# Function to print test summary
print_test_summary() {
    echo "=============================================="
    echo "📊 Test Summary"
    echo "=============================================="
    echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
    echo -e "${RED}Tests Failed: $((TESTS_RUN - TESTS_PASSED))${NC}"
    echo -e "${YELLOW}Total Tests: $TESTS_RUN${NC}"
    
    if [ "$TESTS_PASSED" -eq "$TESTS_RUN" ]; then
        echo -e "${GREEN}🎉 All tests passed!${NC}"
        return 0
    else
        echo -e "${RED}❌ Some tests failed.${NC}"
        return 1
    fi
}

# Example test function (override in actual test modules)
run_tests() {
    echo -e "${BLUE}🧪 Running Test Module${NC}"
    echo "=============================================="
    
    # Check compilation first
    if ! check_compilation; then
        return 1
    fi
    
    # Show compilation info
    get_compilation_info
    
    # Add your specific tests here
    # Example:
    # run_test "Test 1" "echo 'test command here'"
    # run_test "Test 2" "echo 'another test command'"
    
    # Print summary
    print_test_summary
}

# Export functions for other scripts to use
export -f run_test
export -f check_compilation
export -f get_compilation_info
export -f print_test_summary
export -f run_tests 