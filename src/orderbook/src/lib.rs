mod memory;
mod types;

use candid::Principal;
use types::{FusionError, FusionOrder, OrderStatus, Token};

/// Create a new fusion order with Dutch auction (Enhanced Fusion+ Protocol) - PLACEHOLDER
/// This function will be implemented in future tasks to avoid IC CDK parameter limit issues
/// Currently returns a placeholder error to maintain compilation
fn create_fusion_order_placeholder() -> Result<String, FusionError> {
    // TODO: Implement enhanced fusion order creation with Dutch auction
    // This will be done in Task 4: Update order creation functionality
    Err(FusionError::NotImplemented)
}

/// Create a new fusion order with 1inch LOP compatibility (Enhanced)
#[ic_cdk::update]
async fn create_fusion_order(
    // 1inch LOP Order Parameters
    salt: String,
    maker_asset: String,
    taker_asset: String,
    making_amount: u64,
    taking_amount: u64,
    maker_traits: String,

    // Cross-chain parameters
    hashlock: String, // Secret hash for atomic swap
    expiration: u64,

    // Optional EIP-712 signature for ETH→ICP orders
    eip712_signature: Option<types::EIP712Signature>,
) -> Result<String, FusionError> {
    let caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();

    // Validate 1inch LOP parameters
    validate_lop_parameters(
        &salt,
        &maker_asset,
        &taker_asset,
        making_amount,
        taking_amount,
        &maker_traits,
    )?;

    // Validate hashlock format (should be 64 hex characters)
    if hashlock.len() != 64 || !hashlock.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(FusionError::InvalidSecretHash);
    }

    // Validate expiration is in the future (at least 1 hour)
    if expiration <= current_time + 3600_000_000_000 {
        return Err(FusionError::InvalidExpiration);
    }

    // Generate unique order ID
    let order_id = generate_order_id();

    // Determine order direction based on assets
    let is_eth_to_icp = is_eth_asset(&maker_asset);

    // Validate EIP-712 signature requirement for ETH→ICP orders
    if is_eth_to_icp && eip712_signature.is_none() {
        return Err(FusionError::InvalidEIP712Signature);
    }

    // Create the fusion order with 1inch LOP compatibility
    let mut order = types::FusionOrder::new(
        order_id.clone(),
        caller.to_text(), // Use caller as maker address for ICP side
        caller,
        salt,
        maker_asset,
        taker_asset,
        making_amount,
        taking_amount,
        hashlock,
    );

    // Set additional fields
    order.created_at = current_time;
    order.expires_at = expiration;
    order.maker_traits = maker_traits;
    order.eip712_signature = eip712_signature;

    // Store the order
    memory::store_fusion_order(order.clone())?;

    ic_cdk::println!(
        "✅ Created 1inch LOP compatible fusion order {} for maker {} ({}→{})",
        order_id,
        caller.to_text(),
        order.maker_asset,
        order.taker_asset
    );

    Ok(order_id)
}

/// Create a new fusion order for cross-chain swaps (Legacy function for backward compatibility)
#[ic_cdk::update]
async fn create_order(
    maker_eth_address: String,
    from_token: Token,
    to_token: Token,
    from_amount: u64,
    to_amount: u64,
    expiration: u64,
    secret_hash: String, // Hashlock for atomic swap
) -> Result<String, FusionError> {
    // Convert legacy parameters to 1inch LOP format
    let salt = format!("{}", ic_cdk::api::time()); // Use timestamp as salt
    let maker_asset = token_to_address(&from_token);
    let taker_asset = token_to_address(&to_token);
    let maker_traits =
        "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(); // Default traits

    // Call the enhanced function
    create_fusion_order(
        salt,
        maker_asset,
        taker_asset,
        from_amount,
        to_amount,
        maker_traits,
        secret_hash,
        expiration,
        None, // No EIP-712 signature for legacy orders
    )
    .await
}

