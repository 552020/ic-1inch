#!/bin/bash
# setup-testing.sh - Setup HTLC testing environment

set -e  # Exit on any error

echo "🚀 Setting up HTLC testing environment..."

# Check if dfx is installed
if ! command -v dfx &> /dev/null; then
    echo "❌ dfx not found. Please install dfx first."
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo "❌ dfx.json not found. Please run this script from the project root."
    exit 1
fi

# Create canister first
echo "📦 Creating limit-order canister..."
dfx canister create limit-order --network local || echo "Canister already exists"

# Build canister
echo "🔨 Building limit-order canister..."
dfx build limit-order

# Deploy canister
echo "📦 Deploying limit-order canister..."
dfx deploy limit-order --network local

# Get canister ID
CANISTER_ID=$(dfx canister id limit-order)
echo "📦 Canister deployed: $CANISTER_ID"

echo "✅ Testing environment setup completed!"
echo "📋 Canister ID: $CANISTER_ID"
echo "📋 Network: local" 