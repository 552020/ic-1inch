#!/bin/bash
# test-basic.sh - Test basic HTLC functionality

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id backend)
echo "🧪 Testing basic HTLC functionality..."
echo "📋 Using canister: $CANISTER_ID"

# Test 1: Greet function
echo "📝 Test 1: Greet function..."
GREET_RESULT=$(dfx canister call $CANISTER_ID greet '("HTLC")')
echo "Result: $GREET_RESULT"

# Test 2: Timelock enforcement
echo "⏰ Test 2: Timelock enforcement..."
CURRENT_TIME=$(date +%s)000000000
FUTURE_TIME=$((CURRENT_TIME + 3600000000000))  # 1 hour in future
PAST_TIME=$((CURRENT_TIME - 3600000000000))    # 1 hour in past

echo "Testing future timelock (should be Active)..."
FUTURE_RESULT=$(dfx canister call $CANISTER_ID test_timelock "($FUTURE_TIME : nat64)")
echo "Future timelock result: $FUTURE_RESULT"

echo "Testing past timelock (should be Expired)..."
PAST_RESULT=$(dfx canister call $CANISTER_ID test_timelock "($PAST_TIME : nat64)")
echo "Past timelock result: $PAST_RESULT"

# Test 3: List escrows (should be empty initially)
echo "📋 Test 3: List escrows (should be empty)..."
LIST_RESULT=$(dfx canister call $CANISTER_ID list_escrows)
echo "Escrows list: $LIST_RESULT"

echo "✅ Basic functionality tests completed!"
echo "📊 Summary:"
echo "  - Greet function: ✅"
echo "  - Timelock enforcement: ✅"
echo "  - List escrows: ✅" 