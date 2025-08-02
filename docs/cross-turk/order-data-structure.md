# Fusion Order Data Structure

> **Key Finding**: Most order data is **public** and available to all resolvers. The only **private** piece is the **maker's secret**, which is revealed after both escrows are created.

## Overview

When a user creates a cross-chain swap order, resolvers need specific data to execute the swap. This document details exactly what data is required and when it becomes available.

## Complete Order Data Structure

### 1. **OrderDetails** (Public - Available Immediately)

```solidity
struct OrderDetails {
    address maker;              // User who wants to swap
    address receiver;           // Who receives the destination tokens (usually maker)
    address srcToken;           // Token to swap FROM (on source chain)
    address dstToken;           // Token to swap TO (on destination chain)
    uint256 srcAmount;          // Amount of source tokens
    uint256 dstAmount;          // Amount of destination tokens expected
    uint256 srcSafetyDeposit;   // Safety deposit for source escrow
    uint256 dstSafetyDeposit;   // Safety deposit for destination escrow
    address[] resolvers;        // Whitelisted resolvers (can be empty for public)
    uint32 resolverFee;         // Fee for the resolver
    bytes auctionDetails;       // Auction parameters (see below)
}
```

### 2. **EscrowDetails** (Public - Available Immediately)

```solidity
struct EscrowDetails {
    bytes32 hashlock;           // Hash of the secret (keccak256(secret))
    Timelocks timelocks;        // Time windows for different operations
    bool fakeOrder;             // Testing flag
    bool allowMultipleFills;    // Whether order can be partially filled
}
```

### 3. **Escrow Immutables** (Computed by Resolver)

```solidity
struct Immutables {
    bytes32 orderHash;          // Hash of the limit order
    bytes32 hashlock;           // Hash of the secret
    Address maker;              // User's address
    Address taker;              // Resolver's address
    Address token;              // Token being held in this escrow
    uint256 amount;             // Amount being held
    uint256 safetyDeposit;      // Safety deposit amount
    Timelocks timelocks;        // Time constraints
}
```

### 4. **Auction Details** (Public - Available Immediately)

```solidity
struct AuctionDetails {
    uint24 gasBumpEstimate;     // Estimated gas cost
    uint32 gasPriceEstimate;    // Estimated gas price
    uint32 startTime;           // When auction begins
    uint24 duration;            // How long auction runs (seconds)
    uint32 delay;               // Initial delay before auction starts
    uint24 initialRateBump;     // Starting fee (basis points)
    bytes auctionPoints;        // Fee curve data
}
```

### 5. **Timelocks** (Public - Available Immediately)

```solidity
struct Timelocks {
    uint32 srcWithdrawal;       // When resolver can withdraw from source
    uint32 srcPublicWithdrawal; // When anyone can withdraw from source
    uint32 srcCancellation;     // When resolver can cancel source
    uint32 srcPublicCancellation; // When anyone can cancel source
    uint32 dstWithdrawal;       // When user can withdraw from destination
    uint32 dstPublicWithdrawal; // When anyone can withdraw from destination
    uint32 dstCancellation;     // When resolver can cancel destination
    uint32 dstPublicCancellation; // When anyone can cancel destination
}
```

## What Resolvers Get vs What They Need to Compute

### **📤 Data Provided by Relayer/Orderbook**

```javascript
// This data is distributed to all resolvers
const orderData = {
  // Basic swap parameters
  maker: "0x1234...5678",
  srcToken: "0xA0b8...6969", // USDC on Ethereum
  dstToken: "0x2791...4133", // USDT on Polygon
  srcAmount: "1000000000", // 1000 USDC
  dstAmount: "999000000", // 999 USDT

  // Economic parameters
  srcSafetyDeposit: "100000000000000000", // 0.1 ETH
  dstSafetyDeposit: "100000000000000000", // 0.1 MATIC
  resolverFee: "1000", // 0.1% in basis points

  // Auction parameters
  auctionDetails: {
    startTime: 1703123456,
    duration: 600, // 10 minutes
    initialRateBump: 500, // 0.5% starting fee
    // ... other auction params
  },

  // Security parameters
  hashlock: "0xabcd...ef01", // keccak256(secret) - PUBLIC
  timelocks: {
    srcWithdrawal: 3600, // 1 hour
    srcCancellation: 7200, // 2 hours
    // ... other timelock values
  },

  // Resolver whitelist (optional)
  resolvers: ["0xResolver1", "0xResolver2"], // Empty = public auction

  // Order signature
  signature: {
    r: "0x1234...",
    vs: "0x5678...",
  },
};
```

### **🔒 The ONLY Private Data: Maker's Secret**

