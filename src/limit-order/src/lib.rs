mod limit_orders;
mod memory;
#[cfg(test)]
mod mock_icrc1_token;
#[cfg(test)]
mod test_utils;
mod types;

use types::{Order, OrderError, OrderId, SystemStats};

// Keep the hello world function for testing
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// ============================================================================
// CORE LOP FUNCTIONS - Order Management and Token Swaps
// ============================================================================

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
