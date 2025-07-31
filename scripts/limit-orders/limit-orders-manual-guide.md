# ICP Limit Order Protocol - Manual Testing Guide

## Overview

Step-by-step instructions for manually testing the ICP Limit Order Protocol MVP.

**Test Objective:** Verify that users can create, discover, and fill limit orders on a local ICP environment with zero gas fees.

**Note:** This guide uses two test tokens (`test_token_a` and `test_token_b`) for local testing instead of ICP, since the ICP ledger is not available on the local testnet. This allows full testing of the limit order functionality without requiring external dependencies.

**Important:** This implementation uses ICRC-2 tokens which require explicit approval before transfers. Both the maker and taker must approve the backend canister to spend their tokens before orders can be filled. This is a crucial step that must be completed for successful order execution.

# Prerequisites

## Setup Instructions

### 1. Environment Setup

````bash
# Navigate to project directory
cd ic-1inch

# Deploy all canisters (backend, frontend, test_token_a, test_token_b) for local development
./scripts/deploy-local.sh
```

## Test Scenarios

## Scenario 1: Maker Creates a Limit Order

### User Story

> "As a maker, I want to create a limit order to sell 10 ICP for 0.001 ckBTC, so that I can get the exchange rate I want without actively monitoring the market."

### Step-by-Step Test

#### Step 1.1: Verify System is Ready

```bash
# Test backend connection
dfx canister call backend greet '("Prime")'
````

**Expected Result:** `("Hello, Test User!")`

#### Step 1.2: Create Your First Limit Order

```bash
# First, run the setup script to get principals
./scripts/limit-order-manual-test-setup.sh

# Load the environment variables into your current shell
source .env.test

# Switch to maker identity to create the order
dfx identity use maker
dfx identity whoami

# Fund the maker with TOKEN_A tokens so they can create orders
# This is needed because the maker needs TOKEN_A tokens to sell
dfx canister call test_token_a mint_tokens "(principal \"$MAKER_PRINCIPAL\", 2000000000:nat)"

# Check maker balance after minting
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })"

# Create order: Sell 10 TOKEN_A for 0.001 TOKEN_B
# Parameters explained:
# - principal "$MAKER_PRINCIPAL"     # receiver (who gets the taker asset)
# - principal "$TEST_TOKEN_A"        # maker_asset (what the maker is selling)
# - principal "$TEST_TOKEN_B"        # taker_asset (what the taker is buying)
# - 1000000000:nat64                 # making_amount (10 TOKEN_A in 8 decimals)
# - 100000:nat64                     # taking_amount (0.001 TOKEN_B in 8 decimals)
# - $(($(date +%s) + 3600))000000000:nat64 # expiration (1 hour from now in nanoseconds)
#
# Alternative: Sell TOKEN_B for TOKEN_A
# - principal "$TEST_TOKEN_B"        # maker_asset (what the maker is selling)
# - principal "$TEST_TOKEN_A"        # taker_asset (what the taker is buying)
dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_TOKEN_A\",
  principal \"$TEST_TOKEN_B\",
  1000000000:nat64,
  100000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

**Expected Result:** `(variant { Ok = 1 : nat64 })` (Order ID 1 created)

#### Step 1.3: Approve Backend Canister to Spend Maker's Tokens

```bash
# The maker needs to approve the backend canister to spend their TOKEN_A tokens
# This is required for ICRC-2 tokens (approve/transfer_from pattern)
dfx canister call test_token_a icrc2_approve "(record {
  from_subaccount = null;
  spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
  amount = 1000000000000:nat;
  expires_at = null;
  fee = null;
  memo = null;
  created_at_time = null;
})"

