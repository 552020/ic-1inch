# Delegation Mechanism and Gasless Order Implementation Challenge

## Issue Summary

**Problem:** How to implement both the delegation mechanism (resolver authorization) and the gasless, asynchronous order pattern on ICP that enables makers to create off-chain orders that resolvers can execute on-chain without maker involvement.

## Background

### 1inch Fusion+ Architecture

- **Maker signs EIP-712 order** off-chain (no gas fees, no blockchain transaction)
- **Resolver executes on-chain** (pays gas, submits transaction)
- **Off-chain intent → On-chain execution** pattern
- **Dutch auction mechanism** enabled by signed orders
- **Delegation mechanism** - Resolver proves maker authorized the swap

### The Challenge for ICP Integration

- **ICP has different architecture** than Ethereum (no gas fees, different authentication)
- **How do we enable the same gasless, asynchronous order pattern** on ICP?
- **How do we implement delegation mechanism** for resolver authorization?
- **How do resolvers execute maker's intent** without maker being online?

## Core Problem

**Both delegation and gasless patterns are fundamental to 1inch Fusion+:**

### Delegation Mechanism

1. **Maker authorization** - Resolver must prove maker authorized the swap
2. **Token locking** - Resolver locks maker's tokens in source escrow
3. **Cross-chain coordination** - Same hashlock/timelock on both chains
4. **Atomic execution** - Either both sides complete or both refund

### Gasless Pattern

1. **Off-chain order creation** - Maker signs once, no blockchain transaction
2. **On-chain execution** - Resolver executes anytime, pays costs
3. **Intent preservation** - Maker's intent is cryptographically verifiable
4. **Competitive execution** - Multiple resolvers can compete to fill orders

**Without both mechanisms, our ICP integration cannot work properly.**

## Implementation Options

### Option 1: ICP-Native Signing Standard + Delegation

- **Approach:** Create ICP-specific signing standard for orders + delegation mechanism
- **Challenge:** Different from EIP-712, requires new tooling and delegation patterns
- **Risk:** Breaks compatibility with existing 1inch infrastructure

### Option 2: Cross-Chain EIP-712 Verification + Gasless Pattern

- **Approach:** Verify Ethereum EIP-712 signatures on ICP + implement gasless order creation
- **Challenge:** Complex cross-chain signature verification + ICP gasless implementation
- **Risk:** High implementation complexity, potential security issues

### Option 3: Relayer-Mediated Orders + Delegation

- **Approach:** Relayer handles order verification, execution, and delegation
- **Challenge:** Requires trusted relayer infrastructure for both patterns
- **Risk:** Introduces centralization, trust assumptions

### Option 4: Hybrid Approach

- **Approach:** Combine multiple order creation and delegation mechanisms
- **Challenge:** Complex coordination between different systems
- **Risk:** Increased attack surface, maintenance overhead

## Impact

### High Priority

- **Core UX feature** - Gasless order creation is fundamental to 1inch Fusion+
- **Security mechanism** - Delegation is essential for cross-chain swaps
- **Competitive advantage** - Enables Dutch auction mechanism
- **User adoption** - Users expect gasless, asynchronous orders

### Dependencies

- **Order signing** (Step 1.2) depends on gasless pattern
- **Escrow creation** (Step 1.3) requires delegation verification
- **Dutch auction** (Step 1.4) relies on signed orders
- **Atomic execution** (Step 2) depends on proper delegation

## Success Criteria

### Must Have

- **Gasless order creation** - Maker pays no fees to create order
- **Asynchronous execution** - Resolver can execute without maker online
- **Intent preservation** - Maker's intent is cryptographically verifiable
- **Competitive execution** - Multiple resolvers can compete
- **Secure delegation** - Resolver can prove maker authorization
- **Cross-chain compatibility** - Works with existing 1inch infrastructure
- **Atomic execution** - Same security guarantees as 1inch Fusion+

### Should Have

- **Simple implementation** - Minimal complexity
- **Auditable** - Clear security model
- **Extensible** - Works for future chains

## Next Steps

1. **Research ICP signing patterns** for off-chain orders
2. **Research delegation patterns** in cross-chain protocols
3. **Evaluate implementation complexity** of each option
4. **Prototype most promising approach**
5. **Security review** of chosen mechanism
6. **Integration testing** with existing 1inch infrastructure

## Related Issues

- ICP order signing standards
- Cross-chain order verification
- Cross-chain delegation mechanisms
- 1inch Fusion+ integration strategy
