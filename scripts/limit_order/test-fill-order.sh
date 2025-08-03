#!/bin/bash

# Test script for the new clean fill_order implementation
# This tests the core 1inch LOP fill_order function

set -e

echo "🧪 Testing Clean fill_order Implementation"
echo "=========================================="

# Check if dfx is running
if ! dfx ping; then
    echo "❌ dfx is not running. Please start dfx with: dfx start --clean"
    exit 1
fi

echo "✅ dfx is running"

# Create test identities if they don't exist
echo "🔧 Setting up test identities..."

if ! dfx identity list | grep -q "maker"; then
    dfx identity new maker --disable-encryption
fi

if ! dfx identity list | grep -q "taker"; then
    dfx identity new taker --disable-encryption
fi

echo "✅ Test identities ready"

# Switch to maker identity
echo "👤 Switching to maker identity..."
dfx identity use maker
MAKER_PRINCIPAL=$(dfx identity get-principal)
echo "Maker principal: $MAKER_PRINCIPAL"

# Create a test order
echo "📝 Creating test order..."
ORDER_SALT=12345
MAKER_ASSET="rdmx6-jaaaa-aaaah-qcaiq-cai"  # ICP ledger
TAKER_ASSET="rdmx6-jaaaa-aaaah-qcaiq-cai"  # ICP ledger
MAKING_AMOUNT=1000000000  # 1 ICP
TAKING_AMOUNT=2000000000  # 2 ICP
EXPIRATION=$(($(date +%s) + 3600))000000000  # 1 hour from now

# Create order JSON
ORDER_JSON=$(cat <<EOF
{
  "salt": $ORDER_SALT,
  "maker": "$MAKER_PRINCIPAL",
  "receiver": "$MAKER_PRINCIPAL",
  "maker_asset": "$MAKER_ASSET",
  "taker_asset": "$TAKER_ASSET",
  "making_amount": $MAKING_AMOUNT,
  "taking_amount": $TAKING_AMOUNT,
  "maker_traits": {"None": null},
  "expiration": $EXPIRATION
}
EOF
)

echo "Order JSON: $ORDER_JSON"

# Switch to taker identity
echo "👤 Switching to taker identity..."
dfx identity use taker
TAKER_PRINCIPAL=$(dfx identity get-principal)
echo "Taker principal: $TAKER_PRINCIPAL"

# Test fill_order function
echo "🔄 Testing fill_order function..."
echo "Calling fill_order with order and signature..."

# Create a dummy signature (in real implementation, this would be ECDSA signature)
SIGNATURE="0x1234567890abcdef"

# Test the fill_order call
echo "📞 Calling fill_order..."
RESULT=$(dfx canister call limit_order fill_order "(
  $ORDER_JSON,
  vec {$(echo $SIGNATURE | sed 's/0x//' | sed 's/../0x&, /g' | sed 's/, $//')},
  1000000000,
  variant { None }
)" 2>&1 || true)

echo "Result: $RESULT"

# Test hash_order function
echo "🔍 Testing hash_order function..."
HASH_RESULT=$(dfx canister call limit_order hash_order "($ORDER_JSON)" 2>&1 || true)
echo "Hash result: $HASH_RESULT"

# Test remaining_invalidator_for_order
echo "📊 Testing remaining_invalidator_for_order..."
REMAINING_RESULT=$(dfx canister call limit_order remaining_invalidator_for_order "(
  principal \"$MAKER_PRINCIPAL\",
  vec {}
)" 2>&1 || true)
echo "Remaining result: $REMAINING_RESULT"

echo "✅ Test completed!"
echo ""
echo "📋 Summary:"
echo "- fill_order: $RESULT"
echo "- hash_order: $HASH_RESULT"
echo "- remaining_invalidator_for_order: $REMAINING_RESULT" 