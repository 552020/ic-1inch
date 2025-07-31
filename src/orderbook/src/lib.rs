mod memory;
mod types;

use candid::Principal;
use types::{FusionError, FusionOrder, OrderStatus, Token};

/// Create a new fusion order for cross-chain swaps - Used by: Makers
#[ic_cdk::update]
async fn create_fusion_order(
    maker_eth_address: String,
    from_token: Token,
    to_token: Token,
    from_amount: u64,
    to_amount: u64,
    expiration: u64,
) -> Result<String, FusionError> {
    let caller = ic_cdk::caller();

    // Generate unique order ID
    let order_id = generate_order_id();

    // Create the fusion order
    let order = FusionOrder {
        id: order_id.clone(),
        maker_eth_address: maker_eth_address.clone(),
        maker_icp_principal: caller,
        from_token,
        to_token,
        from_amount,
        to_amount,
        status: OrderStatus::Pending,
        created_at: ic_cdk::api::time(),
        expires_at: expiration,
    };

    // Store the order
    memory::store_fusion_order(order)?;

    ic_cdk::println!("Created fusion order {} for maker {}", order_id, maker_eth_address);

    Ok(order_id)
}

/// Accept a fusion order as a resolver - Used by: Resolvers
#[ic_cdk::update]
async fn accept_fusion_order(
    order_id: String,
    resolver_eth_address: String,
) -> Result<(), FusionError> {
    let caller = ic_cdk::caller();

    // Get the order
    let mut order = memory::get_fusion_order(&order_id)?;

    // Verify order is still pending
    if order.status != OrderStatus::Pending {
        return Err(FusionError::SystemError);
    }

    // Check if order has expired
    if ic_cdk::api::time() > order.expires_at {
        order.status = OrderStatus::Failed;
        memory::store_fusion_order(order)?;
        return Err(FusionError::SystemError);
    }

    // Update order with resolver info
    order.status = OrderStatus::Accepted;
    memory::store_fusion_order(order)?;

    ic_cdk::println!("Order {} accepted by resolver {}", order_id, resolver_eth_address);

    Ok(())
}

/// Get all active fusion orders - Used by: Frontend/Resolvers
#[ic_cdk::query]
fn get_active_fusion_orders() -> Vec<FusionOrder> {
    memory::get_all_fusion_orders()
        .into_iter()
        .filter(|order| matches!(order.status, OrderStatus::Pending | OrderStatus::Accepted))
        .collect()
}

/// Get a specific fusion order by ID - Used by: Frontend/Users
#[ic_cdk::query]
fn get_fusion_order_status(order_id: String) -> Option<FusionOrder> {
    memory::get_fusion_order(&order_id).ok()
}

/// Get orders created by a specific maker - Used by: Frontend/Makers
#[ic_cdk::query]
fn get_orders_by_maker(maker_principal: Principal) -> Vec<FusionOrder> {
    memory::get_all_fusion_orders()
        .into_iter()
        .filter(|order| order.maker_icp_principal == maker_principal)
        .collect()
}

/// Update order status (for relayer coordination) - Used by: Relayer
#[ic_cdk::update]
fn update_order_status(order_id: String, status: OrderStatus) -> Result<(), FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;
    order.status = status;
    memory::store_fusion_order(order)?;
    Ok(())
}

/// Register or update cross-chain identity - Used by: Users
#[ic_cdk::update]
fn register_cross_chain_identity(
    eth_address: String,
    role: types::UserRole,
) -> Result<(), types::FusionError> {
    let caller = ic_cdk::caller();

    let identity =
        types::CrossChainIdentity { eth_address: eth_address.clone(), icp_principal: caller, role };

    memory::store_cross_chain_identity(identity)?;

    ic_cdk::println!("Registered cross-chain identity: {} -> {}", eth_address, caller.to_text());

    Ok(())
}

/// Get cross-chain identity by ETH address - Used by: Frontend/Users
#[ic_cdk::query]
fn get_cross_chain_identity(eth_address: String) -> Option<types::CrossChainIdentity> {
    memory::get_cross_chain_identity(&eth_address).ok()
}

/// Generate a unique order ID
fn generate_order_id() -> String {
    let timestamp = ic_cdk::api::time();
    let caller = ic_cdk::caller();
    format!("fusion_{}_{}", timestamp, caller.to_text())
}

ic_cdk::export_candid!();
