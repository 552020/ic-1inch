# Fusion+ Mechanical Turk - Manual Testing Guide

## Overview

Step-by-step instructions for manually testing the Fusion+ Mechanical Turk cross-chain swap system between ICP and Ethereum.

**Test Objective:** Verify that the basic canister functions work correctly for order management and escrow operations with **real token transfers**. This tests the foundation before building the full cross-chain system.

**Architecture:** This implementation uses separate `orderbook` and `escrow` canisters to properly separate concerns - the orderbook manages orders (relayer role) while the escrow handles ICP token custody (pure escrow logic) with **real ICRC-1 token transfers**.

**Important:** These tests focus on **canister function validation with real token transfers** using test tokens. Full cross-chain swaps require Ethereum contracts, MetaMask integration, and frontend components that are not covered in these basic tests.

# Prerequisites

## Setup Instructions

### 1. Environment Setup

```bash
# Navigate to project directory
cd ic-1inch

# Deploy fusion canisters (orderbook, escrow, and test tokens)
./scripts/deploy-mechanical-turk.sh

# Set up test identities and environment
./scripts/mechanical-turk/mechanical-turk-test-setup.sh

# Load environment variables
source .env.mechanical-turk
```

### 2. Verify System is Ready

```bash
# Test orderbook canister
dfx canister call orderbook get_active_fusion_orders '()'

# Test escrow canister
dfx canister call escrow list_fusion_escrows '()'

# Test token canisters
dfx canister call test_token_icp icrc1_name '()'
dfx canister call test_token_eth icrc1_name '()'

# Check identities are created
dfx identity list
```

**Expected Result:** All canisters respond, identities (maker, taker, relayer) exist

---

## Test Scenarios

## Scenario 1: ICP → ETH Cross-Chain Swap with Real Token Transfers

### User Story

> "As a maker, I want to swap my ICP tokens for ETH on Ethereum, using a gasless Web2-like interface, so I can access Ethereum DeFi without complex cross-chain operations."

### Step-by-Step Test

#### Step 1.1: Fund Test Identities with Real Tokens

```bash
# Switch to maker identity
dfx identity use maker
dfx identity whoami

# Fund maker with test tokens (simulating ICP balance)
dfx canister call test_token_icp mint_tokens "(principal \"$MAKER_PRINCIPAL\", 1000000000:nat)"  # 10 tokens
dfx canister call test_token_eth mint_tokens "(principal \"$MAKER_PRINCIPAL\", 1000000000:nat)"  # 10 tokens

# Switch to taker identity
dfx identity use taker
dfx identity whoami

# Fund taker with test tokens
dfx canister call test_token_icp mint_tokens "(principal \"$TAKER_PRINCIPAL\", 1000000000:nat)"  # 10 tokens
dfx canister call test_token_eth mint_tokens "(principal \"$TAKER_PRINCIPAL\", 1000000000:nat)"  # 10 tokens

# Switch back to maker for order creation
dfx identity use maker
```

**Expected Result:** Both identities funded with test tokens

#### Step 1.2: Verify Token Balances

```bash
# Check maker's token balances
dfx canister call test_token_icp icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })"
dfx canister call test_token_eth icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })"

# Check taker's token balances
dfx canister call test_token_icp icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\"; subaccount = null })"
dfx canister call test_token_eth icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\"; subaccount = null })"
```

**Expected Result:** Both identities show 10 tokens (1000000000) in each token canister

#### Step 1.3: Maker Creates Cross-Chain Order

```bash
# Create ICP → ETH fusion order
# Parameters explained:
# - "0x742d35Cc6634C0532925a3b8D4C0532925a3b8D4"  # maker's ETH address
# - variant { ICP }                                   # from_token (what maker is selling)
# - variant { ETH }                                   # to_token (what maker wants)
# - 1000000000:nat64                                  # from_amount (10 ICP in 8 decimals)
# - 10000000000000000:nat64                          # to_amount (0.01 ETH in 18 decimals)
# - expiration timestamp in nanoseconds
dfx canister call orderbook create_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ICP },
  variant { ETH },
  ${ICP_AMOUNT}:nat64,
  10000000000000000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

**Expected Result:** `(variant { Ok = "fusion_1234567890_..." })` (Order ID returned)

#### Step 1.4: Verify Order Creation

```bash
# Get the order ID from the previous result
export ORDER_ID="fusion_1234567890_..."  # Replace with actual ID

# Check order details
dfx canister call orderbook get_fusion_order_status "(\"$ORDER_ID\")"

