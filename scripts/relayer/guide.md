# 🚀 1inch Fusion+ Relayer - Complete Guide

This guide walks you through testing the clean relayer canister that mocks the 1inch Fusion+ API.

## 📋 Overview

The relayer canister implements 6 core 1inch API endpoints:

- `POST /fusion-plus/relayer/v1.0/submit` → `fusion_plus_relayer_submit`
- `GET /fusion-plus/orders/v1.0/order/active` → `fusion_plus_orders_active`
- `GET /fusion-plus/orders/v1.0/order/status/{orderHash}` → `fusion_plus_order_status`
- `GET /fusion-plus/orders/v1.0/order/escrow` → `fusion_plus_order_escrow`
- `GET /fusion-plus/orders/v1.0/order/secrets/{orderHash}` → `fusion_plus_order_secrets`
- `GET /fusion-plus/orders/v1.0/order/ready-to-accept-secret-fills/{orderHash}` → `fusion_plus_order_ready_to_accept_secret_fills`

## 🛠️ Prerequisites

1. **DFX installed** and running: `dfx start --clean`
2. **Two identities** for testing:
   - **Maker identity** (submits orders)
   - **Taker identity** (fetches and accepts orders)

## 🆔 Step 1: Create Identities

Create separate identities for maker and taker to simulate real-world usage:

```bash
# Create maker identity
dfx identity new maker --storage-mode=plaintext
dfx identity use maker
echo "🏭 Maker Principal: $(dfx identity get-principal)"

# Create taker identity
dfx identity new taker --storage-mode=plaintext
dfx identity use taker
echo "🛒 Taker Principal: $(dfx identity get-principal)"

# Switch back to default for deployment
dfx identity use default
```

## 🏗️ Step 2: Deploy Relayer

Run the setup script to deploy the relayer canister:

```bash
./scripts/relayer/setup_and_compile.sh
```

This will:

- ✅ Compile the relayer canister
- ✅ Deploy it locally
- ✅ Generate Candid declarations
- ✅ Verify deployment

## 📋 Step 3: Maker Submits Order

Switch to maker identity and submit an order:

```bash
# Switch to maker identity
dfx identity use maker

# Submit order using our script
./scripts/relayer/maker-submit-order.sh
```

The script will:

- ✅ Create a sample cross-chain order (ETH → USDC)
- ✅ Submit it using `fusion_plus_relayer_submit`
- ✅ Return an order ID (hash)
- ✅ Display next steps

**Sample Output:**

```
✅ Order submitted successfully!
🆔 Order ID: 0xa1b2c3d4e5f6...
```

## 🔍 Step 4: Taker Fetches Orders

Switch to taker identity and fetch available orders:

```bash
# Switch to taker identity
dfx identity use taker

# Fetch active orders using our script
./scripts/relayer/taker-fetch-orders.sh
```

The script will:

- ✅ Fetch all active orders using `fusion_plus_orders_active`
- ✅ Display formatted order details
- ✅ Show order IDs for easy reference
- ✅ Provide helpful next-step commands

**Sample Output:**

```
✅ Successfully fetched active orders!
📋 Found 1 active order(s):

🔸 Order:
  id = "0xa1b2c3d4e5f6..."
  maker_eth_address = "0x1234..."
  making_amount = "1000000000000000000"
  taking_amount = "2000000000000000000"
  status = Pending
```

## 🔧 Step 5: Manual Order Investigation

Once you have order IDs, you can manually investigate them:

### Check Order Status

```bash
dfx canister call relayer fusion_plus_order_status '("0xa1b2c3d4e5f6...")'
```

### Check Order Escrow

```bash
dfx canister call relayer fusion_plus_order_escrow '("0xa1b2c3d4e5f6...", 1)'
```

### Get Order Secrets

```bash
dfx canister call relayer fusion_plus_order_secrets '("0xa1b2c3d4e5f6...")'
```

### Check if Ready for Secret Fills

```bash
dfx canister call relayer fusion_plus_order_ready_to_accept_secret_fills '("0xa1b2c3d4e5f6...")'
```

## 📊 Complete Workflow Example

Here's a complete test sequence:

```bash
# 1. Setup environment
dfx start --clean
./scripts/relayer/setup_and_compile.sh

# 2. Create and use maker identity
dfx identity use maker
./scripts/relayer/maker-submit-order.sh
# Note the returned Order ID

# 3. Create and use taker identity
dfx identity use taker
./scripts/relayer/taker-fetch-orders.sh

# 4. Investigate specific order (replace with actual order ID)
dfx canister call relayer fusion_plus_order_status '("0xa1b2c3d4e5f6...")'
dfx canister call relayer fusion_plus_order_secrets '("0xa1b2c3d4e5f6...")'
```

## 🎯 Key Features Tested

### ✅ 1inch API Compliance

- Exact endpoint naming (`fusion_plus_*`)
- Correct JSON payload structure
- Proper error handling

### ✅ Order Lifecycle

- **Pending**: Order created, waiting for taker
- **Accepted**: Taker accepted the order
- **Completed**: Swap successful
- **Failed**: Swap failed
- **Cancelled**: Order cancelled

### ✅ Security Features

- Principal-based identity management
- Order hash generation for uniqueness
- Secret hash management for HTLC

### ✅ Data Validation

- Ethereum address validation
- Amount validation (non-zero, valid format)
- Salt validation (non-empty)
- Signature format validation

## 🚨 Troubleshooting

### DFX Not Running

```bash
dfx start --clean
```

### Canister Not Found

```bash
# Redeploy the relayer
./scripts/relayer/setup_and_compile.sh
```

### Permission Denied on Scripts

```bash
chmod +x scripts/relayer/*.sh
```

### No Orders Found

```bash
# Make sure you submitted an order first as maker
dfx identity use maker
./scripts/relayer/maker-submit-order.sh
```

## 📁 Script Files

- **`setup_and_compile.sh`** - Deploy and setup relayer canister
- **`maker-submit-order.sh`** - Submit orders as maker
- **`taker-fetch-orders.sh`** - Fetch active orders as taker
- **`check-escrow-status.sh`** - Check escrow status (legacy)

## 🎯 Next Steps

1. **Cross-Chain Integration**: Connect with actual EVM chains
2. **HTLC Implementation**: Add real hashlock/timelock contracts
3. **Frontend Integration**: Build web interface
4. **Production Deployment**: Deploy to IC mainnet

---

**🎉 Happy testing with the 1inch Fusion+ Relayer!**
