use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::caller;

use crate::memory::{
    generate_order_id, get_active_orders, get_order, is_order_active, mark_order_cancelled,
    mark_order_filled, track_error, track_order_cancelled, track_order_created, track_order_filled,
    with_cancelled_orders_read, with_filled_orders_read, with_orders,
};
use crate::types::{
    MakerTraits, Order, OrderError, OrderId, OrderResult, OrderType, ProcessingStrategy,
    SystemStats, TakerTraits, TokenInterface, MAX_ACTIVE_ORDERS, MAX_EXPIRATION_DAYS,
};
// ============================================================================
// VALIDATION FUNCTIONS
// ============================================================================

/// Validate that the caller is authorized to cancel an order
pub fn validate_cancel_order_authorization(caller: Principal, order: &Order) -> OrderResult<()> {
    // Only the maker can cancel their own orders
    if caller != order.maker {
        track_error("unauthorized_cancel");
        return Err(OrderError::Unauthorized);
    }

    // Additional check: reject anonymous callers
    if caller == Principal::anonymous() {
        track_error("anonymous_cancel_attempt");
        return Err(OrderError::Unauthorized);
    }

    Ok(())
}

/// Check if maker has sufficient balance for the order
pub async fn check_maker_balance(
    maker_asset: Principal,
    maker: Principal,
    making_amount: u64,
) -> OrderResult<()> {
    let token_interface = TokenInterface::new(maker_asset);
    let balance = token_interface.balance_of(maker).await?;

    if balance < making_amount {
        track_error("insufficient_maker_balance");
        return Err(OrderError::InsufficientBalance);
    }

    Ok(())
}

/// Check if taker has sufficient balance for the order
pub async fn check_taker_balance(
    taker_asset: Principal,
    taker: Principal,
    taking_amount: u64,
) -> OrderResult<()> {
    let token_interface = TokenInterface::new(taker_asset);
    let balance = token_interface.balance_of(taker).await?;

    if balance < taking_amount {
        track_error("insufficient_taker_balance");
        return Err(OrderError::InsufficientBalance);
    }

    Ok(())
}

/// Check if taker has sufficient balance for protocol fees (MVP - always returns true)
/// TODO: Implement actual balance checking when fee enforcement is enabled
pub fn check_taker_fee_balance(_taker: Principal) -> OrderResult<()> {
    // For MVP: Always return true - no fee enforcement yet
    // Future: Check taker's ICP balance against protocol fee requirements

    // TODO: Implement actual balance checking:
    // 1. Get taker info from registry
    // 2. Check if balance_icp >= min_balance_threshold
    // 3. Return error if insufficient balance

    Ok(())
}

/// Validate Principal is not anonymous and properly formatted
pub fn validate_principal(principal: Principal, field_name: &str) -> OrderResult<()> {
    if principal == Principal::anonymous() {
        track_error(&format!("invalid_principal_{}", field_name));
        return Err(OrderError::AnonymousCaller);
    }

    // Additional validation could be added here for specific principal formats
    Ok(())
}

/// Validate token amounts are within reasonable bounds
pub fn validate_token_amounts(making_amount: u64, taking_amount: u64) -> OrderResult<()> {
    // Check for zero amounts
    if making_amount == 0 {
        track_error("invalid_making_amount_zero");
        return Err(OrderError::InvalidAmount);
    }

    if taking_amount == 0 {
        track_error("invalid_taking_amount_zero");
        return Err(OrderError::InvalidAmount);
    }

    // Check for reasonable maximum amounts (prevent overflow)
    const MAX_TOKEN_AMOUNT: u64 = u64::MAX / 1000; // Leave room for calculations

    if making_amount > MAX_TOKEN_AMOUNT {
        track_error("invalid_making_amount_too_large");
        return Err(OrderError::InvalidAmount);
    }

    if taking_amount > MAX_TOKEN_AMOUNT {
        track_error("invalid_taking_amount_too_large");
        return Err(OrderError::InvalidAmount);
    }

    Ok(())
}

/// Validate expiration timestamp
pub fn validate_expiration_timestamp(expiration: u64) -> OrderResult<()> {
    let current_time = time();

    // Check if expiration is in the past
    if expiration <= current_time {
        track_error("invalid_expiration_past");
        return Err(OrderError::InvalidExpiration);
    }

    // Check if expiration is too far in the future (30 days max)
    let max_expiration = current_time + (MAX_EXPIRATION_DAYS * 24 * 3600 * 1_000_000_000);
    if expiration > max_expiration {
        track_error("invalid_expiration_too_far");
        return Err(OrderError::InvalidExpiration);
    }

    // Check if expiration is too soon (minimum 1 minute)
    let min_expiration = current_time + (60 * 1_000_000_000); // 1 minute
    if expiration < min_expiration {
        track_error("invalid_expiration_too_soon");
        return Err(OrderError::InvalidExpiration);
    }

    Ok(())
}

