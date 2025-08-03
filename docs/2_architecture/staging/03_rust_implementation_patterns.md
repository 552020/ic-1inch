# Rust Implementation Patterns for Cross-Chain Coordination

## Overview

This document provides high-level Rust-style pseudocode and implementation patterns for coordinating cross-chain escrow swaps between ICP canisters and EVM contracts, following the patterns and best practices from ICP documentation.

## Core Implementation Pattern

### Asynchronous Inter-Canister Calls with Error Handling

```rust
// Pseudocode for ICP Orderbook Canister coordinating with ICP EscrowFactory and EVM EscrowFactory

// 1. User initiates swap request
async fn initiate_swap(user: Principal, amount: u64, hashlock: [u8; 32], timelock: u64) {
    // Step 1: Create ICP escrow
    let icp_escrow_result = call_icp_escrow_factory_create(user, amount, hashlock, timelock).await;
    if icp_escrow_result.is_err() {
        // Handle ICP escrow creation failure
        return;
    }

    // Step 2: Create EVM escrow via EVM RPC canister
    let evm_escrow_result = call_evm_escrow_factory_create(user, amount, hashlock, timelock).await;
    if evm_escrow_result.is_err() {
        // Compensating action: refund ICP escrow
        call_icp_escrow_factory_refund(user, amount).await;
        return;
    }

    // Step 3: Update virtual escrow state to "active"
    update_virtual_escrow_state(user, "active");
}

// 2. Periodically check EVM escrow state (using EVM RPC canister)
async fn poll_evm_escrow_state(evm_escrow_address: String, expected_hashlock: [u8; 32]) {
    let logs = call_evm_rpc_get_logs(evm_escrow_address.clone()).await;
    if logs.contains_unlock_event_with_hashlock(expected_hashlock) {
        // Hashlock revealed, trigger ICP escrow release
        call_icp_escrow_factory_release(evm_escrow_address).await;
        update_virtual_escrow_state(evm_escrow_address, "completed");
    } else if current_time() > get_timelock(evm_escrow_address) {
        // Timelock expired, trigger refunds
        call_icp_escrow_factory_refund(evm_escrow_address).await;
        call_evm_escrow_factory_refund(evm_escrow_address).await;
        update_virtual_escrow_state(evm_escrow_address, "refunded");
    }
}

// 3. Error handling for partial failures
async fn handle_partial_failure(user: Principal, icp_escrow_created: bool, evm_escrow_created: bool) {
    if icp_escrow_created && !evm_escrow_created {
        // Refund ICP escrow
        call_icp_escrow_factory_refund(user, amount).await;
    }
    // Add more compensating actions as needed
}

// 4. Inter-canister call example (asynchronous, non-atomic)
async fn call_icp_escrow_factory_create(user: Principal, amount: u64, hashlock: [u8; 32], timelock: u64) -> Result<(), Error> {
    // Example inter-canister call
    // Call::unbounded_wait(escrow_factory_principal, "create_escrow")
    //     .with_arg(&(user, amount, hashlock, timelock))
    //     .await
    //     .map_err(|e| Error::from(e))
    Ok(())
}

// 5. EVM RPC canister call example
async fn call_evm_escrow_factory_create(user: Principal, amount: u64, hashlock: [u8; 32], timelock: u64) -> Result<(), Error> {
    // Use EVM RPC canister to send transaction to EVM EscrowFactory
    // See: https://internetcomputer.org/docs/building-apps/chain-fusion/overview#how-to-build-applications-using-chain-fusion
    Ok(())
}
```

## State Machine Implementation

### Order State Enum

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum OrderState {
    Created,
    ICPEscrowCreated,
    EVMEscrowCreated,
    BothEscrowsReady,
    SwapCompleted,
    Cancelled,
    Refunded,
    Failed
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub maker: Principal,
    pub taker: Principal,
    pub maker_asset: String,
    pub taker_asset: String,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub hashlock: [u8; 32],
    pub timelock: u64,
    pub state: OrderState,
    pub created_at: u64,
    pub updated_at: u64,
}
```

### State Transition Logic

```rust
impl Order {
    pub async fn transition_to(&mut self, new_state: OrderState) -> Result<(), StateTransitionError> {
        // Validate state transition
        if !self.can_transition_to(&new_state) {
            return Err(StateTransitionError::InvalidTransition);
        }

        // Update state
        self.state = new_state;
        self.updated_at = ic_cdk::api::time();

        // Trigger side effects based on new state
        match new_state {
            OrderState::ICPEscrowCreated => {
                self.create_evm_escrow().await?;
            },
            OrderState::EVMEscrowCreated => {
                self.verify_both_escrows().await?;
            },
            OrderState::BothEscrowsReady => {
                self.start_swap_execution().await?;
            },
            OrderState::SwapCompleted => {
                self.finalize_swap().await?;
            },
            OrderState::Cancelled | OrderState::Refunded => {
                self.handle_rollback().await?;
            },
            _ => {}
        }

        Ok(())
    }

    fn can_transition_to(&self, new_state: &OrderState) -> bool {
        match (&self.state, new_state) {
            (OrderState::Created, OrderState::ICPEscrowCreated) => true,
            (OrderState::ICPEscrowCreated, OrderState::EVMEscrowCreated) => true,
            (OrderState::EVMEscrowCreated, OrderState::BothEscrowsReady) => true,
            (OrderState::BothEscrowsReady, OrderState::SwapCompleted) => true,
            (_, OrderState::Cancelled) => true,
            (_, OrderState::Refunded) => true,
            (_, OrderState::Failed) => true,
            _ => false
        }
    }
}
```

## Error Handling and Recovery

### Error Types

```rust
#[derive(Debug)]
pub enum CrossChainError {
    ICPEscrowCreationFailed(String),
    EVMEscrowCreationFailed(String),
    StateTransitionError(StateTransitionError),
    HashlockVerificationFailed,
    TimelockExpired,
    NetworkError(String),
    InvalidState(String),
}

