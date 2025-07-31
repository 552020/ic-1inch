# Fusion+ Mechanical Turk - Real Token Testing Guide

## Overview

This guide provides step-by-step instructions for testing the Fusion+ Mechanical Turk system with **real token transfers** using test tokens. This validates our Task 2.1.1 implementation where we replaced simulated transfers with actual ICRC-1 token operations.

**Test Objective:** Verify that the escrow canister correctly handles real token transfers, balance checking, and error scenarios.

**Architecture:**

- **test_token_a**: Mock ICP tokens (ICRC-1 standard)
- **test_token_b**: Mock ETH tokens (ICRC-1 standard)
- **orderbook**: Order management
- **escrow**: Real token custody with ICRC-1 transfers

---

## Prerequisites

### 1. Environment Setup

```bash
# Navigate to project directory
cd ic-1inch

# Deploy all canisters including test tokens
./scripts/deploy-mechanical-turk.sh

# Set up test identities and environment
./scripts/mechanical-turk/mechanical-turk-test-setup.sh

# Load environment variables
source .env.mechanical-turk
```

### 2. Verify System is Ready

```bash
# Test all canisters respond
dfx canister call orderbook get_active_fusion_orders '()'
dfx canister call escrow list_fusion_escrows '()'
dfx canister call test_token_a icrc1_name '()'
dfx canister call test_token_b icrc1_name '()'

# Check identities are created
dfx identity list
```

**Expected Result:** All canisters respond, identities (maker, taker, relayer) exist

---

## Test Scenarios

## Scenario 1: Real Token Transfer Testing

### User Story

> "As a tester, I want to verify that the escrow canister correctly transfers real tokens between users, so I can ensure the mechanical turk system works with actual token custody."

### Step-by-Step Test

#### Step 1.1: Fund Test Identities with Tokens

```bash
# Switch to maker identity
dfx identity use maker
dfx identity whoami

# Fund maker with test tokens (simulating ICP balance)
dfx canister call test_token_a mint_tokens "(principal \"$MAKER_PRINCIPAL\", 1000000000:nat)"  # 10 tokens
dfx canister call test_token_b mint_tokens "(principal \"$MAKER_PRINCIPAL\", 1000000000:nat)"  # 10 tokens

# Switch to taker identity
dfx identity use taker
dfx identity whoami

# Fund taker with test tokens
dfx canister call test_token_a mint_tokens "(principal \"$TAKER_PRINCIPAL\", 1000000000:nat)"  # 10 tokens
dfx canister call test_token_b mint_tokens "(principal \"$TAKER_PRINCIPAL\", 1000000000:nat)"  # 10 tokens

# Switch back to maker for testing
dfx identity use maker
```

**Expected Result:** Both identities funded with test tokens

#### Step 1.2: Verify Token Balances

```bash
# Check maker's token balances
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })"
dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })"

# Check taker's token balances
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\"; subaccount = null })"
dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\"; subaccount = null })"
```

**Expected Result:** Both identities show 10 tokens (1000000000) in each token canister

#### Step 1.3: Create Fusion Order

```bash
# Create ICP → ETH fusion order
dfx canister call orderbook create_fusion_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ICP },
  variant { ETH },
  ${ICP_AMOUNT}:nat64,
  10000000000000000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

**Expected Result:** `(variant { Ok = "fusion_1234567890_..." })` (Order ID returned)

#### Step 1.4: Accept Order as Taker

```bash
# Switch to taker identity
dfx identity use taker

# Accept the order
dfx canister call orderbook accept_fusion_order "(
  \"$ORDER_ID\",
  \"$TAKER_ETH_ADDRESS\"
)"
```

**Expected Result:** `(variant { Ok })` (Order accepted successfully)

#### Step 1.5: Lock Real Tokens in Escrow

```bash
# Switch back to maker
dfx identity use maker

# Lock real tokens in escrow (this will transfer tokens from maker to escrow)
dfx canister call escrow lock_icp_for_swap "(
  \"$ORDER_ID\",
  ${ICP_AMOUNT}:nat64,
  principal \"$TAKER_PRINCIPAL\",
  $(($(date +%s) + 7200))000000000:nat64
)"
```

**Expected Result:** `(variant { Ok = "escrow_fusion_1234567890_..." })` (Escrow ID returned)

#### Step 1.6: Verify Real Token Transfer

```bash
# Get escrow ID from previous result
export ESCROW_ID="escrow_fusion_1234567890_..."  # Replace with actual ID

