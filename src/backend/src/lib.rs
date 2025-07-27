mod escrows;
mod memory;
mod types;

use escrows::{get_timelock_status, TimelockStatus};
use types::{CreateEscrowParams, Escrow, EscrowError};

// Keep the hello world function for testing
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Test function for timelock enforcement
#[ic_cdk::query]
fn test_timelock(timelock: u64) -> TimelockStatus {
    get_timelock_status(timelock)
}

// Escrow lifecycle functions

/// Create escrow with hashlock and timelock - Used by: Resolver
#[ic_cdk::update]
async fn create_escrow(params: CreateEscrowParams) -> Result<String, EscrowError> {
    escrows::create_escrow(params).await
}

/// Deposit tokens to fund escrow - Used by: Resolver
#[ic_cdk::update]
async fn deposit_tokens(escrow_id: String, amount: u64) -> Result<(), EscrowError> {
    escrows::deposit_tokens(escrow_id, amount).await
}

/// Claim tokens by revealing secret - Used by: Maker
#[ic_cdk::update]
async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    escrows::claim_escrow(escrow_id, preimage).await
}

/// Refund tokens after timelock expires - Used by: Anyone
#[ic_cdk::update]
async fn refund_escrow(escrow_id: String) -> Result<(), EscrowError> {
    escrows::refund_escrow(escrow_id).await
}

/// Get escrow status and details - Used by: Resolver
#[ic_cdk::query]
fn get_escrow_status(escrow_id: String) -> Result<Escrow, EscrowError> {
    escrows::get_escrow_status(escrow_id)
}

/// List all escrows for debugging - Used by: Developers
#[ic_cdk::query]
fn list_escrows() -> Vec<Escrow> {
    escrows::list_escrows()
}

ic_cdk::export_candid!();
