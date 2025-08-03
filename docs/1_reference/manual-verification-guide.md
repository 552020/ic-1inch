# Manual Verification Guide

_Detailed explanation of manual operation for MVP demo_

---

## Overview

This guide explains how to manually perform all the functions that would be automated by relayers and resolvers in production. For our MVP demo, we'll act as both the user and the resolver manually.

---

## 1. Manual Order Submission

### **How We Submit Orders:**

#### **Direct API Call:**

```bash
curl -X POST "https://api.1inch.dev/fusion/relayer/v2.0/1/order/submit" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "order": {
      "salt": "0x...",
      "maker": "0x...",
      "receiver": "0x...",
      "makerAsset": "0x...",
      "takerAsset": "0x...",
      "makingAmount": "1000000000000000000",
      "takingAmount": "1000000000000000000",
      "makerTraits": "0x...",
      "secretHashes": ["0x..."]
    },
    "signature": "0x..."
  }'
```

### **Key Question: How does the API know about our ICP canisters?**

#### **Answer: The API doesn't need to know about our canisters directly!**

**How it works:**

1. **1inch API** receives the order (standard Fusion+ order)

   - **Technical:** `curl` POST to `/fusion/relayer/v2.0/{chain}/order/submit`
   - **Process:** Order stored in 1inch's order book

2. **Resolver** (us, manually) monitors the API for new orders

   - **Technical:** `curl` GET to `/fusion/orders/v2.0/{chain}/order/active`
   - **Process:** Polling API every few seconds to check for new orders
   - **Implementation:** Script or manual API calls to check order status

3. **Resolver** sees the order and decides to fill it

   - **Technical:** Parse JSON response from API call
   - **Process:** Review order details (tokens, amounts, timing)
   - **Decision:** Determine if profitable to fill this order

4. **Resolver** creates escrows on both chains (Ethereum + ICP)

   - **Ethereum:** Via 1inch Limit Order Protocol contracts
   - **ICP:** `dfx canister call` to our escrow canisters
   - **Process:** Deploy escrow contracts/canisters with same secret hash

5. **Resolver** executes the swap using both escrows
   - **Technical:** Reveal secret to both escrows
   - **Ethereum:** Call contract function with secret
   - **ICP:** `dfx canister call` with secret parameter
   - **Process:** Atomic execution on both chains

**The API just handles the order - the resolver (us) handles the cross-chain execution!**

---

## 2. How Orders Reach Our Canisters

### **The Flow:**

```
1. We submit order → 1inch API
2. We (as resolver) monitor API → See our own order
3. We create Ethereum escrow → Using existing 1inch contracts
4. We create ICP escrow → Using our canisters
5. We execute swap → Using both escrows
```

### **Our Canisters Are NOT Called by the API:**

**Important:** The 1inch API doesn't directly call our ICP canisters. Instead:

1. **API receives order** (standard Fusion+ format)
2. **We monitor API** for orders we want to fill
3. **We manually create escrows** on both chains
4. **We execute the swap** using both escrows

**Our canisters are part of the resolver's execution, not the API's order handling!**

---

## 3. Manual Cross-Chain Coordination

### **What Events We Need to Verify:**

#### **Ethereum Events (via Etherscan API):**

```bash
# Check if Ethereum escrow was created
curl "https://api.etherscan.io/api?module=logs&action=getLogs&address=ESCROW_ADDRESS&fromBlock=START_BLOCK&toBlock=latest&apikey=YOUR_KEY"

# Check if tokens were deposited
curl "https://api.etherscan.io/api?module=logs&action=getLogs&address=ESCROW_ADDRESS&topic0=0x...&apikey=YOUR_KEY"

# Check if secret was revealed
curl "https://api.etherscan.io/api?module=logs&action=getLogs&address=ESCROW_ADDRESS&topic0=0x...&apikey=YOUR_KEY"
```

#### **ICP Events (via our canister queries):**

```bash
# Check ICP escrow state
dfx canister call ESCROW_CANISTER getEscrowState '()'

# Check if tokens were deposited
dfx canister call ESCROW_CANISTER getBalance '()'

# Check if secret was verified
dfx canister call ESCROW_CANISTER verifySecret '(blob "SECRET_HERE")'
```

### **Events to Monitor:**

#### **Ethereum Side:**

- **EscrowCreated** - Escrow contract deployed
- **TokensDeposited** - Resolver deposited tokens
- **SecretRevealed** - Secret was revealed for withdrawal
- **TokensWithdrawn** - Tokens were withdrawn

#### **ICP Side:**

- **EscrowCreated** - Our canister was created
- **TokensDeposited** - Resolver deposited ICP tokens
- **SecretVerified** - Secret was verified
- **TokensWithdrawn** - ICP tokens were withdrawn

---

## 4. Manual Resolver Role

### **What "Act as Resolver Manually" Means:**

#### **Resolver Responsibilities (We Do These Manually):**

**1. Monitor Orders:**

