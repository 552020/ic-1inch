# ICP Limit Order Protocol - Manual Testing Guide

## Overview

This document provides step-by-step instructions for manually testing the ICP Limit Order Protocol MVP. The guide covers the complete user journey from setup to order execution, designed for developers to verify functionality before deployment.

**Test Objective:** Verify that users can create, discover, and fill limit orders on a local ICP environment with zero gas fees.

---

## Prerequisites

### Required Software

- [dfx](https://github.com/dfinity/sdk) (Latest version)
- Node.js (v16+)
- Rust (Latest stable)
- Git

### Verify Installation

```bash
dfx --version          # Should show dfx 0.15.0 or later
node --version         # Should show v16+
cargo --version        # Should show rustc 1.70+
```

---

## Setup Instructions

### 1. Environment Setup

```bash
# Navigate to project directory
cd ic-1inch

# Start local IC replica (keep this running in separate terminal)
dfx start --clean

# In another terminal, deploy the backend canister
dfx deploy backend

# Deploy frontend assets
dfx deploy frontend

# Verify deployment
dfx canister status backend
dfx canister status frontend
```

**Expected Result:** Both canisters should show "Status: Running"

### 2. Get Canister URLs

```bash
# Get backend canister ID
dfx canister id backend

# Get frontend URL
echo "Frontend: http://localhost:4943/?canisterId=$(dfx canister id frontend)"

# Get backend Candid UI
echo "Backend UI: http://localhost:4943/?canisterId=$(dfx canister id __Candid_UI)&id=$(dfx canister id backend)"
```

**Expected Result:** You should have 3 URLs for testing

---

## Test Scenarios

## Scenario 1: Maker Creates a Limit Order

### User Story

> "As a maker, I want to create a limit order to sell 10 ICP for 0.001 ckBTC, so that I can get the exchange rate I want without actively monitoring the market."

### Step-by-Step Test

#### Step 1.1: Verify System is Ready

```bash
# Test backend connection
dfx canister call backend greet '("Test User")'
```

**Expected Result:** `("Hello, Test User!")`

#### Step 1.2: Create Your First Limit Order

```bash
# Create order: Sell 10 ICP for 0.001 ckBTC
dfx canister call backend create_order '(
  principal "rdmx6-jaaaa-aaaah-qcaiq-cai",  # receiver (can be same as maker)
  principal "aaaaa-aa",                      # maker_asset (using management canister for testing)
  principal "be2us-64aaa-aaaah-qcabq-cai",  # taker_asset (different principal for testing)
  1000000000: nat64,                        # making_amount (10 ICP in 8 decimals)
  100000: nat64,                            # taking_amount (0.001 ckBTC in 8 decimals)
  '$(echo $(date -d "+1 hour" +%s)000000000)': nat64,  # expiration (1 hour from now in nanoseconds)
  null                                      # allowed_taker (public order)
)'
```

**Expected Result:** `(variant { Ok = 1 : nat64 })` (Order ID 1 created)

#### Step 1.3: Verify Order was Created

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
dfx canister call backend get_active_orders_list '()'
```

**Expected Result:** Array with your order listed as active

---

## Scenario 2: Taker Discovers and Fills Order

### User Story

> "As a taker, I want to discover available limit orders and fill one that offers a good exchange rate, so I can get the tokens I need at the price I want."

### Step-by-Step Test

#### Step 2.1: Switch to Taker Identity

```bash
# Create a new identity for the taker
dfx identity new taker
dfx identity use taker

# Verify you're using the taker identity
dfx identity whoami
```

**Expected Result:** `taker`

#### Step 2.2: Discover Available Orders

```bash
# View available orders as taker
dfx canister call backend get_active_orders_list '()'
```

**Expected Result:** Should see the order created in Scenario 1

#### Step 2.3: Fill the Order

```bash
# Fill order ID 1
dfx canister call backend fill_order '(1: nat64)'
```

**Expected Result:** `(variant { Ok })`

#### Step 2.4: Verify Order was Filled

```bash
# Check order status - should no longer be active
dfx canister call backend get_active_orders_list '()'

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
# Switch back to default identity (maker)
dfx identity use default
dfx identity whoami
```

**Expected Result:** `default`

#### Step 3.2: Create Another Order to Cancel

```bash
# Create a new order for cancellation test
dfx canister call backend create_order '(
  principal "rdmx6-jaaaa-aaaah-qcaiq-cai",
  principal "aaaaa-aa",
  principal "be2us-64aaa-aaaah-qcabq-cai",
  500000000: nat64,                         # 5 ICP
  50000: nat64,                            # 0.0005 ckBTC
  '$(echo $(date -d "+2 hours" +%s)000000000)': nat64,
  null
)'
```

**Expected Result:** `(variant { Ok = 2 : nat64 })`

#### Step 3.3: Verify Order is Active

```bash
dfx canister call backend get_active_orders_list '()'
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
dfx canister call backend get_active_orders_list '()'

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
   - Maker Asset: `ICP`
   - Taker Asset: `ckBTC`
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
dfx canister call backend create_order '(
  principal "rdmx6-jaaaa-aaaah-qcaiq-cai",
  principal "aaaaa-aa",
  principal "be2us-64aaa-aaaah-qcabq-cai",
  0: nat64,                                # Invalid: zero amount
  100000: nat64,
  '$(echo $(date -d "+1 hour" +%s)000000000)': nat64,
  null
)'
```

**Expected Result:** `(variant { Err = variant { InvalidAmount } })`

#### Step 5.2: Test Same Asset Pair

```bash
# Try to create order with same maker and taker asset
dfx canister call backend create_order '(
  principal "rdmx6-jaaaa-aaaah-qcaiq-cai",
  principal "aaaaa-aa",
  principal "aaaaa-aa",                    # Invalid: same as maker_asset
  1000000000: nat64,
  100000: nat64,
  '$(echo $(date -d "+1 hour" +%s)000000000)': nat64,
  null
)'
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
dfx canister call backend get_orders_by_asset_pair '(
  principal "aaaaa-aa",
  principal "be2us-64aaa-aaaah-qcabq-cai"
)'
```

**Expected Result:** Relevant orders returned based on query parameters

---

## Performance Tests

### Scenario 7: Load Testing

#### Step 7.1: Create Multiple Orders

```bash
# Create 5 orders quickly
for i in {1..5}; do
  dfx canister call backend create_order "(
    principal \"rdmx6-jaaaa-aaaah-qcaiq-cai\",
    principal \"aaaaa-aa\",
    principal \"be2us-64aaa-aaaah-qcabq-cai\",
    $(($i * 100000000)): nat64,
    $(($i * 10000)): nat64,
    $(echo $(date -d "+$(($i + 1)) hours" +%s)000000000): nat64,
    null
  )"
done
```

**Expected Result:** All orders should be created successfully

#### Step 7.2: Verify Performance

```bash
# Check all orders were created
dfx canister call backend get_active_orders_list '()'

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
