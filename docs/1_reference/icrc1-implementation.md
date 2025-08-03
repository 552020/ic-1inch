# ICRC-1 Token Standard Implementation

## Overview

This document explains the ICRC-1 token standard, why it's crucial for our ICP Limit Order Protocol MVP, and how we've implemented a compliant mock for testing purposes.

## What is ICRC-1?

**ICRC-1** (Internet Computer Request for Comments - 1) is the **primary token standard** for the Internet Computer Protocol (ICP). It defines a standardized interface for fungible tokens, similar to how ERC-20 works on Ethereum, but designed specifically for ICP's unique architecture.

### Key Characteristics of ICRC-1:

- **Native ICP Integration**: Built for canister-to-canister calls
- **Account Model**: Uses `(Principal, Option<Vec<u8>>)` for full account support
- **Fee Structure**: Standardized transfer fee handling
- **Block-based Ledger**: Each transaction gets a unique block index
- **Metadata Standards**: Defined fields for token information
- **Error Handling**: Comprehensive error types for robust dApps

### ICRC-1 vs Other Token Standards:

| **Feature**              | **ICRC-1 (ICP)**            | **ERC-20 (Ethereum)**          |
| ------------------------ | --------------------------- | ------------------------------ |
| **Gas Model**            | Reverse gas (canister pays) | User pays gas                  |
| **Account Type**         | Principal + Subaccount      | Ethereum address               |
| **Inter-contract Calls** | Native async calls          | External calls with gas limits |
| **Fee Model**            | Fixed transfer fees         | Variable gas fees              |
| **Upgradability**        | Canister upgrade support    | Contract immutability          |

## Why ICRC-1 for Our Limit Order Protocol?

### 1. **Native ICP Token Integration**

Our limit order protocol needs to interact with any token on ICP. ICRC-1 provides the standard interface that all ICP tokens implement, ensuring compatibility.

### 2. **Atomic Operations**

ICRC-1's async call model allows us to perform token transfers atomically within our limit order fills, essential for secure trading.

### 3. **Future ChainFusion+ Compatibility**

While our MVP uses ICRC-1 (direct transfers), ChainFusion+ will require ICRC-2 (approval/allowance) for resolver-based cross-chain operations.

### 4. **Cost Efficiency**

ICP's reverse gas model means our canister pays for token operations, providing a gasless experience for users.

## Our Mock ICRC-1 Implementation

### Architecture Overview

```rust
/// Mock ICRC-1 token for MVP testing
pub struct MockICRC1Token {
    // Core ICRC-1 compliance
    pub balances: RefCell<HashMap<(Principal, Option<Vec<u8>>), u64>>,
    pub total_supply: RefCell<u64>,
    pub block_index: RefCell<u64>,
    pub recent_transactions: RefCell<HashMap<Vec<u8>, u64>>,

    // Token metadata
    pub transfer_fee: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}
```

### Core ICRC-1 Methods Implemented

#### 1. **icrc1_balance_of**

```rust
pub fn icrc1_balance_of(&self, account: Account) -> u64 {
    let account_key = (account.owner, account.subaccount);
    *self.balances.borrow().get(&account_key).unwrap_or(&0)
}
```

- ✅ **Full subaccount support**
- ✅ **Standard Account structure**
- ✅ **Zero balance fallback**

#### 2. **icrc1_transfer**

```rust
pub fn icrc1_transfer(&self, from: Principal, args: TransferArgs) -> Result<u64, TransferError>
```

- ✅ **Proper account key handling**
- ✅ **Fee validation and deduction**
- ✅ **Balance validation**
- ✅ **Atomic balance updates**
- ✅ **Unique block index generation**
- ✅ **Basic memo-based deduplication**

#### 3. **icrc1_metadata**

```rust
pub fn icrc1_metadata(&self) -> Vec<(String, String)> {
    vec![
        ("icrc1:name".to_string(), self.name.clone()),
        ("icrc1:symbol".to_string(), self.symbol.clone()),
        ("icrc1:decimals".to_string(), self.decimals.to_string()),
        ("icrc1:fee".to_string(), self.transfer_fee.to_string()),
    ]
}
```

- ✅ **Standard metadata fields**
- ✅ **Proper ICRC-1 naming convention**

#### 4. **icrc1_total_supply**

```rust
pub fn icrc1_total_supply(&self) -> u64 {
    *self.total_supply.borrow()
}
```

### ICRC-1 Compliance Features

#### ✅ **What We Implemented (Full Compliance)**

1. **Account Structure**

   - `(Principal, Option<Vec<u8>>)` keys for full subaccount support
   - Proper handling of default subaccounts (`None`)

2. **Transfer Logic**

   - Fee validation with `BadFee` errors
   - Insufficient balance checking with `InsufficientFunds` errors
   - Atomic balance updates (sender debit, receiver credit)

3. **Block Index Management**

   - Unique, incremental block indices for each transaction
   - Realistic transaction tracking

4. **Error Handling**

   - Complete ICRC-1 `TransferError` enum
   - Proper error variants: `BadFee`, `InsufficientFunds`, `Duplicate`, etc.

5. **Metadata Standards**

   - All required ICRC-1 metadata fields
   - Proper naming conventions (`icrc1:name`, `icrc1:symbol`, etc.)

6. **Transaction Deduplication**
   - Basic memo-based duplicate prevention
   - `Duplicate` error with reference to original block

