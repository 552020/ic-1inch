# EVM-ICP Integration Issues

## Core Question: Cross-Chain-Swap Reusability

**Problem**: 1inch cross-chain-swap assumes both escrows on EVM. We need one EVM, one ICP.

---

## Issue 1: Factory Escrow Creation Mismatch

### Current Flow:

```
Resolver calls LOP.fillOrderArgs() which triggers:
1. LOP._postInteraction() → EscrowFactory → creates EscrowSrc (EVM)
2. LOP.takerInteraction() → EscrowFactory → validates Merkle proofs only
```

### Our Required Flow:

```
Resolver calls LOP.fillOrderArgs() which should trigger:
1. LOP._postInteraction() → EscrowFactory → creates EscrowSrc (EVM) ✅
2. LOP.takerInteraction() → Modified EscrowFactory → triggers ICP escrow creation
```

### Questions:

1. **Can we modify EscrowFactory to only create EscrowSrc?**
2. **How do we trigger ICP escrow creation atomically?**
3. **Who coordinates the EVM→ICP escrow creation?**

---

## Issue 2: Cross-Chain Coordination

### Current: EVM Factory manages both escrows

### Needed: EVM Factory + ICP Canister coordination

### Options:

**A. Chain Fusion Integration**

- ICP canister monitors EVM events
- Creates ICP escrow when EVM escrow detected

**B. Relayer Coordination**

- External service triggers both
- Handles failure recovery

**C. Modified Factory**

- Fork cross-chain-swap factory
- Add ICP integration hooks

### Questions:

1. **Which coordination pattern is most reliable?**
2. **How do we handle partial failures?**

---

## Issue 3: Deterministic Addressing

### Current: CREATE2 for predictable EVM addresses

### Needed: EVM ↔ ICP address coordination

### Questions:

1. **How do we ensure same parameters = same escrow across chains?**
2. **Can ICP canister addresses be deterministic?**

---

## Issue 4: Order Integration

### Current: EIP-712 orders on EVM LOP

### Our Approach: Store EIP-712 on ICP, let LOP verify on EVM

### Questions:

1. **Can we use unmodified LOP for EVM side?**
2. **How does ICP limit-order canister integrate with EVM escrow creation?**

---

## Next Steps

**Priority 1**: Analyze EscrowFactory modification requirements
**Priority 2**: Design coordination mechanism (Chain Fusion vs Relayer)
**Priority 3**: Test deterministic addressing patterns

---

_MVP Focus: Get basic coordination working, optimize later_
