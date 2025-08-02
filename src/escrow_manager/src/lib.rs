mod memory;
mod types;

use candid::Principal;
use types::{EscrowError, EscrowStatus, HTLCEscrow, CrossChainEscrow, Token, CoordinationState, EscrowType, TimelockConfig};

/// Create HTLC escrow for cross-chain swap - Used by: Makers
#[ic_cdk::update]
async fn create_htlc_escrow(
    order_hash: String,
    hashlock: String,
    maker: String,
    taker: String,
    token: String,
    amount: u64,
    safety_deposit: u64,
    timelock: u64,
    src_chain_id: u64,
    dst_chain_id: u64,
    src_token: String,
    dst_token: String,
    src_amount: u64,
    dst_amount: u64,
    escrow_type: EscrowType,
) -> Result<String, EscrowError> {
    let _caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();

    // Validate timelock is in the future
    if timelock <= current_time {
        return Err(EscrowError::TimelockExpired);
    }

    // Create HTLC escrow record
    let escrow = HTLCEscrow {
        order_hash: order_hash.clone(),
        hashlock,
        maker,
        taker,
        token,
        amount,
        safety_deposit,
        timelock,
        src_chain_id,
        dst_chain_id,
        src_token,
        dst_token,
        src_amount,
        dst_amount,
        escrow_type: escrow_type.clone(),
        status: EscrowStatus::Created,
        address: format!("htlc_{}", order_hash),
        timelock_config: TimelockConfig::default_config(),
        threshold_ecdsa_key_id: None,
        chain_health_status: None,
        partial_fill_info: None,
        events: Vec::new(),
        created_at: current_time,
        updated_at: current_time,
    };

    // Store escrow
    memory::store_htlc_escrow(escrow)?;

    ic_cdk::println!(
        "🔒 Created HTLC escrow for order {} (type: {:?})",
        order_hash,
        escrow_type.clone()
    );

    Ok(order_hash)
}

/// Get HTLC escrow status - Used by: Frontend/Users
#[ic_cdk::query]
fn get_htlc_escrow_status(order_hash: String) -> Option<HTLCEscrow> {
    memory::get_htlc_escrow(&order_hash).ok()
}

/// List all HTLC escrows for debugging - Used by: Developers
#[ic_cdk::query]
fn list_htlc_escrows() -> Vec<HTLCEscrow> {
    memory::get_all_htlc_escrows()
}

/// Create cross-chain escrow coordination - Used by: System
#[ic_cdk::update]
async fn create_cross_chain_escrow(
    order_id: String,
    icp_escrow: HTLCEscrow,
    evm_escrow: HTLCEscrow,
) -> Result<String, EscrowError> {
    let current_time = ic_cdk::api::time();

    let cross_chain_escrow = CrossChainEscrow {
        order_id: order_id.clone(),
        icp_escrow,
        evm_escrow,
        coordination_state: CoordinationState::Pending,
        events: Vec::new(),
        icp_finality_lag: 0,
        evm_finality_lag: 0,
        failed_transactions: 0,
        created_at: current_time,
        updated_at: current_time,
    };

    // Store cross-chain escrow
    memory::store_cross_chain_escrow(cross_chain_escrow)?;

    ic_cdk::println!(
        "🔗 Created cross-chain escrow coordination for order {}",
        order_id
    );

    Ok(order_id)
}

/// Get cross-chain escrow status - Used by: Frontend/Users
#[ic_cdk::query]
fn get_cross_chain_escrow_status(order_id: String) -> Option<CrossChainEscrow> {
    memory::get_cross_chain_escrow(&order_id).ok()
}

/// List all cross-chain escrows for debugging - Used by: Developers
#[ic_cdk::query]
fn list_cross_chain_escrows() -> Vec<CrossChainEscrow> {
    memory::get_all_cross_chain_escrows()
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

ic_cdk::export_candid!();
