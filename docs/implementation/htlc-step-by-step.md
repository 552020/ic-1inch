# HTLC Step-by-Step Implementation Guide

## Overview

This guide walks through implementing HTLC (Hashed Timelock Contract) functionality in our ICP canister, starting from a simple "hello world" and building up to a complete cross-chain atomic swap system.

## Step 0: Hello World (Current State)

Let's start with what we have now:

```rust
// src/backend/src/lib.rs
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
struct Greeting {
    message: String,
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Enable Candid export
ic_cdk::export_candid!();
```

**Test it:**

```bash
dfx deploy
dfx canister call backend greet '("World")'
# Returns: "Hello, World!"
```

## Step 1: Basic Data Structures

Add the core HTLC data types:

```rust
use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use std::collections::HashMap;

// Core HTLC data structures
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowParams {
    pub hashlock: Vec<u8>,           // SHA256 hash of secret
    pub timelock: u64,               // Nanoseconds since epoch
    pub token_canister: Principal,   // ICRC-1 token canister ID
    pub amount: u64,                 // Token amount in smallest unit
    pub recipient: Principal,        // Token recipient on completion
    pub depositor: Principal,        // Original token depositor
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Escrow {
    pub id: String,
    pub hashlock: Vec<u8>,
    pub timelock: u64,
    pub token_canister: Principal,
    pub amount: u64,
    pub recipient: Principal,
    pub depositor: Principal,
    pub state: EscrowState,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum EscrowState {
    Created,    // Escrow created, not funded
    Funded,     // Tokens deposited, waiting for secret
    Claimed,    // Secret revealed, tokens transferred
    Refunded,   // Timelock expired, tokens returned
    Expired,    // Escrow expired, cleanup needed
}

// Global state
static mut ESCROWS: Option<HashMap<String, Escrow>> = None;

// Helper function to get escrows
fn get_escrows() -> &'static mut HashMap<String, Escrow> {
    unsafe {
        if ESCROWS.is_none() {
            ESCROWS = Some(HashMap::new());
        }
        ESCROWS.as_mut().unwrap()
    }
}

// Keep the hello world function for testing
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

ic_cdk::export_candid!();
```

**Test it:**

```bash
dfx deploy
dfx canister call backend greet '("HTLC")'
# Returns: "Hello, HTLC!"
```

## Step 2: Hashlock Verification

Add SHA256 hashing functionality:

```rust
use sha2::{Sha256, Digest};

// Add this function to your lib.rs
pub fn verify_hashlock(preimage: &[u8], expected_hash: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let computed_hash = hasher.finalize();
    computed_hash.as_slice() == expected_hash
}

// Add a test function
#[ic_cdk::update]
fn test_hashlock(secret: Vec<u8>, expected_hash: Vec<u8>) -> bool {
    verify_hashlock(&secret, &expected_hash)
}
```

**Test it:**

```bash
# Create a test secret and hash
echo -n "my_secret_123" | sha256sum
# Returns: a1b2c3d4e5f6... (the hash)

dfx canister call backend test_hashlock '(
  blob "my_secret_123",
  blob "a1b2c3d4e5f6..."
)'
# Returns: true if hash matches
```

## Step 3: Timelock Enforcement

Add time-based access control:

```rust
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum TimelockStatus {
    Active,     // Timelock not expired
    Expired,    // Timelock has expired
}

pub fn check_timelock(timelock: u64) -> TimelockStatus {
    let current_time = time();

    if current_time < timelock {
        TimelockStatus::Active
    } else {
        TimelockStatus::Expired
    }
}

// Add a test function
#[ic_cdk::query]
fn test_timelock(timelock: u64) -> TimelockStatus {
    check_timelock(timelock)
}
```

**Test it:**

```bash
# Test with future timelock (should be Active)
dfx canister call backend test_timelock '(1_700_000_000_000_000_000 : nat64)'
# Returns: variant { Active }

# Test with past timelock (should be Expired)
dfx canister call backend test_timelock '(1_600_000_000_000_000_000 : nat64)'
# Returns: variant { Expired }
```

## Step 4: Create Escrow Function

Add the core escrow creation logic:

```rust
#[ic_cdk::update]
fn create_escrow(params: CreateEscrowParams) -> Result<String, String> {
    // Validate parameters
    if params.amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }

    if params.timelock <= time() {
        return Err("Timelock must be in the future".to_string());
    }

    // Generate unique escrow ID
    let escrow_id = format!("escrow_{}", time());

    // Create escrow
    let escrow = Escrow {
        id: escrow_id.clone(),
        hashlock: params.hashlock,
        timelock: params.timelock,
        token_canister: params.token_canister,
        amount: params.amount,
        recipient: params.recipient,
        depositor: params.depositor,
        state: EscrowState::Created,
        created_at: time(),
        updated_at: time(),
    };

    // Store escrow
    let escrows = get_escrows();
    escrows.insert(escrow_id.clone(), escrow);

    Ok(escrow_id)
}

// Add a query function to get escrow status
#[ic_cdk::query]
fn get_escrow_status(escrow_id: String) -> Result<Escrow, String> {
    let escrows = get_escrows();
    escrows.get(&escrow_id)
        .cloned()
        .ok_or_else(|| "Escrow not found".to_string())
}
```

**Test it:**

```bash
# Create an escrow
dfx canister call backend create_escrow '(
  record {
    hashlock = blob "a1b2c3d4e5f6...";
    timelock = 1_700_000_000_000_000_000 : nat64;
    token_canister = principal "rdmx6-jaaaa-aaaah-qcaiq-cai";
    amount = 1_000_000 : nat64;
    recipient = principal "user_principal";
    depositor = principal "resolver_principal";
  }
)'
# Returns: "escrow_1234567890"

# Check escrow status
dfx canister call backend get_escrow_status '("escrow_1234567890")'
# Returns: escrow details
```

## Step 5: Deposit Tokens

Add token deposit functionality:

```rust
#[ic_cdk::update]
fn deposit_tokens(escrow_id: String, amount: u64) -> Result<(), String> {
    let escrows = get_escrows();

    // Get escrow
    let escrow = escrows.get_mut(&escrow_id)
        .ok_or_else(|| "Escrow not found".to_string())?;

    // Check escrow state
    if escrow.state != EscrowState::Created {
        return Err("Escrow is not in Created state".to_string());
    }

    // Check amount matches
    if amount != escrow.amount {
        return Err("Deposit amount does not match escrow amount".to_string());
    }

    // Update escrow state
    escrow.state = EscrowState::Funded;
    escrow.updated_at = time();

    Ok(())
}
```

**Test it:**

```bash
# Deposit tokens
dfx canister call backend deposit_tokens '("escrow_1234567890", 1_000_000 : nat64)'
# Returns: ()

# Check updated status
dfx canister call backend get_escrow_status '("escrow_1234567890")'
# Returns: state = variant { Funded }
```

## Step 6: Claim Escrow

Add secret revelation and claiming:

```rust
#[ic_cdk::update]
fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), String> {
    let escrows = get_escrows();

    // Get escrow
    let escrow = escrows.get_mut(&escrow_id)
        .ok_or_else(|| "Escrow not found".to_string())?;

    // Check escrow state
    if escrow.state != EscrowState::Funded {
        return Err("Escrow is not in Funded state".to_string());
    }

    // Verify hashlock
    if !verify_hashlock(&preimage, &escrow.hashlock) {
        return Err("Invalid preimage".to_string());
    }

    // Check timelock
    if check_timelock(escrow.timelock) == TimelockStatus::Expired {
        return Err("Timelock has expired".to_string());
    }

    // Update escrow state
    escrow.state = EscrowState::Claimed;
    escrow.updated_at = time();

    // TODO: Transfer tokens to recipient (Step 7)

    Ok(())
}
```

**Test it:**

```bash
# Claim with correct secret
dfx canister call backend claim_escrow '(
  "escrow_1234567890",
  blob "my_secret_123"
)'
# Returns: ()

# Check final status
dfx canister call backend get_escrow_status '("escrow_1234567890")'
# Returns: state = variant { Claimed }
```

## Step 7: Token Transfers

Add ICRC-1 token integration:

```rust
use icrc_ledger_types::{
    icrc1::transfer::{TransferArgs, TransferError},
    icrc1::account::Account,
};

async fn transfer_tokens(
    token_canister: Principal,
    to: Principal,
    amount: u64,
) -> Result<u64, TransferError> {
    let transfer_args = TransferArgs {
        from_subaccount: None,
        to: Account {
            owner: to,
            subaccount: None,
        },
        amount: candid::Nat::from(amount),
        fee: None,
        memo: None,
        created_at_time: Some(time()),
    };

    ic_cdk::call(token_canister, "icrc1_transfer", (transfer_args,)).await
        .map(|result: Result<u64, TransferError>| result)
        .unwrap_or(Err(TransferError::GenericError { message: "Transfer failed".to_string() }))
}

// Update claim_escrow to include token transfer
#[ic_cdk::update]
async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), String> {
    let escrows = get_escrows();

    // Get escrow
    let escrow = escrows.get_mut(&escrow_id)
        .ok_or_else(|| "Escrow not found".to_string())?;

    // Check escrow state
    if escrow.state != EscrowState::Funded {
        return Err("Escrow is not in Funded state".to_string());
    }

    // Verify hashlock
    if !verify_hashlock(&preimage, &escrow.hashlock) {
        return Err("Invalid preimage".to_string());
    }

    // Check timelock
    if check_timelock(escrow.timelock) == TimelockStatus::Expired {
        return Err("Timelock has expired".to_string());
    }

    // Transfer tokens to recipient
    match transfer_tokens(escrow.token_canister, escrow.recipient, escrow.amount).await {
        Ok(_) => {
            // Update escrow state
            escrow.state = EscrowState::Claimed;
            escrow.updated_at = time();
            Ok(())
        },
        Err(e) => Err(format!("Token transfer failed: {:?}", e)),
    }
}
```

## Step 8: Refund Function

Add timelock expiration refund:

```rust
#[ic_cdk::update]
async fn refund_escrow(escrow_id: String) -> Result<(), String> {
    let escrows = get_escrows();

    // Get escrow
    let escrow = escrows.get_mut(&escrow_id)
        .ok_or_else(|| "Escrow not found".to_string())?;

    // Check escrow state
    if escrow.state != EscrowState::Funded {
        return Err("Escrow is not in Funded state".to_string());
    }

    // Check timelock has expired
    if check_timelock(escrow.timelock) != TimelockStatus::Expired {
        return Err("Timelock has not expired".to_string());
    }

    // Transfer tokens back to depositor
    match transfer_tokens(escrow.token_canister, escrow.depositor, escrow.amount).await {
        Ok(_) => {
            // Update escrow state
            escrow.state = EscrowState::Refunded;
            escrow.updated_at = time();
            Ok(())
        },
        Err(e) => Err(format!("Token transfer failed: {:?}", e)),
    }
}
```

## Step 9: Error Handling

Add comprehensive error handling:

```rust
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum EscrowError {
    EscrowNotFound,
    InvalidState,
    InvalidHashlock,
    TimelockNotExpired,
    TimelockExpired,
    InsufficientBalance,
    TransferFailed,
    Unauthorized,
}

impl std::fmt::Display for EscrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EscrowError::EscrowNotFound => write!(f, "Escrow not found"),
            EscrowError::InvalidState => write!(f, "Invalid escrow state"),
            EscrowError::InvalidHashlock => write!(f, "Invalid secret provided"),
            EscrowError::TimelockNotExpired => write!(f, "Timelock has not expired"),
            EscrowError::TimelockExpired => write!(f, "Timelock has expired"),
            EscrowError::InsufficientBalance => write!(f, "Insufficient token balance"),
            EscrowError::TransferFailed => write!(f, "Token transfer failed"),
            EscrowError::Unauthorized => write!(f, "Unauthorized operation"),
        }
    }
}
```

## Step 10: Complete Integration

Put it all together with proper error handling and state management.

## Testing Strategy

### Unit Tests

```bash
# Test each component individually
dfx canister call backend test_hashlock '(...)'
dfx canister call backend test_timelock '(...)'
```

### Integration Tests

```bash
# Test complete escrow lifecycle
dfx canister call backend create_escrow '(...)'
dfx canister call backend deposit_tokens '(...)'
dfx canister call backend claim_escrow '(...)'
```

### End-to-End Tests

```bash
# Test with real tokens on testnet
dfx deploy --network ic
# Run complete swap workflow
```

## Next Steps

1. **Add HTTP outcalls** for Ethereum verification
2. **Implement cross-chain coordination** scripts
3. **Add cycle management** and monitoring
4. **Create testing tools** for end-to-end validation

This step-by-step approach builds understanding incrementally while creating a working HTLC system! 🚀
