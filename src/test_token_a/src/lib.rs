use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{caller, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

// ICRC-1 Token Implementation
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub from_subaccount: Option<Vec<u8>>,
    pub to: Account,
    pub amount: u128,
    pub fee: Option<u128>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferResult {
    pub ok: Option<u64>,
    pub err: Option<TransferError>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferError {
    BadFee { expected_fee: u128 },
    BadBurn { min_burn_amount: u128 },
    InsufficientFunds { balance: u128 },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u64 },
    TemporarilyUnavailable,
    GenericError { error_code: u128, message: String },
}

// Storage
thread_local! {
    static BALANCES: RefCell<HashMap<Principal, u128>> = RefCell::new(HashMap::new());
    static TOTAL_SUPPLY: RefCell<u128> = RefCell::new(1_000_000_000_000); // 10,000 tokens with 8 decimals
}

// Initialize balances for testing
fn init_balances() {
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        // Give tokens to common test principals and identities
        balances.insert(Principal::from_text("aaaaa-aa").unwrap(), 1_000_000_000); // 10 tokens
        balances
            .insert(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(), 1_000_000_000);
        // 10 tokens

        // Pre-fund common test identities (these will be updated when identities are created)
        // Users should call mint_tokens() for their specific test identities
    });
}

// ICRC-1 Methods
#[query]
pub fn icrc1_name() -> String {
    "Token A".to_string()
}

#[query]
pub fn icrc1_symbol() -> String {
    "TOKEN_A".to_string()
}

#[query]
pub fn icrc1_decimals() -> u8 {
    8
}

#[query]
pub fn icrc1_fee() -> u128 {
    0 // No fees for testing
}

#[query]
pub fn icrc1_metadata() -> Vec<(String, String)> {
    vec![
        ("icrc1:name".to_string(), "Test Token".to_string()),
        ("icrc1:symbol".to_string(), "TEST".to_string()),
        ("icrc1:decimals".to_string(), "8".to_string()),
        ("icrc1:fee".to_string(), "0".to_string()),
    ]
}

#[query]
pub fn icrc1_total_supply() -> u128 {
    TOTAL_SUPPLY.with(|supply| *supply.borrow())
}

#[query]
pub fn icrc1_supported_standards() -> Vec<HashMap<String, String>> {
    vec![HashMap::from([
        ("name".to_string(), "ICRC-1".to_string()),
        ("url".to_string(), "https://github.com/dfinity/ICRC-1".to_string()),
    ])]
}

#[query]
pub fn icrc1_balance_of(account: Account) -> u128 {
    BALANCES.with(|balances| balances.borrow().get(&account.owner).copied().unwrap_or(0))
}

#[update]
pub async fn icrc1_transfer(args: TransferArgs) -> TransferResult {
    // Simple transfer implementation for testing
    let from = caller();
    let to = args.to.owner;
    let amount = args.amount;

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        let from_balance = balances.get(&from).copied().unwrap_or(0);
        if from_balance < amount {
            return TransferResult {
                ok: None,
                err: Some(TransferError::InsufficientFunds { balance: from_balance }),
            };
        }

        // Update balances
        *balances.entry(from).or_insert(0) -= amount;
        *balances.entry(to).or_insert(0) += amount;

        TransferResult {
            ok: Some(1), // Mock transfer ID
            err: None,
        }
    })
}

// Mint tokens for testing (only for test environments)
#[update]
pub async fn mint_tokens(to: Principal, amount: u128) -> Result<u64, String> {
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        *balances.entry(to).or_insert(0) += amount;
        Ok(1) // Mock transaction ID
    })
}

// Mint tokens for the caller (convenience function for testing)
#[update]
pub async fn mint_for_caller(amount: u128) -> Result<u64, String> {
    let caller = caller();
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        *balances.entry(caller).or_insert(0) += amount;
        Ok(1) // Mock transaction ID
    })
}

// Initialize function
#[ic_cdk::init]
fn init() {
    init_balances();
}

ic_cdk::export_candid!();