/// Accept a fusion order as a resolver - Used by: Resolvers
#[ic_cdk::update]
async fn accept_fusion_order(
    order_id: String,
    resolver_eth_address: String,
) -> Result<String, FusionError> {
    let caller = ic_cdk::caller();

    // Get the order
    let mut order = memory::get_fusion_order(&order_id)?;

    // Verify order is still pending
    if order.status != OrderStatus::Pending {
        return Err(FusionError::OrderNotPending);
    }

    // Check if order has expired
    if ic_cdk::api::time() > order.expires_at {
        order.status = OrderStatus::Failed;
        memory::store_fusion_order(order)?;
        return Err(FusionError::OrderExpired);
    }

    // Update order with resolver info
    order.status = OrderStatus::Accepted;
    order.resolver_eth_address = Some(resolver_eth_address.clone());
    order.resolver_icp_principal = Some(caller);
    order.accepted_at = Some(ic_cdk::api::time());

    // Note: Escrow creation is now handled by the escrow factory
    // The resolver will coordinate with the escrow factory directly
    // The orderbook will be notified via notify_escrow_created when escrows are ready

    memory::store_fusion_order(order.clone())?;

    ic_cdk::println!(
        "✅ Order {} accepted by resolver {} ({})",
        order_id,
        resolver_eth_address,
        caller.to_text()
    );

    // Return the order data needed for ETH escrow creation
    Ok(format!(
        "{{\"order_id\":\"{}\",\"secret_hash\":\"{}\",\"amount\":{},\"timelock\":{}}}",
        order_id, order.secret_hash, order.from_amount, order.timelock_duration
    ))
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

    // Update status and completion timestamp if completed
    order.status = status.clone();
    if status == OrderStatus::Completed {
        order.completed_at = Some(ic_cdk::api::time());
    }

    memory::store_fusion_order(order)?;

    ic_cdk::println!("📊 Order {} status updated to {:?}", order_id, status);
    Ok(())
}

/// Complete order with secret revelation - Used by: Resolvers
#[ic_cdk::update]
async fn complete_order_with_secret(order_id: String, secret: String) -> Result<(), FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;

    // Verify order is accepted
    if order.status != OrderStatus::Accepted {
        return Err(FusionError::OrderNotPending);
    }

    // Validate secret format (64 hex characters)
    if secret.len() != 64 || !secret.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(FusionError::InvalidSecret);
    }

    // Verify secret matches hash (simplified validation for MVP)
    let expected_hash = compute_secret_hash(&secret);
    if expected_hash != order.secret_hash {
        return Err(FusionError::InvalidSecret);
    }

    // Call escrow canister to complete the swap
    let escrow_canister_id = Principal::from_text("uzt4z-lp777-77774-qaabq-cai")
        .map_err(|_| FusionError::SystemError)?;

    let result: Result<(Result<(), String>,), _> =
        ic_cdk::call(escrow_canister_id, "complete_swap_with_secret", (order_id.clone(), secret))
            .await;

    match result {
        Ok((Ok(()),)) => {
            order.status = OrderStatus::Completed;
            order.completed_at = Some(ic_cdk::api::time());
            memory::store_fusion_order(order)?;

            ic_cdk::println!("🎉 Order {} completed successfully", order_id);
            Ok(())
        }
        Ok((Err(error_msg),)) => {
            ic_cdk::println!("Escrow completion error: {}", error_msg);
            Err(FusionError::SystemError)
        }
        Err(_) => Err(FusionError::SystemError),
    }
}

/// Cancel an expired or failed order - Used by: Makers/Resolvers
#[ic_cdk::update]
async fn cancel_order(order_id: String) -> Result<(), FusionError> {
    let caller = ic_cdk::caller();
    let mut order = memory::get_fusion_order(&order_id)?;

    // Check authorization (maker or resolver can cancel)
    let is_maker = order.maker_icp_principal == caller;
    let is_resolver = order.resolver_icp_principal == Some(caller);

    if !is_maker && !is_resolver {
        return Err(FusionError::Unauthorized);
    }

    // Check if order can be cancelled (expired or failed)
    let current_time = ic_cdk::api::time();
    let can_cancel = current_time > order.expires_at || order.status == OrderStatus::Failed;

    if !can_cancel && order.status != OrderStatus::Pending {
        return Err(FusionError::OrderNotCancellable);
    }

    // Call escrow canister to handle cancellation
    let escrow_canister_id = Principal::from_text("uzt4z-lp777-77774-qaabq-cai")
        .map_err(|_| FusionError::SystemError)?;

    let result: Result<(Result<(), String>,), _> =
        ic_cdk::call(escrow_canister_id, "cancel_order", (order_id.clone(),)).await;

    match result {
        Ok((Ok(()),)) => {
            order.status = OrderStatus::Cancelled;
            memory::store_fusion_order(order)?;

            ic_cdk::println!("❌ Order {} cancelled by {}", order_id, caller.to_text());
            Ok(())
        }
        Ok((Err(error_msg),)) => {
            ic_cdk::println!("Escrow cancellation error: {}", error_msg);
            Err(FusionError::SystemError)
        }
        Err(_) => Err(FusionError::SystemError),
    }
}

