# Minimal Limit Order Flow

## Overview

This document outlines the **minimal flow** required for a limit order to work successfully. We assume no cancellations, no errors, and focus only on the essential functions needed.

## 🎯 Core Flow

### **Step 1: Order Creation (Off-chain)**

- **Maker creates order** with details (assets, amounts, prices)
- **Maker signs order** using their private key
- **Order is published** to order book/marketplace

### **Step 2: Order Discovery (Off-chain)**

- **Taker finds order** on order book/marketplace
- **Taker validates order** (check prices, amounts, expiration)
- **Taker prepares transaction** with order data

### **Step 3: Order Execution (On-chain)**

- **Taker calls `fillOrder()`** with:
  - Order structure
  - Maker's signature (r, vs)
  - Fill amount
  - Taker traits
- **Contract verifies signature** (checks it's from maker)
- **Contract executes trade** (transfers tokens):
  - **Maker → Taker:** `makerAsset` (e.g., WETH)
  - **Taker → Maker:** `takerAsset` (e.g., USDC)
- **Contract emits `OrderFilled` event**

## 🔧 Required Functions

### **1. Order Hashing**

```solidity
function hashOrder(Order calldata order) external view returns(bytes32)
```

**Purpose:** Calculate order hash for signature verification

### **2. Order Filling**

```solidity
function fillOrder(
    Order calldata order,
    bytes32 r,
    bytes32 vs,
    uint256 amount,
    TakerTraits takerTraits
) external payable returns(uint256 makingAmount, uint256 takingAmount, bytes32 orderHash)
```

**Purpose:** Execute the limit order trade

### **3. Domain Separator**

```solidity
function DOMAIN_SEPARATOR() external view returns(bytes32)
```

**Purpose:** Get EIP-712 domain separator for order signing

## 📊 Data Flow

```
Maker Wallet → Order Creation → Order Signing → Order Publishing
                                                      ↓
Taker Wallet ← Order Discovery ← Order Validation ← Order Book
                                                      ↓
Contract ← fillOrder() ← Transaction Preparation ← Taker
                                                      ↓
Token Transfer ← Trade Execution ← Signature Verification
```

## 🎯 Minimal Success Path

### **For Maker:**

1. **Create order** (off-chain)
2. **Sign order** (off-chain)
3. **Publish order** (off-chain)

### **For Taker:**

1. **Find order** (off-chain)
2. **Call `fillOrder()`** (on-chain)

### **For Contract:**

1. **Verify signature** (on-chain)
2. **Execute trade** (on-chain):
   - Transfer `makerAsset` from maker to taker
   - Transfer `takerAsset` from taker to maker
3. **Emit event** (on-chain)

## 🔍 Key Events

### **OrderFilled**

```solidity
event OrderFilled(bytes32 orderHash, uint256 remainingAmount)
```

**Emitted when:** Order is successfully filled

## 💡 Flow Summary

**Minimal Flow = 3 Steps:**

1. **Create & Sign** (Maker)
2. **Find & Validate** (Taker)
3. **Execute** (Contract)

**Only 1 Main Function Needed:**

- `fillOrder()` - **Does everything in one call:**
  - Verifies signature
  - Transfers tokens (both directions)
  - Returns amounts and order hash
  - Emits event

**Helper Functions:**

- `hashOrder()` - For order hashing (off-chain)
- `DOMAIN_SEPARATOR()` - For signature creation (off-chain)

**That's it!** Everything else is optional for basic limit order functionality.

## 📊 Required Data Structures

### **1. Order Structure**

```solidity
struct Order {
    uint256 salt;           // Unique identifier
    address maker;          // Order creator
    address receiver;       // Who receives proceeds
    address makerAsset;     // Token being offered
    address takerAsset;     // Token being requested
    uint256 makingAmount;   // Amount of makerAsset
    uint256 takingAmount;   // Amount of takerAsset
    uint256 makerTraits;    // Packed configuration flags
}
```

### **2. Signature Components**

```javascript
// For EOA signatures
const r = "0x..."; // 32 bytes
const vs = "0x..."; // 32 bytes (s + v packed)
```

### **3. TakerTraits**

```solidity
struct TakerTraits {
    uint256 info; // Packed configuration flags
}
```

### **4. Amount**

```javascript
const amount = "500000000000000000"; // uint256 in wei/smallest unit
```