# Verify the approval was set
echo "Maker's TOKEN_A allowance for backend:"
dfx canister call test_token_a icrc2_allowance "(record {
  account = record { owner = principal \"$MAKER_PRINCIPAL\"; subaccount = null };
  spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
})"
```

**Expected Result:** Approval should return `(variant { Ok = 1 : nat64 })` and allowance should show the approved amount.

#### Step 1.4: Verify Order was Created

```bash
# Get order details
dfx canister call backend get_order_by_id '(1: nat64)'
```

**Expected Result:** Order details showing:

- `id = 1`
- `maker = your_principal`
- `making_amount = 1000000000`
- `taking_amount = 100000`
- Order state should be active

#### Step 1.4: View All Active Orders

```bash
# List all active orders
dfx canister call backend get_active_orders '()'
```

**Expected Result:** Array with your order listed as active

#### Step 1.5: Create a Second Order to Test Multiple Orders

```bash
# Create a second order: Sell 5 TOKEN_A for 0.0005 TOKEN_B
# Parameters explained:
# - principal "$MAKER_PRINCIPAL"     # receiver (who gets the taker asset)
# - principal "$TEST_TOKEN_A"        # maker_asset (test_token_a canister for local testing)
# - principal "$TEST_TOKEN_B"        # taker_asset (test_token_b canister for local testing)
# - 500000000:nat64                  # making_amount (5 TOKEN_A in 8 decimals)
# - 50000:nat64                      # taking_amount (0.0005 TOKEN_B in 8 decimals)
# - $(($(date +%s) + 7200))000000000:nat64 # expiration (2 hours from now in nanoseconds)
dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_TOKEN_A\",
  principal \"$TEST_TOKEN_B\",
  500000000:nat64,
  50000:nat64,
  $(($(date +%s) + 7200))000000000:nat64
)"
```

**Expected Result:** `(variant { Ok = 2 : nat64 })` (Order ID 2 created)

#### Step 1.6: Verify Both Orders are Active

```bash
# List all active orders again
dfx canister call backend get_active_orders '()'
```

**Expected Result:** Array with both orders (ID 1 and ID 2) listed as active

---

## Scenario 2: Taker Discovers and Fills Order

### User Story

> "As a taker, I want to discover available limit orders and fill one that offers a good exchange rate, so I can get the tokens I need at the price I want."

### Step-by-Step Test

#### Step 2.1: Switch to Taker Identity

```bash
# Use the taker identity from our setup
dfx identity use taker

# Verify you're using the taker identity
dfx identity whoami
```

**Expected Result:** `taker`

#### Step 2.2: Fund Taker with TEST Tokens

```bash
# Check taker balance before minting
dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })"

# Fund the taker identity with TOKEN_B tokens so they can fill orders
# This is needed because the taker needs TOKEN_B tokens to pay for the order
dfx canister call test_token_b mint_tokens "(principal \"$TAKER_PRINCIPAL\", 1000000:nat)"

# Check taker balance after minting
dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })"

# Note: If you don't have $TAKER_PRINCIPAL, you can use:
# dfx canister call test_token_b mint_tokens "(principal \"$(dfx identity get-principal --identity taker)\", 1000000:nat)"

# Alternative: Mint for current identity (if you're already using taker identity)
# dfx canister call test_token_b mint_for_caller '(1000000:nat)'
```

**Expected Result:** `(variant { Ok = 1 : nat64 })` (1,000,000 TOKEN_B tokens minted = 10 tokens in 8 decimals)

**Expected Result:** Should see the order created in Scenario 1

#### Step 2.3: Discover Available Orders

```bash
# View available orders as taker
dfx canister call backend get_active_orders '()'
```

**Expected Result:** Should see the order created in Scenario 1

#### Step 2.4: Approve Backend Canister to Spend Taker's Tokens

```bash
# The taker needs to approve the backend canister to spend their TOKEN_B tokens
# This is required for ICRC-2 tokens (approve/transfer_from pattern)
dfx canister call test_token_b icrc2_approve "(record {
  from_subaccount = null;
  spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
  amount = 1000000000000:nat;
  expires_at = null;
  fee = null;
  memo = null;
  created_at_time = null;
})"

# Also approve the backend canister to spend taker's TOKEN_A tokens (for receiving)
dfx canister call test_token_a icrc2_approve "(record {
  from_subaccount = null;
  spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
  amount = 1000000000000:nat;
  expires_at = null;
  fee = null;
  memo = null;
  created_at_time = null;
})"

# Verify the approvals were set
echo "Taker's TOKEN_B allowance for backend:"
dfx canister call test_token_b icrc2_allowance "(record {
  account = record { owner = principal \"$TAKER_PRINCIPAL\"; subaccount = null };
  spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
})"

