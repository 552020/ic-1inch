# Hashlock Verification (Step 2)

## Overview

Hashlock verification is the core security mechanism of HTLC (Hashed Timelock Contract). It ensures that funds can only be unlocked by someone who knows the secret preimage that generates the hashlock.

## What is Hashlock Verification?

### **The Problem**

In cross-chain atomic swaps, we need a way to:

1. **Lock funds** on both chains with the same secret
2. **Unlock funds** only when the secret is revealed
3. **Prevent double-spending** or unauthorized access

### **The Solution: Hashlock**

- **Hashlock** = SHA256 hash of a secret
- **Secret** = Random preimage that generates the hashlock
- **Verification** = Prove you know the secret by revealing it

## How It Works

### **1. Secret Generation**

```javascript
// Generate a random secret (32 bytes) - happens on frontend/client
const secret = generateRandomSecret(); // 32 random bytes

// Create hashlock from secret - happens on frontend/client
const hashlock = sha256(secret);

// Only the hashlock is sent to 1inch API, secret stays private
// Maker submits signed intent with hashlock (not secret) to 1inch
```

**Note**: Secret generation happens on the **frontend/client side**, not in our ICP canister. Only the hashlock is communicated through the 1inch API.

### **2. Escrow Creation**

```rust
// Resolver calls our canister with hashlock from signed intent
#[update]
pub async fn create_escrow(params: CreateEscrowParams) -> Result<String, EscrowError> {
    // params.hashlock comes from resolver who fetched it from 1inch API
    let escrow = Escrow {
        hashlock: params.hashlock,  // Only the hash is stored, not the secret
        // ... other fields
    };
}
```

### **3. Resolver Verification**

```rust
// Resolver verifies escrow was created correctly before depositing
#[query]
pub fn get_escrow_status(escrow_id: String) -> Result<EscrowStatus, EscrowError> {
    // Resolver calls this to verify:
    // - hashlock matches the one from signed intent
    // - escrow state is "Created"
    // - all parameters are correct
}
```

### **3. Fund Unlocking**

```rust
// Maker calls this endpoint to reveal their secret and claim tokens
#[update]
pub async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    // Maker reveals their secret (preimage) to unlock funds
    let is_valid = verify_hashlock(&preimage, &escrow.hashlock);
    if is_valid {
        // Transfer tokens to maker
        // Update escrow state to "Claimed"
    }
}
```

## Implementation Details

### **SHA256 Hashing**

```rust
use sha2::{Sha256, Digest};

pub fn verify_hashlock(preimage: &[u8], expected_hash: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let computed_hash = hasher.finalize();
    computed_hash.as_slice() == expected_hash
}
```

### **Security Properties**

- **One-way function** - Can't derive secret from hashlock
- **Collision resistant** - Different secrets produce different hashlocks
- **Deterministic** - Same secret always produces same hashlock

## Cross-Chain Coordination

### **1inch Fusion+ Flow**

1. **User creates order** with hashlock (not secret)
2. **Resolver creates escrows** on both chains with same hashlock
3. **User reveals secret** to unlock destination escrow
4. **Resolver uses same secret** to unlock source escrow

### **Why This Works**

- **Same hashlock** on both chains ensures atomicity
- **Secret revelation** on one chain enables unlocking on both
- **Timelock** provides safety if secret is never revealed

## Testing Hashlock Verification

### **Test Cases**

```rust
#[test]
fn test_hashlock_verification() {
    // Test with correct secret
    let secret = b"my_secret_123";
    let hashlock = sha256(secret);
    assert!(verify_hashlock(secret, &hashlock));

    // Test with wrong secret
    let wrong_secret = b"wrong_secret";
    assert!(!verify_hashlock(wrong_secret, &hashlock));
}
```

### **Integration Testing**

```bash
# Generate test hashlock
echo -n "my_secret_123" | sha256sum
# Returns: a1b2c3d4e5f6...

# Test via canister
dfx canister call backend test_hashlock '(
  blob "my_secret_123",
  blob "a1b2c3d4e5f6..."
)'
# Should return: true
```

## Security Considerations

### **Secret Management**

- **Never store secrets** in escrow state
- **Only store hashlock** (one-way hash)
- **Reveal secret** only when claiming funds

### **Hashlock Generation**

- **Use cryptographically secure** random secrets
- **32 bytes minimum** for sufficient entropy
- **Unique per swap** to prevent replay attacks

### **Verification Timing**

