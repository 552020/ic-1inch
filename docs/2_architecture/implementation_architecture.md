# Implementation Architecture: ICP <> EVM Cross-Chain Fusion

## Overview

This document outlines the high-level architecture for the ICP backend canisters that coordinate cross-chain Fusion swaps between ICP and EVM chains. The architecture uses Chain Fusion primitives to eliminate the need for external relayers.

## Canister Architecture

### 1. Orderbook Canister (`orderbook_canister`)

**Purpose**: Central coordination hub that manages virtual escrow state and orchestrates cross-chain operations.

**Main Functions**:

- `create_order()` - Accepts new swap orders from users
- `process_order()` - Initiates escrow creation on both chains
- `reveal_secret()` - Reveals hashlock secret to complete swaps
- `handle_timeout()` - Manages timelock expirations and refunds
- `get_order_status()` - Query function for frontend state verification

**Key Data Structures**:

```rust
struct Order {
    id: String,
    maker: Principal,
    taker: Principal,
    maker_asset: String,    // "ICP" or "ETH"
    taker_asset: String,    // "ETH" or "ICP"
    making_amount: u64,
    taking_amount: u64,
    hashlock: [u8; 32],
    timelock: u64,
    state: OrderState,
    icp_escrow_id: Option<String>,
    evm_escrow_address: Option<String>,
    created_at: u64,
    updated_at: u64,
}

enum OrderState {
    Created,
    ICPEscrowCreated,
    EVMEscrowCreated,
    BothEscrowsReady,
    SwapCompleted,
    Cancelled,
    Refunded,
    Failed
}
```

**Responsibilities**:

- Maintains virtual escrow representations
- Coordinates state transitions between ICP and EVM
- Handles hashlock/timelock logic
- Provides certified data for frontend trust
- Manages periodic state checks via timers

### 2. ICP Escrow Factory Canister (`icp_escrow_factory`)

**Purpose**: Creates and manages actual ICP escrows for ICP-side assets.

**Main Functions**:

- `create_escrow()` - Creates new ICP escrow with hashlock/timelock
- `release_escrow()` - Releases funds when hashlock is revealed
- `refund_escrow()` - Refunds funds after timelock expiration
- `get_escrow_state()` - Query escrow state for verification

**Key Data Structures**:

```rust
struct ICPEscrow {
    id: String,
    owner: Principal,
    asset: String,          // "ICP" or ICP token identifier
    amount: u64,
    hashlock: [u8; 32],
    timelock: u64,
    state: EscrowState,
    created_at: u64,
}

enum EscrowState {
    Locked,
    Released,
    Refunded,
    Expired
}
```

**Responsibilities**:

- Manages ICP asset locking/unlocking
- Enforces hashlock and timelock conditions
- Integrates with ICP token ledger
- Provides escrow state verification

### 3. EVM Coordinator Canister (`evm_coordinator`)

**Purpose**: Handles all EVM chain interactions via Chain Fusion primitives.

**Main Functions**:

- `create_evm_escrow()` - Submits EVM escrow creation transaction
- `verify_evm_escrow()` - Verifies EVM escrow state via RPC
- `submit_evm_transaction()` - Generic EVM transaction submission
- `poll_evm_events()` - Polls EVM logs for state changes

**Key Data Structures**:

```rust
struct EVMTransaction {
    to: String,
    data: Vec<u8>,
    value: u64,
    gas_limit: u64,
    nonce: u64,
}

struct EVMEscrowInfo {
    address: String,
    owner: String,
    asset: String,
    amount: u64,
    hashlock: [u8; 32],
    timelock: u64,
    state: String,
}
```

**Responsibilities**:

- Uses threshold ECDSA signing for Ethereum transactions
- Manages EVM RPC canister interactions
- Handles multi-provider consensus for reliability
- Monitors EVM state changes via HTTPS outcalls

### 4. Token Ledger Canister (`token_ledger`)

**Purpose**: Manages ICP token balances and transfers for the protocol.

**Main Functions**:

- `transfer_to_escrow()` - Transfers tokens to escrow
- `transfer_from_escrow()` - Transfers tokens from escrow to recipient
- `get_balance()` - Query user token balance
- `mint_tokens()` - Mint ICP tokens (for testing)

**Key Data Structures**:

```rust
struct TokenBalance {
    owner: Principal,
    token: String,
    amount: u64,
    last_updated: u64,
}

struct Transfer {
    from: Principal,
    to: Principal,
    token: String,
    amount: u64,
    timestamp: u64,
}
```

**Responsibilities**:

- Manages ICP token balances
- Handles token transfers to/from escrows
- Provides balance verification
- Integrates with ICP token standards

## Inter-Canister Communication Flow

### Order Creation Flow:

1. **Frontend** → `orderbook_canister.create_order()`
2. **Orderbook** → `icp_escrow_factory.create_escrow()`
3. **Orderbook** → `evm_coordinator.create_evm_escrow()`
4. **Orderbook** → Update virtual state to "BothEscrowsReady"

### Swap Execution Flow:

1. **Resolver** → `orderbook_canister.reveal_secret()`
2. **Orderbook** → `icp_escrow_factory.release_escrow()`
3. **Orderbook** → `evm_coordinator.submit_release_tx()`
4. **Orderbook** → Update state to "SwapCompleted"

### Timeout/Refund Flow:

1. **Timer** → `orderbook_canister.handle_timeout()`
2. **Orderbook** → `icp_escrow_factory.refund_escrow()`
3. **Orderbook** → `evm_coordinator.submit_refund_tx()`
4. **Orderbook** → Update state to "Refunded"

## Key Design Principles

### 1. ICP as the Brain

- All coordination logic lives in ICP canisters
- EVM is treated as a "dumb executor"
- No external relayers needed

### 2. State Machine Design

- Explicit state transitions with validation
- Each state change triggers side effects
- Rollback mechanisms for failures

### 3. Certified Data

- All canisters provide certified state data
- Frontend can verify state without trusting canisters
- Cryptographic proofs for escrow states

### 4. Timer-Based Coordination

- Periodic state checks via `ic_cdk_timers`
- Automatic timeout handling
- Event-driven state transitions

### 5. Error Handling

- Comprehensive error types and recovery
- Compensating actions for partial failures
- Idempotent operations for retry safety

## Deployment Considerations

### Canister Dependencies:

- `orderbook_canister` depends on all other canisters
- `icp_escrow_factory` depends on `token_ledger`
- `evm_coordinator` is independent but called by `orderbook_canister`

### Configuration:

- EVM RPC endpoints and providers
- Threshold ECDSA key configuration
- Timelock durations and retry policies
- Gas limits and transaction parameters

### Security:

- Access control for admin functions
- Rate limiting for public endpoints
- Input validation and sanitization
- Certified data for frontend verification

This architecture provides a complete foundation for cross-chain Fusion swaps with robust coordination, error handling, and security measures.
