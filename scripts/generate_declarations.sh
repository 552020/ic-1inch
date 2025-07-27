#!/bin/bash

# Generate declarations script for ic-1inch project
# This script generates DID files and TypeScript declarations for all canisters

set -e

echo "🔧 Generating DID files and declarations..."

# Generate DID files for each canister
echo "📝 Generating DID files..."
generate-did backend

# Generate TypeScript declarations
echo "📝 Generating TypeScript declarations..."
dfx generate

echo "✅ Declarations generated successfully!"
echo "📁 Generated files:"
echo "   - src/declarations/ (TypeScript declarations)"
echo "   - src/backend/backend.did (Backend interface)" 