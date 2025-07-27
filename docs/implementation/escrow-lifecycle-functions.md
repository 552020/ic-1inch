# Escrow Lifecycle Functions (Step 4)

## Overview

This document outlines the implementation of core escrow lifecycle functions for our ICP HTLC canister. These functions manage the complete lifecycle of an escrow from creation to completion or refund.

**Note**: This is Step 4 in our implementation. Step 3 was Timelock Enforcement.

## Core Functions

### **1. Create Escrow**

```rust
#[update]
pub async fn create_escrow(params: CreateEscrowParams) -> Result<String, EscrowError>
```

**Purpose**: Creates a new escrow with the specified parameters.

**Parameters**:

- `hashlock`: SHA256 hash of the secret (from maker's signed intent)
- `timelock`: Expiration time in nanoseconds
- `token_canister`: ICRC-1 token canister ID
- `amount`: Token amount in smallest unit
- `recipient`: Token recipient on completion
- `depositor`: Original token depositor

**Returns**: Escrow ID (unique identifier)

**State Transition**: None → `Created`

**Usage**: Called by resolver during Phase 2 Step 2.3

### **2. Deposit Tokens**

```rust
#[update]
pub async fn deposit_tokens(escrow_id: String, amount: u64) -> Result<(), EscrowError>
```

**Purpose**: Deposits tokens into an existing escrow.

**Parameters**:

- `escrow_id`: ID of the escrow to fund
- `amount`: Amount of tokens to deposit

**Returns**: Success or error

**State Transition**: `Created` → `Funded`

**Usage**: Called by resolver during Phase 2 Step 2.4

### **3. Claim Escrow**

```rust
#[update]
pub async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError>
```

**Purpose**: Claims tokens by revealing the secret preimage.

**Parameters**:

- `escrow_id`: ID of the escrow to claim
- `preimage`: The secret that generates the hashlock

**Returns**: Success or error

**State Transition**: `Funded` → `Claimed`

**Usage**: Called by maker during Phase 3 Step 3.1

**Hashlock Verification**: Uses `verify_hashlock(preimage, stored_hashlock)`

### **4. Refund Escrow**

```rust
#[update]
pub async fn refund_escrow(escrow_id: String) -> Result<(), EscrowError>
```

**Purpose**: Refunds tokens to depositor after timelock expiration.

**Parameters**:

- `escrow_id`: ID of the escrow to refund

**Returns**: Success or error

**State Transition**: `Funded` → `Refunded`

**Usage**: Called by anyone after timelock expires (Phase 4)

## Query Functions

### **5. Get Escrow Status**

```rust
#[query]
pub fn get_escrow_status(escrow_id: String) -> Result<EscrowStatus, EscrowError>
```

**Purpose**: Returns the current status and details of an escrow.

**Parameters**:

- `escrow_id`: ID of the escrow to query

**Returns**: Complete escrow status including state, hashlock, timelock, etc.

**Usage**: Called by resolver during Phase 2 Step 2.4 for verification

### **6. List Escrows**

```rust
#[query]
pub fn list_escrows() -> Vec<EscrowStatus>
```

**Purpose**: Returns all escrows (for debugging/testing).

**Returns**: List of all escrow statuses

**Usage**: Development and testing purposes

## State Transitions

### **Valid State Flow**

```
Created → Funded → Claimed
     ↓
  Refunded
```

### **State Rules**

- **Created**: Escrow exists but not funded
- **Funded**: Tokens deposited, waiting for secret
- **Claimed**: Secret revealed, tokens transferred to recipient
- **Refunded**: Timelock expired, tokens returned to depositor
- **Expired**: Escrow expired, cleanup needed

## Error Handling

### **Common Error Scenarios**

1. **EscrowNotFound**: Escrow ID doesn't exist
2. **InvalidState**: Operation not allowed in current state
3. **InvalidHashlock**: Secret doesn't match hashlock (claim_escrow only)
4. **TimelockNotExpired**: Cannot refund before timelock expires
5. **TimelockExpired**: Cannot claim after timelock expires
6. **InsufficientBalance**: Not enough tokens for operation
7. **TransferFailed**: ICRC-1 transfer failed
8. **Unauthorized**: Caller not authorized for operation
9. **InvalidAmount**: Amount is zero or invalid
10. **InvalidTimelock**: Timelock is in the past

## Implementation Details

### **Memory Management**

- Uses `with_escrows()` for safe access to global state
- Thread-local storage for single-threaded ICP environment
- No unsafe code or static mutable state

### **Token Integration**

- ICRC-1 token transfers via cross-canister calls
- Balance checking before operations
- Fee handling for token transfers

### **Timelock Handling**

- Uses `ic_cdk::api::time()` for current time
- Nanosecond precision for timelock comparisons
- Automatic expiration detection

## Testing Strategy

### **Unit Tests**

- State transition validation
- Hashlock verification with valid/invalid secrets
- Timelock expiration logic
- Error condition handling

### **Integration Tests**

- End-to-end escrow lifecycle
- Cross-canister token transfers
- Concurrent operation handling

## Security Considerations

### **Access Control**

- Anyone can create escrows (resolver role)
- Anyone can claim with valid secret (maker role)
- Anyone can refund after timelock (safety mechanism)

### **Hashlock Security**

- SHA256 verification for all claims
- Secret never stored, only hashlock
- Collision-resistant hashing

### **Timelock Safety**

- Nanosecond precision prevents timing attacks
- Automatic expiration prevents stuck funds
- Clear refund mechanism

## Integration with 1inch Fusion+

### **Phase 2: Deposit**

- Resolver calls `create_escrow()` with hashlock from signed intent
- Resolver calls `deposit_tokens()` to fund the escrow
- Resolver calls `get_escrow_status()` to verify creation

### **Phase 3: Execution**

- Maker calls `claim_escrow()` with secret preimage
- Hashlock verification ensures atomicity
- Tokens transferred to maker

### **Phase 4: Recovery**

- Anyone can call `refund_escrow()` after timelock
- Safety mechanism for stuck escrows
- Tokens returned to depositor

## Next Steps

1. **Implement core functions** in `escrows.rs`
2. **Add query functions** to `lib.rs`
3. **Integrate with memory management**
4. **Add comprehensive error handling**
5. **Implement token transfer logic**
6. **Add testing framework**

---

**This step implements the complete escrow lifecycle that enables cross-chain atomic swaps!** 🔄