```bash
# Check for new orders
curl "https://api.1inch.dev/fusion/orders/v2.0/1/order/active" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

**2. Provide Liquidity:**

```bash
# We use our own tokens for the demo
# We have ETH on Ethereum and ICP on ICP
# We act as the liquidity provider
```

**3. Create Escrows:**

```bash
# Create Ethereum escrow (using existing 1inch contracts)
# Create ICP escrow (using our canisters)
```

**4. Execute Transactions:**

```bash
# Deposit tokens in both escrows
# Reveal secret to complete swap
# Withdraw tokens from both escrows
```

**5. Handle Gas Fees:**

```bash
# Pay ETH gas fees for Ethereum transactions
# Pay ICP cycles for ICP canister calls
```

### **Manual Resolver Flow:**

```
1. We submit order (as user)
2. We monitor API (as resolver)
3. We see our own order
4. We decide to fill it (using our own tokens)
5. We create escrows on both chains
6. We execute the complete swap
7. We verify both sides completed
```

---

## 5. Manual Secret Management

### **What "Handle Secrets Manually" Means:**

#### **Secret Generation:**

```bash
# Generate random secret
openssl rand -hex 32
# Output: a1b2c3d4e5f6...

# Hash the secret
echo -n "a1b2c3d4e5f6..." | sha256sum
# Output: hash123...
```

#### **Secret Usage in Swap:**

**1. Create Order with Secret Hash:**

```json
{
  "order": {
    "secretHashes": ["hash123..."]
  }
}
```

**2. Create Escrows with Same Hash:**

```bash
# Ethereum escrow (1inch contract)
# Uses the same secret hash

# ICP escrow (our canister)
dfx canister call ESCROW_CANISTER createEscrow '(
  record {
    secretHash = blob "hash123...";
    amount = 1000000000;
    token = principal "TOKEN_CANISTER";
  }
)'
```

**3. Reveal Secret to Complete Swap:**

```bash
# Reveal on Ethereum
# (via 1inch contract)

# Reveal on ICP
dfx canister call ESCROW_CANISTER withdraw '(
  record {
    secret = blob "a1b2c3d4e5f6...";
    target = principal "USER_PRINCIPAL";
  }
)'
```

#### **Secret Verification:**

```bash
# Verify secret matches hash
dfx canister call ESCROW_CANISTER verifySecret '(
  record {
    secret = blob "a1b2c3d4e5f6...";
    expectedHash = blob "hash123...";
  }
)'
```

---

## Complete Manual Demo Flow

### **Step-by-Step Process:**

#### **1. Setup (One-time):**

```bash
# Deploy our ICP canisters
dfx deploy

# Get testnet tokens (Sepolia ETH + ICP testnet tokens)
# Setup API keys for 1inch and Etherscan
```

#### **2. Order Submission:**

```bash
# Generate secret and hash
SECRET=$(openssl rand -hex 32)
HASH=$(echo -n $SECRET | sha256sum | cut -d' ' -f1)

# Submit order via 1inch API
curl -X POST "https://api.1inch.dev/fusion/relayer/v2.0/1/order/submit" \
  -H "Authorization: Bearer $API_KEY" \
  -d '{"order": {...}, "secretHashes": ["'$HASH'"]}'
```

#### **3. Resolver Execution:**

```bash
# Monitor for our order
curl "https://api.1inch.dev/fusion/orders/v2.0/1/order/active" \
  -H "Authorization: Bearer $API_KEY"

# Create Ethereum escrow (via 1inch contract)
# Create ICP escrow (via our canister)
dfx canister call ESCROW_CANISTER createEscrow '(...)'

# Deposit tokens in both escrows
# Execute swap by revealing secret
dfx canister call ESCROW_CANISTER withdraw '(
  record { secret = blob "'$SECRET'"; ... }
)'
```

#### **4. Verification:**

```bash
# Check Ethereum events
curl "https://api.etherscan.io/api?module=logs&action=getLogs&..."

# Check ICP state
dfx canister call ESCROW_CANISTER getEscrowState '()'

# Verify tokens were transferred on both chains
```

---

## Key Insights

### **✅ Why This Works:**

1. **API handles orders** - Standard Fusion+ protocol
2. **We handle execution** - Manual resolver role
3. **Both chains synchronized** - Same secret, same hash
4. **Atomic execution** - Both succeed or both fail

### **✅ Demo Success Criteria:**

1. **Order submitted** via 1inch API
2. **Escrows created** on both chains
3. **Tokens deposited** in both escrows
4. **Secret revealed** to complete swap
5. **Tokens transferred** on both chains
6. **Verification successful** on both chains

### **✅ Production vs Demo:**

| Aspect                | Demo (Manual)       | Production (Automated) |
| --------------------- | ------------------- | ---------------------- |
| **Order Monitoring**  | Manual API calls    | Automated relayer      |
| **Escrow Creation**   | Manual transactions | Automated resolver     |
| **Secret Management** | Manual generation   | Automated handling     |
| **Verification**      | Manual checking     | Automated monitoring   |

---

_This manual process proves the core functionality works before building production automation._