# Check escrow status
dfx canister call escrow get_fusion_escrow_status "(\"$ESCROW_ID\")"

# Verify tokens were actually transferred to escrow
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$(dfx canister id escrow)\"; subaccount = null })"

# Check maker's reduced balance
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null })"
```

**Expected Result:**

- Escrow status shows `Funded`
- Escrow canister holds 10 tokens (1000000000)
- Maker's balance reduced by 10 tokens

#### Step 1.7: Claim Tokens as Taker

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

#### Step 1.8: Verify Final Token Balances

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

## Scenario 2: Error Testing

### Step 2.1: Test Insufficient Balance

```bash
# Switch to maker identity
dfx identity use maker

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

## Scenario 3: Refund Testing

### Step 3.1: Create Escrow for Refund Test

```bash
# Create new order and escrow
dfx canister call orderbook create_fusion_order "(
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
# Create ETH → ICP fusion order
dfx canister call orderbook create_fusion_order "(
  \"$MAKER_ETH_ADDRESS\",
  variant { ETH },
  variant { ICP },
  10000000000000000:nat64,
  ${ICP_AMOUNT}:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

### Step 4.2: Test with ETH Tokens

```bash
# Lock ETH tokens (using test_token_b)
dfx canister call escrow lock_icp_for_swap "(
  \"$ETH_ORDER_ID\",
  10000000000000000:nat64,
  principal \"$RESOLVER_PRINCIPAL\",
  $(($(date +%s) + 7200))000000000:nat64
)"
```

**Expected Result:** ETH tokens transferred to escrow

---

## Test Results Checklist

### ✅ Real Token Transfer Validation

- [ ] Tokens actually transfer from maker to escrow
- [ ] Escrow canister holds real tokens
- [ ] Tokens transfer from escrow to taker on claim
- [ ] Tokens refund to original locker on timeout
- [ ] Balance checking works correctly
- [ ] Error handling for insufficient balance works

### ✅ Error Scenario Validation

- [ ] Insufficient balance errors handled
- [ ] Unauthorized access blocked
- [ ] Expired timelock validation works
- [ ] Invalid receipt handling works
- [ ] System error handling works

### ✅ Integration Validation

- [ ] Orderbook and escrow work together
- [ ] Test tokens integrate correctly
- [ ] All canisters communicate properly
- [ ] State management works correctly

---

## Troubleshooting

### Common Issues

#### Issue: "Canister not found"

**Solution:** Deploy all canisters with `./scripts/deploy-mechanical-turk.sh`

#### Issue: "Insufficient balance"

**Solution:** Fund identities with `mint_tokens()` calls

#### Issue: "Transfer failed"

**Solution:** Check token canister IDs and ensure proper ICRC-1 interface

#### Issue: "Unauthorized"

**Solution:** Use correct identity for each operation

### Reset Test Environment

```bash
# Stop and clean dfx
dfx stop
dfx start --clean

# Redeploy all canisters
./scripts/deploy-mechanical-turk.sh

# Recreate test environment
./scripts/mechanical-turk/mechanical-turk-test-setup.sh
source .env.mechanical-turk
```

---

## Success Criteria

### Real Token Integration is Working When:

1. ✅ **Token transfers** work with real ICRC-1 calls
2. ✅ **Balance checking** prevents insufficient fund errors
3. ✅ **Escrow custody** actually holds tokens
4. ✅ **Claim operations** transfer tokens to takers
5. ✅ **Refund operations** return tokens to original lockers
6. ✅ **Error handling** provides clear feedback
7. ✅ **Integration** works between all canisters
8. ✅ **State management** tracks token movements correctly

### Ready for Production When:

- All real token transfer tests pass
- Error scenarios are handled properly
- Integration between canisters is stable
- Token custody is secure and reliable

**Congratulations!** 🎉 If all tests pass, your Fusion+ Mechanical Turk system has real token integration working correctly.

---

## Next Steps

1. **Automated Testing**: Create automated scripts for regression testing
2. **Performance Testing**: Test with larger token amounts and multiple users
3. **Security Testing**: Audit token transfer security
4. **Production Readiness**: Prepare for real ICP ledger integration

The mechanical turk approach now has real token integration - a major milestone! 🚀
