#!/bin/bash

# Test script for timelock module validation
# Tests timelock module structure and integration

set -e

echo "⏰ Testing Timelock Module Organization"
echo "======================================"

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
    echo "📦 Checking compilation with timelock module..."
    
    if cargo check --quiet; then
        echo -e "${GREEN}✅ Compilation successful${NC}"
        return 0
    else
        echo -e "${RED}❌ Compilation failed${NC}"
        return 1
    fi
}

# Function to test timelock module structure
test_timelock_module_structure() {
    echo "🏗️ Testing timelock module structure..."
    
    # Check if timelock module file exists
    if [ -f "src/timelock.rs" ]; then
        echo -e "${GREEN}✅ timelock.rs module file exists${NC}"
    else
        echo -e "${RED}❌ timelock.rs module file not found${NC}"
        return 1
    fi
    
    # Check if module is imported in lib.rs
    if grep -q "mod timelock;" src/lib.rs; then
        echo -e "${GREEN}✅ timelock module imported in lib.rs${NC}"
    else
        echo -e "${RED}❌ timelock module not imported in lib.rs${NC}"
        return 1
    fi
    
    return 0
}

# Function to test timelock constants
test_timelock_constants() {
    echo "🔢 Testing timelock constants..."
    
    # Check if constants module exists
    if grep -q "pub mod constants" src/timelock.rs; then
        echo -e "${GREEN}✅ Constants module defined${NC}"
    else
        echo -e "${RED}❌ Constants module not found${NC}"
        return 1
    fi
    
    # Check if buffer constants are defined
    if grep -q "BUFFER_MINUTES.*3" src/timelock.rs; then
        echo -e "${GREEN}✅ 3-minute buffer constant defined${NC}"
    else
        echo -e "${RED}❌ 3-minute buffer constant not found${NC}"
        return 1
    fi
    
    if grep -q "FINALITY_BUFFER_NS.*2.*60" src/timelock.rs; then
        echo -e "${GREEN}✅ Finality buffer constant defined${NC}"
    else
        echo -e "${RED}❌ Finality buffer constant not found${NC}"
        return 1
    fi
    
    if grep -q "COORDINATION_BUFFER_NS.*1.*60" src/timelock.rs; then
        echo -e "${GREEN}✅ Coordination buffer constant defined${NC}"
    else
        echo -e "${RED}❌ Coordination buffer constant not found${NC}"
        return 1
    fi
    
    return 0
}

# Function to test timelock calculation functions
test_timelock_calculation_functions() {
    echo "🧮 Testing timelock calculation functions..."
    
    # Check if conservative timelock calculation function exists
    if grep -q "pub fn calculate_conservative_timelocks" src/timelock.rs; then
        echo -e "${GREEN}✅ calculate_conservative_timelocks function defined${NC}"
    else
        echo -e "${RED}❌ calculate_conservative_timelocks function not found${NC}"
        return 1
    fi
    
    # Check if validation function exists
    if grep -q "pub fn validate_timelock_duration" src/timelock.rs; then
        echo -e "${GREEN}✅ validate_timelock_duration function defined${NC}"
    else
        echo -e "${RED}❌ validate_timelock_duration function not found${NC}"
        return 1
    fi
    
    # Check if configuration creation function exists
    if grep -q "pub fn create_conservative_timelock_config" src/timelock.rs; then
        echo -e "${GREEN}✅ create_conservative_timelock_config function defined${NC}"
    else
        echo -e "${RED}❌ create_conservative_timelock_config function not found${NC}"
        return 1
    fi
    
    return 0
}

# Function to test timelock data structures
test_timelock_data_structures() {
    echo "📋 Testing timelock data structures..."
    
    # Check if ConservativeTimelocks struct exists
    if grep -q "pub struct ConservativeTimelocks" src/timelock.rs; then
        echo -e "${GREEN}✅ ConservativeTimelocks struct defined${NC}"
    else
        echo -e "${RED}❌ ConservativeTimelocks struct not found${NC}"
        return 1
    fi
    
    # Check if TimelockValidation struct exists
    if grep -q "pub struct TimelockValidation" src/timelock.rs; then
        echo -e "${GREEN}✅ TimelockValidation struct defined${NC}"
    else
        echo -e "${RED}❌ TimelockValidation struct not found${NC}"
        return 1
    fi
    
    # Check if TimelockStatus enum exists
    if grep -q "pub enum TimelockStatus" src/timelock.rs; then
        echo -e "${GREEN}✅ TimelockStatus enum defined${NC}"
    else
        echo -e "${RED}❌ TimelockStatus enum not found${NC}"
        return 1
    fi
    
    return 0
}

