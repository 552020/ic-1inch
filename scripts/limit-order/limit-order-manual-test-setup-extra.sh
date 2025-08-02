#!/bin/bash
# limit-order-manual-test-setup-extra.sh - Additional setup for manual testing

set -e  # Exit on any error

echo "🔧 Setting up additional test environment..."

# Source the main environment
if [ -f ".env.test" ]; then
    source .env.test
    echo "✅ Loaded .env.test"
else
    echo "❌ .env.test not found. Run limit-order-manual-test-setup.sh first"
    exit 1
fi

# Create additional test aliases
echo "📝 Creating additional test aliases..."

# Add to .env.test
cat >> .env.test << 'EOF'

# Additional test aliases
alias test-create-order='dfx canister call limit-order create_order "(
  record {
    maker = principal "$MAKER_PRINCIPAL";
    token_a = principal "$TEST_TOKEN_A_ID";
    token_b = principal "$TEST_TOKEN_B_ID";
    amount_a = $TEST_MAKING_AMOUNT : nat64;
    amount_b = $TEST_TAKING_AMOUNT : nat64;
    price = $TEST_TAKING_AMOUNT : nat64;
  }
)"'

alias test-fill-order='dfx canister call limit-order fill_order "(1:nat64)"'
alias test-cancel-order='dfx canister call limit-order cancel_order "(1:nat64)"'
alias test-list-orders='dfx canister call limit-order get_active_orders "()"'
alias test-get-stats='dfx canister call limit-order get_system_stats "()"'

# Quick test functions
quick-test() {
    echo "🚀 Running quick test..."
    switch-to-maker
    test-create-order
    switch-to-taker
    test-fill-order 1
    test-list-orders
    test-get-stats
    echo "✅ Quick test completed!"
}

# Status check function
check-status() {
    echo "📊 Current status:"
    echo "  Maker: $(dfx identity whoami)"
    echo "  Active orders:"
    test-list-orders
    echo "  System stats:"
    test-get-stats
}
EOF

echo "✅ Additional test aliases created"
echo "📋 Available commands:"
echo "  - test-create-order: Create a test order"
echo "  - test-fill-order: Fill order with ID 1"
echo "  - test-cancel-order: Cancel order with ID 1"
echo "  - test-list-orders: List active orders"
echo "  - test-get-stats: Get system statistics"
echo "  - quick-test: Run complete test cycle"
echo "  - check-status: Check current system status" 