- **Verify before state changes** - ensure secret is valid
- **Fail fast** - reject invalid secrets immediately
- **Log verification attempts** - for audit purposes

## Integration with Our ICP Canister

### **Current Implementation**

- **Step 1**: Basic data structures with hashlock field
- **Step 2**: Add hashlock verification function
- **Future steps**: Integrate with escrow lifecycle

### **When Hashlock Verification is Used in Our Backend Canister**

#### **Phase 3: Execution (Secret Revelation) - ONLY TIME WE USE IT**

- **Step 3.1: Secret Revelation** - **CRITICAL MOMENT**: When Alice reveals her secret to claim ICP tokens
  ```
  Alice calls: claim_escrow(escrow_id, preimage)
  Our canister: verify_hashlock(preimage, stored_hashlock) → true/false
  ```
  - **This is the ONLY time** our canister uses hashlock verification
  - **Function**: `verify_hashlock(preimage, hashlock)` is called inside `claim_escrow()`

#### **Phase 2: Deposit - NO VERIFICATION NEEDED**

- **Step 2.3: Create ICP Escrow** - We just store the hashlock, no verification needed
- **Step 2.4: Deposit Tokens** - We just accept deposits, no verification needed

#### **Phase 4: Recovery - NO VERIFICATION NEEDED**

- **Step 4.1: Refund Process** - We check timelock expiry, not hashlock
- **Step 4.2: State Cleanup** - We just update state, no verification needed

#### **Phase 2: Deposit (Escrow Creation)**

- **Step 2.3: Create ICP Escrow** - Verify that the hashlock provided matches the expected format and is cryptographically valid
  - **Source**: Resolver fetches hashlock from maker's signed intent offchain through 1inch API
  - **Process**: Resolver passes hashlock (along with other relevant information) to our backend canister
  - **Validation**: Ensure hashlock is 32 bytes SHA256 hash format
  - **Uniqueness**: Check that hashlock hasn't been used in previous escrows
- **Step 2.4: Deposit Tokens** - **Resolver verifies** that the ICP escrow was created successfully with the correct hashlock before depositing their ICP tokens
  - **Who**: Resolver (Bob) is doing the verification
  - **What**: Verifying that the ICP escrow was created with the same hashlock from the maker's intent
  - **Why**: Resolver won't deposit their ICP unless they're sure the escrow is properly set up
  - **How**: Resolver calls `get_escrow_status(escrow_id)` to verify the escrow details before depositing
  - **Implementation**: We need a query endpoint that returns escrow details for verification

#### **Phase 3: Execution (Secret Revelation)**

- **Step 3.1: Secret Revelation** - **CRITICAL MOMENT**: When Alice reveals her secret to claim ICP tokens
  ```
  Alice: "Here's my secret: my_super_secret_123"
  ICP Escrow: verify_hashlock(secret, stored_hashlock) → true/false
  ```
  - **Who**: Alice calls our ICP canister directly via `claim_escrow(escrow_id, secret)`
  - **Not via 1inch API**: Alice reveals secret directly to our canister, not to 1inch
  - **Why**: Our canister needs to verify the secret and transfer ICP tokens
- **Step 3.2: Token Transfers** - Only proceed with token transfer if hashlock verification succeeds

#### **Phase 4: Recovery (Timelock Expiry)**

- **Step 4.1: Refund Process** - Verify that timelock has expired before allowing refunds
- **Step 4.2: State Cleanup** - Ensure escrow state transitions are valid

### **Key Verification Points**

1. **Escrow Creation** - Validate hashlock format and uniqueness
2. **Secret Revelation** - **Most critical** - verify preimage matches hashlock
3. **State Transitions** - Ensure escrow moves from `Funded` → `Claimed` only after valid verification
4. **Error Handling** - Reject invalid secrets and log verification attempts

### **Next Steps**

1. **Add SHA256 dependency** to Cargo.toml
2. **Implement verify_hashlock()** function
3. **Add test function** for verification
4. **Integrate with escrow creation** and claiming

## Why This Matters for Our MVP

### **Core HTLC Functionality**

- **Essential security mechanism** - without this, HTLC doesn't work
- **Cross-chain atomicity** - ensures both chains use same secret
- **1inch Fusion+ compatibility** - matches their hashlock approach

### **MVP Requirements**

- **Working hashlock verification** - prove the concept works
- **Testable functionality** - demonstrate security mechanism
- **Foundation for escrow lifecycle** - enable create/claim/refund

---

**This step implements the cryptographic foundation that makes HTLC secure and enables cross-chain atomic swaps!** 🔐