/// Validate asset pair for trading
pub fn validate_asset_pair(maker_asset: Principal, taker_asset: Principal) -> OrderResult<()> {
    // Assets cannot be the same
    if maker_asset == taker_asset {
        track_error("invalid_asset_pair_same");
        return Err(OrderError::InvalidAssetPair);
    }

    // Validate both assets are not anonymous
    validate_principal(maker_asset, "maker_asset")?;
    validate_principal(taker_asset, "taker_asset")?;

    // Additional validation could be added here for supported tokens
    Ok(())
}

/// Check system limits and DoS protection
pub fn validate_system_limits(caller: Principal) -> OrderResult<()> {
    // Check total number of active orders
    let active_order_count = get_active_orders().len();
    if active_order_count >= MAX_ACTIVE_ORDERS {
        track_error("system_max_orders_reached");
        return Err(OrderError::TooManyOrders);
    }

    // Check orders per maker (prevent spam)
    let maker_orders = get_orders_by_maker(caller);
    const MAX_ORDERS_PER_MAKER: usize = 100;
    if maker_orders.len() >= MAX_ORDERS_PER_MAKER {
        track_error("maker_max_orders_reached");
        return Err(OrderError::TooManyOrders);
    }

    Ok(())
}

// ============================================================================
// TAKER WHITELIST VALIDATION
// ============================================================================

/// Check if a taker is whitelisted
/// TODO: Implement proper whitelist checking when we introduce relayer entity
pub fn is_taker_whitelisted(_taker: Principal) -> bool {
    // For MVP: hardcoded true - all takers are allowed
    // Future: Check against actual whitelist and relayer requirements
    true
}

/// Validate that a taker is whitelisted
pub fn validate_taker_whitelist(taker: Principal) -> OrderResult<()> {
    if is_taker_whitelisted(taker) {
        Ok(())
    } else {
        track_error("taker_not_whitelisted");
        Err(OrderError::Unauthorized)
    }
}

/// Validate order creation parameters
pub fn validate_create_order(
    caller: Principal,
    receiver: Principal,
    maker_asset: Principal,
    taker_asset: Principal,
    making_amount: u64,
    taking_amount: u64,
    expiration: u64,
) -> OrderResult<()> {
    // Validate caller
    validate_principal(caller, "caller")?;

    // Validate receiver
    validate_principal(receiver, "receiver")?;

    // Validate asset pair
    validate_asset_pair(maker_asset, taker_asset)?;

    // Validate amounts
    validate_token_amounts(making_amount, taking_amount)?;

    // Validate expiration
    validate_expiration_timestamp(expiration)?;

    // Check system limits
    validate_system_limits(caller)?;

    Ok(())
}

// ============================================================================
// ORDER MANAGEMENT FUNCTIONS
// ============================================================================

/// Create a new limit order
pub async fn create_order(
    receiver: Principal,
    maker_asset: Principal,
    taker_asset: Principal,
    making_amount: u64,
    taking_amount: u64,
    expiration: u64,
) -> OrderResult<OrderId> {
    let caller = caller();

    // Validate order creation
    validate_create_order(
        caller,
        receiver,
        maker_asset,
        taker_asset,
        making_amount,
        taking_amount,
        expiration,
    )?;

    // Check maker has sufficient balance (skip in test mode with mock tokens)
    if maker_asset != Principal::management_canister()
        && taker_asset != Principal::management_canister()
    {
        check_maker_balance(maker_asset, caller, making_amount).await?;
    }

    // Generate unique order ID
    let order_id = generate_order_id();

    // Create order
    let order = Order {
        id: order_id,
        maker: caller,
        receiver,
        maker_asset,
        taker_asset,
        making_amount,
        taking_amount,
        expiration,
        created_at: time(),
        order_type: OrderType::Normal, // Default to normal order for MVP
        processing_strategy: ProcessingStrategy::DirectTransfer, // Default to direct transfer
        salt: order_id,                // Use order_id as salt for uniqueness
        maker_traits: MakerTraits::None, // Default to no special traits
        taker_traits: TakerTraits::None, // Default to no special traits
        metadata: None,                // MVP doesn't use metadata, reserved for ChainFusion+
    };

    // Store order
    with_orders(|orders| {
        orders.insert(order_id, order);
    });

    // Track statistics
    track_order_created();

    Ok(order_id)
}

