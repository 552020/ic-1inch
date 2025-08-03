# Context Overview: ICP Fusion+ Implementation

> **Quick Context**: Building 1inch Fusion+ atomic cross-chain swaps between Ethereum and ICP, preserving hashlock/timelock functionality for non-EVM chains.

---

## 🎯 **THE CORE CHALLENGE**

**Integration Question**: How do ICP canisters coordinate atomically with Ethereum escrow contracts to ensure both src/dst escrows are created and synchronized?

**Current Options:**

- Chain Fusion (ICP → Ethereum direct)
- Off-chain coordination (relayer-based)
- Hybrid approach (partial on-chain, partial off-chain)

**Reference**: Solana implementation exists but uses PDA system (not applicable to ICP)

---

## 🏗️ **ARCHITECTURE MAP**

### **Core Implementation (Focus)**

```
ICP CANISTERS                    ETHEREUM CONTRACTS
├── limit-order/                 ├── cross-chain-swap/ (deployed Base Sepolia)
│   └── Replicates LOP           │   ├── EscrowFactory
├── escrow_manager/              │   ├── EscrowSrc/EscrowDst
│   └── Replicates escrow logic  │   └── BaseEscrow + Timelocks
└── relayer/ (helper)            └── limit-order-protocol/ (deployed Base Sepolia)
    └── MVP coordination
```

### **Supporting Components**

```
OFFICIAL REFERENCES              YOUR ANALYSIS & DESIGN
├── internal/1inch/repos/        ├── .kiro/specs/ (multiple project specs)
│   ├── limit-order-protocol     ├── docs/masterplan/ (strategy)
│   ├── cross-chain-swap         ├── docs/1_1Inch/ (1inch analysis)
│   └── fusion-sdk               └── docs/cross-turk/ (abandoned simple impl)
└── fusion-resolver-example/
    └── Official resolver impl
```

### **Legacy/Reference**

- `evm/` - Abandoned first attempt, keep as reference
- `docs/cross-turk/` - Simple mechanical turk attempt, now abandoned

---

## 📚 **REFERENCE HIERARCHY**

### **1. Hackathon Requirements (PRIMARY FOCUS)**

**Core Requirements (MUST IMPLEMENT):**

- ✅ Preserve hashlock/timelock for non-EVM implementation
- ✅ Bidirectional swaps (ETH ↔ ICP)
- ✅ On-chain execution demo (mainnet/testnet)

**Stretch Goals (NOT required):** UI, Partial fills

### **2. Technical Specification**

- **Primary**: `docs/1_1Inch/1inch-fusion-plus-whitepaper.md`
  - Section 2.2: Secret management system
  - 4-Phase Flow: Announcement → Deposit → Withdrawal → Recovery
  - ❌ Section 2.3: Dutch Auction (NOT required for hackathon)
  - ❌ Section 2.5: Partial fills (stretch goal only)

### **3. Official 1inch Implementations**

- **Solidity Reference**: `internal/1inch/repos/cross-chain-swap/`
  - EscrowFactory pattern
  - BaseEscrow timelock system
  - Safety deposit mechanisms
- **LOP Reference**: `internal/1inch/repos/limit-order-protocol/`
  - Fill order pattern
  - Order validation
  - Signature verification
- **Resolver Pattern**: `fusion-resolver-example/`
  - How resolvers interact with escrows
  - Settlement examples

### **4. Your Design Decisions**

- **All Project Specs**: `.kiro/specs/` (5 projects)
  - `icp-limit-order-protocol-mvp/` - Core LOP implementation
  - `escrow_manager_fusion+/` - HTLC escrow logic
  - `relayer_canister_fusion+/` - Order coordination
  - `fusion-plus-icp-mvp/` - Overall ICP integration
  - `fusion-plus-mechanical-turk/` - Simple implementation attempt
- **Architecture Strategy**: `docs/masterplan/docs/implementation_architecture.md`
- **Integration Strategy**: `docs/masterplan/docs/icp_cross_chain_coordination.md`

---

## 🔧 **IMPLEMENTATION STATUS QUICK REF**

### **✅ Foundation Complete**

```
src/escrow_manager/     → HTLC logic, Chain Fusion integration
src/limit-order/        → Core LOP functions, hashlock/timelock
cross-chain-swap/       → Complete Solidity implementation (deployed)
```

### **❌ Missing Core Features (HACKATHON REQUIREMENTS)**

