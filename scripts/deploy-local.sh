#!/bin/bash

# Deploy limit-order, frontend, and test tokens for local development
set -e

dfx deploy limit-order
dfx deploy test_token_a
dfx deploy test_token_b
dfx deploy frontend

./scripts/generate_declarations.sh

echo ""
echo "Frontend: http://localhost:4943/?canisterId=$(dfx canister id frontend)"
echo "Next: ./scripts/limit-order-manual-test-setup.sh" 