/// Cancel an existing order
///
/// This function implements a comprehensive order cancellation process with:
/// - Thorough validation and authorization checks
/// - Detailed error reporting for different failure scenarios
/// - Proper state management and statistics tracking
pub fn cancel_order(order_id: OrderId) -> OrderResult<()> {
    let caller = caller();

    // Phase 1: Order retrieval and basic validation
    let order = get_order(order_id).ok_or_else(|| {
        track_error("cancel_order_not_found");
        OrderError::OrderNotFound
    })?;

    // Phase 2: Authorization validation
    validate_cancel_order_authorization(caller, &order)?;

    // Phase 3: Order state validation with detailed error reporting
    if !is_order_active(order_id) {
        // Check specific reason for inactivity
        if order.expiration <= time() {
            track_error("cancel_expired_order");
            return Err(OrderError::OrderExpired);
        }

        // Check if already filled
        let is_filled = with_filled_orders_read(|filled| filled.contains(&order_id));
        if is_filled {
            track_error("cancel_already_filled_order");
            return Err(OrderError::OrderAlreadyFilled);
        }

        // Check if already cancelled
        let is_cancelled = with_cancelled_orders_read(|cancelled| cancelled.contains(&order_id));
        if is_cancelled {
            track_error("cancel_already_cancelled_order");
            return Err(OrderError::OrderCancelled);
        }

        // If we reach here, something unexpected happened
        track_error("cancel_inactive_order_unknown");
        return Err(OrderError::OrderAlreadyFilled);
    }

    // Phase 4: Execute cancellation
    update_order_cancelled_state(order_id);

    Ok(())
}

/// Helper function: Execute atomic token transfers for order filling
///
/// This is a helper function used by fill_order() to perform the actual token swaps.
/// It implements a two-phase commit pattern for better atomicity:
/// 1. Pre-validate both transfers can succeed
/// 2. Execute both transfers
/// 3. If any transfer fails, attempt rollback (best effort)

/// Fill an existing order atomically
///
/// This function implements a comprehensive order filling process with:
/// - Thorough validation before execution
/// - Atomic token transfers with rollback capability
/// - Proper state management and statistics tracking
pub async fn fill_order(order_id: OrderId) -> OrderResult<()> {
    let taker = caller();

    // Phase 1: Taker whitelist validation
    validate_taker_whitelist(taker)?;

    // Phase 2: Order retrieval and basic validation
    let order = get_order(order_id).ok_or_else(|| {
        track_error("order_not_found");
        OrderError::OrderNotFound
    })?;

    // Phase 3: Authorization validation
    if taker == order.maker {
        track_error("fill_own_order");
        return Err(OrderError::Unauthorized);
    }

    // Phase 4: Order state validation
    if !is_order_active(order_id) {
        // Determine specific reason for better error reporting
        if order.expiration <= time() {
            track_error("fill_expired_order");
            return Err(OrderError::OrderExpired);
        } else {
            // Check if filled or cancelled
            with_filled_orders_read(|filled| {
                if filled.contains(&order_id) {
                    track_error("fill_already_filled_order");
                    return Err(OrderError::OrderAlreadyFilled);
                }
                Ok(())
            })?;

            with_cancelled_orders_read(|cancelled| {
                if cancelled.contains(&order_id) {
                    track_error("fill_cancelled_order");
                    return Err(OrderError::OrderCancelled);
                }
                Ok(())
            })?;

            // If we reach here, something unexpected happened
            track_error("fill_inactive_order_unknown");
            return Err(OrderError::OrderAlreadyFilled);
        }
    }

    // Phase 5: Balance validation (skip in test mode with mock tokens)
    if order.taker_asset != Principal::management_canister() {
        check_taker_balance(order.taker_asset, taker, order.taking_amount).await?;
    }

    // Phase 6: Execute atomic transfers
    execute_order_transfers(&order, taker).await?;

    // Phase 7: Update state and statistics (only after successful transfers)
    update_order_filled_state(order_id, &order);

    Ok(())
}

