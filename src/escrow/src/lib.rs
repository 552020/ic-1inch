mod memory;
mod types;

use candid::{Nat, Principal};
use ic_cdk::call;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};
use types::{EscrowError, EscrowStatus, FusionEscrow, Token};

/// Lock ICP tokens for a cross-chain swap - Used by: Makers
#[ic_cdk::update]
async fn lock_icp_for_swap(
    order_id: String,
    amount: u64,
    resolver: Principal,
    timelock: u64,
) -> Result<String, EscrowError> {
    let caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();

    // Validate timelock is in the future
    if timelock <= current_time {
        return Err(EscrowError::TimelockExpired);
    }

    // Generate escrow ID
    let escrow_id = generate_escrow_id(&order_id);

    // Create escrow record
    let escrow = FusionEscrow {
        id: escrow_id.clone(),
        order_id: order_id.clone(),
        token: Token::ICP,
        amount,
        locked_by: caller,
        resolver,
        timelock,
        eth_receipt: None,
        locked_at: current_time,
        status: EscrowStatus::Created,
    };

    // Store escrow
    memory::store_fusion_escrow(escrow)?;

    // Get test token canister for ICP
    let token_canister = get_test_token_canister(&Token::ICP)?;

    // Transfer tokens from maker to escrow canister
    transfer_tokens_to_escrow(token_canister, caller, amount).await?;

    // Mark escrow as funded after successful transfer
    fund_escrow(escrow_id.clone())?;

    ic_cdk::println!(
        "🔒 Locked {} ICP tokens for fusion swap {} (order: {})",
        amount,
        escrow_id,
        order_id
    );

    Ok(escrow_id)
}

/// Lock ICP tokens for an order (called by orderbook) - Used by: Orderbook
#[ic_cdk::update]
async fn lock_icp_for_order(
    order_id: String,
    amount: u64,
    timelock: u64,
) -> Result<String, EscrowError> {
    let caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();

    // Validate timelock is in the future
    if timelock <= current_time {
        return Err(EscrowError::TimelockExpired);
    }

    // Verify this is an ICP → ETH order by checking the order details
    let is_icp_to_eth = verify_order_direction(&order_id).await?;
    if !is_icp_to_eth {
        return Err(EscrowError::InvalidState);
    }

    // Verify caller is the maker of this order
    let is_maker = verify_caller_is_maker(&order_id, caller).await?;
    if !is_maker {
        return Err(EscrowError::Unauthorized);
    }

    // Generate escrow ID
    let escrow_id = generate_escrow_id(&order_id);

    // Create escrow record (resolver will be set later when order is accepted)
    let escrow = FusionEscrow {
        id: escrow_id.clone(),
        order_id: order_id.clone(),
        token: Token::ICP,
        amount,
        locked_by: caller,
        resolver: Principal::anonymous(), // Will be updated when resolver accepts
        timelock,
        eth_receipt: None,
        locked_at: current_time,
        status: EscrowStatus::Created,
    };

    // Store escrow
    memory::store_fusion_escrow(escrow)?;

    // Get test token canister for ICP
    let token_canister = get_test_token_canister(&Token::ICP)?;

    // Transfer tokens from maker to escrow canister
    transfer_tokens_to_escrow(token_canister, caller, amount).await?;

    // Mark escrow as funded after successful transfer
    fund_escrow(escrow_id.clone())?;

    ic_cdk::println!(
        "🔒 Automatically locked {} ICP tokens for ICP→ETH order {} (escrow: {})",
        amount,
        order_id,
        escrow_id
    );

    Ok(escrow_id)
}

