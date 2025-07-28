# Maker vs Recipient in HTLC Escrows

## Overview

This document clarifies the distinction between `maker` and `recipient` in our HTLC escrow implementation, explaining why both are needed and how they relate to 1inch Fusion+ architecture.

## ✅ TL;DR

- **No** — you don't need both `recipient` and `maker`
- **Yes** — in 1inch Fusion+, the `maker` IS the recipient
- **Yes** — we simplified the API to use only `maker` field

## 🧠 Clarifying Roles in HTLC + 1inch Fusion+

### 1. **Maker**

- Creates and signs an intent off-chain (e.g., "I want to swap 100 ETH for 1000 ICP")
- Publishes it via 1inch Fusion+
- Is the one who **knows the secret preimage**
- On Ethereum, **funds the ETH escrow**
- On ICP, **claims the ICP escrow** by revealing the secret

### 2. **Resolver**

- Picks up the signed intent
- **Creates** and **funds** the escrows (both Ethereum + ICP)
- On ICP, creates escrow with:
  - `hashlock = maker's hashlock`
  - `depositor = resolver`
  - `recipient = maker`

### 3. **Recipient**

- The person who receives the ICP tokens when they provide the preimage
- **Same as maker** in 1inch Fusion+ destination-chain escrows

## 🔄 Do You Need Both `recipient` and `maker`?

**No, you don't need both.** Here's why:

| Role        | Purpose                          | Implementation |
| ----------- | -------------------------------- | -------------- |
| `maker`     | Off-chain identity, signs intent | Single field   |
| `recipient` | On-chain receiver of ICP         | Same as maker  |

**Simplified approach:**

- `maker` serves both purposes: off-chain identity AND on-chain recipient
- No need for separate `recipient` field
- Eliminates validation complexity

## 🔒 Should You Enforce `recipient == maker`?

**No validation needed.** We simplified the approach:

- **Removed `recipient` field** entirely
- **Use only `maker`** for both off-chain identity and on-chain recipient
- **No validation complexity** - can't have mismatched values
- **Cleaner API** - fewer parameters to manage

## ✅ Recommended Action

- Keep both `maker` (off-chain) and `recipient` (on-chain)
- Pass `recipient` explicitly in `CreateEscrowParams`
- **Add validation** to ensure `recipient == maker`
- Log both values for audit trails

## ✨ Final Notes

So:

- **Yes**, `recipient` is needed
- **Yes**, it's **always the maker** in 1inch Fusion+
- **Yes**, you should validate equality
- **Yes**, the resolver sets it when creating the escrow — based on the maker's signed intent

This aligns with official 1inch Fusion+ protocol specifications.

## 🔧 Implementation Implications

### Current Code Structure

```rust
pub struct CreateEscrowParams {
    pub hashlock: Vec<u8>,
    pub timelock: u64,
    pub token_canister: Principal,
    pub amount: u64,
    pub recipient: Principal,  // On-chain receiver
    pub maker: Principal,      // Off-chain identity
}
```

### Recommended Approach

1. **Keep both fields** in the data structure
2. **Add validation** to ensure `recipient == maker`
3. **Log both values** for audit trails
4. **Let resolver decide** the recipient based on the signed intent (but validate it matches maker)

### Example Use Cases

#### Standard Case (Only Valid in 1inch Fusion+)

- `maker` = Alice's off-chain identity
- `recipient` = Alice's on-chain principal
- Result: Alice receives ICP tokens

**Note**: Delegated cases and smart contract recipients are not supported in 1inch Fusion+ protocol

## 📋 Next Steps

1. **Add validation** to ensure `recipient == maker`
2. **Add logging** for both `maker` and `recipient`
3. **Update API documentation** to reflect this requirement
4. **Test validation** with error scenarios

---

**Last Updated**: [Current Date]
**Status**: Implementation Decision Made
