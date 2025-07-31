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
    let current_time = ic_cdk::api::time();

    // Validate amounts
    if from_amount == 0 || to_amount == 0 {
        return Err(FusionError::InvalidAmount);
    }

    // Validate expiration is in the future
    if expiration <= current_time {
        return Err(FusionError::InvalidExpiration);
    }

    // Generate unique order ID
    let order_id = generate_order_id();

    // Create the fusion order
    let order = FusionOrder {
        id: order_id.clone(),
        maker_eth_address: maker_eth_address.clone(),
        maker_icp_principal: caller,
        resolver_eth_address: None,
        resolver_icp_principal: None,
        from_token,
        to_token,
        from_amount,
        to_amount,
        status: OrderStatus::Pending,
        created_at: current_time,
        expires_at: expiration,
        accepted_at: None,
        completed_at: None,
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
    let current_time = ic_cdk::api::time();

    // Get the order
    let mut order = memory::get_fusion_order(&order_id)?;

    // Verify order is still pending
    if order.status != OrderStatus::Pending {
        return Err(FusionError::OrderNotPending);
    }

    // Check if order has expired
    if current_time > order.expires_at {
        order.status = OrderStatus::Failed;
        memory::store_fusion_order(order)?;
        return Err(FusionError::OrderExpired);
    }

    // TODO: Check if resolver is whitelisted (for production)
    // For MVP, allow any resolver
    // if !is_resolver_whitelisted(caller) {
    //     return Err(FusionError::ResolverNotWhitelisted);
    // }

    // Update order with resolver info
    order.status = OrderStatus::Accepted;
    order.resolver_eth_address = Some(resolver_eth_address.clone());
    order.resolver_icp_principal = Some(caller);
    order.accepted_at = Some(current_time);
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
    let current_time = ic_cdk::api::time();
    
    order.status = status.clone();
    
    // Update completion timestamp if order is completed
    if status == OrderStatus::Completed {
        order.completed_at = Some(current_time);
    }
    
    memory::store_fusion_order(order)?;
    Ok(())
}

/// Cancel an order (for makers) - Used by: Makers
#[ic_cdk::update]
fn cancel_fusion_order(order_id: String) -> Result<(), FusionError> {
    let caller = ic_cdk::caller();
    let mut order = memory::get_fusion_order(&order_id)?;

    // Verify caller is the maker
    if order.maker_icp_principal != caller {
        return Err(FusionError::Unauthorized);
    }

    // Only allow cancellation of pending orders
    if order.status != OrderStatus::Pending {
        return Err(FusionError::OrderNotPending);
    }

    order.status = OrderStatus::Failed;
    memory::store_fusion_order(order)?;

    ic_cdk::println!("Order {} cancelled by maker {}", order_id, caller.to_text());
    Ok(())
}

/// Get orders by status - Used by: Frontend/Users
#[ic_cdk::query]
fn get_orders_by_status(status: OrderStatus) -> Vec<FusionOrder> {
    memory::get_all_fusion_orders()
        .into_iter()
        .filter(|order| order.status == status)
        .collect()
}

/// Get expired orders - Used by: System/Cleanup
#[ic_cdk::query]
fn get_expired_orders() -> Vec<FusionOrder> {
    let current_time = ic_cdk::api::time();
    memory::get_all_fusion_orders()
        .into_iter()
        .filter(|order| current_time > order.expires_at && order.status == OrderStatus::Pending)
        .collect()
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

/// Get cross-chain identity by ICP principal - Used by: Frontend/Users
#[ic_cdk::query]
fn get_cross_chain_identity_by_principal(
    principal: Principal,
) -> Option<types::CrossChainIdentity> {
    // Search through all identities to find one with matching principal
    memory::get_all_cross_chain_identities()
        .into_iter()
        .find(|identity| identity.icp_principal == principal)
}

/// Derive ICP principal from ETH address using SIWE provider - Used by: Frontend/Users
#[ic_cdk::update]
async fn derive_principal_from_eth_address(
    eth_address: String,
) -> Result<Principal, types::FusionError> {
    // Note: In practice, the SIWE provider canister ID should be retrieved from environment
    // For now, using a placeholder - this would be configured during deployment
    let siwe_provider_id = Principal::from_text("rdmx6-jaaaa-aaaah-qcaiq-cai")
        .map_err(|_| types::FusionError::SystemError)?;

    let result: Result<(Result<Vec<u8>, String>,), _> =
        ic_cdk::call(siwe_provider_id, "get_principal", (eth_address,)).await;

    match result {
        Ok((Ok(principal_bytes),)) => {
            Principal::try_from_slice(&principal_bytes).map_err(|_| types::FusionError::SystemError)
        }
        Ok((Err(_),)) => Err(types::FusionError::OrderNotFound),
        Err(_) => Err(types::FusionError::SystemError),
    }
}

/// Generate a unique order ID
fn generate_order_id() -> String {
    let timestamp = ic_cdk::api::time();
    let caller = ic_cdk::caller();
    format!("fusion_{}_{}", timestamp, caller.to_text())
}

// ============================================================================
// CANISTER UPGRADE HOOKS
// ============================================================================

/// Pre-upgrade hook: Save state to stable memory
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let state = memory::serialize_orderbook_state();
    // Save state to stable memory, but don't panic if it fails
    if let Err(e) = ic_cdk::storage::stable_save((state,)) {
        // Log the error but don't panic - this allows the upgrade to proceed
        ic_cdk::print(format!("Warning: Failed to save orderbook state during upgrade: {:?}", e));
    }
}

/// Post-upgrade hook: Restore state from stable memory
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    // Try to restore state, but handle the case where no state exists (fresh deployment)
    match ic_cdk::storage::stable_restore() {
        Ok((state,)) => {
            let (orders, identities) = state;
            memory::deserialize_orderbook_state(orders, identities);
        }
        Err(_) => {
            // No existing state found - this is a fresh deployment
            // Initialize with empty state (default values)
            memory::deserialize_orderbook_state(vec![], vec![]);
        }
    }
}

ic_cdk::export_candid!();
