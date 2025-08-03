# ICP Cross-Chain Coordination Patterns

## Overview

Coordinating state between multiple ICP canisters and external blockchain systems (like EVM chains) is a complex challenge, especially when atomicity and consistency are required across fundamentally different execution environments. This document outlines the recommended patterns and best practices for our ICP <> EVM Fusion protocol.

## Core Challenge

**True atomicity across chains is not natively supported**—neither ICP nor EVM blockchains provide cross-chain atomic transactions. Instead, protocols must be designed to be robust against partial failures and to allow for eventual consistency or safe rollbacks.

## Senior-Level Implementation: Chain Fusion Approach

### Problem Recap

You want **atomic state sync** between:

1. `ICP Orderbook` (virtual state machine)
2. `ICP EscrowFactory` (real ICP asset escrows)
3. `EVM EscrowFactory` (real EVM asset escrows)

But you're operating **cross-chain** — i.e., **no shared execution context**, no shared memory, no rollback across trust domains. So atomicity is impossible in the strict DB sense.

### What You Can Actually Do: Chain Fusion

ICP now supports **Chain Fusion** which means:

- ICP can **sign and submit Ethereum txs**
- ICP can **query EVM state via HTTPS outcalls**
- You can do most coordination logic **on-chain** in the ICP Orderbook + EscrowFactory

**→ That means you can replace the relayer with deterministic canister logic.**

### Recommended Pattern: 2-Phase Commit Emulation

You mimic a 2PC pattern using **timeouts, hashlocks, and canonical Order state** in ICP.

#### Step-by-step:

1. **Maker submits order to `Orderbook`** with secret hash `H(secret)` and timelocks `T1`, `T2`
2. **Orderbook** calls `ICP EscrowFactory` to create ICP escrow
3. **Orderbook** constructs **Ethereum escrow transaction** via `threshold_ecdsa_sign` and **submits it to EVM**
4. Both escrows hold safety deposits and are hashlocked
5. Once both escrows confirmed (via on-chain events or Ethereum RPC query), **Orderbook reveals secret**
6. Both escrows release funds with the same secret

**Timelock fallback logic:**
If step 5 doesn't happen, both escrows become refundable to the maker/taker after T1/T2.

### Inter-Canister Communication (ICP → ICP)

- **Make everything actor-based**: `Orderbook → EscrowFactory → TokenLedger`
- Always **commit to a versioned state in Orderbook** before triggering side effects
- Propagate **intent, not effect** (i.e. "Create escrow with hash H" not "Send 5 ICP")
- Validate response callbacks and retry with exponential backoff if fail

### ICP ↔ EVM Coordination Patterns (Chain Fusion Style)

#### Pattern: ICP as Coordinator, EVM as Dumb Executor

1. Orderbook creates ICP escrow
2. Generates Ethereum escrow tx → signed with `threshold_ecdsa_sign_with_seed`
3. Submits via HTTPS outcall (to a quorum of RPCs for reliability)
4. Verifies EVM escrow existence by:
   - Computing CREATE2 address
   - Checking contract state via HTTPS outcall
5. On success, reveals secret

**This is all on-chain.** No external relayer.

### Handling Partial Failures

| Failure Type                   | Recommended Recovery                                       |
| ------------------------------ | ---------------------------------------------------------- |
| ICP escrow created, EVM failed | Cancel ICP escrow after `T1`                               |
| EVM escrow created, ICP failed | Cancel EVM escrow after `T2`                               |
| Secret revealed too early      | Use Merkle tree for partial fills                          |
| Timeout reached, no reveal     | Refund both escrows to original parties                    |
| Network partition (RPC down)   | Retry outcalls with fallback providers (Chain Fusion spec) |

**Each canister has a deterministic `EscrowState` enum** and exposes that in query mode for frontend trust.

### Summary Tools You'll Use

| Task                   | Tool                             |
| ---------------------- | -------------------------------- |
| Ethereum tx creation   | `threshold_ecdsa_sign_with_seed` |
| Ethereum tx submission | `http_request` via outcall       |
| Ethereum state read    | `http_request` with eth_call     |
| ICP timers             | `ic_cdk_timers`                  |
| Certified state        | `ic_certified_data`              |
| Canister-level retries | `await` + exponential backoff    |

### Mental Model

Think like this:

