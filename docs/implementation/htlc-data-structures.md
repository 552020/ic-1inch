# HTLC Data Structures Reference

## Overview

This document defines the data structures needed for HTLC (Hashed Timelock Contract) implementation, based on 1inch Fusion+ protocol and adapted for ICP canisters.

## 1inch Fusion+ HTLC Structures

### Core Fusion+ Order Structure

Based on the 1inch Fusion+ whitepaper and documentation:

```rust
// 1inch Fusion+ Order Structure (for reference)
// Note: This is the Ethereum-side structure we need to understand
// Our ICP implementation will use different types (Principal vs String, etc.)

struct FusionOrder {
    // HTLC Parameters
    hashlock: String,         // SHA-256 hash of secret
    timelock: u64,           // Unix timestamp for expiration

    // Token Details
    token_in: String,         // Source token address
    token_out: String,        // Destination token address
    amount_in: String,        // Input amount (wei)
    amount_out: String,       // Expected output amount

    // Participants
    maker: String,            // Order creator
    recipient: String,        // Token recipient
    resolver: String,         // Resolver address

    // Escrow References
    source_escrow: String,    // Source chain escrow address
    destination_escrow: String, // Destination chain escrow address

    // Safety Deposit
    safety_deposit: String,   // Resolver incentive deposit
}
```

### Fusion+ Escrow Contract Structure

```solidity
// 1inch Fusion+ Escrow Contract (Ethereum)
struct EscrowData {
    bytes32 hashlock;         // SHA-256 hash of secret
    uint256 timelock;         // Expiration timestamp
    address token;            // Token contract address
    uint256 amount;           // Token amount
    address recipient;        // Token recipient
    address depositor;        // Original depositor
    address resolver;         // Resolver address
    uint256 safetyDeposit;    // Resolver incentive
    EscrowState state;        // Current escrow state
}

enum EscrowState {
    Created,    // Escrow created, not funded
    Funded,     // Tokens deposited
    Claimed,    // Secret revealed, tokens transferred
    Refunded,   // Timelock expired, tokens returned
    Expired     // Escrow expired
}
```

## Our ICP Implementation Structures

### Core Data Types

```rust
use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use std::collections::HashMap;

// Principal type for ICP canister IDs and user identities
use candid::Principal;
```

### CreateEscrowParams

Parameters for creating a new HTLC escrow:

```rust
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowParams {
    pub hashlock: Vec<u8>,           // SHA256 hash of secret (32 bytes)
    pub timelock: u64,               // Nanoseconds since epoch
    pub token_canister: Principal,   // ICRC-1 token canister ID
    pub amount: u64,                 // Token amount in smallest unit
    pub recipient: Principal,        // Token recipient on completion
    pub depositor: Principal,        // Original token depositor
    pub eth_escrow_address: String,  // Corresponding Ethereum escrow address
}
```

### Escrow State

Complete escrow data structure:

```rust
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Escrow {
    pub id: String,                  // Unique escrow identifier
    pub hashlock: Vec<u8>,           // SHA256 hash of secret
    pub timelock: u64,               // Expiration timestamp (nanoseconds)
    pub token_canister: Principal,   // ICRC-1 token canister ID
    pub amount: u64,                 // Token amount
    pub recipient: Principal,        // Token recipient
    pub depositor: Principal,        // Original depositor
    pub state: EscrowState,          // Current escrow state
    pub created_at: u64,             // Creation timestamp
    pub updated_at: u64,             // Last update timestamp
    pub eth_escrow_address: String,  // Corresponding Ethereum escrow
}
```

### Escrow State Enum

Lifecycle states for escrow management:

```rust
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum EscrowState {
    Created,    // Escrow created, not funded
    Funded,     // Tokens deposited, waiting for secret
    Claimed,    // Secret revealed, tokens transferred
    Refunded,   // Timelock expired, tokens returned
    Expired,    // Escrow expired, cleanup needed
}
```

### Timelock Status

Time-based access control:

```rust
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum TimelockStatus {
    Active,     // Timelock not expired
    Expired,    // Timelock has expired
}
```

## Mapping: 1inch Fusion+ → ICP

### Structure Comparison

| 1inch Fusion+ Element | ICP Implementation                  | Notes                         |
| --------------------- | ----------------------------------- | ----------------------------- |
| `hashlock`            | `CreateEscrowParams.hashlock`       | SHA-256 hash (32 bytes)       |
| `timelock`            | `CreateEscrowParams.timelock`       | Nanoseconds vs Unix timestamp |
| `token`               | `CreateEscrowParams.token_canister` | Principal vs Ethereum address |
| `amount`              | `CreateEscrowParams.amount`         | Same concept, different units |
| `recipient`           | `CreateEscrowParams.recipient`      | Principal vs Ethereum address |
| `depositor`           | `CreateEscrowParams.depositor`      | Principal vs Ethereum address |
| `state`               | `EscrowState`                       | Same lifecycle states         |
| `sourceEscrow`        | `eth_escrow_address`                | Cross-chain reference         |