# Function to test timelock utility functions
test_timelock_utility_functions() {
    echo "🛠️ Testing timelock utility functions..."
    
    # Check if timelock expiry checking function exists
    if grep -q "pub fn is_timelock_expired" src/timelock.rs; then
        echo -e "${GREEN}✅ is_timelock_expired function defined${NC}"
    else
        echo -e "${RED}❌ is_timelock_expired function not found${NC}"
        return 1
    fi
    
    # Check if duration formatting function exists
    if grep -q "pub fn format_duration" src/timelock.rs; then
        echo -e "${GREEN}✅ format_duration function defined${NC}"
    else
        echo -e "${RED}❌ format_duration function not found${NC}"
        return 1
    fi
    
    # Check if cross-chain validation function exists
    if grep -q "pub fn validate_cross_chain_coordination" src/timelock.rs; then
        echo -e "${GREEN}✅ validate_cross_chain_coordination function defined${NC}"
    else
        echo -e "${RED}❌ validate_cross_chain_coordination function not found${NC}"
        return 1
    fi
    
    return 0
}

# Function to test timelock integration with lib.rs
test_timelock_integration() {
    echo "🔗 Testing timelock integration..."
    
    # Check if lib.rs uses timelock module
    if grep -q "timelock::calculate_conservative_timelocks" src/lib.rs; then
        echo -e "${GREEN}✅ lib.rs uses timelock calculation function${NC}"
    else
        echo -e "${RED}❌ lib.rs does not use timelock calculation function${NC}"
        return 1
    fi
    
    # Check if lib.rs uses timelock validation
    if grep -q "timelock::validate_timelock_duration" src/lib.rs; then
        echo -e "${GREEN}✅ lib.rs uses timelock validation function${NC}"
    else
        echo -e "${RED}❌ lib.rs does not use timelock validation function${NC}"
        return 1
    fi
    
    # Check if ConservativeTimelocks is imported
    if grep -q "use timelock::ConservativeTimelocks" src/lib.rs; then
        echo -e "${GREEN}✅ ConservativeTimelocks imported in lib.rs${NC}"
    else
        echo -e "${RED}❌ ConservativeTimelocks not imported in lib.rs${NC}"
        return 1
    fi
    
    return 0
}

# Function to verify tests are removed (clean code)
test_no_tests_present() {
    echo "🧹 Verifying no tests present (clean code)..."
    
    # Check that no test module exists
    if ! grep -q "#\[cfg(test)\]" src/timelock.rs; then
        echo -e "${GREEN}✅ No test module found (clean)${NC}"
    else
        echo -e "${RED}❌ Test module still present${NC}"
        return 1
    fi
    
    # Check that no unit tests are present
    if ! grep -q "#\[test\]" src/timelock.rs; then
        echo -e "${GREEN}✅ No unit tests found (clean)${NC}"
    else
        echo -e "${RED}❌ Unit tests still present${NC}"
        return 1
    fi
    
    return 0
}

# Main test execution
echo "Starting timelock module validation..."
echo ""

# Run tests
run_test "Compilation" "check_compilation"
run_test "Timelock Module Structure" "test_timelock_module_structure"
run_test "Timelock Constants" "test_timelock_constants"
run_test "Timelock Calculation Functions" "test_timelock_calculation_functions"
run_test "Timelock Data Structures" "test_timelock_data_structures"
run_test "Timelock Utility Functions" "test_timelock_utility_functions"
run_test "Timelock Integration" "test_timelock_integration"
run_test "No Tests Present (Clean Code)" "test_no_tests_present"

# Summary
echo ""
echo "📊 Test Summary:"
echo "================="
echo -e "${GREEN}✅ Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}❌ Tests Failed: $TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 All timelock module tests passed!${NC}"
    echo "✅ Timelock module is properly organized and structured"
    echo "✅ Conservative timelock calculation is modularized"
    echo "✅ Timelock validation and utility functions are available"
    echo "✅ Clean code without unnecessary test bloat"
    echo "✅ Clean integration with main lib.rs module"
    echo "✅ Constants are properly organized and accessible"
    exit 0
else
    echo ""
    echo -e "${RED}💥 Some tests failed. Please review the timelock module organization.${NC}"
    exit 1
fi