# TransferError Type Mismatch Issue

## 🐛 **Problem**

When trying to fill limit orders, the backend receives a `TokenCallFailed` error with the message:

```
"Transfer call failed: (CanisterError, \"failed to decode canister response as (core::result::Result<u64, icrc_ledger_types::icrc1::transfer::TransferError>,): Fail to decode argument 0\")"
```

## 🔍 **Root Cause**

The **test tokens** (`test_token_a` and `test_token_b`) are using a **custom `TransferError` type**, while the **backend expects** the `icrc_ledger_types::icrc1::transfer::TransferError` type.

### **Type Mismatch:**

**Test Token (Current):**

```rust
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferError {
    BadFee { expected_fee: u128 },
    BadBurn { min_burn_amount: u128 },
    InsufficientFunds { balance: u128 },  // ← u128
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u64 },
    TemporarilyUnavailable,
    GenericError { error_code: u128, message: String },
}
```

**Backend Expects:**

```rust
// From icrc_ledger_types::icrc1::transfer::TransferError
InsufficientFunds { balance: candid::Nat },  // ← candid::Nat
```

## 🎯 **Solution**

### **1. Update Test Token Dependencies**

Add `icrc-ledger-types = "0.1"` to both test token `Cargo.toml` files.

### **2. Replace Custom TransferError**

Remove the custom `TransferError` enum and import from `icrc_ledger_types`:

```rust
use icrc_ledger_types::icrc1::transfer::TransferError;
```

### **3. Fix Type Conversions**

Update the `InsufficientFunds` error to use `candid::Nat`:

```rust
TransferError::InsufficientFunds {
    balance: candid::Nat::from(from_balance)
}
```

## 🔧 **Files to Update**

- `src/test_token_a/Cargo.toml` - Add dependency
- `src/test_token_a/src/lib.rs` - Replace TransferError
- `src/test_token_b/Cargo.toml` - Add dependency
- `src/test_token_b/src/lib.rs` - Replace TransferError

## ✅ **Expected Result**

After redeployment, the `fill_order` calls should work correctly without decoding errors.

## 📝 **Status**

- [x] Identified the problem
- [x] Updated test token code
- [ ] Redeploy canisters
- [ ] Test fill_order functionality