### Key Differences

1. **Address Format**: Ethereum addresses vs ICP Principals
2. **Time Format**: Unix timestamps vs ICP nanoseconds
3. **Token Standard**: ERC-20 vs ICRC-1
4. **Cross-chain Reference**: Added `eth_escrow_address` for coordination

## Global State Management

### Escrow Storage

```rust
// Global escrow storage
static mut ESCROWS: Option<HashMap<String, Escrow>> = None;

// Thread-safe access to escrows
fn get_escrows() -> &'static mut HashMap<String, Escrow> {
    unsafe {
        if ESCROWS.is_none() {
            ESCROWS = Some(HashMap::new());
        }
        ESCROWS.as_mut().unwrap()
    }
}
```

### State Management Functions

```rust
// Create new escrow
fn create_escrow(params: CreateEscrowParams) -> Result<String, String> {
    let escrow_id = generate_escrow_id();
    let escrow = Escrow::new(escrow_id.clone(), params);
    get_escrows().insert(escrow_id.clone(), escrow);
    Ok(escrow_id)
}

// Get escrow by ID
fn get_escrow(escrow_id: &str) -> Option<Escrow> {
    get_escrows().get(escrow_id).cloned()
}

// Update escrow state
fn update_escrow_state(escrow_id: &str, new_state: EscrowState) -> Result<(), String> {
    if let Some(escrow) = get_escrows().get_mut(escrow_id) {
        escrow.state = new_state;
        escrow.updated_at = time();
        Ok(())
    } else {
        Err("Escrow not found".to_string())
    }
}
```

## Error Handling

### Escrow Error Types

```rust
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum EscrowError {
    // Creation errors
    InvalidHashlock,
    InvalidTimelock,
    InvalidTokenCanister,
    InvalidAmount,

    // State errors
    EscrowNotFound,
    InvalidState,
    AlreadyClaimed,
    AlreadyRefunded,

    // Timelock errors
    TimelockNotExpired,
    TimelockExpired,

    // Token errors
    InsufficientBalance,
    TransferFailed,
    TokenNotSupported,

    // Authorization errors
    Unauthorized,
    InvalidCaller,
}
```

## Candid Interface

### Generated Candid Types

```candid
// backend.did
type CreateEscrowParams = record {
    hashlock: blob;
    timelock: nat64;
    token_canister: principal;
    amount: nat64;
    recipient: principal;
    depositor: principal;
    eth_escrow_address: text;
};

type Escrow = record {
    id: text;
    hashlock: blob;
    timelock: nat64;
    token_canister: principal;
    amount: nat64;
    recipient: principal;
    depositor: principal;
    state: EscrowState;
    created_at: nat64;
    updated_at: nat64;
    eth_escrow_address: text;
};

type EscrowState = variant {
    Created;
    Funded;
    Claimed;
    Refunded;
    Expired;
};

type TimelockStatus = variant {
    Active;
    Expired;
};

service : {
    create_escrow: (CreateEscrowParams) -> (text) oneway;
    get_escrow_status: (text) -> (Escrow) query;
    deposit_tokens: (text, nat64) -> () oneway;
    claim_escrow: (text, blob) -> () oneway;
    refund_escrow: (text) -> () oneway;
}
```

## Validation Rules

### Parameter Validation

```rust
fn validate_create_escrow_params(params: &CreateEscrowParams) -> Result<(), EscrowError> {
    // Hashlock validation
    if params.hashlock.len() != 32 {
        return Err(EscrowError::InvalidHashlock);
    }

    // Timelock validation
    if params.timelock <= time() {
        return Err(EscrowError::InvalidTimelock);
    }

    // Amount validation
    if params.amount == 0 {
        return Err(EscrowError::InvalidAmount);
    }

    // Principal validation
    if params.recipient == params.depositor {
        return Err(EscrowError::InvalidCaller);
    }

    Ok(())
}
```

## References

- [1inch Fusion+ Whitepaper](https://1inch.io/assets/1inch-fusion-plus.pdf)
- [1inch Fusion+ API Documentation](https://1inch.dev/fusion-plus-api/)
- [ICP Candid Language](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)
- [ICRC-1 Token Standard](https://github.com/dfinity/ICRC-1)

---

This document provides the complete data structure foundation for implementing HTLC functionality in our ICP canister! 🔐
