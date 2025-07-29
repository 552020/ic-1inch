# ICRC-2 Implementation for Limit Order Protocol

## Problem

ICRC-1 tokens don't support third-party transfers. Backend canister can't transfer tokens on behalf of users, breaking order execution.

**Error**: `TransferFailed = "Transfer failed: InsufficientFunds { balance: Nat(0) }"`

## Root Cause

```rust
// BROKEN - ignores from parameter
pub async fn transfer(&self, _from: Principal, to: Principal, amount: u64) -> OrderResult<u64> {
    // Always transfers from backend canister (has 0 tokens)
}
```

## Solution: ICRC-2 Migration

### Week 1: Test Tokens

- [ ] Add `icrc2_approve` to test_token_a
- [ ] Add `icrc2_approve` to test_token_b
- [ ] Add `icrc2_transfer_from` to both tokens
- [ ] Test allowance functionality

**Example Implementation:**

```rust
#[update]
pub async fn icrc2_approve(args: ApproveArgs) -> Result<ApproveResult, ApproveError> {
    // Implementation for user approval
}

#[update]
pub async fn icrc2_transfer_from(args: TransferFromArgs) -> Result<TransferFromResult, TransferFromError> {
    // Implementation for backend transfer
}
```

**Testing Commands:**

```bash
# User approves backend
dfx canister call test_token_a icrc2_approve "(record { amount = 100_000; spender = record{owner = principal \"<BACKEND_ID>\";} })"

# Backend transfers on behalf
dfx canister call test_token_a icrc2_transfer_from "(record { amount = 90_000; from = record{owner = principal \"<USER_ID>\"}; to= record{owner = principal \"<DEST_ID>\"}; })"
```

### Week 2: Backend Updates

- [ ] Update `TokenInterface::transfer` to use `icrc2_transfer_from`
- [ ] Add allowance checks in order validation
- [ ] Update order execution flow

**Backend Changes:**

```rust
pub async fn transfer(&self, from: Principal, to: Principal, amount: u64) -> OrderResult<u64> {
    let transfer_from_args = TransferFromArgs {
        from: Account { owner: from, subaccount: None },
        to: Account { owner: to, subaccount: None },
        amount: amount.into(),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let result: std::result::Result<(std::result::Result<u64, TransferFromError>,), _> =
        ic_cdk::call(self.canister_id, "icrc2_transfer_from", (transfer_from_args,)).await;

    // Handle result...
}
```

### Week 3: Integration

- [ ] Update manual testing guide
- [ ] Test complete order lifecycle
- [ ] Fix any issues found

### Week 4: Production Ready

- [ ] Deploy to testnet
- [ ] Final testing
- [ ] Documentation updates

## Why ICRC-2?

- **ICRC-1**: Only owner can transfer (broken)
- **ICRC-2**: Owner can approve backend to transfer (works)

**Benefits:**

- ✅ Standards-compliant solution
- ✅ Secure third-party transfers
- ✅ Interoperable with DeFi protocols
- ✅ Future-proof architecture

## Alternative: Quick MVP Fix

Add `transfer_from_backend` method to test tokens for immediate testing.

**Which approach?** ICRC-2 (proper) or quick fix (MVP)?

## Migration Steps Summary

1. **Token Canisters**: Implement `icrc2_approve` and `icrc2_transfer_from`
2. **Backend Logic**: Use `icrc2_transfer_from` instead of direct transfers
3. **Testing**: Include approval steps in order lifecycle
4. **Production**: Deploy and validate on testnet

**References:**

- [ICRC-2 Standard](https://internetcomputer.org/docs/defi/token-standards#icrc-2)
- [Motoko Example](https://internetcomputer.org/docs/references/samples/motoko/token_transfer_from/)
- [Rust Example](https://internetcomputer.org/docs/references/samples/rust/token_transfer_from/)