```
4-Phase Orchestration  → Components exist, no coordination
Secret Distribution    → Manual only, no conditional transmission
Bidirectional Flow     → ICP→ETH partial, ETH→ICP basic only
Integration Layer      → THE core challenge
```

### **❌ Missing Stretch Goals (Optional)**

```
Dutch Auction          → No price curves/time decay (NOT required)
Partial Fills          → No Merkle tree secrets (stretch goal)
```

### **🎯 THE INTEGRATION CHALLENGE**

```
Problem: How to ensure atomic escrow creation across ICP ↔ Ethereum?

Current Approach:
├── ICP escrow_manager creates ICP-side escrow
├── ??? coordination mechanism ???
└── Ethereum cross-chain-swap creates EVM-side escrow

Options:
1. Chain Fusion (src/escrow_manager/src/chain_fusion.rs)
2. Off-chain relayer coordination
3. Hybrid approach
```

---

## 🚀 **WORKING WITH THIS PROJECT**

### **When Implementing Features:**

**Cross-Chain Coordination (CORE REQUIREMENT)** → Check:

- `docs/masterplan/docs/icp_cross_chain_coordination.md`
- `src/escrow_manager/src/chain_fusion.rs`
- `cross-chain-swap/contracts/BaseEscrow.sol`

**Secret Management (CORE REQUIREMENT)** → Check:

- Whitepaper Section 2.2
- `src/relayer/src/lib.rs` (order coordination)
- `.kiro/specs/relayer_canister_fusion+/`

**Bidirectional Swaps (CORE REQUIREMENT)** → Check:

- `.kiro/specs/fusion-plus-icp-mvp/`
- `src/escrow_manager/src/lib.rs` (escrow creation)
- `cross-chain-swap/scripts/` (testing patterns)

**Partial Fills (STRETCH GOAL)** → Check:

- Whitepaper Section 2.5
- `internal/1inch/repos/limit-order-protocol/` partial logic
- Implement in: `src/limit-order/src/` (new partial_fills.rs)

### **When Debugging:**

**Escrow Issues** → Check:

- `src/escrow_manager/src/lib.rs` (ICP side)
- `cross-chain-swap/contracts/` (Ethereum side)
- `cross-chain-swap/scripts/` (deployment/testing)

**Order Issues** → Check:

- `src/limit-order/src/lib.rs` (core functions)
- `internal/1inch/repos/limit-order-protocol/` (reference pattern)

**Integration Issues** → Check:

- `src/escrow_manager/src/chain_fusion.rs` (Chain Fusion)
- `src/relayer/src/lib.rs` (coordination helper)

---

## 💡 **KEY ICP ADAPTATIONS**

### **Design Decisions Made:**

```
1. On-chain orders (vs EIP-712 off-chain)
   └── Reason: ICP reverse gas model enables gasless UX

2. Conservative timelock strategy
   └── ICP escrow: Full duration, EVM escrow: Earlier expiration

3. Single canister architecture
   └── vs Factory pattern: Cost-efficient for MVP

4. Chain Fusion integration
   └── Direct ICP → Ethereum coordination capability
```

### **Still Deciding:**

```
1. Integration mechanism (Chain Fusion vs off-chain vs hybrid)
2. Deterministic address generation (no PDA equivalent on ICP)
3. Secret distribution system (on-chain vs off-chain relayer)
```

---

## 🔗 **QUICK FILE NAVIGATION**

### **Core Implementation Files:**

- `src/escrow_manager/src/lib.rs` - Main escrow coordination
- `src/limit-order/src/lib.rs` - Core LOP functions
- `src/relayer/src/lib.rs` - Order submission/coordination
- `cross-chain-swap/contracts/BaseEscrow.sol` - Ethereum escrow logic

### **Key Reference Documents:**

- `docs/1_1Inch/1inch-fusion-plus-whitepaper.md` - The specification
- `.kiro/specs/icp-limit-order-protocol-mvp/design.md` - Your architecture
- `internal/implementation-status-assessment.md` - Current status vs whitepaper

### **Testing & Deployment:**

- `cross-chain-swap/scripts/` - Ethereum deployment scripts
- `scripts/limit-order/` - ICP testing scripts
- Base Sepolia: Both limit-order-protocol and cross-chain-swap deployed

---

**Last Updated**: Current implementation has solid foundations but needs Dutch auction, partial fills, and THE integration solution between ICP and Ethereum.
