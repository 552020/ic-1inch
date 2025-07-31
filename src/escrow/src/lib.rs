mod memory;
mod types;

use candid::Principal;
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

    // TODO: Implement actual ICP token transfer to escrow
    // This would integrate with the ICP ledger canister
    // For now, we simulate the transfer by marking as funded
    fund_escrow(escrow_id.clone())?;

    ic_cdk::println!(
        "🔒 Locked {} ICP tokens for fusion swap {} (order: {})",
        amount,
        escrow_id,
        order_id
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

    // Update escrow status
    escrow.status = EscrowStatus::Claimed;
    escrow.eth_receipt = Some(eth_receipt);
    memory::store_fusion_escrow(escrow.clone())?;

    // TODO: Transfer ICP tokens to resolver
    // This would integrate with the ICP ledger canister

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

    // Update escrow status
    escrow.status = EscrowStatus::Refunded;
    memory::store_fusion_escrow(escrow.clone())?;

    // TODO: Refund ICP tokens to original locker
    // This would integrate with the ICP ledger canister

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

ic_cdk::export_candid!();
