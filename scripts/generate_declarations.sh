#!/bin/bash

# Generate declarations script for ic-1inch project
# This script generates DID files and TypeScript declarations for all canisters

set -e

# Build canisters to generate their .did files
echo "📝 Building canisters to generate .did files..."
cargo build --target wasm32-unknown-unknown --release --package limit_order
cargo build --target wasm32-unknown-unknown --release --package test_token_icp
cargo build --target wasm32-unknown-unknown --release --package test_token_eth


cargo build --target wasm32-unknown-unknown --release --package orderbook
cargo build --target wasm32-unknown-unknown --release --package escrow

echo "🔧 Generating DID files and declarations..."

# Generate DID files for each canister
echo "📝 Generating DID files..."
generate-did orderbook
generate-did escrow
generate-did limit_order
generate-did test_token_icp
generate-did test_token_eth



# Generate TypeScript declarations for Rust canisters only
echo "📝 Generating TypeScript declarations..."
dfx generate orderbook
dfx generate escrow
dfx generate limit_order
dfx generate ic_siwe_provider

# Generate declarations for test tokens if .did files exist
if [ -f "src/test_token_icp/test_token_icp.did" ]; then
    dfx generate test_token_icp
else
    echo "⚠️  Skipping test_token_icp declarations - .did file not found"
fi

if [ -f "src/test_token_eth/test_token_eth.did" ]; then
    dfx generate test_token_eth
else
    echo "⚠️  Skipping test_token_eth declarations - .did file not found"
fi

echo "✅ Declarations generated successfully!"
echo "📁 Generated files:"
if [ -d "src/declarations" ]; then
    echo "   - src/declarations/ (TypeScript declarations)"
fi
if [ -f "src/orderbook/orderbook.did" ]; then
    echo "   - src/orderbook/orderbook.did (Orderbook interface)"
fi
if [ -f "src/escrow/escrow.did" ]; then
    echo "   - src/escrow/escrow.did (Escrow interface)"
fi
if [ -f "src/limit_order/limit_order.did" ]; then
    echo "   - src/limit_order/limit_order.did (Limit order interface)"
fi
if [ -f "src/test_token_icp/test_token_icp.did" ]; then
    echo "   - src/test_token_icp/test_token_icp.did (Test token ICP interface)"
fi
if [ -f "src/test_token_eth/test_token_eth.did" ]; then
    echo "   - src/test_token_eth/test_token_eth.did (Test token ETH interface)"
fi 