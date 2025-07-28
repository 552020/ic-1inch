use candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;

/// Mock ICRC-1 token implementation for MVP testing
///
/// Note: This implements ICRC-1 standard (direct transfers) for the MVP.
/// ICRC-2 (approval/allowance) will be needed later for ChainFusion+ where
/// resolvers need to transfer tokens on behalf of users.
///
/// ICRC-1 Compliance Improvements (based on ICP documentation):
/// ✅ Proper subaccount support using (Principal, Option<Vec<u8>>) keys
/// ✅ Incremental block indices for unique transaction tracking
/// ✅ Basic transaction deduplication using memo field
/// ✅ Standard ICRC-1 error variants and metadata fields
///
/// Limitations (acceptable for MVP testing):
/// - No timestamp validation for created_at_time
/// - No minting account or burning logic
/// - Limited deduplication (memo-based only)
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub from_subaccount: Option<Vec<u8>>,
    pub to: Account,
    pub amount: u64,
    pub fee: Option<u64>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferError {
    BadFee { expected_fee: u64 },
    BadBurn { min_burn_amount: u64 },
    InsufficientFunds { balance: u64 },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u64 },
    TemporarilyUnavailable,
    GenericError { error_code: u64, message: String },
}

/// Mock ICRC-1 token state for testing
///
/// Implements ICRC-1 methods needed for MVP with improved compliance:
/// - icrc1_balance_of (with proper subaccount support)
/// - icrc1_transfer (with unique block indices and deduplication)
/// - icrc1_metadata
/// - icrc1_total_supply
///
/// Improvements over basic mock:
/// - Uses (Principal, Option<Vec<u8>>) as balance key for full account support
/// - Incremental block indices for realistic transaction tracking  
/// - Basic transaction deduplication by memo
///
/// Future: Will need MockICRC2Token for ChainFusion+ approval workflows
pub struct MockICRC1Token {
    pub balances: RefCell<HashMap<(Principal, Option<Vec<u8>>), u64>>,
    pub total_supply: RefCell<u64>,
    pub block_index: RefCell<u64>,
    pub recent_transactions: RefCell<HashMap<Vec<u8>, u64>>, // memo -> block_index for deduplication
    pub transfer_fee: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl MockICRC1Token {
    /// Create new mock token with initial supply
    pub fn new(name: String, symbol: String, decimals: u8, transfer_fee: u64) -> Self {
        Self {
            balances: RefCell::new(HashMap::new()),
            total_supply: RefCell::new(0),
            block_index: RefCell::new(0),
            recent_transactions: RefCell::new(HashMap::new()),
            transfer_fee,
            name,
            symbol,
            decimals,
        }
    }

    /// Mint tokens to account (for testing)
    pub fn mint(&self, to: Principal, amount: u64) {
        let account_key = (to, None); // Default subaccount
        let mut balances = self.balances.borrow_mut();
        let current_balance = *balances.get(&account_key).unwrap_or(&0);
        balances.insert(account_key, current_balance + amount);
        drop(balances); // Explicitly drop to avoid borrow checker issues

        let mut total_supply = self.total_supply.borrow_mut();
        *total_supply += amount;
    }

    /// Set balance directly (for testing)
    pub fn set_balance(&self, account: Principal, amount: u64) {
        let account_key = (account, None); // Default subaccount
        self.balances.borrow_mut().insert(account_key, amount);
    }

    /// ICRC-1 balance_of implementation with proper subaccount support
    pub fn icrc1_balance_of(&self, account: Account) -> u64 {
        let account_key = (account.owner, account.subaccount);
        *self.balances.borrow().get(&account_key).unwrap_or(&0)
    }

    /// ICRC-1 transfer implementation with proper account support and block indices
    pub fn icrc1_transfer(
        &self,
        from: Principal,
        args: TransferArgs,
    ) -> Result<u64, TransferError> {
        // Check for transaction deduplication if memo is provided
        if let Some(ref memo) = args.memo {
            let transactions = self.recent_transactions.borrow();
            if let Some(&existing_block) = transactions.get(memo) {
                return Err(TransferError::Duplicate {
                    duplicate_of: existing_block,
                });
            }
        }

        let from_account = (from, args.from_subaccount.clone());
        let to_account = (args.to.owner, args.to.subaccount.clone());

        let mut balances = self.balances.borrow_mut();

        // Check sender balance
        let sender_balance = *balances.get(&from_account).unwrap_or(&0);
        let total_amount = args.amount + self.transfer_fee;

        if sender_balance < total_amount {
            return Err(TransferError::InsufficientFunds {
                balance: sender_balance,
            });
        }

        // Check fee
        if args.fee.is_some() && args.fee.unwrap() != self.transfer_fee {
            return Err(TransferError::BadFee {
                expected_fee: self.transfer_fee,
            });
        }

        // Execute transfer
        balances.insert(from_account, sender_balance - total_amount);

        let receiver_balance = *balances.get(&to_account).unwrap_or(&0);
        balances.insert(to_account, receiver_balance + args.amount);
        drop(balances);

        // Generate unique block index
        let mut block_index = self.block_index.borrow_mut();
        *block_index += 1;
        let current_block = *block_index;
        drop(block_index);

        // Store transaction for deduplication if memo provided
        if let Some(memo) = args.memo {
            self.recent_transactions
                .borrow_mut()
                .insert(memo, current_block);
        }

        Ok(current_block)
    }

    /// Get metadata
    pub fn icrc1_metadata(&self) -> Vec<(String, String)> {
        vec![
            ("icrc1:name".to_string(), self.name.clone()),
            ("icrc1:symbol".to_string(), self.symbol.clone()),
            ("icrc1:decimals".to_string(), self.decimals.to_string()),
            ("icrc1:fee".to_string(), self.transfer_fee.to_string()),
        ]
    }

    /// Get total supply
    pub fn icrc1_total_supply(&self) -> u64 {
        *self.total_supply.borrow()
    }

    /// Clear all balances (for testing cleanup)
    pub fn clear(&self) {
        self.balances.borrow_mut().clear();
        self.recent_transactions.borrow_mut().clear();
        *self.total_supply.borrow_mut() = 0;
        *self.block_index.borrow_mut() = 0;
    }
}

// Global mock ICRC-1 tokens for MVP testing
thread_local! {
    static MOCK_TOKEN_A: MockICRC1Token = MockICRC1Token::new(
        "Test Token A".to_string(),
        "TTA".to_string(),
        8,
        10_000  // ICRC-1 transfer fee
    );
    static MOCK_TOKEN_B: MockICRC1Token = MockICRC1Token::new(
        "Test Token B".to_string(),
        "TTB".to_string(),
        6,
        5_000   // ICRC-1 transfer fee
    );
}

// Future: Add ICRC-2 mock tokens for ChainFusion+ testing
// thread_local! {
//     static MOCK_TOKEN_A_ICRC2: MockICRC2Token = MockICRC2Token::new(...);
//     static MOCK_TOKEN_B_ICRC2: MockICRC2Token = MockICRC2Token::new(...);
// }

/// ICRC-1 test helper functions for MVP
pub fn with_mock_token_a<T>(f: impl FnOnce(&MockICRC1Token) -> T) -> T {
    MOCK_TOKEN_A.with(f)
}

pub fn with_mock_token_b<T>(f: impl FnOnce(&MockICRC1Token) -> T) -> T {
    MOCK_TOKEN_B.with(f)
}

/// Setup ICRC-1 test tokens with initial balances for MVP testing
pub fn setup_test_tokens(maker: Principal, taker: Principal, maker_amount: u64, taker_amount: u64) {
    with_mock_token_a(|token| {
        token.clear();
        token.mint(maker, maker_amount);
    });

    with_mock_token_b(|token| {
        token.clear();
        token.mint(taker, taker_amount);
    });
}

/// Cleanup ICRC-1 test tokens
pub fn cleanup_test_tokens() {
    with_mock_token_a(|token| token.clear());
    with_mock_token_b(|token| token.clear());
}
