# MoleSwap Reference Implementation

_Cross-chain swap from TON to Ethereum using 1inch Fusion+ & BlockScout tracking_

---

## Project Overview

**Project:** MoleSwap  
**Event:** ETHGlobal Prague  
**Prize:** Winner of Blockscout Prize Pool + 1inch Extensions Prize Pool

### Description

A fully scalable, intent-based cross-chain swap protocol enabling seamless token transfers from TON to Ethereum, with support for both EVM and non-EVM networks. The solution utilizes 1inch Fusion+ and the 1inch Limit Order Protocol for efficient and decentralized trade execution.

---

## Architecture Components

### 🔧 Core Components

#### **TON Smart Contracts**

- Escrow contract factory and individual order contracts
- Manage source-side of atomic swap
- Hold TON tokens in escrow secured by hashlock

#### **EVM Smart Contracts**

- Escrow contract factory and order contracts on Ethereum (Sepolia testnet)
- Mirror TON contract behavior
- Handle ERC-20 tokens and ensure atomicity

#### **Hashlock Security**

- Swaps secured using hashlock-based mechanism
- Funds only released if correct secret is revealed
- Enables trustless execution between unrelated chains

#### **Backend Indexer**

- Custom indexer connecting to both TON and Ethereum RPC nodes
- Continuously tracks smart contract activity
- Indexes swaps and exposes real-time data
- Blockchain-agnostic, ready for additional chains

#### **Extended BlockScout**

- Customized to support non-EVM chains (TON)
- Extended schema and data models
- Cross-chain transaction linking
- New UI/UX for swap status, secrets, escrow details

---

## Atomic Swap Flow

1. ✅ **Maker creates escrow on TON** - locks tokens with hash of secret
2. ✅ **Taker responds** - creates matching escrow on Ethereum, locks ERC-20 tokens
3. ✅ **Maker claims ERC-20 tokens** - by revealing the secret
4. ✅ **Taker observes secret** - uses it to claim TON escrow
5. 🎯 **Taker withdraws safety deposit** - completes atomic swap cycle

---

## Key Innovations

### **Notable Hacks & Innovations**

- **Modified BlockScout** to support non-EVM chain (TON)
- **Unified cross-chain swap logic** in chain-agnostic backend indexer
- **Combined 1inch Fusion+ routing** with custom escrow logic
- **Intent-based order flow** with secure user-driven execution

### **Infrastructure & Deployment**

- **TON Testnet**
- **Ethereum Sepolia**
- **DigitalOcean and AWS** for backend services, indexer, and UI hosting

---

## Relevance to ICP Extension

### **Similarities**

- **Non-EVM to EVM** cross-chain swaps (TON→ETH vs ICP→ETH)
- **Hashlock-based security** mechanism
- **Escrow contract factories** on both chains
- **Backend indexer** for cross-chain tracking
- **Intent-based order flow**

### **Key Differences**

- **TON** vs **ICP** (different non-EVM architecture)
- **BlockScout extension** vs **custom tracking solution**
- **Specific token implementations**

---

## Lessons for ICP Implementation

1. **Start with escrows** - proven approach from MoleSwap
2. **Hashlock mechanism** works across different architectures
3. **Backend indexer** is crucial for cross-chain tracking
4. **Intent-based flow** can be adapted for different chains
5. **Testnet deployment** strategy (Sepolia + chain-specific testnet)

---

_Reference: ETHGlobal Prague Hackathon Project_
