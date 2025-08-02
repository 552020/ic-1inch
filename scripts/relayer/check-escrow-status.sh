#!/bin/bash

# Relayer script to check if tokens are locked in both ICP and ETH escrows
# Usage: ./check-escrow-status.sh <order_id>

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <order_id>"
    echo "Example: $0 fusion_1753986569703800000_nn52d-qid5y-wdg5t-k5q4b-pb554-u4zvb-eyiv2-46bum-xv5ok-57va7-7qe"
    exit 1
fi

ORDER_ID="$1"

echo "🔍 Checking escrow status for order: $ORDER_ID"
echo "=================================================="

# Check ICP escrow status
echo "📊 ICP Escrow Status:"
ICP_LOCKED=$(dfx canister call escrow is_tokens_locked "(\"$ORDER_ID\")" --output idl | grep -o 'true\|false')
echo "   Tokens locked: $ICP_LOCKED"

# Check ETH escrow status (placeholder - would call Ethereum contract)
echo "📊 ETH Escrow Status:"
echo "   Tokens locked: false (placeholder - would check Ethereum contract)"

echo "=================================================="

# Determine overall status
if [ "$ICP_LOCKED" = "true" ]; then
    echo "✅ ICP tokens are locked"
    echo "⚠️  ETH tokens status needs manual verification"
    echo ""
    echo "📋 Relayer Action Required:"
    echo "   1. Verify ETH tokens are locked in Ethereum contract"
    echo "   2. If both are locked, proceed with swap coordination"
    echo "   3. If not, wait for taker to lock ETH tokens"
else
    echo "❌ ICP tokens are not locked"
    echo ""
    echo "📋 Relayer Action Required:"
    echo "   1. Wait for maker to lock ICP tokens"
    echo "   2. Then verify ETH tokens are locked"
fi

echo ""
echo "💡 For production:"
echo "   - Add Ethereum contract calls to check ETH escrow status"
echo "   - Add automatic monitoring with webhooks"
echo "   - Add notification system for status changes" 