```javascript
// This is kept private by the maker until both escrows are deployed
const secret = "0x123456789abcdef..."; // 32 bytes

// What everyone sees publicly:
const hashlock = keccak256(secret); // This is in the order data

// Revelation flow:
// 1. Resolver creates both escrows using hashlock
// 2. Maker reveals secret to resolver (off-chain)
// 3. Resolver uses secret to withdraw from both escrows
```

### **🧮 Data Computed by Resolver**

```javascript
class ResolverLogic {
  computeEscrowData(orderData) {
    // Compute deterministic escrow addresses
    const srcEscrowAddress = this.computeEscrowAddress(orderData, "source", this.resolverAddress);

    const dstEscrowAddress = this.computeEscrowAddress(orderData, "destination", this.resolverAddress);

    // Build immutables for source escrow
    const srcImmutables = {
      orderHash: this.computeOrderHash(orderData),
      hashlock: orderData.hashlock,
      maker: orderData.maker,
      taker: this.resolverAddress, // Resolver is taker
      token: orderData.srcToken,
      amount: orderData.srcAmount,
      safetyDeposit: orderData.srcSafetyDeposit,
      timelocks: orderData.timelocks,
    };

    // Build immutables for destination escrow
    const dstImmutables = {
      orderHash: this.computeOrderHash(orderData),
      hashlock: orderData.hashlock, // Same hashlock!
      maker: this.resolverAddress, // Resolver is maker on dst
      taker: orderData.maker, // User is taker on dst
      token: orderData.dstToken,
      amount: orderData.dstAmount,
      safetyDeposit: orderData.dstSafetyDeposit,
      timelocks: orderData.timelocks,
    };

    return { srcImmutables, dstImmutables };
  }
}
```

## Execution Flow with Data

### **Phase 1: Order Distribution (All Data Public Except Secret)**

```
1. User creates order with all parameters
2. User signs order (signature is public)
3. Relayer distributes order to all resolvers
4. Resolvers can compute escrow addresses deterministically
5. Secret remains private with maker
```

### **Phase 2: Resolver Execution**

```javascript
async function executeSwap(orderData) {
  // 1. Compute escrow parameters
  const { srcImmutables, dstImmutables } = this.computeEscrowData(orderData);

  // 2. Deploy source escrow (on source chain)
  const srcTx = await this.deploySrcEscrow(srcImmutables, orderData);

  // 3. Deploy destination escrow (on destination chain)
  const dstTx = await this.deployDstEscrow(dstImmutables);

  // 4. Wait for maker to reveal secret
  const secret = await this.waitForSecret(orderData.hashlock);

  // 5. Withdraw from both escrows using secret
  await this.withdrawFromSrc(secret);
  await this.withdrawFromDst(secret);
}
```

### **Phase 3: Secret Revelation & Completion**

```
1. Both escrows are deployed and funded
2. Maker reveals secret to winning resolver (off-chain)
3. Resolver uses secret to withdraw from both escrows
4. Swap is complete
```

## Security Considerations

### **What's Public vs Private**

| Data Type          | Visibility     | When Available                  |
| ------------------ | -------------- | ------------------------------- |
| Order parameters   | 🌍 Public      | Immediately                     |
| Auction details    | 🌍 Public      | Immediately                     |
| Hashlock           | 🌍 Public      | Immediately                     |
| Timelocks          | 🌍 Public      | Immediately                     |
| Resolver whitelist | 🌍 Public      | Immediately                     |
| Order signature    | 🌍 Public      | Immediately                     |
| **Secret**         | 🔒 **Private** | **After both escrows deployed** |

### **Why This Design is Secure**

1. **No Front-Running**: All resolvers have same information simultaneously
2. **Atomic Execution**: Secret is only revealed after both escrows exist
3. **Time-Bounded**: Timelocks ensure swap completes or gets cancelled
4. **Economic Security**: Safety deposits ensure resolver compliance

## Implementation Requirements

### **For Relayers/Orderbook Managers**

```javascript
// Must distribute complete order data
const orderDistribution = {
  orderDetails: completeOrderData,
  signature: makerSignature,
  chainInfo: {
    sourceChain: 1, // Ethereum
    destChain: 137, // Polygon
  },
  escrowAddresses: {
    srcFactory: "0x...",
    dstFactory: "0x...",
  },
};

// Broadcast to all whitelisted resolvers
await this.broadcastToResolvers(orderDistribution);
```

### **For Resolvers**

```javascript
// Must be able to:
1. Parse all order data structures
2. Compute deterministic escrow addresses
3. Deploy escrows on both chains
4. Handle secret revelation protocol
5. Execute withdrawals atomically
```

## Conclusion

**99% of the data needed by resolvers is public and available immediately**. The only private piece is the maker's secret, which is strategically revealed only after both escrows are properly deployed and funded. This ensures atomic execution while maintaining transparency in the auction process.

The resolver that wins the auction gets access to the secret and can complete the swap, while all other resolvers' transactions will fail since the escrows become unavailable once the first resolver succeeds.
