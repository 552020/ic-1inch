#!/bin/bash

set -e          # Exit immediately if a command exits with a non-zero status
set -u          # Treat unset variables as an error
set -o pipefail # Ensure errors propagate in pipelines

echo "🔧 Deploying SIWE Provider Canister..."

# Create canister if it doesn't exist
dfx canister create ic_siwe_provider --network local 2>/dev/null || true

# Deploy SIWE provider with proper initialization arguments
dfx deploy ic_siwe_provider --network local --argument "(
    record {
        domain = \"127.0.0.1\";
        uri = \"http://127.0.0.1:5173\";
        salt = \"salt\";
        chain_id = opt 8453;
        scheme = opt \"http\";
        statement = opt \"Login to the Fusion+ Mechanical Turk demo app\";
        sign_in_expires_in = opt 300000000000; /* 5 minutes */
        session_expires_in = opt 604800000000000; /* 1 week */
        targets = opt vec {
            \"$(dfx canister id ic_siwe_provider --network local)\";
            \"$(dfx canister id orderbook --network local)\";
            \"$(dfx canister id escrow --network local)\";
        };
    }
)"

# Get the canister ID and update environment
SIWE_CANISTER_ID=$(dfx canister id ic_siwe_provider --network local)
echo "✅ SIWE Provider deployed with ID: $SIWE_CANISTER_ID"

# Update .env file with SIWE canister ID
if grep -q "CANISTER_ID_IC_SIWE_PROVIDER" .env; then
    # Update existing entry
    sed -i '' "s/CANISTER_ID_IC_SIWE_PROVIDER=.*/CANISTER_ID_IC_SIWE_PROVIDER='$SIWE_CANISTER_ID'/" .env
else
    # Add new entry
    echo "CANISTER_ID_IC_SIWE_PROVIDER='$SIWE_CANISTER_ID'" >> .env
fi

echo "✅ Updated .env with SIWE canister ID"

# Generate declarations for frontend
dfx generate ic_siwe_provider

echo "✅ SIWE Provider deployment complete!"
echo "🔗 Canister ID: $SIWE_CANISTER_ID" 