# List all active orders
dfx canister call orderbook get_active_fusion_orders '()'
```

**Expected Result:** Order shows status `Pending`, correct amounts, and maker details

#### Step 1.5: Check Orders by Maker

```bash
# Get orders created by current maker
dfx canister call orderbook get_orders_by_maker "(principal \"$MAKER_PRINCIPAL\")"
```

**Expected Result:** Shows the order you just created

---

### Step 2: Taker Accepts Order

#### Step 2.1: Switch to Taker Identity

```bash
# Switch to taker identity
dfx identity use taker
dfx identity whoami
```

**Expected Result:** `taker`

#### Step 2.2: Discover Available Orders

```bash
# View available orders as taker
dfx canister call orderbook get_active_fusion_orders '()'
```

**Expected Result:** Shows the ICP → ETH order created by maker

#### Step 2.3: Accept the Order

```bash
# Accept the order as taker
# Parameters explained:
# - "fusion_1234567890_..."                          # order_id
# - "0x8ba1f109551bD432803012645Hac189451b934"      # taker's ETH address
dfx canister call orderbook accept_fusion_order "(
  \"$ORDER_ID\",
  \"$TAKER_ETH_ADDRESS\"
)"
```

**Expected Result:** `(variant { Ok })` (Order accepted successfully)

#### Step 2.4: Verify Order Status Changed

```bash
# Check order status after acceptance
dfx canister call orderbook get_fusion_order_status "(\"$ORDER_ID\")"
```

**Expected Result:** Order status changed from `Pending` to `Accepted`

---

### Step 3: Maker Locks Real ICP Tokens

#### Step 3.1: Switch Back to Maker

```bash
# Switch to maker identity
dfx identity use maker
dfx identity whoami
```

**Expected Result:** `maker`

#### Step 3.2: Create ICP Escrow with Real Token Transfer

```bash
# Create escrow to lock ICP tokens (Mechanical Turk approach)
# Parameters explained:
# - "fusion_1234567890_..."                          # order_id
# - 1000000000:nat64                                 # amount (10 ICP in 8 decimals)
# - principal "..."                                   # taker principal (not hashlock)
# - timelock (2 hours from now in nanoseconds)
dfx canister call escrow lock_icp_for_swap "(
  \"$ORDER_ID\",
  ${ICP_AMOUNT}:nat64,
  principal \"$TAKER_PRINCIPAL\",
  $(($(date +%s) + 7200))000000000:nat64
)"
```

**Expected Result:** `(variant { Ok = "escrow_fusion_1234567890_..." })` (Escrow ID returned)

#### Step 3.3: Verify Real Token Transfer to Escrow

```bash
# Get the escrow ID from the previous result
export ESCROW_ID="escrow_fusion_1234567890_..."  # Replace with actual ID

# Check escrow details
dfx canister call escrow get_fusion_escrow_status "(\"$ESCROW_ID\")"

# Verify tokens were actually transferred to escrow
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$(dfx canister id escrow)\"; subaccount = null })"

# Check maker's reduced balance
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })"
```

**Expected Result:**

- Escrow shows status `Funded`
- Escrow canister holds 10 tokens (1000000000)
- Maker's balance reduced by 10 tokens

#### Step 3.4: List All Escrows

```bash
# List all escrows
dfx canister call escrow list_fusion_escrows '()'
```

**Expected Result:** Shows the escrow you just created

---

### Step 4: Cross-Chain Coordination (Manual Relayer)

#### Step 4.1: Switch to Relayer Identity

```bash
# Switch to relayer identity (infrastructure owner)
dfx identity use relayer
dfx identity whoami
```

**Expected Result:** `relayer`

#### Step 4.2: Verify Cross-Chain State

```bash
# Relayer checks ICP side
echo "=== ICP SIDE VERIFICATION ==="
dfx canister call orderbook get_fusion_order_status "(\"$ORDER_ID\")"
dfx canister call escrow get_fusion_escrow_status "(\"$ESCROW_ID\")"

# Simulate ETH side verification
echo "=== ETH SIDE VERIFICATION (SIMULATED) ==="
echo "✅ Taker has locked 0.01 ETH in Sepolia escrow contract"
echo "✅ ETH escrow contract address: 0x123...abc"
echo "✅ Transaction hash: 0xdef456..."
echo "✅ Block confirmation: 12/12"
```

**Expected Result:** ICP escrow is funded with real tokens, ETH escrow is simulated as funded

#### Step 4.3: Approve Swap for Completion

```bash
# Relayer approves the swap to proceed
# This simulates the relayer verifying both chains and giving the go-ahead
dfx canister call orderbook update_order_status "(
  \"$ORDER_ID\",
  variant { Accepted }
)"
```

**Expected Result:** `(variant { Ok })` (Status updated successfully)

---

### Step 5: Taker Claims Real Tokens

#### Step 5.1: Switch to Taker Identity

```bash
# Switch to taker identity
dfx identity use taker