#### 🟡 **What We Simplified (Acceptable for MVP Testing)**

1. **Timestamp Validation**

   - **Omitted**: `created_at_time` validation
   - **Reason**: Complex timezone handling not needed for local testing
   - **Impact**: None for MVP functionality

2. **Advanced Deduplication**

   - **Simplified**: Only memo-based deduplication
   - **Full Standard**: Time-based + memo-based deduplication
   - **Reason**: Complex timing logic not needed for controlled tests

3. **Minting Account Logic**

   - **Simplified**: Direct `mint()` method for testing
   - **Full Standard**: Designated minting account with permissions
   - **Reason**: MVP doesn't need production minting workflows

4. **Burning Logic**
   - **Omitted**: Token burning capabilities
   - **Reason**: Not needed for limit order testing scenarios

#### ❌ **What We Excluded (Not Needed for MVP)**

1. **ICRC-2 Extensions**

   - **Excluded**: `approve()`, `allowance()`, `transfer_from()`
   - **Reason**: MVP uses direct transfers only
   - **Future**: Will add `MockICRC2Token` for ChainFusion+

2. **Production Persistence**

   - **Excluded**: Persistent storage across canister upgrades
   - **Reason**: Mock is recreated for each test

3. **Network Integration**
   - **Excluded**: Integration with ICP ledger canisters
   - **Reason**: Pure in-memory mock for unit testing

## Testing Integration

### Global Mock Instances

```rust
thread_local! {
    static MOCK_TOKEN_A: MockICRC1Token = MockICRC1Token::new(
        "Mock Token A".to_string(),
        "MTKN_A".to_string(),
        8,
        10000,
    );

    static MOCK_TOKEN_B: MockICRC1Token = MockICRC1Token::new(
        "Mock Token B".to_string(),
        "MTKN_B".to_string(),
        8,
        20000,
    );
}
```

### Test Utilities

```rust
impl MockICRC1Token {
    /// Mint tokens for testing setup
    pub fn mint(&self, to: Principal, amount: u64) { ... }

    /// Set balance directly for test scenarios
    pub fn set_balance(&self, account: Principal, amount: u64) { ... }

    /// Clear all state for test cleanup
    pub fn clear(&self) { ... }
}
```

### Usage in Limit Order Tests

```rust
// Setup test environment
MOCK_TOKEN_A.with(|token| {
    token.mint(maker_principal, 1000);
});

MOCK_TOKEN_B.with(|token| {
    token.mint(taker_principal, 2000);
});

// Test limit order creation and filling
// Our limit order functions call icrc1_balance_of and icrc1_transfer
// exactly as they would with real ICRC-1 tokens
```

## Production Considerations

### Migration Path to Real ICRC-1 Tokens

When moving from MVP to production, our code will work seamlessly with real ICRC-1 tokens because:

1. **Interface Compatibility**: Our mock implements the exact ICRC-1 interface
2. **Error Handling**: We handle all ICRC-1 error variants properly
3. **Account Model**: Full subaccount support from day one
4. **Async Patterns**: Mock prepares code for async token calls

### Real ICRC-1 Integration

```rust
// Current mock usage
MOCK_TOKEN_A.with(|token| {
    token.icrc1_balance_of(account)
});

// Future production usage
let balance: Result<(u64,), _> = ic_cdk::call(
    token_canister_id,
    "icrc1_balance_of",
    (account,)
).await;
```

### ICRC-2 Evolution for ChainFusion+

Our architecture supports future ICRC-2 integration:

```rust
// Future ICRC-2 methods for ChainFusion+ resolvers
trait TokenInterfaceExtended {
    async fn icrc2_approve(&self, spender: Principal, amount: u64) -> Result<u64, ApprovalError>;
    async fn icrc2_transfer_from(&self, owner: Principal, to: Principal, amount: u64) -> Result<u64, TransferFromError>;
    async fn icrc2_allowance(&self, owner: Principal, spender: Principal) -> Result<u64, AllowanceError>;
}
```

## Benefits of Our Approach

### 1. **Realistic Testing**

- Behavior matches real ICRC-1 tokens
- Proper error conditions and edge cases
- Authentic account and balance management

### 2. **Development Velocity**

- No need for deployed token canisters during development
- Instant test setup and teardown
- Deterministic test environments

### 3. **Standard Compliance**

- Based on official ICP documentation
- Implements core ICRC-1 requirements
- Ready for production token integration

### 4. **Future-Proof Architecture**

- Extensible to ICRC-2 for ChainFusion+
- Compatible with any ICRC-1 compliant token
- Supports advanced ICP features (subaccounts, etc.)

## Related Documentation

- **ICRC-1 Official Standard**: [internetcomputer.org/docs/references/icrc1-standard](https://internetcomputer.org/docs/references/icrc1-standard)
- **ICP Token Standards**: [internetcomputer.org/docs/defi/token-standards](https://internetcomputer.org/docs/defi/token-standards)
- **Our API Specification**: [api-specification.md](./api-specification.md)
- **Testing Guide**: [testing/](./testing/)

---

**Summary**: Our `MockICRC1Token` provides a production-ready foundation for ICRC-1 integration while enabling comprehensive testing of our Limit Order Protocol MVP. It balances full standard compliance with practical testing needs, ensuring smooth evolution to production ICP deployment.
