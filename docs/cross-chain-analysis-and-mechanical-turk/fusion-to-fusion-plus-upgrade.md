# From Fusion to Fusion+: Upgrade Guide

> **Critical Discovery**: The repository implements **Fusion** (basic), while the whitepaper describes **Fusion+** (advanced). This document explains the differences and provides an upgrade path.

## Overview

After analyzing both the repository code and the official whitepaper, there are **significant differences** between what's implemented and what's described. Here's a complete comparison and upgrade guide.

## 🔍 **Key Differences: Fusion vs Fusion+**

### **1. Secret Management (Major Difference)**

#### **Fusion (Repository Implementation)**

```javascript
// Resolver chooses and controls the secret
const secret = generateRandomSecret();           // Resolver generates
const hashlock = keccak256(secret);             // Resolver computes
const extraData = abi.encode(hashlock, ...);    // Resolver provides in execution

// Flow:
1. Maker signs order (no secret involved)
2. Resolver generates secret during execution
3. Resolver creates escrows with their chosen hashlock
4. Maker reveals secret to resolver (wait, this doesn't make sense!)
```

#### **Fusion+ (Whitepaper Specification)**

```javascript
// Maker controls the secret from the beginning
const secret = generateRandomSecret();           // MAKER generates
const hashlock = keccak256(secret);             // MAKER computes
const order = { ...orderData, secretHash: hashlock }; // MAKER includes in order

// Flow:
1. Maker generates secret and includes hash in order
2. Resolver executes order with known hashlock
3. Relayer service verifies both escrows created
4. Relayer service shares secret with resolvers
```

### **2. Relayer Service Role**

#### **Fusion (Repository)**

```javascript
// Minimal relayer - just order distribution
class SimpleRelayer {
  distributeOrder(order, signature) {
    // Just broadcast to resolvers
    this.resolvers.forEach((resolver) => {
      resolver.notifyNewOrder(order, signature);
    });
  }
}
```

#### **Fusion+ (Whitepaper)**

```javascript
// Central relayer service with active role
class FusionPlusRelayer {
  // Phase 1: Order distribution
  distributeOrder(order, signature, secretHash) {
    this.orders.set(order.hash, {
      order,
      signature,
      secretHash,
      status: "announced",
    });
    this.broadcastToResolvers(order, signature);
  }

  // Phase 3: Secret management
  async handleSecretReveal(orderHash) {
    // Verify both escrows created
    const srcEscrow = await this.verifySrcEscrow(orderHash);
    const dstEscrow = await this.verifyDstEscrow(orderHash);

    if (srcEscrow && dstEscrow && this.finality_locks_passed()) {
      // Share secret with all resolvers
      const secret = this.secrets.get(orderHash);
      this.shareSecretWithResolvers(secret);
    }
  }
}
```

### **3. Resolver Requirements**

#### **Fusion (Repository)**

```javascript
// Permissionless - anyone can be a resolver
const resolver = new ResolverContract();
await resolver.executeSwap(order, signature, extraData);
```

#### **Fusion+ (Whitepaper)**

```javascript
// KYC/KYB required resolvers
// "Resolvers are entities that have passed KYC/KYB procedures
// and have legally enforced agreements with 1inch Network"

class KYCResolver {
  constructor(kycCredentials, legalAgreement) {
    this.verified = true;
    this.kycStatus = kycCredentials;
    this.agreement = legalAgreement;
  }
}
```

### **4. Phase Structure**

#### **Fusion (Repository)**

```javascript
// Simple 2-phase process
1. Resolver executes order → Creates both escrows
2. Secret revealed → Withdrawals complete
```

#### **Fusion+ (Whitepaper)**

```javascript
// Sophisticated 4-phase process
Phase 1: Announcement
  - Maker signs order with secret hash
  - Relayer distributes to resolvers
  - Dutch auction begins

Phase 2: Deposit
  - Resolver creates source escrow
  - Resolver creates destination escrow
  - Finality locks applied

Phase 3: Withdrawal
  - Relayer verifies escrows + finality
  - Relayer shares secret with resolvers
  - Resolvers complete withdrawals

Phase 4: Recovery (if needed)
  - Timeout-based cancellations
  - Safety deposit redistribution
```

### **5. Finality Locks (New in Fusion+)**

#### **Fusion (Repository)**