# Claim tokens from escrow (this will transfer tokens from escrow to taker)
dfx canister call escrow claim_locked_icp "(
  \"$ESCROW_ID\",
  \"0x1234567890abcdef...\"  # Mock ETH receipt
)"
```

**Expected Result:** `(variant { Ok })` (Tokens claimed successfully)

#### Step 5.2: Verify Real Token Transfer

```bash
# Check taker's increased balance
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\"; subaccount = null })"

# Check escrow's empty balance
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$(dfx canister id escrow)\"; subaccount = null })"

# Check final escrow status
dfx canister call escrow get_fusion_escrow_status "(\"$ESCROW_ID\")"
```

**Expected Result:**

- Taker's balance increased by 10 tokens
- Escrow balance is 0 (tokens transferred out)
- Escrow status shows `Claimed`

---

### Step 6: Test Basic Escrow Functions

#### Step 6.1: Test Escrow Status Query

```bash
# Test escrow status query function
dfx canister call escrow get_fusion_escrow_status "(\"$ESCROW_ID\")"

# List all escrows
dfx canister call escrow list_fusion_escrows '()'
```

**Expected Result:** Escrow details returned correctly

#### Step 6.2: Test Order Status Updates

```bash
# Switch to relayer identity
dfx identity use relayer

# Test order status update function
dfx canister call orderbook update_order_status "(
  \"$ORDER_ID\",
  variant { Completed }
)"
```

**Expected Result:** `(variant { Ok })` (Status updated successfully)

#### Step 6.3: Note on Full Cross-Chain Testing

```bash
echo "=== FULL CROSS-CHAIN TESTING REQUIRES ==="
echo "❌ Ethereum contracts (not deployed yet)"
echo "❌ MetaMask integration (frontend needed)"
echo "❌ Real token transfers (requires ledger integration)"
echo "❌ Receipt validation (requires cross-chain communication)"
echo ""
echo "✅ What we CAN test with these scripts:"
echo "• Canister function calls work"
echo "• Order creation and acceptance"
echo "• Real token transfers in escrow"
echo "• Identity management"
echo "• Status updates"
```

**Expected Result:** Clear understanding of test limitations

---

### Step 7: Verify Final State

#### Step 7.1: Check Final Order Status

```bash
# Check final order status
dfx canister call orderbook get_fusion_order_status "(\"$ORDER_ID\")"

# Verify no active orders remain
dfx canister call orderbook get_active_fusion_orders '()'
```

**Expected Result:** Order status is `Completed`, no active orders

#### Step 7.2: Check Final Escrow Status

```bash
# Check final escrow status
dfx canister call escrow get_fusion_escrow_status "(\"$ESCROW_ID\")"
```

**Expected Result:** Escrow status is `Claimed`

#### Step 7.3: Verify Atomic Completion

```bash
echo "=== ATOMIC SWAP VERIFICATION (SIMULATED) ==="
echo "✅ Maker received: 0.01 ETH at $MAKER_ETH_ADDRESS (simulated)"
echo "✅ Taker received: 10 ICP tokens (real transfer)"
echo "✅ Order completed atomically (simulated)"
echo "✅ No funds lost or stuck (simulated)"
```

**Expected Result:** Both parties received their tokens atomically (simulated)

---

## Scenario 2: Error Handling Tests with Real Tokens

### Step 2.1: Test Insufficient Balance

```bash
# Try to lock more tokens than available
dfx canister call escrow lock_icp_for_swap "(
  \"$ORDER_ID\",
  999999999999:nat64,  # Much more than available
  principal \"$TAKER_PRINCIPAL\",
  $(($(date +%s) + 7200))000000000:nat64
)"
```

**Expected Result:** `(variant { Err = variant { InsufficientBalance } })`

### Step 2.2: Test Unauthorized Claim

```bash
# Try to claim escrow with wrong identity
dfx identity use maker  # Should be taker
dfx canister call escrow claim_locked_icp "(
  \"$ESCROW_ID\",
  \"0x1234567890abcdef...\"
)"
```

**Expected Result:** `(variant { Err = variant { Unauthorized } })`

### Step 2.3: Test Expired Timelock

```bash
# Create escrow with expired timelock
dfx canister call escrow lock_icp_for_swap "(
  \"$ORDER_ID\",
  ${ICP_AMOUNT}:nat64,
  principal \"$TAKER_PRINCIPAL\",
  $(($(date +%s) - 3600))000000000:nat64  # Past time
)"
```

**Expected Result:** `(variant { Err = variant { TimelockExpired } })`

---

## Scenario 3: Refund Testing with Real Tokens

### Step 3.1: Create Escrow for Refund Test

```bash
# Create new order and escrow
dfx canister call orderbook create_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ICP },
  variant { ETH },
  ${ICP_AMOUNT}:nat64,
  10000000000000000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"

