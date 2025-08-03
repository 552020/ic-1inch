# 1inch Fusion+ Architecture Deep Dive

_Context document for ICP implementation reference_

---

## 1. Core Architecture (from Whitepaper ✅)

**Source**: `docs/1_reference/1inch-analysis/1inch-fusion-plus-whitepaper.md` - RELIABLE (matches official PDF)

### **Flow Overview**

```
Maker → Signs Intent → Relayer → Resolver Network → HTLC Escrows → Atomic Completion
```

### **4-Phase Process**

1. **Announcement**: Maker signs order, Dutch auction starts
2. **Deposit**: Resolver creates src + dst escrows with secret hash
3. **Withdrawal**: Relayer reveals secret, resolver completes swap
4. **Recovery**: Timeout mechanisms, cancellation

### **Key Innovation: Intent + Professional Execution**

- Maker only signs once (EIP-712)
- Resolver handles all complexity
- Dutch auction ensures competitive pricing
- Self-custodial (no intermediaries hold funds)

---

## 2. Cross-Chain-Swap Implementation (EVM Side ✅)

**Source**: `cross-chain-swap-fork/README.md` + analysis docs - RELIABLE (official 1inch repo)

### **Smart Contract Architecture**

```
EscrowFactory (deploys implementations once)
├── EscrowSrc (holds maker tokens, source chain)
├── EscrowDst (holds resolver tokens, destination chain)
└── BaseEscrow (shared HTLC logic)
```

### **Factory + Proxy Pattern**

- Deploy implementations once per chain (gas efficient)
- Create lightweight proxies per swap (~50 bytes)
- Deterministic addresses via CREATE2
- **Critical**: Same parameters = same address across chains

### **Integration Point**: LOP Dual Callbacks

**Single Resolver Call Triggers Both Escrows:**

```
Resolver.deploySrc() → LOP.fillOrderArgs() → Two Callbacks:
1. LOP._postInteraction() → EscrowFactory → creates EscrowSrc
2. LOP.takerInteraction() → EscrowFactory → validates proofs (could trigger EscrowDst)
```

**Critical**: LOP makes **two callbacks** in one transaction, not separate calls

---

## 3. Limit Order Protocol Integration ✅

**Source**: Multiple docs in `docs/1_reference/limit-order-protocol/` - RELIABLE

### **LOP Role in Cross-Chain**

- **NOT** the cross-chain protocol itself
- **Entry point** for order execution
- **Signature validation** (EIP-712)
- **Token transfers** to escrows
- **Callback mechanism** via `_postInteraction()`

### **Key Functions**

```
Order Creation: EIP-712 signed intent
Order Execution: LOP.settleOrders() → escrow creation
Callback: _postInteraction() → EscrowFactory.createSrcEscrow()
```

---

## 4. Resolver Pattern Analysis ✅

**Source**: `fusion-resolver-example/` + analysis docs - RELIABLE

### **Resolver Responsibilities**

1. **Liquidity provision** (own tokens for swaps)
2. **Gas management** (pays for transactions)
3. **Secret management** (receives from relayer)
4. **Cross-chain coordination** (manual in basic version)
5. **Safety deposits** (incentive alignment)

### **Economic Model**

- Resolver deposits safety funds in both escrows
- Resolver profits from Dutch auction spread
- Safety deposits incentivize completion
- Public cancellation after timeouts

---

## 5. Key Technical Patterns ✅

### **Deterministic Addressing**

```solidity
bytes32 salt = keccak256(abi.encode(immutables));
address escrow = Clones.predictDeterministicAddress(implementation, salt);
```

### **HTLC Core**

- **Hashlock**: `keccak256(secret)` locks funds
- **Timelock**: Block timestamp-based stages
- **Atomic**: Both unlock with same secret or both cancel

### **Immutables Pattern**

Critical parameters encoded in salt:

- Token addresses & amounts
- Secret hash
- Timeout periods
- Recipient addresses

---

## 6. Missing for ICP Implementation

### **From Cross-Chain-Swap (Basic Fusion)**

❌ Fusion+ relayer service (centralized secret distribution)  
❌ Enhanced security (finality locks)  
❌ Non-EVM coordination patterns

### **Core Gaps for ICP**

❌ Principal ID ↔ Address mapping  
❌ ICRC-1/2 ↔ ERC-20 coordination  
❌ Chain Fusion integration patterns  
❌ Threshold ECDSA signing

---

## 7. Architecture Flow (Complete System)

### **Maker Side**

```
1. User signs order intent (EIP-712 on ETH, Principal on ICP)
2. Order broadcast to relayer network
3. Passive waiting for completion
```

### **Resolver Side**

```
1. Monitor order broadcasts
2. Compete in Dutch auction
3. Win order → create both escrows
4. Receive secret from relayer
5. Complete atomic swap
```

### **Critical Integration Points**

- **LOP Entry**: How orders enter the system
- **Secret Distribution**: Relayer → Resolver coordination
- **Escrow Verification**: Cross-chain state validation
- **Atomic Completion**: Simultaneous unlocking

---

## 8. Implementation Reliability Assessment

### **✅ VERIFIED Against Actual Code:**

**Core Architecture:**

- ✅ **Factory+Proxy Pattern**: Confirmed in `BaseEscrowFactory.sol` lines 35-41
- ✅ **Deterministic Addressing**: Confirmed via `Create2.computeAddress()`
- ✅ **LOP Integration**: Confirmed via `_postInteraction()` callback (lines 55-148)
- ✅ **HTLC Implementation**: Confirmed in `BaseEscrow.sol` line 48-50 (`_keccakBytes32(secret) != immutables.hashlock`)

**Flow Verification:**

- ✅ **EscrowSrc**: LOP creates via `_postInteraction()` → `_deployEscrow()`
- ⚠️ **EscrowDst**: Current implementation only validates Merkle proofs in `takerInteraction()`
- ✅ **Secret Validation**: Both use same `immutables.hashlock` check
- ✅ **Timelock Stages**: Complex timelock stages confirmed in both contracts
- ❌ **CORRECTION**: Original analysis missed LOP dual-callback pattern

### **⚠️ Documentation vs Code Gaps:**

- Our docs correctly describe the architecture
- Missing: Detailed timelock stage analysis
- Missing: Partial fills Merkle tree implementation details

### **❌ Still Missing for ICP:**

- Non-EVM address resolution patterns
- Cross-chain state verification mechanisms
- ICP-specific coordination patterns

---

_This document synthesizes multiple reliable sources. Cross-reference with `docs/1_reference/` for detailed technical specifications._
