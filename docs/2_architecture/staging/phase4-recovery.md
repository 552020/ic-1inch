# Phase 4: Recovery

_Timeout handling and public withdrawal/cancellation_

---

## Overview

**Reactive phase** that handles timeout scenarios when the atomic swap fails to complete within the timelock period. This phase enables public withdrawal/cancellation of funds from both escrows.

**Key Principle:** This phase is **only triggered when the swap fails** - successful swaps never reach Phase 4.

**⚠️ Important:** Phase 4 is marked as "optional" in official 1inch Fusion+ documentation. This means:

- **Optional for protocol flow** - Successful swaps never reach Phase 4
- **Optional for basic implementation** - Core HTLC can work without recovery
- **Optional for resolvers** - Recovery can be handled by other parties
- **BUT NOT optional for production** - Complete HTLC systems require recovery mechanisms

---

## TL;DR

### **What We Implement:**

- **ICP side:** `refund_escrow()` method, timelock verification, token refunds, state management
- **Ethereum side:** Nothing - use existing 1inch `EscrowSrc` contract

### **What We Build for Testing:**

- **Timeout monitoring** scripts
- **Refund verification** tools
- **Balance monitoring** to confirm refunds

### **What We Don't Build:**

- **Ethereum smart contracts** - use existing 1inch infrastructure
- **UI/frontend** - manual testing via scripts and CLI

---

## Required Inputs

### **From Phase 2:**

- **Both escrows created** - Ethereum escrow with user's tokens, ICP escrow with resolver's tokens
- **Timelock duration** - Time limit for swap completion
- **Escrow addresses** - Both escrow contract addresses

### **Trigger Conditions:**

- **Timelock expired** - Swap did not complete within time limit
- **Swap failed** - One or both chains failed to execute
- **Manual cancellation** - User or resolver cancels the swap

---

## Recovery Process

### **✅ Single Recovery Operation:**

**Timeout triggers recovery on both chains:**

1. **Timelock verification** - Confirm timeout has occurred
2. **Public withdrawal** - Anyone can trigger refund
3. **Tokens returned** to original owners
4. **Escrows closed** and marked as cancelled

**Either both recoveries succeed or the entire recovery fails.**

---

## Implementation

### **✅ What We Implement:**

#### **ICP Side (Our Development Work):**

- **`refund_escrow()` method** - We implement this in our Rust canister
- **Timelock verification** - We implement timeout checking in our canister
- **Token refund logic** - We implement ICRC-1 token refunds
- **State management** - We implement escrow cancellation state updates

#### **Ethereum Side (Use Existing):**

- **`cancel()` function** - Already exists in 1inch `EscrowSrc` contract
- **Token refund logic** - Already exists in ERC-20 contracts
- **Timelock verification** - Already exists in 1inch contracts

### **✅ MVP Implementation (Manual Process):**

**Recovery triggered by timeout:**

#### **Ethereum Side (No Implementation Needed):**

- **Anyone calls** existing `cancel()` on `EscrowSrc` contract
- **Existing contract verifies** timelock has expired
- **Existing logic refunds** user's tokens to user
- **Existing contract updates** state to cancelled

#### **ICP Side (Our Implementation):**

- **Anyone calls** our `refund_escrow()` method on our canister
- **Our canister verifies** timelock has expired
- **Our canister refunds** resolver's tokens to resolver
- **Our canister updates** state to cancelled

#### **Cross-Chain Coordination:**

- **Manual verification** that both refunds succeeded
- **Balance checks** on both chains
- **Recovery completion** confirmed

**Note:** Cross-chain verification is **manual for MVP** - no automated cross-chain messaging implemented.

### **Testing Tools Needed:**

#### **Ethereum Tools:**

- **Ethers.js** - Call `cancel()` function on escrow contract
- **Etherscan API** - Verify refund transaction success
- **Balance monitoring** - Confirm tokens returned to user

#### **ICP Tools:**

- **dfx CLI** - Call `refund_escrow()` method on our canister
- **Canister queries** - Verify escrow state and refunds
- **Balance monitoring** - Confirm tokens returned to resolver

#### **Coordination Tools:**