/// Get orders by status - Used by: Frontend/Resolvers
#[ic_cdk::query]
fn get_orders_by_status(status: OrderStatus) -> Vec<FusionOrder> {
    memory::get_all_fusion_orders().into_iter().filter(|order| order.status == status).collect()
}

/// Get order statistics - Used by: Frontend/Analytics
#[ic_cdk::query]
fn get_order_statistics() -> types::OrderStatistics {
    let all_orders = memory::get_all_fusion_orders();

    let total_orders = all_orders.len() as u64;
    let pending_orders =
        all_orders.iter().filter(|o| o.status == OrderStatus::Pending).count() as u64;
    let accepted_orders =
        all_orders.iter().filter(|o| o.status == OrderStatus::Accepted).count() as u64;
    let completed_orders =
        all_orders.iter().filter(|o| o.status == OrderStatus::Completed).count() as u64;
    let failed_orders =
        all_orders.iter().filter(|o| o.status == OrderStatus::Failed).count() as u64;
    let cancelled_orders =
        all_orders.iter().filter(|o| o.status == OrderStatus::Cancelled).count() as u64;

    let total_icp_volume = all_orders
        .iter()
        .filter(|o| o.from_token == Token::ICP && o.status == OrderStatus::Completed)
        .map(|o| o.from_amount)
        .sum();

    let total_eth_volume = all_orders
        .iter()
        .filter(|o| o.from_token == Token::ETH && o.status == OrderStatus::Completed)
        .map(|o| o.from_amount)
        .sum();

    types::OrderStatistics {
        total_orders,
        pending_orders,
        accepted_orders,
        completed_orders,
        failed_orders,
        cancelled_orders,
        total_icp_volume,
        total_eth_volume,
    }
}