echo "Taker's TOKEN_A allowance for backend:"
dfx canister call test_token_a icrc2_allowance "(record {
  account = record { owner = principal \"$TAKER_PRINCIPAL\"; subaccount = null };
  spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
})"
```

**Expected Result:** Both approvals should return `(variant { Ok = 1 : nat64 })` and allowances should show the approved amounts.

**Troubleshooting:** If you get `InsufficientAllowance` errors when filling orders, ensure that:

1. The maker has approved the backend canister to spend their TOKEN_A tokens
2. The taker has approved the backend canister to spend their TOKEN_B tokens
3. The taker has also approved the backend canister to spend their TOKEN_A tokens (for receiving tokens)
4. All approvals have sufficient amounts (recommend using large amounts like 1,000,000,000,000 for testing)

#### Step 2.5: Check Balances Before Fill

```bash
# Check balances before filling the order
echo "=== BALANCES BEFORE FILL ==="
echo "Maker TOKEN_A Balance (what they're selling):"
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })"
echo "Taker TOKEN_B Balance (what they're spending):"
dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })"
echo "Maker TOKEN_B Balance (what they'll receive):"
dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })"
echo "Taker TOKEN_A Balance (what they'll receive):"
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })"
```

#### Step 2.6: Fill the Order

```bash
# Fill order ID 1
dfx canister call backend fill_order '(1: nat64)'
```

**Expected Result:** `(variant { Ok })`

#### Step 2.7: Check Balances After Fill

```bash
# Check balances after filling the order
echo "=== BALANCES AFTER FILL ==="
echo "Maker TOKEN_A Balance (should be reduced):"
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })"
echo "Taker TOKEN_B Balance (should be reduced):"
dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })"
echo "Maker TOKEN_B Balance (should be increased):"
dfx canister call test_token_b icrc1_balance_of "(record { owner = principal \"$MAKER_PRINCIPAL\" })"
echo "Taker TOKEN_A Balance (should be increased):"
dfx canister call test_token_a icrc1_balance_of "(record { owner = principal \"$TAKER_PRINCIPAL\" })"
```

#### Step 2.8: Verify Order was Filled

```bash
# Check order status - should no longer be active
dfx canister call backend get_active_orders '()'

# Get order details to confirm it's filled
dfx canister call backend get_order_by_id '(1: nat64)'
```

**Expected Result:**

- Active orders list should be empty
- Order 1 should show as filled in the system

---

## Scenario 3: Maker Cancels an Order

### User Story

> "As a maker, I want to cancel my limit order if market conditions change, so I can create a new order with better terms."

### Step-by-Step Test

#### Step 3.1: Switch Back to Maker

```bash
# Switch back to maker identity
dfx identity use maker
dfx identity whoami
```

**Expected Result:** `maker`

#### Step 3.2: Create Another Order to Cancel

```bash
# Create a new order for cancellation test
# Parameters explained:
# - principal "$MAKER_PRINCIPAL"     # receiver (who gets the taker asset)
# - principal "$TEST_TOKEN_A"        # maker_asset (test_token_a canister for local testing)
# - principal "$TEST_TOKEN_B"        # taker_asset (test_token_b canister for local testing)
# - 500000000: nat64                 # making_amount (5 TOKEN_A in 8 decimals)
# - 50000: nat64                     # taking_amount (0.0005 TOKEN_B in 8 decimals)
# - $(($(date +%s) + 7200))000000000: nat64 # expiration (2 hours from now in nanoseconds)
dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_TOKEN_A\",
  principal \"$TEST_TOKEN_B\",
  500000000: nat64,
  50000: nat64,
  $(($(date +%s) + 7200))000000000: nat64
)"
```

**Expected Result:** `(variant { Ok = 2 : nat64 })`

#### Step 3.3: Verify Order is Active

```bash
dfx canister call backend get_active_orders '()'
```

**Expected Result:** Should show order ID 2 as active

#### Step 3.4: Cancel the Order

```bash
# Cancel order ID 2
dfx canister call backend cancel_order '(2: nat64)'
```

**Expected Result:** `(variant { Ok })`

#### Step 3.5: Verify Cancellation

```bash
# Check active orders - should be empty
dfx canister call backend get_active_orders '()'

