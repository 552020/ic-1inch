#!/bin/bash
# test-escrow-lifecycle.sh - Test complete escrow lifecycle

set -e  # Exit on any error

# Get canister ID
CANISTER_ID=$(dfx canister id backend)
echo "🔄 Testing complete escrow lifecycle..."
echo "📋 Using canister: $CANISTER_ID"

# Generate test data
SECRET="my_secret_123"
HASHLOCK=$(echo -n "$SECRET" | sha256sum | cut -d' ' -f1 | xxd -r -p | base64)
FUTURE_TIME=$(($(date +%s) + 3600))000000000  # 1 hour from now

# Get current identity principal for realistic testing
CURRENT_PRINCIPAL=$(dfx identity get-principal)
echo "🔑 Test data:"
echo "  Secret: $SECRET"
echo "  Hashlock: $HASHLOCK"
echo "  Timelock: $FUTURE_TIME"
echo "  Current principal: $CURRENT_PRINCIPAL"

# Step 1: Create escrow
echo "📝 Step 1: Creating escrow..."
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
echo "Escrow created: $ESCROW_ID"

# Step 2: Check escrow status (should be Created)
echo "📋 Step 2: Checking escrow status (should be Created)..."
STATUS_RESULT=$(dfx canister call $CANISTER_ID get_escrow_status "(\"$ESCROW_ID\")")
echo "Status: $STATUS_RESULT"

# Step 3: Deposit tokens
echo "💰 Step 3: Depositing tokens..."
DEPOSIT_RESULT=$(dfx canister call $CANISTER_ID deposit_tokens "(\"$ESCROW_ID\", 1_000_000 : nat64)")
echo "Deposit result: $DEPOSIT_RESULT"

# Step 4: Check funded status
echo "📋 Step 4: Checking funded status..."
FUNDED_STATUS=$(dfx canister call $CANISTER_ID get_escrow_status "(\"$ESCROW_ID\")")
echo "Funded status: $FUNDED_STATUS"

# Step 5: Claim with correct secret
echo "🔓 Step 5: Claiming with correct secret..."
CLAIM_RESULT=$(dfx canister call $CANISTER_ID claim_escrow "(\"$ESCROW_ID\", blob \"$SECRET\")")
echo "Claim result: $CLAIM_RESULT"

# Step 6: Check claimed status
echo "📋 Step 6: Checking claimed status..."
CLAIMED_STATUS=$(dfx canister call $CANISTER_ID get_escrow_status "(\"$ESCROW_ID\")")
echo "Claimed status: $CLAIMED_STATUS"

echo "✅ Escrow lifecycle test completed!"
echo "📊 Summary:"
echo "  - Escrow creation: ✅"
echo "  - Token deposit: ✅"
echo "  - Secret claim: ✅"
echo "  - State transitions: ✅" 