# Accept order
dfx identity use taker
dfx canister call orderbook accept_fusion_order "(
  \"$NEW_ORDER_ID\",
  \"$TAKER_ETH_ADDRESS\"
)"

# Lock tokens
dfx identity use maker
dfx canister call escrow lock_icp_for_swap "(
  \"$NEW_ORDER_ID\",
  ${ICP_AMOUNT}:nat64,
  principal \"$TAKER_PRINCIPAL\",
  $(($(date +%s) + 7200))000000000:nat64
)"
```

### Step 3.2: Wait and Refund

```bash
# Wait for timelock to expire (or simulate by creating with short timelock)
# Then refund tokens
dfx canister call escrow refund_locked_icp "(\"$NEW_ESCROW_ID\")"
```

**Expected Result:** `(variant { Ok })` (Tokens refunded to original locker)

### Step 3.3: Verify Refund

```bash
# Check maker's balance increased
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })"

# Check escrow is empty
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$(dfx canister id escrow)\"; subaccount = null })"
```

**Expected Result:** Maker's balance restored, escrow empty

---

## Scenario 4: ETH → ICP Swap Testing

### Step 4.1: Create ETH → ICP Order

```bash
# Switch to maker identity
dfx identity use maker

# Create ETH → ICP fusion order
dfx canister call orderbook create_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ETH },
  variant { ICP },
  10000000000000000:nat64,
  ${ICP_AMOUNT}:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

**Expected Result:** New order created with ETH → ICP direction

#### Step 4.2: Simulate EIP-712 Signature

```bash
echo "=== EIP-712 SIGNATURE SIMULATION ==="
echo "Maker signs order with MetaMask:"
echo "Domain: Fusion+ Cross-Chain Swap"
echo "Message: Swap 0.01 ETH for 10 ICP"
echo "Signature: 0x1234567890abcdef..."
echo "✅ Signature verified"
```

**Expected Result:** EIP-712 signature simulated

#### Step 4.3: Continue with Similar Flow

Follow the same pattern as Scenario 1, but with:

- Taker locks both ETH (using EIP-712) and ICP
- Atomic completion with reversed token flow
- ETH escrow on Ethereum, ICP escrow on ICP

---

## Scenario 5: Error Handling Tests

### Step 5.1: Test Invalid Order Creation

```bash
# Try to create order with same from/to token
dfx canister call orderbook create_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ICP },
  variant { ICP },
  ${ICP_AMOUNT}:nat64,
  ${ICP_AMOUNT}:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

**Expected Result:** Error (same token not allowed)

### Step 5.2: Test Unauthorized Operations

```bash
# Try to accept order that doesn't exist
dfx canister call orderbook accept_fusion_order "(
  \"nonexistent_order\",
  \"$TAKER_ETH_ADDRESS\"
)"
```

**Expected Result:** `(variant { Err = variant { OrderNotFound } })`

### Step 5.3: Test Escrow Errors

```bash
# Try to claim escrow without proper authorization
dfx canister call escrow claim_locked_icp "(
  \"nonexistent_escrow\",
  \"invalid_receipt\"
)"
```

**Expected Result:** `(variant { Err = variant { EscrowNotFound } })`

---

## Scenario 6: Taker Whitelisting

### Step 6.1: Test Taker Management

```bash
# Switch to relayer (who manages taker whitelist)
dfx identity use relayer

# Note: Whitelisting functionality would be added to orderbook canister
echo "=== TAKER WHITELISTING (FUTURE FEATURE) ==="
echo "• Add taker to whitelist"
echo "• Remove taker from whitelist"
echo "• Check taker status"
```

**Expected Result:** Framework for taker management

---

## Frontend Integration Tests

### Scenario 7: Web Interface Testing

#### Step 7.1: SIWE Authentication

```bash
echo "=== FRONTEND INTEGRATION TESTS ==="
echo "1. MetaMask connection"
echo "2. SIWE authentication"
echo "3. Cross-chain identity derivation"
echo "4. Order creation UI"
echo "5. Taker interface"
echo "6. Relayer admin panel"
```

**Expected Result:** Framework for frontend testing

---

## Performance and Load Tests

### Scenario 8: Multiple Orders

#### Step 8.1: Create Multiple Orders

```bash
# Create 5 orders quickly
for i in {1..5}; do
  dfx canister call orderbook create_order "(
    \"$MAKER_ETH_ADDRESS\",
    variant { ICP },
    variant { ETH },
    $(($i * 100000000)):nat64,
    $(($i * 1000000000000000)):nat64,
    $(($(date +%s) + $(($i * 3600))))000000000:nat64
  )"