/// Register cross-chain identity with both ETH address and ICP principal - Used by: Frontend after SIWE
#[ic_cdk::update]
fn register_cross_chain_identity(
    eth_address: String,
    icp_principal: Principal,
    role: types::UserRole,
) -> Result<(), types::FusionError> {
    // Validate ETH address format
    if !is_valid_address(&eth_address) {
        return Err(types::FusionError::TokenAddressInvalid);
    }

    let identity =
        types::CrossChainIdentity { eth_address: eth_address.clone(), icp_principal, role };

    memory::store_cross_chain_identity(identity)?;

    ic_cdk::println!(
        "Registered cross-chain identity: {} <-> {}",
        eth_address,
        icp_principal.to_text()
    );

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

/// Get ICP principal from stored identity mapping - Used by: Frontend/Users
#[ic_cdk::query]
fn get_principal_from_eth_address(eth_address: String) -> Result<Principal, types::FusionError> {
    match memory::get_cross_chain_identity(&eth_address) {
        Ok(identity) => Ok(identity.icp_principal),
        Err(_) => Err(types::FusionError::OrderNotFound),
    }
}

/// Validate and store identity pair from frontend/SIWE - Used by: Frontend after SIWE authentication
#[ic_cdk::update]
fn store_siwe_identity(
    eth_address: String,
    icp_principal: Principal,
    role: types::UserRole,
) -> Result<(), types::FusionError> {
    // This is the same as register_cross_chain_identity but with clearer naming for SIWE flow
    register_cross_chain_identity(eth_address, icp_principal, role)
}

/// Generate a unique order ID
fn generate_order_id() -> String {
    let timestamp = ic_cdk::api::time();
    let caller = ic_cdk::caller();
    format!("fusion_{}_{}", timestamp, caller.to_text())
}

/// Validate 1inch LOP parameters
fn validate_lop_parameters(
    salt: &str,
    maker_asset: &str,
    taker_asset: &str,
    making_amount: u64,
    taking_amount: u64,
    maker_traits: &str,
) -> Result<(), FusionError> {
    // Validate salt (should be non-empty)
    if salt.is_empty() {
        return Err(FusionError::InvalidSalt);
    }

    // Validate amounts (should be non-zero)
    if making_amount == 0 || taking_amount == 0 {
        return Err(FusionError::InvalidAmount);
    }

    // Validate asset addresses (should be valid hex addresses)
    if !is_valid_address(maker_asset) || !is_valid_address(taker_asset) {
        return Err(FusionError::TokenAddressInvalid);
    }

    // Validate maker traits (should be valid hex string)
    if !is_valid_hex_string(maker_traits) {
        return Err(FusionError::InvalidMakerTraits);
    }

    Ok(())
}

/// Check if an asset address represents ETH
fn is_eth_asset(asset_address: &str) -> bool {
    // ETH is typically represented as 0x0000000000000000000000000000000000000000
    // or specific ETH token addresses
    asset_address.to_lowercase() == "0x0000000000000000000000000000000000000000"
        || asset_address.to_lowercase().contains("eth")
}

/// Convert legacy Token enum to address string
fn token_to_address(token: &Token) -> String {
    match token {
        Token::ICP => "0x0000000000000000000000000000000000000001".to_string(), // Placeholder ICP address
        Token::ETH => "0x0000000000000000000000000000000000000000".to_string(), // ETH address
    }
}

/// Validate if a string is a valid Ethereum address
fn is_valid_address(address: &str) -> bool {
    // Basic validation: should start with 0x and be 42 characters long
    address.len() == 42
        && address.starts_with("0x")
        && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate if a string is a valid hex string
fn is_valid_hex_string(hex_str: &str) -> bool {
    // Should start with 0x and contain only hex characters
    hex_str.starts_with("0x") && hex_str[2..].chars().all(|c| c.is_ascii_hexdigit())
}

// Removed: Complex Dutch auction system for MVP simplicity

// Removed: Complex partial fill system for MVP simplicity

// ============================================================================
// ESCROW FACTORY NOTIFICATION SYSTEM
// ============================================================================

// Simplified escrow notification system for MVP
/// Called by escrow factory to notify orderbook of escrow completion
#[ic_cdk::update]
fn notify_escrow_completed(order_id: String, escrow_address: String) -> Result<(), FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;

    // Update order status to completed
    order.status = OrderStatus::Completed;
    order.completed_at = Some(ic_cdk::api::time());

    memory::store_fusion_order(order)?;

    ic_cdk::println!("🎉 Escrow completed for order {}: {}", order_id, escrow_address);

    Ok(())
}

/// Called by escrow factory to notify orderbook of escrow cancellation
#[ic_cdk::update]
fn notify_escrow_cancelled(order_id: String, escrow_address: String) -> Result<(), FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;

    // Update order status to cancelled
    order.status = OrderStatus::Cancelled;

    memory::store_fusion_order(order)?;

    ic_cdk::println!("❌ Escrow cancelled for order {}: {}", order_id, escrow_address);

    Ok(())
}

// ============================================================================
// REMOVED: ESCROW CREATION FUNCTIONS
// These functions have been removed as they should be handled by the escrow factory
// The orderbook now uses the notification system to track escrow creation
// ============================================================================

/// Calculate safety deposit based on order amount (5% of order value)
fn calculate_safety_deposit(amount: u64) -> u64 {
    (amount * 5) / 100
}

/// Compute SHA256 hash of secret (simplified for MVP)
fn compute_secret_hash(secret: &str) -> String {
    // In a real implementation, this would use proper SHA256
    // For MVP, we'll use a simple hash simulation
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    secret.hash(&mut hasher);
    format!(
        "{:016x}{:016x}{:016x}{:016x}",
        hasher.finish(),
        hasher.finish(),
        hasher.finish(),
        hasher.finish()
    )
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