- ICP is **the brain** — orchestrates everything
- ICP escrows = **the left hand**
- EVM escrows = **the right hand** (controlled via threshold keys)
- State consistency = **write-only FSM + hashlocks**
- Fallback = **timelock-based finalization + cryptographic guardrails**

## 1. Atomic Coordination Between ICP Canisters and EVM Contracts

### Recommended Patterns

#### Hashlock/Timelock Mechanisms

- Use hashlocks and timelocks (as in HTLCs) to coordinate state transitions
- Both ICP and EVM contracts can enforce these primitives
- Allows for conditional execution and time-based rollbacks if the protocol is not completed in time

#### Virtual Escrow Migration

- The ICP Orderbook canister can represent the "virtual" state
- Only trigger real escrow creation (on both ICP and EVM) once all preconditions are met
- Minimizes the risk of partial state transitions

#### Event-driven Coordination

- Use periodic polling or event listeners (e.g., via the EVM RPC canister) to detect state changes on the EVM side
- Trigger corresponding actions on ICP, or vice versa
- Example: ICP canister can poll EVM logs for escrow creation events and only update its state once the EVM contract confirms the action

## 2. Best Practices for Inter-Canister Communication (with External Chains)

### Asynchronous Messaging

- ICP canisters communicate asynchronously
- State changes are not atomic across multiple canisters or with external chains
- Each inter-canister call is a separate atomic operation
- Failures must be handled explicitly

### Callback Handling

- Use callbacks to handle responses from inter-canister calls
- Be aware that traps (errors) in callbacks only roll back the state changes made in that callback, not the entire transaction

### Idempotency and Safe Retries

- Since calls to external systems (like EVM contracts via the EVM RPC canister) can fail or be uncertain
- Design your protocol to be idempotent
- Repeated attempts to perform the same action should not result in inconsistent state or double execution

### Consensus on External Data

- When querying EVM state, the EVM RPC canister can query multiple RPC providers
- Only return a result if a threshold of providers agree
- Reduces the risk of acting on inconsistent or manipulated data

## 3. Handling Failed Coordination Scenarios

### Compensating Actions

- If a step in the protocol fails (e.g., ICP escrow is created but EVM escrow creation fails)
- Design compensating actions such as refunding the ICP escrow after a timeout or upon explicit cancellation

### Timeouts and Rollbacks

- Use timelocks to ensure that if a cross-chain operation is not completed within a certain period
- Funds can be safely returned to their original owners

### Explicit State Machines

- Model your protocol as a state machine
- Each state transition is only made after confirmation from the relevant chain or canister
- If a transition fails, the system should either retry or move to a safe rollback state

### Error Handling

- Always handle error cases in inter-canister and external calls
- Handle all reject codes and design your protocol to be robust against partial failures

## Example Pattern for Cross-Chain Swap

A typical cross-chain swap might look like:

1. **Orderbook canister** creates a virtual escrow and requests ICP EscrowFactory to create a real escrow
2. Once ICP escrow is confirmed, the Orderbook canister triggers EVM EscrowFactory contract creation via the EVM RPC canister
3. Both escrows are locked with the same hashlock and timelock
4. If either escrow creation fails, the protocol triggers a rollback (e.g., refund on ICP)
5. If both succeed, the swap proceeds; if not completed before timelock expiry, both escrows can be refunded

## Implementation Considerations

### State Machine Design

```rust
enum OrderState {
    Created,
    ICPEscrowCreated,
    EVMEscrowCreated,
    BothEscrowsReady,
    SwapCompleted,
    Cancelled,
    Refunded
}
```

### Error Handling Strategy

- Implement comprehensive error handling for all inter-canister calls
- Use compensating transactions for failed operations
- Implement timeouts and automatic rollback mechanisms

### Monitoring and Observability

- Log all state transitions and coordination events
- Monitor for stuck states and implement recovery mechanisms
- Use the EVM RPC canister's multi-provider consensus for reliable external data

## Final Verdict

You **can** build this with **100% on-chain logic on the ICP side** using Chain Fusion primitives. No relayer needed. Just deterministic state, signed Ethereum txs, and time-based recovery logic.

## Summary

You cannot guarantee atomicity across ICP and EVM, but you can design your protocol to be robust, idempotent, and safe in the face of partial failures by using:

- Hashlocks and timelocks
- Compensating actions
- Careful state machine design
- Explicit error handling
- Multi-provider consensus for external data

Always handle errors and rejections explicitly, and use the EVM RPC canister's multi-provider consensus features to reduce trust in external data sources.
