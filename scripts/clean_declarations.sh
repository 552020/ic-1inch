#!/bin/bash

# Clean declarations script for ic-1inch project
# This script removes all generated DID files and TypeScript declarations

set -e

echo "🧹 Cleaning up declarations..."

# Remove TypeScript declarations
if [ -d "src/declarations" ]; then
    echo "🗑️  Removing TypeScript declarations..."
    rm -rf src/declarations
fi

# Remove generated DID files
if [ -f "src/backend/backend.did" ]; then
    echo "🗑️  Removing generated DID files..."
    rm src/backend/backend.did
fi

echo "✅ Cleanup completed!"
echo "📁 Removed:"
echo "   - src/declarations/ (TypeScript declarations)"
echo "   - src/backend/backend.did (Generated backend interface)" 