/// Claim locked ICP tokens with ETH receipt - Used by: Resolvers
#[ic_cdk::update]
async fn claim_locked_icp(escrow_id: String, eth_receipt: String) -> Result<(), EscrowError> {
    let caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();

    // Get escrow
    let mut escrow = memory::get_fusion_escrow(&escrow_id)?;

    // Validate caller is the resolver
    if caller != escrow.resolver {
        return Err(EscrowError::Unauthorized);
    }

    // Validate escrow state
    if escrow.status != EscrowStatus::Funded {
        return Err(EscrowError::InvalidState);
    }

    // Validate timelock hasn't expired
    if current_time >= escrow.timelock {
        return Err(EscrowError::TimelockExpired);
    }

    // Validate receipt (in mechanical turk, this is manual verification)
    if eth_receipt.is_empty() {
        return Err(EscrowError::InvalidReceipt);
    }

    // Get test token canister for ICP
    let token_canister = get_test_token_canister(&Token::ICP)?;

    // Transfer tokens from escrow to resolver
    transfer_tokens_from_escrow(token_canister, caller, escrow.amount).await?;

    // Update escrow status
    escrow.status = EscrowStatus::Claimed;
    escrow.eth_receipt = Some(eth_receipt);
    memory::store_fusion_escrow(escrow.clone())?;

    ic_cdk::println!(
        "✅ Claimed {} ICP tokens from fusion escrow {} by resolver {}",
        escrow.amount,
        escrow_id,
        caller.to_text()
    );

    Ok(())
}

/// Refund locked ICP tokens after timelock expires - Used by: Makers
#[ic_cdk::update]
async fn refund_locked_icp(escrow_id: String) -> Result<(), EscrowError> {
    let caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();

    // Get escrow
    let mut escrow = memory::get_fusion_escrow(&escrow_id)?;

    // Validate caller is the original locker
    if escrow.locked_by != caller {
        return Err(EscrowError::Unauthorized);
    }

    // Validate escrow state
    if escrow.status != EscrowStatus::Funded {
        return Err(EscrowError::InvalidState);
    }

    // Validate timelock has expired
    if current_time < escrow.timelock {
        return Err(EscrowError::TimelockNotExpired);
    }

    // Get test token canister for ICP
    let token_canister = get_test_token_canister(&Token::ICP)?;

    // Refund tokens from escrow to original locker
    transfer_tokens_from_escrow(token_canister, caller, escrow.amount).await?;

    // Update escrow status
    escrow.status = EscrowStatus::Refunded;
    memory::store_fusion_escrow(escrow.clone())?;

    ic_cdk::println!(
        "💰 Refunded {} ICP tokens to maker {} from fusion escrow {}",
        escrow.amount,
        caller.to_text(),
        escrow_id
    );

    Ok(())
}

/// Get escrow status - Used by: Frontend/Users
#[ic_cdk::query]
fn get_fusion_escrow_status(escrow_id: String) -> Option<FusionEscrow> {
    memory::get_fusion_escrow(&escrow_id).ok()
}

/// List all escrows for debugging - Used by: Developers
#[ic_cdk::query]
fn list_fusion_escrows() -> Vec<FusionEscrow> {
    memory::get_all_fusion_escrows()
}

/// Fund an escrow (mark as funded after token transfer) - Used by: System
#[ic_cdk::update]
fn fund_escrow(escrow_id: String) -> Result<(), EscrowError> {
    let mut escrow = memory::get_fusion_escrow(&escrow_id)?;
    escrow.status = EscrowStatus::Funded;
    memory::store_fusion_escrow(escrow)?;
    Ok(())
}

/// Generate unique escrow ID
fn generate_escrow_id(order_id: &str) -> String {
    let timestamp = ic_cdk::api::time();
    format!("escrow_{}_{}", order_id, timestamp)
}