```solidity
// Basic timelocks only
enum Stage {
    SrcWithdrawal,
    SrcPublicWithdrawal,
    SrcCancellation,
    SrcPublicCancellation
}
```

#### **Fusion+ (Whitepaper)**

```solidity
// Enhanced with finality protection
enum Stage {
    FinalityLock,        // NEW: Chain reorganization protection
    SrcWithdrawal,
    SrcPublicWithdrawal,
    SrcCancellation,
    SrcPublicCancellation
}

// Finality lock prevents withdrawals until chain finality
modifier afterFinalityLock() {
    require(block.timestamp > deployTime + FINALITY_DELAY, "Finality lock active");
    _;
}
```

## 🛠️ **Upgrade Implementation Guide**

### **Phase 1: Secret Management Overhaul**

#### **Current (Fusion) - Resolver Controls Secret**

```solidity
// BaseEscrowFactory._postInteraction()
function _postInteraction(..., bytes calldata extraData) internal {
    (bytes32 hashlock, ...) = abi.decode(extraData, (...));
    // Resolver provides hashlock during execution
}
```

#### **Target (Fusion+) - Maker Controls Secret**

```solidity
// Enhanced Order Structure
struct FusionPlusOrder {
    uint256 salt;
    address maker;
    address receiver;
    address makerAsset;
    address takerAsset;
    uint256 makingAmount;
    uint256 takingAmount;
    bytes32 secretHash;      // NEW: Maker's secret hash
    MakerTraits makerTraits;
}

// Modified _postInteraction
function _postInteraction(..., bytes calldata extraData) internal {
    // Extract hashlock from order, not extraData
    bytes32 hashlock = order.secretHash;  // From maker's order
    // ... rest of implementation
}
```

#### **Implementation Steps**

```javascript
// 1. Update order structure
// 2. Modify signature verification to include secretHash
// 3. Update escrow creation to use order.secretHash
// 4. Remove hashlock from extraData
```

### **Phase 2: Relayer Service Enhancement**

#### **Current (Fusion) - Passive Relayer**

```javascript
class BasicRelayer {
  broadcastOrder(order) {
    // Simple broadcast
  }
}
```

#### **Target (Fusion+) - Active Relayer Service**

```javascript
class FusionPlusRelayer {
  constructor() {
    this.orders = new Map();
    this.secrets = new Map();
    this.escrowMonitor = new EscrowMonitor();
  }

  // Phase 1: Enhanced order management
  async announceOrder(order, signature, secret) {
    const secretHash = keccak256(secret);
    require(secretHash === order.secretHash, "Secret mismatch");

    this.orders.set(order.hash, {
      order,
      signature,
      status: "announced",
      createdAt: Date.now(),
    });

    this.secrets.set(order.hash, secret);
    await this.broadcastToResolvers(order, signature);
  }

  // Phase 2: Escrow monitoring
  async monitorEscrows(orderHash) {
    const srcEscrow = await this.verifySrcEscrow(orderHash);
    const dstEscrow = await this.verifyDstEscrow(orderHash);

    if (srcEscrow && dstEscrow) {
      this.orders.get(orderHash).status = "escrows_created";
      await this.startFinalityTimer(orderHash);
    }
  }

  // Phase 3: Secret revelation
  async revealSecret(orderHash) {
    const order = this.orders.get(orderHash);
    if (order.status === "finality_passed") {
      const secret = this.secrets.get(orderHash);
      await this.shareSecretWithResolvers(orderHash, secret);
    }
  }
}
```

### **Phase 3: Finality Lock Implementation**

#### **Add Finality Protection**

```solidity
contract FusionPlusEscrow is BaseEscrow {
    uint256 public immutable finalityDelay;
    uint256 public immutable deployTime;

    constructor(..., uint256 _finalityDelay) {
        finalityDelay = _finalityDelay;
        deployTime = block.timestamp;
    }

    modifier afterFinalityLock() {
        require(
            block.timestamp > deployTime + finalityDelay,
            "Finality lock active"
        );
        _;
    }

    function withdraw(bytes32 secret) external afterFinalityLock {
        // Existing withdraw logic
    }
}
```

### **Phase 4: KYC/Legal Framework**

#### **Add Resolver Verification**