# Check order details
dfx canister call backend get_order_by_id '(2: nat64)'
```

**Expected Result:**

- No active orders
- Order 2 should be marked as cancelled

---

## Scenario 4: Frontend Testing

### User Story

> "As a user, I want to use a web interface to manage my limit orders without using command line tools."

### Step-by-Step Test

#### Step 4.1: Open Frontend Application

```bash
# Get frontend URL
echo "Open: http://localhost:4943/?canisterId=$(dfx canister id frontend)"
```

#### Step 4.2: Test Authentication

1. **Open the frontend URL in your browser**
2. **Click "Connect Wallet"**
3. **Verify mock authentication works**

**Expected Result:** You should see authenticated UI with maker/taker view options

#### Step 4.3: Test Maker Interface

1. **Navigate to "Maker" view**
2. **Fill out the order creation form:**
   - Maker Asset: `TOKEN_A` (test token)
   - Taker Asset: `TOKEN_B` (test token)
   - Making Amount: `15`
   - Taking Amount: `0.0015`
   - Expiration: Select future date
3. **Click "Create Order"**

**Expected Result:** Success message and order creation confirmation

#### Step 4.4: Test Taker Interface

1. **Navigate to "Taker" view**
2. **Verify order book displays your order**
3. **Click "Fill Order" on your order**
4. **Confirm the action**

**Expected Result:** Order fill confirmation and updated order book

---

## Scenario 5: Error Handling Tests

### User Story

> "As a developer, I want to verify the system handles errors gracefully and provides helpful error messages."

### Step-by-Step Test

#### Step 5.1: Test Invalid Order Creation

```bash
# Try to create order with zero amounts
# Parameters explained:
# - principal "$MAKER_PRINCIPAL"     # receiver (who gets the taker asset)
# - principal "$TEST_TOKEN_A"        # maker_asset (test_token_a canister for local testing)
# - principal "$TEST_TOKEN_B"        # taker_asset (test_token_b canister for local testing)
# - 0: nat64                         # making_amount (Invalid: zero amount)
# - 100000: nat64                    # taking_amount (0.001 TOKEN_B tokens in 8 decimals)
# - $(($(date +%s) + 3600))000000000: nat64 # expiration (1 hour from now in nanoseconds)
dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_TOKEN_A\",
  principal \"$TEST_TOKEN_B\",
  0: nat64,
  100000: nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

**Expected Result:** `(variant { Err = variant { InvalidAmount } })`

#### Step 5.2: Test Same Asset Pair

```bash
# Try to create order with same maker and taker asset
# Parameters explained:
# - principal "$MAKER_PRINCIPAL"     # receiver (who gets the taker asset)
# - principal "$TEST_TOKEN_A"        # maker_asset (test_token_a canister for local testing)
# - principal "$TEST_TOKEN_A"        # taker_asset (Invalid: same as maker_asset)
# - 1000000000: nat64                # making_amount (10 TOKEN_A in 8 decimals)
# - 100000: nat64                    # taking_amount (0.001 TOKEN_B tokens in 8 decimals)
# - $(($(date +%s) + 3600))000000000: nat64 # expiration (1 hour from now in nanoseconds)
dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$TEST_TOKEN_A\",
  principal \"$TEST_TOKEN_A\",
  1000000000: nat64,
  100000: nat64,
  $(($(date +%s) + 3600))000000000: nat64
)"
```

**Expected Result:** `(variant { Err = variant { InvalidAssetPair } })`

#### Step 5.3: Test Non-existent Order

```bash
# Try to fill non-existent order
dfx canister call backend fill_order '(999: nat64)'
```

**Expected Result:** `(variant { Err = variant { OrderNotFound } })`

#### Step 5.4: Test Unauthorized Cancellation

```bash
# Switch to taker identity
dfx identity use taker

# Try to cancel order created by default identity
dfx canister call backend cancel_order '(1: nat64)'
```

**Expected Result:** `(variant { Err = variant { Unauthorized } })`

---

## System Monitoring Tests

### Scenario 6: System Statistics

#### Step 6.1: Check System Health

```bash
# Switch back to default identity
dfx identity use default

# Get system statistics
dfx canister call backend get_system_statistics '()'
```

**Expected Result:** Statistics showing:

- `orders_created > 0`
- `orders_filled > 0`
- `orders_cancelled > 0`
- Volume data for tested tokens

#### Step 6.2: Test Query Functions

```bash
# Get orders by maker (should return orders you created)
dfx canister call backend get_orders_by_maker '(principal "your_principal_here")'

# Get orders by asset pair
dfx canister call backend get_orders_by_asset_pair "(
  principal \"$TEST_MAKER_ASSET\",
  principal \"$TEST_TAKER_ASSET\"
)"
```

**Expected Result:** Relevant orders returned based on query parameters

---

## Performance Tests

### Scenario 7: Load Testing

#### Step 7.1: Create Multiple Orders

```bash
# Create 5 orders quickly
# Parameters explained for each iteration:
# - principal "$MAKER_PRINCIPAL"     # receiver (who gets the taker asset)
# - principal "$TEST_TOKEN_A"        # maker_asset (test_token_a canister for local testing)
# - principal "$TEST_TOKEN_B"        # taker_asset (test_token_b canister for local testing)
# - $(($i * 100000000)): nat64      # making_amount (i * 10 TOKEN_A in 8 decimals)
# - $(($i * 10000)): nat64          # taking_amount (i * 0.0001 TOKEN_B tokens in 8 decimals)
# - $(($(date +%s) + $(($i + 1) * 3600)))000000000: nat64 # expiration (i+1 hours from now)
for i in {1..5}; do
  dfx canister call backend create_order "(
    principal \"$MAKER_PRINCIPAL\",
    principal \"$TEST_TOKEN_A\",
    principal \"$TEST_TOKEN_B\",
    $(($i * 100000000)): nat64,
    $(($i * 10000)): nat64,
    $(($(date +%s) + $(($i + 1) * 3600)))000000000: nat64
  )"
done
```