- **Manual verification** - Check both chains for successful refunds
- **Balance verification** - Confirm final token balances
- **State verification** - Confirm both escrows marked as cancelled

### **Stretch Goals Implementation:**

#### **Automated Recovery:**

- **Automated timeout monitoring** - Scripts that detect expired timelocks
- **Automatic refund triggering** - Scripts that call both chains
- **Automatic completion verification** - Confirm refund success

#### **Advanced Features:**

- **Partial refund handling** - Handle partial failures
- **Gas/cycle optimization** - Efficient refund execution
- **Monitoring and logging** - Detailed recovery tracking

---

## Reference Implementations

### **From First Attempt ICP Implementation:**

- **Timelock verification** - Timeout checking patterns
- **Token refund** - ICRC-1 refund mechanisms
- **State management** - Escrow cancellation patterns

### **From SwappaTEE:**

- **Timeout handling** - Recovery mechanisms
- **Cross-chain coordination** - Synchronized refunds
- **Failure scenarios** - Partial refund handling

### **From Solana Fusion Protocol:**

- **Order cancellation** - Timeout patterns
- **State updates** - Cancellation state management
- **Verification** - Refund confirmation

---

## Technical Details

### **Recovery Process:**

#### **Ethereum Recovery:**

1. **Anyone calls** `cancel()` on `EscrowSrc` contract
2. **Contract verifies** timelock has expired
3. **If verification succeeds** - proceed to token refund
4. **If verification fails** - transaction reverts

#### **ICP Recovery:**

1. **Anyone calls** `refund_escrow()` on our canister
2. **Canister verifies** timelock has expired
3. **If verification succeeds** - proceed to token refund
4. **If verification fails** - method returns error

### **Token Refund Process:**

#### **Ethereum Token Refund:**

1. **Escrow contract refunds** user's tokens to user's address
2. **Refund executed** via ERC-20 transfer function
3. **Escrow state updated** to cancelled
4. **Transaction confirmed** on Ethereum

#### **ICP Token Refund:**

1. **Canister refunds** resolver's tokens to resolver's address
2. **Refund executed** via ICRC-1 transfer function
3. **Canister state updated** to cancelled
4. **Refund confirmed** on ICP

### **Verification Requirements:**

- **Both timelock verifications** must succeed
- **Both token refunds** must succeed
- **Both state updates** must complete
- **Cross-chain verification** confirms recovery completion

### **Failure Scenarios:**

- **Timelock verification fails** on one chain - recovery fails
- **Token refund fails** on one chain - recovery fails
- **Network issues** prevent completion - recovery remains pending
- **Insufficient gas/cycles** - transaction fails

### **Partial Recovery Handling:**

- **MVP assumption:** Both refunds succeed or both fail
- **Partial failure scenario:** If refund on Ethereum succeeds but ICP fails (or vice versa)
- **MVP approach:** Manual retry by resolver - no automated recovery
- **Stretch goal:** Automated partial failure handling and retry mechanisms

---

## MVP Recommendation

### **Manual Recovery:**

- **Manual timeout monitoring** - Check timelock expiration
- **Manual refund triggering** - Call both chains when timeout occurs
- **Manual verification** of token refunds
- **Simple and reliable** for MVP demo

### **Key Success Criteria:**

- **Both escrows successfully** verify timelock expiration
- **All tokens refunded** to original owners
- **Both chains show** escrows as cancelled
- **Cross-chain verification** confirms recovery completion

---

## Subject Requirements Alignment

### **Implicit HTLC Requirement:**

- **Not explicitly mentioned** in subject requirements
- **Required for complete HTLC functionality** - timeout handling is essential
- **Production readiness** - Recovery mechanisms are necessary
- **Protocol completeness** - Atomic swaps need recovery mechanisms

### **Optional vs Required:**

- **Optional in protocol flow** - Only triggered when swaps fail
- **Optional in basic implementation** - Core functionality works without it
- **Required for production** - Real-world systems need recovery mechanisms
- **Required for user safety** - Users need timeout protection guarantees

### **MVP Scope:**

- **Simple timeout handling** - Basic refund functionality
- **Manual recovery process** - No automation required
- **Cross-chain verification** - Confirm both chains recovered
- **Foundation for stretch goals** - Automated recovery systems
