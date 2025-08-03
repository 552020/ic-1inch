# Using Cross-Chain-Swap Repository as EVM Foundation

> **Purpose**: Guide for using the 1inch cross-chain-swap repository as the EVM foundation for ICP<>EVM Fusion+ architecture
> **Target**: ICP<>EVM Fusion+ implementation
> **Status**: ✅ **Deployable as-is for EVM side**

## 📋 **Table of Contents**

- [Overview](#overview)
- [Key Assessment](#key-assessment)
- [Critical Limitations](#critical-limitations)
- [Architecture Integration Strategy](#architecture-integration-strategy)
- [Deployment Instructions](#deployment-instructions)
- [Integration Points](#integration-points)
- [Complexity Assessment](#complexity-assessment)
- [Integration Workflow](#integration-workflow)
- [Recommended Deployment Strategy](#recommended-deployment-strategy)
- [Security Considerations](#security-considerations)
- [Implementation Checklist](#implementation-checklist)
- [Conclusion](#conclusion)

## 🔗 **Quick Reference Endpoints**

### **Deployment**

- [Deploy EVM Foundation](#step-1-deploy-evm-foundation)
- [Verify Deployment](#step-2-verify-deployment)
- [Configure for Your Use Case](#step-3-configure-for-your-use-case)

### **Integration Points**

- [Event Monitoring](#1-event-monitoring-use-as-is)
- [Factory Pattern](#2-factory-pattern-use-as-is)
- [Deterministic Addressing](#3-deterministic-addressing-use-as-is)

### **Implementation Phases**

- [Phase 1: EVM Foundation](#phase-1-deploy-evm-foundation-week-1)
- [Phase 2: ICP Side](#phase-2-build-icp-side-week-2-3)
- [Phase 3: Fusion+ Relayer](#phase-3-build-fusion-relayer-week-4)
- [Phase 4: Frontend](#phase-4-build-frontend-week-5-6)

### **Deployment Options**

- [Base Sepolia (Recommended)](#option-1-base-sepolia-recommended-for-poc)
- [Ethereum Sepolia](#option-2-ethereum-sepolia)

### **Security & Best Practices**

- [Security Considerations](#security-considerations)
- [Implementation Checklist](#implementation-checklist)

## Overview

The `cross-chain-swap` repository provides a **production-ready EVM foundation** for your ICP<>EVM Fusion+ system. While it implements basic Fusion (not Fusion+), it contains battle-tested smart contracts that can serve as the EVM side of your cross-chain architecture.

## 🎯 **Key Assessment: YES, Deployable as Foundation**

### ✅ **What Works Out of the Box:**

1. **Production-Ready Smart Contracts**: Already deployed on multiple chains (mainnet, polygon, arbitrum, etc.)
2. **Factory + Proxy Pattern**: Gas-efficient architecture with deterministic addressing
3. **Event System**: Robust event monitoring for asset locking verification
4. **Battle-Tested Code**: Used in production by 1inch Network
5. **Foundry Deployment**: Standard deployment scripts and tooling

### ⚠️ **Critical Limitations for ICP<>EVM:**

#### **1. This is Fusion (Basic), Not Fusion+ (Advanced)**

```solidity
// Repository: Basic Fusion
// - Resolver generates secret during execution
// - Simple escrow creation
// - Basic timelocks
// - EVM ↔ EVM only

// You Need: Fusion+ (from whitepaper)
// - Maker controls secret from beginning
// - Centralized relayer service
// - Enhanced security features
// - ICP ↔ EVM coordination
```

#### **2. Missing Fusion+ Components**

The repository lacks key Fusion+ features you'll need:

- ❌ **Centralized relayer service** (off-chain coordination)
- ❌ **Maker-controlled secret management**
- ❌ **Finality lock mechanisms**
- ❌ **Enhanced security features**
- ❌ **ICP-specific components**

#### **3. EVM-Only Architecture**

Current implementation is designed for **EVM ↔ EVM** chains:

```solidity
// Current: EVM ↔ EVM
address escrowAddress = Clones.predictDeterministicAddress(
    ESCROW_SRC_IMPLEMENTATION,
    salt,
    address(this)
);

// You Need: ICP ↔ EVM
// - Different addressing schemes (Principal IDs vs Addresses)
// - Different signature verification (Threshold ECDSA vs ECDSA)
// - Different cross-chain coordination (Off-chain relayer)
```

### **4. Independent Escrow Operation**

**✅ Important: EVM escrows work completely independently**

The EVM escrow contracts from this repository operate **independently** and don't need to know about other escrows or chains:

```solidity
// From contracts/EscrowSrc.sol
contract EscrowSrc {
    // No references to other chains or escrows
    // Works purely on EVM chain
    // Handles only its own state and logic
    // Self-contained: balances, timelocks, withdrawals
}
```

**Cross-chain coordination happens off-chain:**

```javascript
// Relayer monitors BOTH escrows independently
class FusionPlusRelayer {
  async monitorEscrows(orderHash) {
    // Monitor EVM escrow (this repo)
    const evmEscrow = await this.verifyEVMEscrow(orderHash);

    // Monitor ICP escrow (you build)
    const icpEscrow = await this.verifyICPEscrow(orderHash);

    // Relayer decides when both are ready
    if (evmEscrow && icpEscrow) {
      this.proceedWithSwap(orderHash);
    }
  }
}
```

**This means the EVM escrow from this repo will work perfectly as-is** - it doesn't need any modifications to work with your ICP escrow. The relayer service you build will handle the cross-chain coordination between them.

## 🏗️ **Architecture Integration Strategy**

### **Recommended Approach: Foundation + Extension**

```
┌─────────────────────────────────────────────────────────────┐
│                    YOUR ICP<>EVM FUSION+                   │
├─────────────────────────────────────────────────────────────┤
│ ICP Side (Build New)                                      │
│ ├── ICP Canisters (Rust)                                  │
│ ├── Threshold ECDSA                                       │
│ └── Principal ID Management                               │
│                                                            │
│ EVM Side (Use This Repo) ✅                              │
│ ├── EscrowFactory.sol (Deploy as-is)                     │
│ ├── EscrowSrc.sol (Deploy as-is)                         │
│ ├── EscrowDst.sol (Deploy as-is)                         │
│ └── Event System (Use as-is)                             │
│                                                            │
│ Fusion+ Relayer (Build New)                               │
│ ├── Off-chain coordination                                │
│ ├── Cross-chain monitoring                                │
│ └── Secret management                                     │
│                                                            │
│ Frontend (Build New)                                      │
│ ├── SIWE authentication                                   │
│ ├── Unified ECDSA                                         │
│ └── Cross-chain UX                                        │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 **Deployment Instructions**

### **Step 1: Deploy EVM Foundation**

```bash
# Clone the repository
git clone https://github.com/1inch/cross-chain-swap.git
cd cross-chain-swap

# Install dependencies
forge install

# Configure environment
export ETH_RPC="https://base-sepolia.g.alchemy.com/v2/YOUR_KEY"
export PRIVATE_KEY="your_private_key"
export CHAIN_ID="84532"  # Base Sepolia

# Deploy contracts
forge script script/DeployEscrowFactory.s.sol \
  --rpc-url $ETH_RPC \
  --private-key $PRIVATE_KEY \
  --broadcast \
  --verify
```

### **Step 2: Verify Deployment**

```bash
# Check deployment artifacts
cat deployments/base-sepolia/EscrowFactory.json

# Expected output:
{
  "transactions": [
    {
      "hash": "0x...",
      "contractAddress": "0x...",
      "function": "deploy"
    }
  ]
}
```

### **Step 3: Configure for Your Use Case**

```javascript
// Your Fusion+ relayer configuration
const EVM_CONFIG = {
  chainId: 84532, // Base Sepolia
  factoryAddress: "0x...", // From deployment
  rpcUrl: "https://base-sepolia.g.alchemy.com/v2/YOUR_KEY",
  eventSignatures: {
    srcEscrowCreated: "0x0e534c62f0afd2fa0f0fa71198e8aa2d549f24daf2bb47de0d5486c7ce9288ca",
    dstEscrowCreated: "0xc30e111dcc74fddc2c3a4d98ffb97adec4485c0a687946bf5b22c2a99c7ff96d",
  },
};
```

### **📝 Note: Use Examples for Testing**

The repository includes a comprehensive **examples/** folder with practical testing tools:

```bash
# 1. Test your EVM deployment using the examples
cd examples
cp config/config.json config/my-icp-evm-config.json
# Edit config for your Base Sepolia deployment

# 2. Run the automated testing script
chmod +x scripts/create_order.sh
./scripts/create_order.sh

# 3. What the examples provide:
# - create_order.sh: Complete workflow automation
# - config/config.json: Configuration template
# - script/CreateOrder.s.sol: Foundry script (16KB, 422 lines)
# - Mock deployment for local testing
# - Escrow lifecycle testing (deploy, withdraw, cancel)
# - Timelock and safety deposit testing
```

**Use these examples to:**

- ✅ Test your EVM escrow deployment before ICP integration
- ✅ Understand the complete escrow lifecycle
- ✅ Verify timelock and withdrawal mechanisms
- ✅ Debug any issues with the EVM foundation

## 🔧 **Integration Points**

### **1. Event Monitoring (Use As-Is)**

The repository's event system works perfectly for your use case:

```javascript
// From docs/relayer-asset-locking-monitoring.md
class EscrowMonitor {
  constructor(web3Providers) {
    this.ethProvider = web3Providers.ethereum;
    this.eventCache = new Map();
  }

  async startMonitoring() {
    // Monitor source chain events
    this.ethProvider.on("log", (log) => {
      if (log.topics[0] === SRC_ESCROW_CREATED_EVENT_SIGNATURE) {
        this.handleSrcEscrowCreated(log);
      }
    });
  }
}
```

### **2. Factory Pattern (Use As-Is)**

The factory + proxy pattern is ideal for your architecture:

```solidity
// From contracts/BaseEscrowFactory.sol
function _deployEscrow(bytes32 salt, uint256 value, address implementation)
    internal returns (address) {
    return Clones.cloneDeterministic(implementation, salt);
}
```

### **3. Deterministic Addressing (Use As-Is)**

CREATE2 deterministic addressing works for EVM side:

```solidity
// From contracts/BaseEscrowFactory.sol
address escrowAddress = Clones.predictDeterministicAddress(
    ESCROW_SRC_IMPLEMENTATION,
    salt,
    address(this)
);
```

## 📊 **Complexity Assessment**

| Component           | Current Repo | Your Needs | Effort | Status        |
| ------------------- | ------------ | ---------- | ------ | ------------- |
| EVM Smart Contracts | ✅ Ready     | ✅ Ready   | 0%     | **USE AS-IS** |
| ICP Canisters       | ❌ None      | ❌ Build   | 100%   | **BUILD NEW** |
| Fusion+ Relayer     | ❌ Basic     | ❌ Fusion+ | 80%    | **BUILD NEW** |
| Cross-Chain Logic   | ❌ EVM-only  | ❌ ICP+EVM | 90%    | **BUILD NEW** |
| Frontend            | ❌ None      | ❌ Build   | 100%   | **BUILD NEW** |

## 🔄 **Integration Workflow**

### **Phase 1: Deploy EVM Foundation (Week 1)**

```bash
# 1. Deploy existing contracts
forge script script/DeployEscrowFactory.s.sol --rpc-url $BASE_SEPOLIA_RPC --broadcast

# 2. Verify deployment
forge verify-contract 0x... EscrowFactory --chain-id 84532

# 3. Test basic functionality
forge test
```

### **Phase 2: Build ICP Side (Week 2-3)**

```rust
// Following docs/icp-fusion-plus-implementation-01.md
#[derive(CandidType, Deserialize)]
pub struct EscrowState {
    pub maker: Principal,
    pub taker: Principal,
    pub amount: u64,
    pub hashlock: Vec<u8>,
    pub status: EscrowStatus,
}
```

### **Phase 3: Build Fusion+ Relayer (Week 4)**

```typescript
// Following docs/icp-fusion-plus-implementation-02-relayer.md
class FusionPlusRelayer {
  async monitorEscrows(orderHash) {
    const srcEscrow = await this.verifySrcEscrow(orderHash);
    const dstEscrow = await this.verifyDstEscrow(orderHash);

    if (srcEscrow && dstEscrow) {
      this.orders.get(orderHash).status = "escrows_created";
    }
  }
}
```

### **Phase 4: Build Frontend (Week 5-6)**

```typescript
// Following docs/icp-fusion-plus-implementation-04-mechanical-turk.md
class MechanicalTurkFrontend {
  async createOrder(icpAmount: number, ethAmount: number) {
    // SIWE authentication
    const siweMessage = await this.createSIWEMessage();
    const signature = await this.signWithEthereum(siweMessage);

    // Create order
    const order = await this.createFusionPlusOrder({
      icpAmount,
      ethAmount,
      signature,
    });
  }
}
```

## 🎯 **Recommended Deployment Strategy**

### **Option 1: Base Sepolia (Recommended for PoC)**

```bash
# Advantages:
# - Low gas costs (~0.001 ETH per transaction vs ~0.01-0.05 ETH on Ethereum Sepolia)
# - Fast finality (~2 seconds vs ~12 seconds on Ethereum Sepolia)
# - Good tooling support
# - Already has 1inch contracts deployed
# - Better UX for cross-chain coordination

export CHAIN_ID="84532"  # Base Sepolia
export RPC_URL="https://base-sepolia.g.alchemy.com/v2/YOUR_KEY"
```

### **Option 2: Ethereum Sepolia**

```bash
# Advantages:
# - Higher security guarantees (L1 security vs L2)
# - Better liquidity for testing with real tokens
# - More realistic mainnet conditions
# - Standard testnet environment

export CHAIN_ID="11155111"  # Ethereum Sepolia
export RPC_URL="https://eth-sepolia.alchemyapi.io/v2/YOUR_KEY"
```

### **📊 Recommendation Matrix:**

| Factor                | Base Sepolia | Ethereum Sepolia |
| --------------------- | ------------ | ---------------- |
| **Gas Costs**         | ✅ Low       | ❌ High          |
| **Finality**          | ✅ Fast      | ❌ Slow          |
| **Security**          | ⚠️ L2        | ✅ L1            |
| **Liquidity**         | ⚠️ Limited   | ✅ Good          |
| **Tooling**           | ✅ Excellent | ✅ Good          |
| **1inch Integration** | ✅ Available | ❌ None          |

### **🎯 For Your ICP<>EVM PoC:**

**Base Sepolia is better because:**

1. **Cost-effective development** (multiple escrow deployments)
2. **Faster iteration** (quick finality for testing)
3. **Better UX** (faster cross-chain coordination)
4. **Existing 1inch infrastructure**

**Switch to Ethereum Sepolia if:**

- You need L1 security guarantees
- You're testing with significant amounts
- You want to simulate mainnet conditions more closely

For a **Mechanical Turk PoC**, Base Sepolia gives you the best development experience with minimal costs.

## 🔒 **Security Considerations**

### **Using Repository Contracts Safely:**

1. **Audited Code**: The contracts are already audited and used in production
2. **Event Verification**: Use the existing event system for asset locking verification
3. **Proxy Pattern**: Gas-efficient and secure deployment pattern
4. **Timelocks**: Built-in safety mechanisms for withdrawals and cancellations

### **Additional Security for Fusion+:**

1. **Finality Locks**: Implement Fusion+ finality lock mechanisms
2. **Enhanced Monitoring**: Build robust cross-chain monitoring
3. **Secret Management**: Implement secure secret sharing protocols
4. **Access Control**: Add proper access controls for relayer service

## 📋 **Implementation Checklist**

### **Week 1: EVM Foundation**

- [ ] Deploy `EscrowFactory.sol` on Base Sepolia
- [ ] Deploy `EscrowSrc.sol` implementation
- [ ] Deploy `EscrowDst.sol` implementation
- [ ] Verify all contracts on Etherscan
- [ ] Test basic escrow creation and withdrawal

### **Week 2-3: ICP Side**

- [ ] Build ICP escrow canisters
- [ ] Implement threshold ECDSA integration
- [ ] Build cross-chain identity mapping
- [ ] Test ICP escrow functionality

### **Week 4: Fusion+ Relayer**

- [ ] Build off-chain relayer service
- [ ] Implement cross-chain monitoring
- [ ] Add secret management
- [ ] Test end-to-end flow

### **Week 5-6: Frontend**

- [ ] Build SIWE authentication
- [ ] Implement unified ECDSA
- [ ] Create cross-chain UX
- [ ] Test complete user flow

## 🎉 **Conclusion**

**YES, you can absolutely use this repository as your EVM foundation!**

The `cross-chain-swap` repository provides:

- ✅ **Production-ready EVM smart contracts**
- ✅ **Battle-tested factory + proxy architecture**
- ✅ **Robust event monitoring system**
- ✅ **Gas-efficient deployment patterns**

You'll need to build:

- 🔨 **ICP side** (canisters, threshold ECDSA)
- 🔨 **Fusion+ relayer** (off-chain coordination)
- 🔨 **Frontend** (SIWE, unified UX)

This approach gives you a **solid, tested foundation** for the EVM side while allowing you to build the ICP-specific components and Fusion+ enhancements on top of it.

### **Next Steps:**

1. **Deploy the contracts** on Base Sepolia
2. **Follow our ICP implementation docs** for the ICP side
3. **Build the Fusion+ relayer** using our documentation
4. **Create the frontend** using our Mechanical Turk approach

This strategy minimizes risk while maximizing the value of existing, tested code.
