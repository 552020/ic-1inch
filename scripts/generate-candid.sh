#!/bin/bash
# generate-candid.sh - Generate Candid interface declarations

set -e  # Exit on any error

echo "🔄 Generating Candid interface declarations..."

# Check if dfx is available
if ! command -v dfx &> /dev/null; then
    echo "❌ dfx not found. Please install dfx first."
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo "❌ dfx.json not found. Please run this script from the project root."
    exit 1
fi

# Build the escrow_manager canister to generate Candid files
echo "🔨 Building escrow_manager canister..."
dfx build escrow_manager

# Check if Candid file was generated
CANDID_FILE=".dfx/local/canisters/escrow_manager/escrow_manager.did"
if [ ! -f "$CANDID_FILE" ]; then
    echo "❌ Candid file not found at $CANDID_FILE"
    exit 1
fi

echo "✅ Candid interface generated successfully!"
echo "📄 File location: $CANDID_FILE"

# Copy to a more accessible location for frontend development
FRONTEND_CANDID_DIR="src/frontend/src/declarations"
mkdir -p "$FRONTEND_CANDID_DIR"

# Copy the generated declarations
if [ -d ".dfx/local/canisters/escrow_manager" ]; then
    echo "📋 Copying declarations to frontend..."
    cp -r .dfx/local/canisters/escrow_manager/* "$FRONTEND_CANDID_DIR/"
    echo "✅ Declarations copied to $FRONTEND_CANDID_DIR"
fi

# Display the Candid interface
echo ""
echo "🔍 Generated Candid Interface:"
echo "================================"
cat "$CANDID_FILE"
echo ""
echo "================================"

echo "✅ Candid generation completed!"
echo "💡 You can now use these declarations in your frontend application."
echo "📚 Documentation: https://internetcomputer.org/docs/current/developer-docs/build/candid/candid-concepts" 