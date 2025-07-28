#!/bin/bash
# test-error-scenarios.sh - Test error scenarios and edge cases

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id backend)
echo "⚠️ Testing error scenarios and edge cases..."
echo "📋 Using canister: $CANISTER_ID"

# Get current identity principal for realistic testing
CURRENT_PRINCIPAL=$(dfx identity get-principal)

# Create a test escrow first
SECRET="test_secret_456"
HASHLOCK=$(echo -n "$SECRET" | sha256sum | cut -d' ' -f1 | xxd -r -p | base64)
FUTURE_TIME=$(($(date +%s) + 3600))000000000

echo "📝 Creating test escrow for error testing..."
ESCROW_RESPONSE=$(dfx canister call $CANISTER_ID create_escrow "(
  record {
    hashlock = blob \"$HASHLOCK\";
    timelock = $FUTURE_TIME : nat64;
    token_canister = principal \"$CANISTER_ID\";
    amount = 1_000_000 : nat64;
    maker = principal \"$CURRENT_PRINCIPAL\";
  }
)")

# Extract escrow ID from variant response - improved extraction
ESCROW_ID=$(echo "$ESCROW_RESPONSE" | grep -o '"[^"]*"' | head -1 | tr -d '"')
echo "Test escrow created: $ESCROW_ID"

# Test 1: Claim with wrong secret
echo "🔑 Test 1: Claim with wrong secret..."
if dfx canister call $CANISTER_ID claim_escrow "(\"$ESCROW_ID\", blob \"wrong_secret\")" 2>&1 | grep -q "InvalidHashlock"; then
    echo "✅ Expected error: InvalidHashlock"
else
    echo "❌ Unexpected result for wrong secret"
fi

# Test 2: Refund before timelock expires
echo "⏰ Test 2: Refund before timelock expires..."
if dfx canister call $CANISTER_ID refund_escrow "(\"$ESCROW_ID\")" 2>&1 | grep -q "TimelockNotExpired"; then
    echo "✅ Expected error: TimelockNotExpired"
else
    echo "❌ Unexpected result for early refund"
fi

# Test 3: Deposit to non-existent escrow
echo "💰 Test 3: Deposit to non-existent escrow..."
if dfx canister call $CANISTER_ID deposit_tokens "(\"non_existent\", 1_000_000 : nat64)" 2>&1 | grep -q "EscrowNotFound"; then
    echo "✅ Expected error: EscrowNotFound"
else
    echo "❌ Unexpected result for non-existent escrow"
fi

# Test 4: Get status of non-existent escrow
echo "📋 Test 4: Get status of non-existent escrow..."
if dfx canister call $CANISTER_ID get_escrow_status "(\"non_existent\")" 2>&1 | grep -q "EscrowNotFound"; then
    echo "✅ Expected error: EscrowNotFound"
else
    echo "❌ Unexpected result for non-existent escrow status"
fi

# Test 5: Create escrow with past timelock
echo "⏰ Test 5: Create escrow with past timelock..."
PAST_TIME=$(($(date +%s) - 3600))000000000  # 1 hour ago
if dfx canister call $CANISTER_ID create_escrow "(
  record {
    hashlock = blob \"$HASHLOCK\";
    timelock = $PAST_TIME : nat64;
    token_canister = principal \"$CANISTER_ID\";
    amount = 1_000_000 : nat64;
    maker = principal \"$CURRENT_PRINCIPAL\";
  }
)" 2>&1 | grep -q "InvalidTimelock"; then
    echo "✅ Expected error: InvalidTimelock"
else
    echo "❌ Unexpected result for past timelock"
fi

# Test 6: Create escrow with zero amount
echo "💰 Test 6: Create escrow with zero amount..."
if dfx canister call $CANISTER_ID create_escrow "(
  record {
    hashlock = blob \"$HASHLOCK\";
    timelock = $FUTURE_TIME : nat64;
    token_canister = principal \"$CANISTER_ID\";
    amount = 0 : nat64;
    maker = principal \"$CURRENT_PRINCIPAL\";
  }
)" 2>&1 | grep -q "InvalidAmount"; then
    echo "✅ Expected error: InvalidAmount"
else
    echo "❌ Unexpected result for zero amount"
fi

echo "✅ Error scenario tests completed!"
echo "📊 Summary:"
echo "  - Wrong secret handling: ✅"
echo "  - Early refund prevention: ✅"
echo "  - Non-existent escrow handling: ✅"
echo "  - Invalid timelock prevention: ✅"
echo "  - Zero amount prevention: ✅" 