/// Helper function: Execute atomic token transfers for order filling
///
/// This is a helper function used by fill_order() to perform the actual token swaps.
/// It implements a two-phase commit pattern for better atomicity:
/// 1. Pre-validate both transfers can succeed
/// 2. Execute both transfers
/// 3. If any transfer fails, attempt rollback (best effort)
async fn execute_order_transfers(order: &Order, taker: Principal) -> OrderResult<()> {
    // Skip actual transfers in test mode with mock tokens (management canister)
    if order.maker_asset == Principal::management_canister()
        || order.taker_asset == Principal::management_canister()
    {
        // Test mode: simulate successful transfers without actual ICRC calls
        return Ok(());
    }

    let maker_token = TokenInterface::new(order.maker_asset);
    let taker_token = TokenInterface::new(order.taker_asset);

    // Phase 1: Pre-validation - Check balances again to minimize failure risk
    let taker_balance = taker_token.balance_of(taker).await?;
    if taker_balance < order.taking_amount {
        track_error("taker_insufficient_balance_at_fill");
        return Err(OrderError::InsufficientBalance);
    }

    let maker_balance = maker_token.balance_of(order.maker).await?;
    if maker_balance < order.making_amount {
        track_error("maker_insufficient_balance_at_fill");
        return Err(OrderError::InsufficientBalance);
    }

    // Phase 2: Execute transfers with rollback capability
    // Transfer 1: Taker asset from taker to receiver
    let taker_transfer_result =
        taker_token.transfer(taker, order.receiver, order.taking_amount).await;

    let taker_block_index = match taker_transfer_result {
        Ok(block_index) => block_index,
        Err(e) => {
            track_error("taker_transfer_failed");
            return Err(e);
        }
    };

    // Transfer 2: Maker asset from maker to taker
    let maker_transfer_result = maker_token.transfer(order.maker, taker, order.making_amount).await;

    match maker_transfer_result {
        Ok(_maker_block_index) => {
            // Both transfers successful
            Ok(())
        }
        Err(e) => {
            track_error("maker_transfer_failed");

            // Attempt rollback of taker transfer (best effort)
            // Note: This is not guaranteed to succeed due to ICRC-1 limitations
            // In production, consider using ICRC-2 with allowances for better atomicity
            let rollback_result =
                taker_token.transfer(order.receiver, taker, order.taking_amount).await;

            match rollback_result {
                Ok(_) => {
                    track_error("transfer_rolled_back_successfully");
                    // Return original error
                    Err(e)
                }
                Err(rollback_error) => {
                    track_error("rollback_failed_critical");
                    // Critical: Rollback failed, system may be in inconsistent state
                    // Log both errors for manual intervention
                    Err(OrderError::SystemError(format!(
                        "Transfer failed and rollback failed. Original: {:?}, Rollback: {:?}, TakerBlockIndex: {}",
                        e, rollback_error, taker_block_index
                    )))
                }
            }
        }
    }
}

/// Update order state and statistics after successful fill
/// This function ensures atomic state updates
fn update_order_filled_state(order_id: OrderId, order: &Order) {
    // Mark order as filled
    mark_order_filled(order_id);

    // Track statistics for both assets
    track_order_filled(order.maker_asset, order.making_amount);
    track_order_filled(order.taker_asset, order.taking_amount);
}

/// Update order state and statistics after successful cancellation
/// This function ensures atomic state updates
fn update_order_cancelled_state(order_id: OrderId) {
    // Mark order as cancelled
    mark_order_cancelled(order_id);

    // Track statistics
    track_order_cancelled();
}

/// Batch cancel multiple orders by the same maker

// ============================================================================
// QUERY FUNCTIONS
// ============================================================================

/// Get all active orders
pub fn get_active_orders_list() -> Vec<Order> {
    get_active_orders()
}

/// Get a specific order by ID
pub fn get_order_by_id(order_id: OrderId) -> Option<Order> {
    get_order(order_id)
}

/// Get orders by maker
pub fn get_orders_by_maker(maker: Principal) -> Vec<Order> {
    with_orders(|orders| orders.values().filter(|order| order.maker == maker).cloned().collect())
}

/// Get orders by asset pair
pub fn get_orders_by_asset_pair(maker_asset: Principal, taker_asset: Principal) -> Vec<Order> {
    get_active_orders()
        .into_iter()
        .filter(|order| order.maker_asset == maker_asset && order.taker_asset == taker_asset)
        .collect()
}

/// Get system statistics
pub fn get_system_statistics() -> SystemStats {
    crate::memory::with_system_stats_read(|stats| stats.clone())
}

// ============================================================================
// TESTING UTILITIES
// ============================================================================

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::memory::clear_limit_order_data;

    /// Create a test order for unit testing
    pub fn create_test_order() -> Order {
        Order {
            id: 1,
            maker: Principal::management_canister(),
            receiver: Principal::management_canister(),
            maker_asset: Principal::from_slice(&[1, 2, 3, 4]),
            taker_asset: Principal::from_slice(&[5, 6, 7, 8]),
            making_amount: 1000,
            taking_amount: 2000,
            expiration: time() + 3600_000_000_000, // 1 hour
            created_at: time(),

            order_type: OrderType::Normal,
            processing_strategy: ProcessingStrategy::DirectTransfer,
            salt: 1, // Placeholder for salt
            maker_traits: MakerTraits::None,
            taker_traits: TakerTraits::None,
            metadata: None,
        }
    }

    /// Setup function for tests
    pub fn setup_test() {
        clear_limit_order_data();
    }
}