done
```

**Expected Result:** All orders created successfully

#### Step 8.2: Verify System Performance

```bash
# Check all orders
dfx canister call orderbook get_active_fusion_orders '()'

# Verify system remains responsive
time dfx canister call orderbook get_active_fusion_orders '()'
```

**Expected Result:** System handles multiple orders efficiently

---

## Test Results Checklist

### ✅ Core Cross-Chain Functionality

- [ ] ICP → ETH orders can be created
- [ ] ETH → ICP orders can be created
- [ ] Takers can accept orders
- [ ] ICP escrow system works correctly
- [ ] Real token transfers work
- [ ] Cross-chain coordination functions
- [ ] Atomic completion prevents fund loss

### ✅ Real Token Integration

- [ ] Tokens actually transfer from maker to escrow
- [ ] Escrow canister holds real tokens
- [ ] Tokens transfer from escrow to taker on claim
- [ ] Tokens refund to original locker on timeout
- [ ] Balance checking works correctly
- [ ] Error handling provides clear feedback

### ✅ Mechanical Turk Simulation

- [ ] Manual relayer coordination works
- [ ] Cross-chain state verification simulated
- [ ] Receipt-based completion demonstrated
- [ ] Best-case scenario flows validated

### ✅ Error Handling

- [ ] Invalid orders are rejected
- [ ] Unauthorized operations blocked
- [ ] Non-existent resources handled gracefully
- [ ] Clear error messages provided

### ✅ System Architecture

- [ ] Orderbook and escrow separation works
- [ ] Identity management functions
- [ ] Multiple user roles supported
- [ ] Canister communication reliable

---

## Troubleshooting

### Common Issues

#### Issue: "Canister not found"

**Solution:** Deploy fusion canisters with `./scripts/deploy-mechanical-turk.sh`

#### Issue: "Identity not found"

**Solution:** Run setup script: `./scripts/mechanical-turk/mechanical-turk-test-setup.sh`

#### Issue: "Order not found"

**Solution:** Use correct order ID from creation response

#### Issue: "Environment variables not set"

**Solution:** Load environment: `source .env.mechanical-turk`

#### Issue: "Insufficient balance"

**Solution:** Fund identities with `mint_tokens()` calls

#### Issue: "Transfer failed"

**Solution:** Check token canister is deployed and responding

### Reset Test Environment

```bash
# Stop and clean dfx
dfx stop
dfx start --clean

# Redeploy fusion canisters
./scripts/deploy-mechanical-turk.sh

# Recreate test environment
./scripts/mechanical-turk/mechanical-turk-test-setup.sh
source .env.mechanical-turk
```

---

## Success Criteria

### Basic Canister Functions are Working When:

1. ✅ **Order creation** functions work correctly
2. ✅ **Order acceptance** by takers functions
3. ✅ **Escrow creation** functions work
4. ✅ **Real token transfers** work correctly
5. ✅ **Status queries** return correct data
6. ✅ **Order updates** can be performed by relayer
7. ✅ **Error handling** provides clear feedback
8. ✅ **Multiple identities** can interact with canisters
9. ✅ **Canister separation** (orderbook vs escrow) works

### Ready for Frontend Integration When:

- All basic canister functions pass
- Real token transfers work correctly
- Error handling works properly
- Identity management is functional
- Canister interfaces are stable

### Ready for Full Cross-Chain Testing When:

- Frontend with MetaMask is built
- Ethereum contracts are deployed
- Real token integration is complete
- Cross-chain communication is implemented

**Congratulations!** 🎉 If all tests pass, your Fusion+ Mechanical Turk system is ready for frontend development and Ethereum contract integration.

---

## Next Steps

1. **Frontend Development**: Build React components for cross-chain swaps
2. **Ethereum Integration**: Deploy Hardhat contracts on Sepolia
3. **SIWE Authentication**: Implement real MetaMask integration
4. **End-to-End Testing**: Test with real cross-chain transactions
5. **Production Readiness**: Add monitoring, logging, and error recovery

The mechanical turk approach has validated the core concept with real token transfers - now you can build the full system with confidence! 🚀
