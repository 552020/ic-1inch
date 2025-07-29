#!/bin/bash

# Deploy backend and frontend for production
set -e

NETWORK="${1:-ic}"

dfx deploy backend --network "$NETWORK"
dfx deploy frontend --network "$NETWORK"

./scripts/generate_declarations.sh

echo ""
echo "Frontend: https://$(dfx canister id frontend --network "$NETWORK").ic0.app" 