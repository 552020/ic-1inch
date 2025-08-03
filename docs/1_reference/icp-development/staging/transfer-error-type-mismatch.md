# TransferError and TransferArg Type Mismatch Issues

> **TODO:** This document needs to be reformulated in more general terms to be useful as ICP development reference. Currently it's too specific to test token implementation issues. Should be generalized to cover ICRC-1 type compatibility patterns, error handling best practices, and common type mismatch troubleshooting for ICP token integration.

## 🐛 **Problem**

When trying to fill limit orders, the backend receives a `TokenCallFailed` error with the message:

```
"Transfer call failed: (CanisterError, \"failed to decode canister response as (core::result::Result<u64, icrc_ledger_types::icrc1::transfer::TransferError>,): Fail to decode argument 0\")"
```

## 🔍 **Root Causes**

There were **two type mismatches** between the test tokens and the backend:

### **1. TransferError Type Mismatch**

The **test tokens** (`test_token_a` and `test_token_b`) were using a **custom `TransferError` type**, while the **backend expects** the `icrc_ledger_types::icrc1::transfer::TransferError` type.

**Test Token (Before):**

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

### **2. TransferArg Type Mismatch**

The **test tokens** were using a **custom `TransferArgs` struct**, while the **backend expects** the `icrc_ledger_types::icrc1::transfer::TransferArg` type.

**Test Token (Before):**

```rust
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub from_subaccount: Option<Vec<u8>>,
    pub to: Account,
    pub amount: u128,  // ← u128
    pub fee: Option<u128>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}
```

**Backend Expects:**

```rust
// From icrc_ledger_types::icrc1::transfer::TransferArg
pub struct TransferArg {
    pub from_subaccount: Option<Vec<u8>>,
    pub to: Account,
    pub amount: candid::Nat,  // ← candid::Nat
    pub fee: Option<candid::Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}
```

## 🎯 **Solutions**

### **1. Update Test Token Dependencies**

Add `icrc-ledger-types = "0.1"` to both test token `Cargo.toml` files.

### **2. Replace Custom Types with Standard Types**

Remove the custom `TransferError` and `TransferArgs` and import from `icrc_ledger_types`:

```rust
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};
```

### **3. Fix Type Conversions**

Update the transfer function to handle `candid::Nat` properly:

```rust
pub async fn icrc1_transfer(args: TransferArg) -> TransferResult {
    let amount = args.amount.0.try_into().unwrap_or(0u128);
    // ... rest of implementation
}
```

### **4. Fix Error Types**

Update the `InsufficientFunds` error to use `candid::Nat`:

```rust
TransferError::InsufficientFunds {
    balance: candid::Nat::from(from_balance)
}
```

## 🔧 **Files Updated**

- `src/test_token_a/Cargo.toml` - Added dependency
- `src/test_token_a/src/lib.rs` - Replaced custom types with standard types
- `src/test_token_b/Cargo.toml` - Added dependency
- `src/test_token_b/src/lib.rs` - Replaced custom types with standard types

## ✅ **Expected Result**

After redeployment, the `fill_order` calls should work correctly without decoding errors.

## 📝 **Status**

- [x] Identified both problems
- [x] Updated test token code to use standard ICRC-1 types
- [ ] Redeploy canisters
- [ ] Test fill_order functionality

## 🧠 **Why This Happened**

The test tokens were initially implemented with custom types for simplicity, but the backend was designed to work with the standard `icrc_ledger_types` library. This created a type mismatch when the backend tried to decode responses from the test tokens.

The fix ensures that both the backend and test tokens use the same standard ICRC-1 types, making them compatible with real ICRC-1 tokens.
