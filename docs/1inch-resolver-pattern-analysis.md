# 1inch Cross-Chain Resolver Pattern Analysis

## Overview

This document analyzes the official 1inch cross-chain resolver example (`docs/1inch-resolver-example/`) and compares it with our planned mechanical turk approach for ICP↔ETH atomic swaps.

## Current 1inch Implementation Analysis

### Architecture Overview

The 1inch example implements a sophisticated cross-chain atomic swap system with the following components:

1. **Escrow Factory Pattern**

   - `EscrowFactory` deploys escrows on both chains
   - Deterministic escrow addresses based on parameters
   - Safety deposits for gas costs and fees

2. **Resolver Contract**

   - `Resolver.sol` coordinates cross-chain operations
   - Manual resolver role (similar to our mechanical turk)
   - Handles deployment, withdrawal, and cancellation

3. **Cross-Chain Coordination**
   - Source chain: Deploy escrow, lock funds
   - Destination chain: Deploy escrow, complete swap
   - Manual coordination between chains

### Key Functions Analysis

#### `deploySrc()` - Source Chain Escrow Deployment

```solidity
function deploySrc(
    IBaseEscrow.Immutables calldata immutables,
    IOrderMixin.Order calldata order,
    bytes32 r,
    bytes32 vs,
    uint256 amount,
    TakerTraits takerTraits,
    bytes calldata args
) external payable onlyOwner
```

**What it does:**

- Deploys escrow on source chain with deterministic address
- Sends safety deposit to escrow
- Fills limit order with escrow as taker
- Locks funds in escrow

**Our adaptation:**

- Simplify to basic ETH locking in `FusionEscrow.sol`
- Remove complex order mixing
- Focus on manual coordination

#### `deployDst()` - Destination Chain Escrow Deployment

```solidity
function deployDst(
    IBaseEscrow.Immutables calldata dstImmutables,
    uint256 srcCancellationTimestamp
) external onlyOwner payable
```

**What it does:**

- Deploys escrow on destination chain
- Links to source chain cancellation timestamp
- Prepares for swap completion

**Our adaptation:**

- ICP canister handles destination side
- Manual relayer coordinates between chains
- Simplified state management

#### `withdraw()` - Complete Swap

```solidity
function withdraw(
    IEscrow escrow,
    bytes32 secret,
    IBaseEscrow.Immutables calldata immutables
) external
```

**What it does:**

- Completes swap with cryptographic secret
- Releases funds to recipient
- Atomic operation

**Our adaptation:**

- Manual secret sharing between chains
- Receipt-based verification
- Simplified for mechanical turk

#### `cancel()` - Cancel and Refund

```solidity
function cancel(
    IEscrow escrow,
    IBaseEscrow.Immutables calldata immutables
) external
```

**What it does:**

- Cancels swap after timelock expires
- Returns funds to original owner
- Safety mechanism

**Our adaptation:**

- Timelock-based refunds
- Manual coordination for cancellation
- Cross-chain state verification

## Our Mechanical Turk Approach vs 1inch

### Key Differences

| Aspect          | 1inch Approach               | Our Mechanical Turk         |
| --------------- | ---------------------------- | --------------------------- |
| **Automation**  | Fully automated resolver     | Manual relayer coordination |
| **Complexity**  | Sophisticated escrow factory | Simple direct escrow        |
| **Cross-Chain** | Cryptographic secrets        | Receipt-based verification  |
| **Gas Costs**   | Resolver pays all gas        | Relayer pays gas            |
| **Trust Model** | Trustless cryptographic      | Trusted manual coordination |

### Our Simplified Architecture

```
┌─────────────────┐    ┌─────────────────┐
│   ICP Canister  │    │  ETH Contract   │
│                 │    │                 │
│ • Lock ICP      │◄──►│ • Lock ETH      │
│ • Verify receipt│    │ • Verify receipt│
│ • Release funds │    │ • Release funds │
└─────────────────┘    └─────────────────┘
         ▲                       ▲
         │                       │
         └─────── Manual ────────┘
              Relayer
```

## Adaptation Strategy

### 1. Simplify Escrow Pattern

**1inch**: Complex factory with deterministic addresses
**Our approach**: Simple direct escrow contracts

```solidity
// Our simplified approach
contract FusionEscrow {
    function lockETHForSwap(string memory orderId, uint256 timelock) external payable;
    function claimLockedETH(string memory orderId, string memory icpReceipt) external;
    function refundLockedETH(string memory orderId) external;
}
```

### 2. Manual Coordination Interface

**1inch**: Automated resolver contract
**Our approach**: Manual relayer interface

```typescript
// Our manual coordination
interface ManualRelayer {
  deploySrcEscrow(orderId: string, amount: bigint): Promise<void>;
  deployDstEscrow(orderId: string, receipt: string): Promise<void>;
  completeSwap(orderId: string, secret: string): Promise<void>;
  cancelSwap(orderId: string): Promise<void>;
}
```

### 3. Receipt-Based Verification

**1inch**: Cryptographic hashlock/preimage
**Our approach**: Simple receipt verification

```rust
// ICP side
pub fn verify_eth_receipt(receipt: String) -> bool {
    // Manual verification by relayer
    // Could be enhanced with cryptographic proofs later
}
```

## Implementation Plan

### Phase 1: Core Escrow Functions

1. **ETH Escrow** (`FusionEscrow.sol`)

   - `lockETHForSwap()` - Lock ETH for swap
   - `claimLockedETH()` - Claim with ICP receipt
   - `refundLockedETH()` - Refund after timelock

2. **ICP Escrow** (Canister functions)
   - `lock_icp_for_swap()` - Lock ICP for swap
   - `claim_locked_icp()` - Claim with ETH receipt
   - `refund_locked_icp()` - Refund after timelock

### Phase 2: Manual Coordination

1. **Relayer Interface**

   - Web UI for manual operations
   - Order status tracking
   - Cross-chain state verification

2. **Receipt Management**
   - Simple string-based receipts
   - Manual verification process
   - Audit trail for debugging

### Phase 3: Enhanced Security

1. **Timelock Mechanisms**

   - Configurable timeouts
   - Automatic refunds
   - Safety mechanisms

2. **Error Handling**
   - Comprehensive error types
   - Recovery procedures
   - Manual override capabilities

## Key Learnings from 1inch

### What to Adopt

1. **Timelock Safety**: Essential for cross-chain operations
2. **State Management**: Clear order lifecycle tracking
3. **Error Handling**: Comprehensive error types and recovery
4. **Gas Management**: Proper gas cost handling

### What to Simplify

1. **Escrow Factory**: Too complex for MVP
2. **Cryptographic Secrets**: Manual coordination is simpler
3. **Automated Resolver**: Manual relayer for MVP
4. **Complex Order Mixing**: Direct escrow approach

### What to Enhance

1. **Manual Interface**: Better UX for relayer operations
2. **Receipt Verification**: Simple but auditable
3. **Cross-Chain Communication**: Clear status updates
4. **Error Recovery**: Manual intervention capabilities

## Conclusion

The 1inch resolver example provides excellent patterns for cross-chain atomic swaps, but our mechanical turk approach requires significant simplification. We'll adopt the core safety mechanisms (timelocks, state management) while replacing complex automation with manual coordination.

This analysis will guide our implementation of Tasks 2.3 (cross-chain coordination logic) and 3.2 (manual relayer coordination tools), ensuring we build on proven patterns while maintaining the simplicity required for our MVP approach.
