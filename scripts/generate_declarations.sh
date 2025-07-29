#!/bin/bash

# Generate declarations script for ic-1inch project
# This script generates DID files and TypeScript declarations for all canisters

set -e

echo "🔧 Generating DID files and declarations..."

# Generate DID files for each canister
echo "📝 Generating DID files..."
generate-did backend

# Generate TypeScript declarations for Rust canisters only
echo "📝 Generating TypeScript declarations..."
dfx generate backend
dfx generate test_token

echo "✅ Declarations generated successfully!"
echo "📁 Generated files:"
if [ -d "src/declarations" ]; then
    echo "   - src/declarations/ (TypeScript declarations)"
fi
if [ -f "src/backend/backend.did" ]; then
    echo "   - src/backend/backend.did (Backend interface)"
fi
if [ -f "src/test_token/test_token.did" ]; then
    echo "   - src/test_token/test_token.did (Test token interface)"
fi 