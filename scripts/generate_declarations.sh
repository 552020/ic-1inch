#!/bin/bash

# Generate declarations script for ic-1inch project
# This script generates DID files and TypeScript declarations for all canisters

set -e

echo "🔧 Generating DID files and declarations..."

# Generate DID files for each canister
echo "📝 Generating DID files..."
generate-did backend

# Build test tokens to generate their .did files
echo "📝 Building test tokens to generate .did files..."
cargo build --target wasm32-unknown-unknown --release --package test_token_a
cargo build --target wasm32-unknown-unknown --release --package test_token_b

# Generate TypeScript declarations for Rust canisters only
echo "📝 Generating TypeScript declarations..."
dfx generate backend

# Generate declarations for test tokens if .did files exist
if [ -f "src/test_token_a/test_token_a.did" ]; then
    dfx generate test_token_a
else
    echo "⚠️  Skipping test_token_a declarations - .did file not found"
fi

if [ -f "src/test_token_b/test_token_b.did" ]; then
    dfx generate test_token_b
else
    echo "⚠️  Skipping test_token_b declarations - .did file not found"
fi

echo "✅ Declarations generated successfully!"
echo "📁 Generated files:"
if [ -d "src/declarations" ]; then
    echo "   - src/declarations/ (TypeScript declarations)"
fi
if [ -f "src/backend/backend.did" ]; then
    echo "   - src/backend/backend.did (Backend interface)"
fi
if [ -f "src/test_token_a/test_token_a.did" ]; then
    echo "   - src/test_token_a/test_token_a.did (Test token A interface)"
fi
if [ -f "src/test_token_b/test_token_b.did" ]; then
    echo "   - src/test_token_b/test_token_b.did (Test token B interface)"
fi 