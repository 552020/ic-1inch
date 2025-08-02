#!/bin/bash
# test-basic.sh - Test basic LOP functionality

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id limit-order)
echo "🧪 Testing basic LOP functionality..."
echo "📋 Using canister: $CANISTER_ID"

# Test 1: Greet function
echo "📝 Test 1: Greet function..."
GREET_RESULT=$(dfx canister call $CANISTER_ID greet '("LOP")')
echo "Result: $GREET_RESULT"

# Test 2: Get system stats
echo "📊 Test 2: Get system stats..."
STATS_RESULT=$(dfx canister call $CANISTER_ID get_system_stats)
echo "System stats: $STATS_RESULT"

# Test 3: List active orders (should be empty initially)
echo "📋 Test 3: List active orders (should be empty)..."
LIST_RESULT=$(dfx canister call $CANISTER_ID get_active_orders)
echo "Active orders: $LIST_RESULT"

echo "✅ Basic LOP functionality tests completed!"
echo "📊 Summary:"
echo "  - Greet function: ✅"
echo "  - System stats: ✅"
echo "  - List active orders: ✅" 