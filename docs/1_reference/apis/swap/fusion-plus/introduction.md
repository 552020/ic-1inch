# 1inch Fusion+ API - Introduction

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/fusion-plus/introduction)_

---

## Overview

**1inch Fusion+ (intent-based atomic cross-chain swaps)**

> **Info**: For a comprehensive technical overview, refer to the [1inch Fusion+ whitepaper](https://1inch.io/assets/1inch-fusion-plus.pdf).

The 1inch Fusion+ API is a powerful solution for secure and efficient cross-chain swaps in DeFi that uses a creative architecture of Dutch auctions and automated recovery, all without relying on a single centralized custodian.

---

## Phases of a Fusion+ Swap

The process typically involves two main participants: the **maker**, who initiates the swap, and the **resolver**, who completes it; and has three phases. However, if any problems arise, there is an optional 4th recovery phase that can be used as a last resort.

### Phase 1: Announcement

The maker initiates the process by signing a 1inch Fusion+ order and broadcasting it to 1inch. This signals their intent to execute a cross-chain swap and sets the process in motion.

The order is distributed to all resolvers, triggering a **Dutch auction**. Resolvers compete by offering progressively better prices as the auction continues until a resolver locks in the order by initiating an escrow on the source chain.

### Phase 2: Deposit

The winning resolver deposits the maker's assets into an escrow contract on the source chain, and then deposits the corresponding assets into an escrow on the destination chain. Both escrows are linked by a **secret hash**, ensuring that assets can only be unlocked once the swap is completed. A small safety deposit is also assigned to each escrow, incentivizing the resolver to successfully complete the order.

### Phase 3: Withdrawal

Once both escrows are verified by the relayer, the secret is revealed, allowing the resolver to unlock the assets on the destination chain for the maker. The resolver then uses the same secret to retrieve their newly acquired assets on the source chain, finalizing the swap.

### Optional Phase: Recovery

In the event of a failed swap (e.g., if a party becomes unresponsive), the protocol includes a recovery mechanism. After the timelock expires, any resolver or any participating entity can cancel the swap and return the assets to their original owners. The safety deposit in each escrow is transferred to any resolver who steps in to complete the swap during this phase.

---

## Swap Flow

The Fusion+ swap process follows a structured flow:

1. **Maker signs intent** → Order broadcast to 1inch
2. **Dutch auction begins** → Resolvers compete for best price
3. **Winning resolver selected** → Escrows created on both chains
4. **Assets deposited** → Secret hash links both escrows
5. **Secret revealed** → Assets unlocked and swap completed
6. **Recovery (if needed)** → Timelock-based asset return

---

## The Partial Fill Feature

When an order is 100% filled, a single secret is used to finalize the transaction between two parties. However, when an order is only partially filled by different resolvers, revealing the secret to the public could let others claim the remainder of the order without completing their part.

To solve this, a **Merkle tree of secrets** is implemented for partial fills, which splits the order into equal parts and generates dedicated secrets for each portion of swap.

### How Partial Fills Work

For example, if an order is divided into four parts:

- The first secret is used for the first 25%
- The second for 50%
- And so on...

If a participant fills a part of the order, the next participant uses the corresponding secret based on the current progress to continue filling the order. This ensures that each participant can only fill their portion without exposing the rest of the order.

### Example Scenario

In a typical partial fill scenario:

- **1st secret**: Used for the initial 0-20% fill, marking the first stage
- **2nd & 3rd secrets**: Not used (order skips from 20% to 80%)
- **4th secret**: Used to fill the order from 20% to 80%
- **5th secret**: Used to complete the final 80-100% of the order

This ensures that the entire order is securely and progressively filled without compromising the security of unfilled portions.

---

## API Reference

For detailed information about each endpoint, refer to the [Fusion+ API Swagger section](https://portal.1inch.dev/documentation/apis/swap/fusion-plus/swagger).

### Available Endpoints

- **Get cross chain swap active orders**
- **Create cross chain swap order**
- **Get cross chain swap order status**
- **Cancel cross chain swap order**
- **Get cross chain swap order history**

---

## Key Features

### **Security**

- **Hash Time-Locked Contracts (HTLCs)** ensure atomic execution
- **Secret-based unlocking** prevents unauthorized access
- **Timelock mechanisms** provide recovery options
- **Merkle tree secrets** for partial fills

### **Efficiency**

- **Dutch auction system** for optimal pricing
- **Gas cost optimization** through resolver competition
- **Automated recovery** mechanisms
- **Partial fill support** for large orders

### **User Experience**

- **Intent-based design** - users only sign, resolvers handle execution
- **Cross-chain interoperability** across multiple blockchains
- **No upfront gas costs** for users
- **MEV protection** through Dutch auctions

---

## Supported Networks

Fusion+ supports cross-chain swaps across multiple blockchain networks, including:

- Ethereum Mainnet
- Arbitrum
- Avalanche
- BNB Chain
- Gnosis
- Sonic
- Optimism
- Polygon
- Base
- Unichain

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
