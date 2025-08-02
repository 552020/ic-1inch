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
    let caller = ic_cdk::caller();
    let current_time = ic_cdk::api::time();

    // Validate amounts
    if from_amount == 0 || to_amount == 0 {
        return Err(FusionError::InvalidAmount);
    }

    // Validate expiration is in the future (at least 1 hour)
    if expiration <= current_time + 3600_000_000_000 {
        return Err(FusionError::InvalidExpiration);
    }

    // Validate secret hash format (should be 64 hex characters)
    if secret_hash.len() != 64 || !secret_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(FusionError::InvalidSecretHash);
    }

    // Generate unique order ID
    let order_id = generate_order_id();

    // Check if this is an ICP → ETH order (for automatic locking)
    let _is_icp_to_eth = from_token == Token::ICP && to_token == Token::ETH;

    // Create the fusion order with enhanced data using new constructor
    let mut order = FusionOrder::new(
        order_id.clone(),
        maker_eth_address.clone(),
        caller,
        format!("{}", current_time), // Use timestamp as salt for now
        format!("0x{:040x}", if from_token == Token::ICP { 1 } else { 0 }), // Placeholder maker_asset
        format!("0x{:040x}", if to_token == Token::ETH { 1 } else { 0 }), // Placeholder taker_asset
        from_amount,
        to_amount,
        secret_hash.clone(),
    );

    // Set additional fields
    order.created_at = current_time;
    order.expires_at = expiration;
    order.from_token = from_token;
    order.to_token = to_token;

    // Store the order
    memory::store_fusion_order(order.clone())?;

    // Note: Escrow creation is now handled by the escrow factory
    // The orderbook will be notified via notify_escrow_created when escrows are ready

    ic_cdk::println!(
        "✅ Created fusion order {} for maker {} ({:?} → {:?})",
        order_id,
        maker_eth_address,
        from_token,
        to_token
    );

    Ok(order_id)
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
// DUTCH AUCTION SYSTEM (Fusion+ Whitepaper 2.3)
// ============================================================================

/// Get current price for a Dutch auction order
#[ic_cdk::query]
fn get_current_price(order_id: String) -> Result<u64, FusionError> {
    let order = memory::get_fusion_order(&order_id)?;
    Ok(order.calculate_current_price())
}

/// Check if order is profitable for resolver at current price
#[ic_cdk::query]
fn is_order_profitable(order_id: String, resolver_fee: u64) -> Result<bool, FusionError> {
    let order = memory::get_fusion_order(&order_id)?;
    Ok(order.is_profitable_for_resolver(resolver_fee))
}

/// Get all orders currently in Dutch auction phase
#[ic_cdk::query]
fn get_orders_in_auction() -> Vec<FusionOrder> {
    memory::get_all_fusion_orders()
        .into_iter()
        .filter(|order| order.fusion_state == types::FusionState::AnnouncementPhase)
        .collect()
}

/// Get orders within a specific price range
#[ic_cdk::query]
fn get_orders_by_price_range(min_price: u64, max_price: u64) -> Vec<FusionOrder> {
    memory::get_all_fusion_orders()
        .into_iter()
        .filter(|order| {
            let current_price = order.calculate_current_price();
            current_price >= min_price && current_price <= max_price
        })
        .collect()
}

/// Update price curve for an order (admin function)
#[ic_cdk::update]
fn update_price_curve(order_id: String, new_curve: types::PriceCurve) -> Result<(), FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;

    // Only allow updates in announcement phase
    if order.fusion_state != types::FusionState::AnnouncementPhase {
        return Err(FusionError::InvalidStateTransition);
    }

    order.price_curve = new_curve;
    memory::store_fusion_order(order)?;

    Ok(())
}

/// Get price curve for an order
#[ic_cdk::query]
fn get_price_curve(order_id: String) -> Result<types::PriceCurve, FusionError> {
    let order = memory::get_fusion_order(&order_id)?;
    Ok(order.price_curve.clone())
}

// ============================================================================
// PARTIAL FILL SYSTEM (Fusion+ Whitepaper 2.5)
// ============================================================================

