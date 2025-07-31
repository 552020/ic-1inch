use candid::Principal;
use ic_cdk::{caller, query, update};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};
use icrc_ledger_types::icrc2::approve::{ApproveArgs, ApproveError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use std::cell::RefCell;
use std::collections::HashMap;

// ICRC-1 Token Implementation with ICRC-2 Support

// Storage
thread_local! {
    static BALANCES: RefCell<HashMap<Principal, u128>> = RefCell::new(HashMap::new());
    static ALLOWANCES: RefCell<HashMap<(Principal, Principal), u128>> = RefCell::new(HashMap::new());
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
pub async fn icrc1_transfer(args: TransferArg) -> std::result::Result<u64, TransferError> {
    // Simple transfer implementation for testing
    let from = caller();
    let to = args.to.owner;
    let amount = args.amount.0.try_into().unwrap_or(0u128);

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        let from_balance = balances.get(&from).copied().unwrap_or(0);
        if from_balance < amount {
            return Err(TransferError::InsufficientFunds {
                balance: candid::Nat::from(from_balance),
            });
        }

        // Update balances
        *balances.entry(from).or_insert(0) -= amount;
        *balances.entry(to).or_insert(0) += amount;

        Ok(1u64) // Mock transfer ID
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

#[update]
pub async fn transfer_from_backend(
    from: Principal,
    to: Principal,
    amount: u128,
) -> Result<u64, String> {
    // Only allow backend canister to call this
    let backend_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    if caller() != backend_principal {
        return Err("Unauthorized".to_string());
    }

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        let from_balance = balances.get(&from).copied().unwrap_or(0);
        if from_balance < amount {
            return Err("InsufficientFunds".to_string());
        }

        // Update balances
        *balances.entry(from).or_insert(0) -= amount;
        *balances.entry(to).or_insert(0) += amount;

        Ok(1u64) // Return block index
    })
}

// ICRC-2 Methods
#[update]
pub async fn icrc2_approve(args: ApproveArgs) -> Result<u64, ApproveError> {
    let owner = caller();
    let spender = args.spender.owner;
    let amount = args.amount.0.try_into().unwrap_or(0u128);

    ALLOWANCES.with(|allowances| {
        let mut allowances = allowances.borrow_mut();
        allowances.insert((owner, spender), amount);
        Ok(1u64) // Return block index
    })
}

#[query]
pub fn icrc2_allowance(
    args: icrc_ledger_types::icrc2::allowance::AllowanceArgs,
) -> Result<icrc_ledger_types::icrc2::allowance::Allowance, String> {
    let owner = args.account.owner;
    let spender = args.spender.owner;

    ALLOWANCES.with(|allowances| {
        let allowance = allowances.borrow().get(&(owner, spender)).copied().unwrap_or(0);
        Ok(icrc_ledger_types::icrc2::allowance::Allowance {
            allowance: candid::Nat::from(allowance),
            expires_at: None,
        })
    })
}

#[update]
pub async fn icrc2_transfer_from(args: TransferFromArgs) -> Result<u64, TransferFromError> {
    let spender = caller();
    let from = args.from.owner;
    let to = args.to.owner;
    let amount = args.amount.0.try_into().unwrap_or(0u128);

    // Check allowance
    let allowance = ALLOWANCES
        .with(|allowances| allowances.borrow().get(&(from, spender)).copied().unwrap_or(0));

    if allowance < amount {
        return Err(TransferFromError::InsufficientAllowance {
            allowance: candid::Nat::from(allowance),
        });
    }

    // Check balance
    let from_balance = BALANCES.with(|balances| balances.borrow().get(&from).copied().unwrap_or(0));

    if from_balance < amount {
        return Err(TransferFromError::InsufficientFunds {
            balance: candid::Nat::from(from_balance),
        });
    }

    // Execute transfer
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        *balances.entry(from).or_insert(0) -= amount;
        *balances.entry(to).or_insert(0) += amount;
    });

    // Update allowance
    ALLOWANCES.with(|allowances| {
        let mut allowances = allowances.borrow_mut();
        let current_allowance = allowances.get(&(from, spender)).copied().unwrap_or(0);
        allowances.insert((from, spender), current_allowance - amount);
    });

    Ok(1u64) // Return block index
}

// Initialize function
#[ic_cdk::init]
fn init() {
    init_balances();
}

ic_cdk::export_candid!();
