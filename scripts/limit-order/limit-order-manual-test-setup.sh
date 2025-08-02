#!/bin/bash
# limit-order-manual-test-setup.sh - Setup environment for manual testing

set -e  # Exit on any error

echo "🔧 Setting up environment for limit-order manual testing..."
echo "========================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if dfx is available
if ! command -v dfx &> /dev/null; then
    print_error "dfx is not installed or not in PATH"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    print_error "Please run this script from the project root directory (where dfx.json is located)"
    exit 1
fi

# Check if dfx is running
print_status "Checking dfx status..."
if ! dfx ping > /dev/null 2>&1; then
    print_error "dfx is not running!"
    print_warning "Please start dfx with: dfx start --background"
    exit 1
fi
print_success "dfx is running"

# Create test identities if they don't exist
print_status "Setting up test identities..."

if ! dfx identity list | grep -q "maker"; then
    print_status "Creating maker identity..."
    dfx identity new maker --disable-encryption
    print_success "maker identity created"
else
    print_success "maker identity already exists"
fi

if ! dfx identity list | grep -q "taker"; then
    print_status "Creating taker identity..."
    dfx identity new taker --disable-encryption
    print_success "taker identity created"
else
    print_success "taker identity already exists"
fi

# Get principal IDs
print_status "Getting principal IDs..."
MAKER_PRINCIPAL=$(dfx identity use maker 2>/dev/null && dfx identity get-principal)
TAKER_PRINCIPAL=$(dfx identity use taker 2>/dev/null && dfx identity get-principal)

print_success "maker: $MAKER_PRINCIPAL"
print_success "taker: $TAKER_PRINCIPAL"

# Get limit-order canister ID
print_status "Getting limit-order canister ID..."
if dfx canister id limit-order &> /dev/null; then
    LIMIT_ORDER_CANISTER_ID=$(dfx canister id limit-order)
    print_success "limit-order: $LIMIT_ORDER_CANISTER_ID"
else
    print_warning "Limit-order canister not found - deploy with: ./scripts/deploy-local.sh"
    LIMIT_ORDER_CANISTER_ID=""
fi

# Get test token canister IDs (if they exist)
print_status "Getting test token canister IDs..."
if dfx canister id test_token_a &> /dev/null; then
    TEST_TOKEN_A_ID=$(dfx canister id test_token_a)
    print_success "test_token_a: $TEST_TOKEN_A_ID"
else
    print_warning "test_token_a not found"
    TEST_TOKEN_A_ID=""
fi

if dfx canister id test_token_b &> /dev/null; then
    TEST_TOKEN_B_ID=$(dfx canister id test_token_b)
    print_success "test_token_b: $TEST_TOKEN_B_ID"
else
    print_warning "test_token_b not found"
    TEST_TOKEN_B_ID=""
fi

# Create .env.test file
print_status "Creating .env.test file..."
cat > .env.test << EOF
# Test Environment Variables for Limit Order Protocol
# Generated on $(date)

# Identity Principals
export MAKER_PRINCIPAL="$MAKER_PRINCIPAL"
export TAKER_PRINCIPAL="$TAKER_PRINCIPAL"

# Canister IDs
export LIMIT_ORDER_CANISTER_ID="$LIMIT_ORDER_CANISTER_ID"
export TEST_TOKEN_A_ID="$TEST_TOKEN_A_ID"
export TEST_TOKEN_B_ID="$TEST_TOKEN_B_ID"

# Test Configuration
export TEST_MAKING_AMOUNT=1000000
export TEST_TAKING_AMOUNT=2000000
export TEST_EXPIRATION_HOURS=24

# Identity switching functions
switch-to-maker() {
    dfx identity use maker
    echo "Switched to maker identity: \$(dfx identity whoami)"
}

switch-to-taker() {
    dfx identity use taker
    echo "Switched to taker identity: \$(dfx identity whoami)"
}

# Test helper functions
test-create-order() {
    dfx canister call limit-order create_order "(
        record {
            maker = principal \"\$MAKER_PRINCIPAL\";
            token_a = principal \"\$TEST_TOKEN_A_ID\";
            token_b = principal \"\$TEST_TOKEN_B_ID\";
            amount_a = \$TEST_MAKING_AMOUNT : nat64;
            amount_b = \$TEST_TAKING_AMOUNT : nat64;
            price = \$TEST_TAKING_AMOUNT : nat64;
        }
    )"
}

test-fill-order() {
    dfx canister call limit-order fill_order "(\$1:nat64)"
}

test-cancel-order() {
    dfx canister call limit-order cancel_order "(\$1:nat64)"
}

test-list-orders() {
    dfx canister call limit-order get_active_orders "()"
}

test-get-stats() {
    dfx canister call limit-order get_system_stats "()"
}
EOF

print_success ".env.test file created"

# Make the file executable
chmod +x .env.test

print_success "Environment setup completed!"
echo ""
echo "📋 Next steps:"
echo "  1. Source the environment: source .env.test"
echo "  2. Switch to maker: switch-to-maker"
echo "  3. Create an order: test-create-order"
echo "  4. Switch to taker: switch-to-taker"
echo "  5. Fill the order: test-fill-order 1"
echo "  6. Check results: test-list-orders"
echo ""
echo "🚀 Ready for manual testing!" 