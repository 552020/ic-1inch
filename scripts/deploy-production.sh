#!/bin/bash

# ICP Limit Order Protocol - Production Deployment Script
# Deploys backend and frontend for production (no test tokens)

set -e  # Exit on any error

echo "🚀 Deploying ICP Limit Order Protocol (Production)"

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

# Check if network is specified
if [ -z "$1" ]; then
    print_error "Please specify the network (e.g., ic, ic_testnet)"
    print_error "Usage: ./scripts/deploy-production.sh <network>"
    print_error "Example: ./scripts/deploy-production.sh ic"
    exit 1
fi

NETWORK="$1"

print_status "Deploying to network: $NETWORK"

print_status "Deploying backend canister..."
dfx deploy backend --network "$NETWORK"

print_status "Deploying frontend assets..."
dfx deploy frontend --network "$NETWORK"

print_status "Getting canister IDs..."

BACKEND_ID=$(dfx canister id backend --network "$NETWORK")
FRONTEND_ID=$(dfx canister id frontend --network "$NETWORK")

print_success "Production deployment completed!"

echo ""
echo "=================================================="
print_success "Production Deployment Results:"
echo ""
echo "Network: $NETWORK"
echo "Backend Canister ID: $BACKEND_ID"
echo "Frontend Canister ID: $FRONTEND_ID"
echo ""
echo "Frontend URL: https://$FRONTEND_ID.ic0.app"
echo "Backend Candid UI: https://$BACKEND_ID.ic0.app"
echo ""
print_warning "Note: No test_token deployed in production"
print_warning "Use real ICRC-1 tokens (ICP, ckBTC, ckETH, etc.)"
echo ""
echo "Production Token Options:"
echo "- ICP: aaaaa-aa"
echo "- ckBTC Ledger: mxzaz-hqaaa-aaaar-qaada-cai"
echo "- ckETH Ledger: ss2fx-dyaaa-aaaar-qacoq-cai"
echo ""
echo "Next steps:"
echo "1. Test with real tokens"
echo "2. Monitor canister performance"
echo "3. Update documentation"
echo "=================================================="

# Create production environment file
cat > .env.production << EOF
# Production Deployment Environment Variables
NETWORK="$NETWORK"
BACKEND_CANISTER_ID="$BACKEND_ID"
FRONTEND_CANISTER_ID="$FRONTEND_ID"

# Production token principals
export PROD_MAKER_ASSET="aaaaa-aa"
export PROD_TAKER_ASSET="ss2fx-dyaaa-aaaar-qacoq-cai"
EOF

print_success "Production environment file created: .env.production"
print_status "To use these variables: source .env.production" 