```solidity
contract FusionPlusFactory is BaseEscrowFactory {
    mapping(address => bool) public verifiedResolvers;
    mapping(address => bytes32) public resolverKYCHash;

    modifier onlyVerifiedResolver() {
        require(verifiedResolvers[msg.sender], "Resolver not verified");
        _;
    }

    function verifyResolver(
        address resolver,
        bytes32 kycHash,
        bytes calldata kycProof
    ) external onlyOwner {
        // Verify KYC documentation
        require(verifyKYCProof(kycHash, kycProof), "Invalid KYC");

        verifiedResolvers[resolver] = true;
        resolverKYCHash[resolver] = kycHash;
    }

    function createEscrowSrc(...) external onlyVerifiedResolver {
        // Existing implementation
    }
}
```

## 📊 **Migration Timeline**

### **Option 1: Gradual Migration (Recommended)**

| Phase       | Duration  | Changes                    | Compatibility          |
| ----------- | --------- | -------------------------- | ---------------------- |
| **Phase 1** | 2-3 weeks | Secret management overhaul | ❌ Breaking change     |
| **Phase 2** | 3-4 weeks | Enhanced relayer service   | ✅ Backward compatible |
| **Phase 3** | 1-2 weeks | Finality locks             | ✅ Optional feature    |
| **Phase 4** | 2-3 weeks | KYC framework              | ✅ Optional feature    |

### **Option 2: Complete Rewrite (Faster)**

| Phase                | Duration  | Changes                        |
| -------------------- | --------- | ------------------------------ |
| **Complete Rewrite** | 6-8 weeks | Implement Fusion+ from scratch |
| **Testing**          | 2-3 weeks | Comprehensive testing          |
| **Migration**        | 1-2 weeks | Deploy and migrate             |

## 🎯 **For Your ICP Implementation**

### **Recommendation: Start with Fusion+ Directly**

Since you're building from scratch, **implement Fusion+ directly** rather than upgrading from Fusion:

#### **ICP Fusion+ Architecture**

```rust
// Canister 1: Relayer Service
#[ic_cdk::update]
async fn announce_order(order: FusionPlusOrder, secret: Vec<u8>) -> Result<(), String> {
    let secret_hash = sha256(&secret);
    if secret_hash != order.secret_hash {
        return Err("Secret hash mismatch".to_string());
    }

    // Store order and secret
    ORDERS.with(|orders| {
        orders.borrow_mut().insert(order.hash(), OrderState {
            order,
            secret,
            status: OrderStatus::Announced,
        });
    });

    // Notify resolvers
    notify_resolvers(order).await;
    Ok(())
}

// Canister 2: Escrow Contract
#[ic_cdk::update]
fn withdraw(secret: Vec<u8>) -> Result<(), String> {
    // Check finality lock
    let now = ic_cdk::api::time();
    if now < STATE.deploy_time + STATE.finality_delay {
        return Err("Finality lock active".to_string());
    }

    // Verify secret
    let hash = sha256(&secret);
    if hash != STATE.hashlock {
        return Err("Invalid secret".to_string());
    }

    // Transfer tokens
    transfer_tokens(STATE.recipient, STATE.amount)?;
    Ok(())
}
```

## 📋 **Summary: Fusion vs Fusion+**

| Aspect                    | Fusion (Repository) | Fusion+ (Whitepaper)    |
| ------------------------- | ------------------- | ----------------------- |
| **Secret Control**        | Resolver generates  | Maker generates         |
| **Relayer Role**          | Passive distributor | Active coordinator      |
| **Resolver Requirements** | Permissionless      | KYC/KYB required        |
| **Phase Structure**       | 2 phases            | 4 phases                |
| **Finality Protection**   | Basic timelocks     | Enhanced finality locks |
| **Complexity**            | Simpler             | More sophisticated      |
| **UX**                    | Basic               | Enhanced                |
| **Implementation Time**   | 4-6 weeks           | 8-12 weeks              |

## 🚀 **Next Steps**

1. **Decide on version**: Fusion (simpler) vs Fusion+ (advanced)
2. **Choose implementation strategy**: Upgrade existing vs build new
3. **Plan architecture**: Especially for ICP integration
4. **Start with core components**: Secret management and relayer service

**Recommendation for ICP**: Implement **Fusion+ directly** since you're building from scratch and want the best UX and features!