**Expected Result:** All orders should be created successfully

#### Step 7.2: Verify Performance

```bash
# Check all orders were created
dfx canister call backend get_active_orders '()'

# Verify system can handle the load
dfx canister call backend get_system_statistics '()'
```

**Expected Result:**

- All 5 orders should be listed
- System should remain responsive
- Statistics should reflect increased activity

---

## Test Results Checklist

### ✅ Core Functionality

- [ ] Order creation works with valid parameters
- [ ] Orders appear in active orders list
- [ ] Order filling works correctly
- [ ] Order cancellation works for maker
- [ ] Orders are removed from active list when filled/cancelled

### ✅ Error Handling

- [ ] Invalid parameters are rejected with appropriate errors
- [ ] Unauthorized operations are blocked
- [ ] Non-existent orders return proper errors
- [ ] System provides helpful error messages

### ✅ Frontend Integration

- [ ] Web interface loads correctly
- [ ] Mock authentication works
- [ ] Order creation form functions
- [ ] Order book displays correctly
- [ ] Fill order workflow works

### ✅ System Health

- [ ] Statistics tracking works
- [ ] Query functions return expected data
- [ ] System handles multiple orders
- [ ] Performance remains acceptable under load

---

## Troubleshooting

### Common Issues

#### Issue: "Canister not found"

**Solution:** Ensure `dfx start` is running and canisters are deployed

#### Issue: "Invalid principal" errors

**Solution:** Use valid principal IDs or `aaaaa-aa` for testing

#### Issue: Frontend not loading

**Solution:** Check that both `frontend` and `backend` canisters are deployed

#### Issue: Orders not appearing

**Solution:** Verify order creation succeeded and check the correct canister

#### Issue: "InsufficientAllowance" errors when filling orders

**Solution:** This is the most common issue with ICRC-2 tokens. Both the maker and taker must approve the backend canister before orders can be filled:

1. **Maker approval** (required for TOKEN_A transfers):

   ```bash
   dfx canister call test_token_a icrc2_approve "(record {
     from_subaccount = null;
     spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
     amount = 1000000000000:nat;
     expires_at = null;
     fee = null;
     memo = null;
     created_at_time = null;
   })"
   ```

2. **Taker approval** (required for TOKEN_B transfers):

   ```bash
   dfx canister call test_token_b icrc2_approve "(record {
     from_subaccount = null;
     spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
     amount = 1000000000000:nat;
     expires_at = null;
     fee = null;
     memo = null;
     created_at_time = null;
   })"
   ```

3. **Taker approval for TOKEN_A** (required for receiving tokens):
   ```bash
   dfx canister call test_token_a icrc2_approve "(record {
     from_subaccount = null;
     spender = record { owner = principal \"$BACKEND_CANISTER_ID\"; subaccount = null };
     amount = 1000000000000:nat;
     expires_at = null;
     fee = null;
     memo = null;
     created_at_time = null;
   })"
   ```

**Why this happens:** ICRC-2 tokens require explicit approval before a canister can transfer tokens from a user's account. The backend canister needs to transfer tokens from both the maker and taker accounts, so both must approve it.

### Reset Test Environment

```bash
# Stop dfx
dfx stop

# Clean and restart (WARNING: This deletes all data)
dfx start --clean

# Redeploy canisters
dfx deploy
```

---

## Success Criteria

### MVP is Working When:

1. ✅ **Orders can be created** with proper validation
2. ✅ **Orders can be discovered** through query functions
3. ✅ **Orders can be filled** atomically
4. ✅ **Orders can be cancelled** by their makers
5. ✅ **Frontend provides** basic UI for all operations
6. ✅ **System handles errors** gracefully
7. ✅ **Zero gas fees** for users (reverse gas model working)
8. ✅ **Real-time updates** through direct canister queries

### Ready for Demo When:

- All test scenarios pass
- Frontend is functional and user-friendly
- Error handling provides clear feedback
- System statistics show proper tracking
- Performance is acceptable for demonstration

**Congratulations!** 🎉 If all tests pass, your ICP Limit Order Protocol MVP is ready for demonstration and further development.