/// Partially fill an order
#[ic_cdk::update]
async fn partially_fill_order(
    order_id: String,
    resolver_address: String,
    fill_amount: u64,
) -> Result<String, FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;

    // Verify order supports partial fills
    if !order.supports_partial_fills() {
        return Err(FusionError::PartialFillsNotSupported);
    }

    // Verify fill amount is valid
    if fill_amount == 0 || fill_amount > order.get_remaining_amount() {
        return Err(FusionError::InvalidFillAmount);
    }

    // Calculate which secret to use based on fill percentage
    let current_fill_percentage = if let Some(ref partial_data) = order.partial_fill_data {
        (partial_data.filled_amount * 100) / order.making_amount
    } else {
        0
    };

    let required_secret_index = (current_fill_percentage * order.secret_hashes.len() as u64) / 100;

    // Create partial fill record
    let partial_fill = types::PartialFill {
        resolver_address: resolver_address.clone(),
        fill_amount,
        secret_index: required_secret_index as u32,
        timestamp: ic_cdk::api::time(),
    };

    // Add partial fill to order
    order.add_partial_fill(partial_fill)?;

    // Update order status if fully filled
    if order.is_fully_filled() {
        order.status = OrderStatus::Completed;
        order.fusion_state = types::FusionState::Completed;
        order.completed_at = Some(ic_cdk::api::time());
    }

    memory::store_fusion_order(order)?;

    let fill_id = format!("partial_fill_{}_{}", order_id, ic_cdk::api::time());

    ic_cdk::println!(
        "📊 Partial fill {} created for order {}: {} tokens by {}",
        fill_id,
        order_id,
        fill_amount,
        resolver_address
    );

    Ok(fill_id)
}

/// Reveal multiple secrets for partial fills
#[ic_cdk::update]
async fn reveal_multiple_secrets(
    order_id: String,
    secrets: Vec<String>,
) -> Result<(), FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;

    // Verify order is in correct state
    if order.fusion_state != types::FusionState::DepositPhase {
        return Err(FusionError::InvalidStateTransition);
    }

    // Verify secrets match stored hashes
    for (i, secret) in secrets.iter().enumerate() {
        if let Some(expected_hash) = order.get_secret_hash(i) {
            let computed_hash = compute_secret_hash(secret);
            if &computed_hash != expected_hash {
                return Err(FusionError::InvalidSecret);
            }
        }
    }

    // Update order state
    order.fusion_state = types::FusionState::WithdrawalPhase;
    memory::store_fusion_order(order)?;

    ic_cdk::println!("🔓 Revealed {} secrets for order {}", secrets.len(), order_id);

    Ok(())
}

/// Submit secret for a specific partial fill
#[ic_cdk::update]
async fn submit_secret_for_partial_fill(
    order_id: String,
    secret: String,
    secret_index: u32,
) -> Result<(), FusionError> {
    let order = memory::get_fusion_order(&order_id)?;

    // Verify secret matches the expected hash at the given index
    if let Some(expected_hash) = order.get_secret_hash(secret_index as usize) {
        let computed_hash = compute_secret_hash(&secret);
        if &computed_hash != expected_hash {
            return Err(FusionError::InvalidSecret);
        }
    } else {
        return Err(FusionError::InvalidSecret);
    }

    ic_cdk::println!("🔑 Secret submitted for partial fill {} of order {}", secret_index, order_id);

    Ok(())
}

// ============================================================================
// ESCROW FACTORY NOTIFICATION SYSTEM
// ============================================================================

/// Called by escrow factory to notify orderbook of escrow creation
#[ic_cdk::update]
fn notify_escrow_created(
    order_id: String,
    escrow_address: String,
    escrow_type: types::EscrowType,
) -> Result<(), FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;

    // Update order with escrow address
    match escrow_type {
        types::EscrowType::Source => {
            order.escrow_src_address = Some(escrow_address.clone());
        }
        types::EscrowType::Destination => {
            order.escrow_dst_address = Some(escrow_address.clone());
        }
    }

    // Update fusion state if both escrows are created
    if order.escrow_src_address.is_some() && order.escrow_dst_address.is_some() {
        order.fusion_state = types::FusionState::DepositPhase;
    }

    memory::store_fusion_order(order)?;

    ic_cdk::println!(
        "📬 Escrow {:?} created for order {}: {}",
        escrow_type,
        order_id,
        escrow_address
    );

    Ok(())
}

/// Called by escrow factory to notify orderbook of escrow completion
#[ic_cdk::update]
fn notify_escrow_completed(order_id: String, escrow_address: String) -> Result<(), FusionError> {
    let mut order = memory::get_fusion_order(&order_id)?;

    // Update order status to completed
    order.status = OrderStatus::Completed;
    order.fusion_state = types::FusionState::Completed;
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
    order.fusion_state = types::FusionState::Cancelled;

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
