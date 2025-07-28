# HTLC (Hashed Timelock Contract) Primer

## Overview

HTLC is a **cryptographic protocol** that enables **atomic cross-chain swaps** by using two security mechanisms:

1. **Hashlock** - Funds locked with a cryptographic hash
2. **Timelock** - Funds have a time-based expiration

## How HTLC Works

### Step 1: Setup

```
├── Alice generates a secret (preimage)
├── Alice creates hash = SHA256(secret)
├── Both chains lock funds with same hash + timelock
```

### Step 2: Execution (Atomic)

```
├── Alice reveals secret to Chain A → unlocks funds
├── Bob learns secret from Chain A
├── Bob reveals secret to Chain B → unlocks funds
├── Both succeed or both fail (atomic)
```

### Step 3: Recovery (if failed)

```
├── After timelock expires
├── Anyone can refund funds to original owners
```

## Key Properties

- **Atomic**: Either both sides complete or both refund
- **Trustless**: No trusted third party needed
- **Time-bounded**: Automatic refund after timeout
- **Secret-based**: Knowledge of secret unlocks funds

## In Our Context: 1inch Fusion+ → ICP

For our **1inch Fusion+ → ICP** implementation:

1. **Ethereum side**: User locks ETH/ERC-20 tokens in 1inch escrow
2. **ICP side**: Resolver locks ICRC-1 tokens in our canister
3. **Secret revelation**: Resolver reveals secret to both chains
4. **Atomic completion**: Both token transfers happen or both refund

## Security Guarantees

- **No double-spending**: Funds locked until secret revealed
- **No front-running**: Secret must be known to claim
- **No permanent locking**: Timelock ensures eventual refund
- **Cross-chain consistency**: Same secret unlocks both sides

## Technical Implementation

### Hashlock Verification

```rust
// Verify secret matches stored hash
pub fn verify_hashlock(preimage: &[u8], expected_hash: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let computed_hash = hasher.finalize();
    computed_hash.as_slice() == expected_hash
}
```

### Timelock Enforcement

```rust
// Check if timelock has expired
pub fn check_timelock(timelock: u64) -> TimelockStatus {
    let current_time = ic_cdk::api::time();

    if current_time < timelock {
        TimelockStatus::Active
    } else {
        TimelockStatus::Expired
    }
}
```

## Escrow States

```rust
pub enum EscrowState {
    Created,    // Escrow created, not funded
    Funded,     // Tokens deposited, waiting for secret
    Claimed,    // Secret revealed, tokens transferred
    Refunded,   // Timelock expired, tokens returned
    Expired,    // Escrow expired, cleanup needed
}
```

## Cross-Chain Flow

```
1. User creates 1inch Fusion+ order (Ethereum)
2. Resolver monitors and matches order
3. Resolver creates escrows on both chains:
   ├── Ethereum: Via 1inch contracts
   └── ICP: Via our Rust canister
4. Resolver deposits tokens in both escrows
5. Resolver reveals secret to both chains
6. Atomic completion: Both transfers succeed
```

## Failure Scenarios

### Partial Execution

- **Ethereum claim succeeds, ICP fails**: Manual recovery needed
- **ICP claim succeeds, Ethereum fails**: Manual recovery needed
- **Network partition**: Timelock ensures eventual refund

### Timelock Expiration

- **Before secret revelation**: Automatic refund to depositors
- **After partial execution**: Manual recovery procedures

## References

- [HTLC Overview (Bitcoin Wiki)](https://en.bitcoin.it/wiki/Hash_Time_Locked_Contracts)
- [1inch Fusion+ Documentation](https://docs.1inch.io/docs/fusion/)
- [ICP Canister Development](https://internetcomputer.org/docs/current/developer-docs/backend/canister-development)

---

This is the **core mechanism** that makes cross-chain atomic swaps possible! 🔐