/// Transfer tokens from user to escrow canister
async fn transfer_tokens_to_escrow(
    token_canister: Principal,
    _from: Principal,
    amount: u64,
) -> Result<(), EscrowError> {
    let escrow_principal = ic_cdk::id();

    let transfer_args = TransferArg {
        from_subaccount: None,
        to: Account { owner: escrow_principal, subaccount: None },
        amount: Nat::from(amount),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let result: Result<(Result<u64, TransferError>,), _> =
        call(token_canister, "icrc1_transfer", (transfer_args,)).await;

    match result {
        Ok((Ok(_),)) => Ok(()),
        Ok((Err(TransferError::InsufficientFunds { balance: _ }),)) => {
            Err(EscrowError::InsufficientBalance)
        }
        Ok((Err(_),)) => Err(EscrowError::TransferFailed),
        Err(_) => Err(EscrowError::SystemError),
    }
}

/// Transfer tokens from escrow canister to recipient
async fn transfer_tokens_from_escrow(
    token_canister: Principal,
    to: Principal,
    amount: u64,
) -> Result<(), EscrowError> {
    let _escrow_principal = ic_cdk::id();

    let transfer_args = TransferArg {
        from_subaccount: None,
        to: Account { owner: to, subaccount: None },
        amount: Nat::from(amount),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let result: Result<(Result<u64, TransferError>,), _> =
        call(token_canister, "icrc1_transfer", (transfer_args,)).await;

    match result {
        Ok((Ok(_),)) => Ok(()),
        Ok((Err(_),)) => Err(EscrowError::TransferFailed),
        Err(_) => Err(EscrowError::SystemError),
    }
}

/// Get test token canister ID based on token type
fn get_test_token_canister(token: &Token) -> Result<Principal, EscrowError> {
    // For mechanical turk testing, we'll use test_token_icp for ICP and test_token_eth for ETH
    // In production, this would be the actual ICP ledger canister
    match token {
        Token::ICP => {
            // Use test_token_icp for ICP simulation
            // TODO: In production, this would be the actual ICP ledger canister
            Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")
                .map_err(|_| EscrowError::SystemError)
        }
        Token::ETH => {
            // Use test_token_eth for ETH simulation
            // TODO: In production, this would be the actual ICP ledger canister
            Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")
                .map_err(|_| EscrowError::SystemError)
        }
    }
}

/// Verify that an order is ICP → ETH direction - Used by: Escrow
async fn verify_order_direction(order_id: &str) -> Result<bool, EscrowError> {
    // Get orderbook canister ID (this would be configured during deployment)
    let orderbook_canister_id = Principal::from_text("uxrrr-q7777-77774-qaaaq-cai")
        .map_err(|_| EscrowError::SystemError)?;

    // Call orderbook to get order details
    let result: Result<(Option<types::FusionOrder>,), _> = ic_cdk::call(
        orderbook_canister_id,
        "get_fusion_order_status",
        (order_id.to_string(),),
    ).await;

    match result {
        Ok((Some(order),)) => {
            // Check if order is ICP → ETH
            let is_icp_to_eth = order.from_token == Token::ICP && order.to_token == Token::ETH;
            Ok(is_icp_to_eth)
        }
        Ok((None,)) => Err(EscrowError::OrderNotFound),
        Err(_) => Err(EscrowError::SystemError),
    }
}

/// Verify that the caller is the maker of the order - Used by: Escrow
async fn verify_caller_is_maker(order_id: &str, caller: Principal) -> Result<bool, EscrowError> {
    // Get orderbook canister ID (this would be configured during deployment)
    let orderbook_canister_id = Principal::from_text("uxrrr-q7777-77774-qaaaq-cai")
        .map_err(|_| EscrowError::SystemError)?;

    // Call orderbook to get order details
    let result: Result<(Option<types::FusionOrder>,), _> = ic_cdk::call(
        orderbook_canister_id,
        "get_fusion_order_status",
        (order_id.to_string(),),
    ).await;

    match result {
        Ok((Some(order),)) => {
            // Check if caller is the maker
            let is_maker = order.maker_icp_principal == caller;
            Ok(is_maker)
        }
        Ok((None,)) => Err(EscrowError::OrderNotFound),
        Err(_) => Err(EscrowError::SystemError),
    }
}

ic_cdk::export_candid!();
