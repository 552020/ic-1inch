#!/bin/bash

# ICP Limit Order Protocol - Extra Setup Features
# This contains additional features that were removed from the main setup for MVP simplicity

# Only run if environment variables are already set
if [ -z "$MAKER_PRINCIPAL" ] || [ -z "$TAKER_PRINCIPAL" ]; then
    echo "❌ Run limit-order-manual-test-setup.sh first to set environment variables"
    exit 1
fi

echo "🔧 Setting up extra features..."

# Test Configuration
export TEST_MAKING_AMOUNT="1000000000"  # 10 ICP (8 decimals)
export TEST_TAKING_AMOUNT="100000"      # 0.001 ckBTC (8 decimals)
export TEST_EXPIRATION_HOURS="1"        # 1 hour from now

# Helper aliases
alias switch-to-maker='dfx identity use maker'
alias switch-to-taker='dfx identity use taker'
alias switch-to-default='dfx identity use default'

# Test commands (use after deployment)
alias test-create-order='dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_MAKER_ASSET\",
  principal \"$TEST_TAKER_ASSET\",
  $TEST_MAKING_AMOUNT:nat64,
  $TEST_TAKING_AMOUNT:nat64,
  $(date -d \"+$TEST_EXPIRATION_HOURS hour\" +%s)000000000:nat64
)"'

alias test-fill-order='dfx canister call backend fill_order "(1:nat64)"'
alias test-cancel-order='dfx canister call backend cancel_order "(1:nat64)"'
alias test-list-orders='dfx canister call backend get_active_orders_list "()"'
alias test-get-stats='dfx canister call backend get_system_statistics "()"'

echo "✅ Extra features loaded:"
echo "- Test configuration variables"
echo "- Helper aliases (switch-to-maker, switch-to-taker, etc.)"
echo "- Test command aliases (test-create-order, test-fill-order, etc.)"
echo ""
echo "Available commands:"
echo "- switch-to-maker, switch-to-taker, switch-to-default"
echo "- test-create-order, test-fill-order, test-cancel-order"
echo "- test-list-orders, test-get-stats" 