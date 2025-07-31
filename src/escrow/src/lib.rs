mod memory;
mod types;

use candid::Principal;
use types::{EscrowError, EscrowStatus, FusionEscrow, Token};

/// Lock ICP tokens for a cross-chain swap - Used by: Makers/Resolvers
#[ic_cdk::update]
async fn lock_icp_for_swap(
    order_id: String,
    amount: u64,
    hashlock: Vec<u8>,
    timelock: u64,
) -> Result<String, EscrowError> {
    let caller = ic_cdk::caller();

    // Generate escrow ID
    let escrow_id = generate_escrow_id(&order_id);

    // Create escrow record
    let escrow = FusionEscrow {
        id: escrow_id.clone(),
        order_id: order_id.clone(),
        token: Token::ICP,
        amount,
        locked_by: caller,
        locked_at: ic_cdk::api::time(),
        status: EscrowStatus::Created,
    };

    // Store escrow
    memory::store_fusion_escrow(escrow)?;

    // TODO: Implement actual ICP token transfer to escrow
    // This would integrate with the ICP ledger canister

    ic_cdk::println!(
        "Created ICP escrow {} for order {} with amount {}",
        escrow_id,
        order_id,
        amount
    );

    Ok(escrow_id)
}

/// Claim locked ICP tokens with receipt validation - Used by: Resolvers
#[ic_cdk::update]
async fn claim_locked_icp(
    escrow_id: String,
    preimage: Vec<u8>,
    eth_receipt: String,
) -> Result<(), EscrowError> {
    let caller = ic_cdk::caller();

    // Get escrow
    let mut escrow = memory::get_fusion_escrow(&escrow_id)?;

    // Verify escrow is funded
    if escrow.status != EscrowStatus::Funded {
        return Err(EscrowError::InvalidState);
    }

    // TODO: Verify preimage matches hashlock
    // TODO: Validate ETH receipt

    // Update escrow status
    escrow.status = EscrowStatus::Claimed;
    memory::store_fusion_escrow(escrow.clone())?;

    // TODO: Transfer ICP tokens to claimer

    ic_cdk::println!("ICP escrow {} claimed by {}", escrow_id, caller.to_text());

    Ok(())
}

/// Refund locked ICP tokens after timeout - Used by: Makers
#[ic_cdk::update]
async fn refund_locked_icp(escrow_id: String) -> Result<(), EscrowError> {
    let caller = ic_cdk::caller();

    // Get escrow
    let mut escrow = memory::get_fusion_escrow(&escrow_id)?;

    // Verify caller is the original locker
    if escrow.locked_by != caller {
        return Err(EscrowError::Unauthorized);
    }

    // TODO: Check if timelock has expired

    // Update escrow status
    escrow.status = EscrowStatus::Refunded;
    memory::store_fusion_escrow(escrow.clone())?;

    // TODO: Refund ICP tokens to original locker

    ic_cdk::println!("ICP escrow {} refunded to {}", escrow_id, caller.to_text());

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