#[derive(Debug)]
pub enum StateTransitionError {
    InvalidTransition,
    EscrowNotFound,
    InsufficientFunds,
    HashlockMismatch,
}
```

### Recovery Mechanisms

```rust
impl Order {
    pub async fn handle_failure(&mut self, error: CrossChainError) -> Result<(), CrossChainError> {
        match error {
            CrossChainError::ICPEscrowCreationFailed(_) => {
                // No compensating action needed - escrow wasn't created
                self.state = OrderState::Failed;
            },
            CrossChainError::EVMEscrowCreationFailed(_) => {
                // Compensating action: refund ICP escrow
                self.refund_icp_escrow().await?;
                self.state = OrderState::Cancelled;
            },
            CrossChainError::HashlockVerificationFailed => {
                // Both escrows exist but hashlock doesn't match
                self.refund_both_escrows().await?;
                self.state = OrderState::Refunded;
            },
            CrossChainError::TimelockExpired => {
                // Automatic refund after timeout
                self.refund_both_escrows().await?;
                self.state = OrderState::Refunded;
            },
            _ => {
                self.state = OrderState::Failed;
            }
        }
        Ok(())
    }
}
```

## Timer-Based Coordination

### Periodic State Checks

```rust
use ic_cdk_timers::set_timer;

pub async fn start_periodic_checks() {
    set_timer(Duration::from_secs(30), || {
        ic_cdk::spawn(async {
            check_pending_orders().await;
        });
    });
}

async fn check_pending_orders() {
    let pending_orders = get_orders_by_state(OrderState::BothEscrowsReady);

    for order in pending_orders {
        // Check if hashlock has been revealed
        if let Ok(secret) = check_hashlock_revelation(&order).await {
            order.reveal_secret(secret).await;
        }

        // Check if timelock has expired
        if order.is_timelock_expired() {
            order.handle_timelock_expiry().await;
        }
    }
}
```

## EVM RPC Integration

### Ethereum Transaction Submission

```rust
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct EVMTransaction {
    pub to: String,
    pub data: Vec<u8>,
    pub value: u64,
    pub gas_limit: u64,
}

pub async fn submit_evm_transaction(tx: EVMTransaction) -> Result<String, CrossChainError> {
    // Use EVM RPC canister to submit transaction
    let response = call_evm_rpc_canister("eth_sendRawTransaction", &tx).await?;

    match response {
        Ok(tx_hash) => Ok(tx_hash),
        Err(_) => Err(CrossChainError::EVMTransactionFailed)
    }
}

pub async fn verify_evm_escrow_creation(escrow_address: &str, expected_hashlock: [u8; 32]) -> Result<bool, CrossChainError> {
    // Query EVM state via RPC canister
    let response = call_evm_rpc_canister("eth_call", &format!("0x{}", escrow_address)).await?;

    // Parse response and verify hashlock
    // Implementation depends on specific contract interface
    Ok(true)
}
```

## Certified Data for Frontend Trust

### State Certification

```rust
use ic_certified_data::set_certified_data;

impl Order {
    pub fn certify_state(&self) {
        let state_hash = self.compute_state_hash();
        set_certified_data(&state_hash);
    }

    fn compute_state_hash(&self) -> Vec<u8> {
        // Compute hash of current order state
        // This provides cryptographic proof to frontend
        let state_bytes = self.serialize_state();
        // Use SHA256 or similar
        sha256(&state_bytes)
    }
}
```

## Key Implementation Points

### 1. Asynchronous Non-Atomic Calls

- ICP inter-canister calls are asynchronous and non-atomic
- Always handle errors and design for idempotency and safe retries
- Use compensating actions for partial failures

### 2. Hashlock/Timelock Logic

- Use hashlocks and timelocks for cross-chain atomicity
- Design compensating actions for partial failures
- Implement automatic refund mechanisms

### 3. EVM RPC Canister Usage

- Use EVM RPC canister for secure, multi-provider EVM state queries
- Implement transaction submission via HTTPS outcalls
- Use threshold signing for Ethereum transactions

### 4. State Machine Design

- Model protocol as explicit state machine
- Each state transition only after confirmation from relevant chain
- Implement rollback mechanisms for failed transitions

### 5. Timer-Based Coordination

- Use `ic_cdk_timers` for periodic state checks
- Implement automatic timeout handling
- Poll for external events and trigger state transitions

### 6. Certified Data

- Use `ic_certified_data` for frontend trust
- Provide cryptographic proof of escrow state
- Enable frontend to verify state without trusting canister

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_order_state_transitions() {
        let mut order = Order::new();

        // Test valid transitions
        assert!(order.transition_to(OrderState::ICPEscrowCreated).await.is_ok());
        assert!(order.transition_to(OrderState::EVMEscrowCreated).await.is_ok());

        // Test invalid transitions
        assert!(order.transition_to(OrderState::Created).await.is_err());
    }

    #[tokio::test]
    async fn test_failure_recovery() {
        let mut order = Order::new();

        // Simulate EVM escrow creation failure
        let error = CrossChainError::EVMEscrowCreationFailed("Network error".to_string());
        assert!(order.handle_failure(error).await.is_ok());
        assert_eq!(order.state, OrderState::Cancelled);
    }
}
```

This implementation provides a robust foundation for cross-chain coordination while handling the inherent challenges of distributed systems.
