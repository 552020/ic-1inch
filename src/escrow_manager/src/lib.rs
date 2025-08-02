// mod chain_fusion; // TODO: Enable in Task 5 (Chain Fusion integration)
mod memory;
mod timelock;
mod types;

use candid::Principal;
// use chain_fusion::{ChainFusionConfig, ChainFusionManager, EVMEscrowParams}; // TODO: Enable in Task 5
use types::{
    ConservativeTimelocks,
    CoordinationState,
    CrossChainEscrow,
    EscrowError,
    EscrowStatus,
    EscrowType,
    HTLCEscrow,
    TimelockConfig,
    Token,
    // ThresholdECDSAHealth, // TODO: Enable in Task 5 for Chain Fusion
};

/// Create phased ICP escrow with conservative timelock calculation - Used by: Makers
#[ic_cdk::update]
async fn create_icp_escrow(
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
) -> Result<String, EscrowError> {
    let _caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();

    // === PHASE 1: INPUT VALIDATION ===
    validate_escrow_inputs(
        &order_hash,
        &hashlock,
        &maker,
        &taker,
        &token,
        amount,
        safety_deposit,
        timelock,
        current_time,
    )?;

    // === PHASE 2: CONSERVATIVE TIMELOCK CALCULATION ===
    let conservative_timelocks =
        timelock::calculate_conservative_timelocks(timelock, current_time)?;

    // === PHASE 3: ICP ESCROW CREATION ===
    let escrow = HTLCEscrow {
        order_hash: order_hash.clone(),
        hashlock,
        maker,
        taker,
        token,
        amount,
        safety_deposit,
        timelock: conservative_timelocks.icp_timelock,
        src_chain_id,
        dst_chain_id,
        src_token,
        dst_token,
        src_amount,
        dst_amount,
        escrow_type: EscrowType::Source, // ICP is source for the swap
        status: EscrowStatus::Created,
        address: format!("icp_htlc_{}", order_hash),
        timelock_config: conservative_timelocks.config,
        threshold_ecdsa_key_id: None,
        chain_health_status: None,
        partial_fill_info: None,
        events: vec![types::CrossChainEscrowEvent::EscrowCreated {
            escrow_id: order_hash.clone(),
            chain: "ICP".to_string(),
        }],
        created_at: current_time,
        updated_at: current_time,
    };

    // Store escrow
    memory::store_htlc_escrow(escrow)?;

    ic_cdk::println!(
        "🔒 Phase 1: Created ICP HTLC escrow for order {} with conservative timelock {} (buffer: {} min)",
        order_hash,
        conservative_timelocks.icp_timelock,
        conservative_timelocks.buffer_minutes
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
        coordination_state: CoordinationState::EscrowsCreated,
        icp_finality_lag: 0,
        evm_finality_lag: 0,
        failed_transactions: 0,
        events: Vec::new(),
        created_at: current_time,
        updated_at: current_time,
    };

    memory::store_cross_chain_escrow(cross_chain_escrow)?;

    ic_cdk::println!("🔗 Created cross-chain escrow coordination for order {}", order_id);

    Ok(order_id)
}

/// List all cross-chain escrows for debugging - Used by: Developers
#[ic_cdk::query]
fn list_cross_chain_escrows() -> Vec<CrossChainEscrow> {
    memory::get_all_cross_chain_escrows()
}

/// Validate escrow creation inputs
fn validate_escrow_inputs(
    order_hash: &str,
    hashlock: &str,
    maker: &str,
    taker: &str,
    token: &str,
    amount: u64,
    safety_deposit: u64,
    timelock: u64,
    current_time: u64,
) -> Result<(), EscrowError> {
    // Validate order_hash
    if order_hash.is_empty() || order_hash.len() < 8 {
        return Err(EscrowError::InvalidOrderHash);
    }

    // Validate hashlock (should be 32-byte hex string)
    if hashlock.is_empty() || hashlock.len() != 64 {
        return Err(EscrowError::InvalidHashlock);
    }

    // Validate maker and taker addresses
    if maker.is_empty() || taker.is_empty() {
        return Err(EscrowError::InvalidAddress);
    }

    if maker == taker {
        return Err(EscrowError::InvalidAddress);
    }

    // Validate token
    if token.is_empty() {
        return Err(EscrowError::InvalidToken);
    }

    // Validate amounts
    if amount == 0 {
        return Err(EscrowError::InvalidAmount);
    }

    if safety_deposit == 0 {
        return Err(EscrowError::InvalidAmount);
    }

    // Validate timelock duration
    let validation = timelock::validate_timelock_duration(timelock, current_time);
    if !validation.is_valid {
        return Err(EscrowError::TimelockTooShort);
    }

    // Check if escrow already exists
    if memory::htlc_escrow_exists(order_hash) {
        return Err(EscrowError::EscrowAlreadyExists);
    }

    Ok(())
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
