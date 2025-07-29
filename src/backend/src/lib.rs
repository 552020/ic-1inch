mod destination_escrow;
mod escrows;
mod limit_orders;
mod memory;
#[cfg(test)]
mod mock_icrc1_token;
mod source_escrow;
#[cfg(test)]
mod test_utils;
mod types;

use escrows::{get_timelock_status, TimelockStatus};
use types::{CreateEscrowParams, DestinationEscrow, Escrow, EscrowError, SourceEscrow};
use types::{Order, OrderError, OrderId, SystemStats};

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

// Legacy escrow functions (for backward compatibility)

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

// Source escrow functions

/// Create source escrow - Used by: Resolver
#[ic_cdk::update]
async fn create_source_escrow(
    maker: candid::Principal,
    taker: candid::Principal,
    hashlock: Vec<u8>,
    token_canister: candid::Principal,
    amount: u64,
    timelock: u64,
) -> Result<String, EscrowError> {
    source_escrow::create_source_escrow(maker, taker, hashlock, token_canister, amount, timelock)
        .await
}

/// Deposit tokens to source escrow - Used by: Resolver
#[ic_cdk::update]
async fn deposit_tokens_to_source(escrow_id: String, amount: u64) -> Result<(), EscrowError> {
    source_escrow::deposit_tokens_to_source(escrow_id, amount).await
}

/// Claim tokens from source escrow - Used by: Taker
#[ic_cdk::update]
async fn claim_source_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    source_escrow::claim_source_escrow(escrow_id, preimage).await
}

/// Refund tokens from source escrow - Used by: Taker
#[ic_cdk::update]
async fn refund_source_escrow(escrow_id: String) -> Result<(), EscrowError> {
    source_escrow::refund_source_escrow(escrow_id).await
}

/// Get source escrow status - Used by: Resolver
#[ic_cdk::query]
fn get_source_escrow_status(escrow_id: String) -> Result<SourceEscrow, EscrowError> {
    source_escrow::get_source_escrow_status(escrow_id)
}

/// List all source escrows - Used by: Developers
#[ic_cdk::query]
fn list_source_escrows() -> Vec<SourceEscrow> {
    source_escrow::list_source_escrows()
}

// Destination escrow functions

/// Create destination escrow - Used by: Resolver
#[ic_cdk::update]
async fn create_destination_escrow(
    maker: candid::Principal,
    taker: candid::Principal,
    hashlock: Vec<u8>,
    token_canister: candid::Principal,
    amount: u64,
    timelock: u64,
) -> Result<String, EscrowError> {
    destination_escrow::create_destination_escrow(
        maker,
        taker,
        hashlock,
        token_canister,
        amount,
        timelock,
    )
    .await
}

/// Deposit tokens to destination escrow - Used by: Resolver
#[ic_cdk::update]
async fn deposit_tokens_to_destination(escrow_id: String, amount: u64) -> Result<(), EscrowError> {
    destination_escrow::deposit_tokens_to_destination(escrow_id, amount).await
}

/// Claim tokens from destination escrow - Used by: Anyone with secret
#[ic_cdk::update]
async fn claim_destination_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    destination_escrow::claim_destination_escrow(escrow_id, preimage).await
}

/// Refund tokens from destination escrow - Used by: Taker
#[ic_cdk::update]
async fn refund_destination_escrow(escrow_id: String) -> Result<(), EscrowError> {
    destination_escrow::refund_destination_escrow(escrow_id).await
}

/// Get destination escrow status - Used by: Resolver
#[ic_cdk::query]
fn get_destination_escrow_status(escrow_id: String) -> Result<DestinationEscrow, EscrowError> {
    destination_escrow::get_destination_escrow_status(escrow_id)
}

/// List all destination escrows - Used by: Developers
#[ic_cdk::query]
fn list_destination_escrows() -> Vec<DestinationEscrow> {
    destination_escrow::list_destination_escrows()
}

// Limit Order Protocol functions

/// Create a new limit order - Used by: Makers
#[ic_cdk::update]
async fn create_order(
    receiver: candid::Principal,
    maker_asset: candid::Principal,
    taker_asset: candid::Principal,
    making_amount: u64,
    taking_amount: u64,
    expiration: u64,
) -> Result<OrderId, OrderError> {
    limit_orders::create_order(
        receiver,
        maker_asset,
        taker_asset,
        making_amount,
        taking_amount,
        expiration,
    )
    .await
}

/// Fill an existing limit order - Used by: Takers/Resolvers
#[ic_cdk::update]
async fn fill_order(order_id: OrderId) -> Result<(), OrderError> {
    limit_orders::fill_order(order_id).await
}

/// Cancel an existing limit order - Used by: Makers
#[ic_cdk::update]
fn cancel_order(order_id: OrderId) -> Result<(), OrderError> {
    limit_orders::cancel_order(order_id)
}

/// Get all active orders - Used by: Takers/Frontend
#[ic_cdk::query]
fn get_active_orders() -> Vec<Order> {
    limit_orders::get_active_orders_list()
}

/// Get a specific order by ID - Used by: Frontend/Users
#[ic_cdk::query]
fn get_order_by_id(order_id: OrderId) -> Option<Order> {
    limit_orders::get_order_by_id(order_id)
}

/// Get orders created by a specific maker - Used by: Frontend/Makers
#[ic_cdk::query]
fn get_orders_by_maker(maker: candid::Principal) -> Vec<Order> {
    limit_orders::get_orders_by_maker(maker)
}

/// Get orders for a specific asset pair - Used by: Frontend/Traders
#[ic_cdk::query]
fn get_orders_by_asset_pair(
    maker_asset: candid::Principal,
    taker_asset: candid::Principal,
) -> Vec<Order> {
    limit_orders::get_orders_by_asset_pair(maker_asset, taker_asset)
}

/// Get system statistics - Used by: Frontend/Monitoring
#[ic_cdk::query]
fn get_system_stats() -> SystemStats {
    limit_orders::get_system_statistics()
}

// ============================================================================
// CANISTER UPGRADE HOOKS
// ============================================================================

/// Pre-upgrade hook: Save state to stable memory
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let state = memory::serialize_limit_order_state();
    // Save state to stable memory, but don't panic if it fails
    if let Err(e) = ic_cdk::storage::stable_save((state,)) {
        // Log the error but don't panic - this allows the upgrade to proceed
        ic_cdk::print(format!("Warning: Failed to save state during upgrade: {:?}", e));
    }
}

/// Post-upgrade hook: Restore state from stable memory
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    // Try to restore state, but handle the case where no state exists (fresh deployment)
    match ic_cdk::storage::stable_restore() {
        Ok((state,)) => {
            let (orders, filled, cancelled, counter, stats) = state;
            memory::deserialize_limit_order_state(orders, filled, cancelled, counter, stats);
        }
        Err(_) => {
            // No existing state found - this is a fresh deployment
            // Initialize with empty state (default values)
            memory::deserialize_limit_order_state(
                vec![],
                vec![],
                vec![],
                0,
                SystemStats::default(),
            );
        }
    }
}

ic_cdk::export_candid!();
