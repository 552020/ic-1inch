#!/bin/bash

# ICP Limit Order Protocol - Local Deployment Script
# Deploys backend, frontend, and test_token for local development

set -e  # Exit on any error

echo "🚀 Deploying ICP Limit Order Protocol (Local Development)"

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

print_status "Starting local IC replica..."
dfx start --background --clean

print_status "Waiting for replica to be ready..."
sleep 5

print_status "Deploying backend canister..."
dfx deploy backend

print_status "Deploying test_token canister (for local testing)..."
dfx deploy test_token

print_status "Deploying frontend assets..."
dfx deploy frontend

print_status "Getting canister IDs..."

BACKEND_ID=$(dfx canister id backend)
FRONTEND_ID=$(dfx canister id frontend)
TEST_TOKEN_ID=$(dfx canister id test_token)

print_success "Deployment completed!"

echo ""
echo "=================================================="
print_success "Local Deployment Results:"
echo ""
echo "Backend Canister ID: $BACKEND_ID"
echo "Frontend Canister ID: $FRONTEND_ID"
echo "Test Token Canister ID: $TEST_TOKEN_ID"
echo ""
echo "Frontend URL: http://localhost:4943/?canisterId=$FRONTEND_ID"
echo "Backend Candid UI: http://localhost:4943/?canisterId=$BACKEND_ID&id=$BACKEND_ID"
echo ""
echo "Test Token (for orders): $TEST_TOKEN_ID"
echo ""
echo "Next steps:"
echo "1. Run: ./scripts/limit-order-manual-test-setup.sh"
echo "2. Test: ./scripts/test-limit-orders-manual.sh"
echo "=================================================="

# Create environment file with canister IDs
cat > .env.local << EOF
# Local Deployment Environment Variables
BACKEND_CANISTER_ID="$BACKEND_ID"
FRONTEND_CANISTER_ID="$FRONTEND_ID"
TEST_TOKEN_CANISTER_ID="$TEST_TOKEN_ID"

# For testing orders
export TEST_MAKER_ASSET="aaaaa-aa"
export TEST_TAKER_ASSET="$TEST_TOKEN_ID"
EOF

print_success "Environment file created: .env.local"
print_status "To use these variables: source .env.local" 