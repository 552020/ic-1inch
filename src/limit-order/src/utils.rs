use candid::Principal;
use ic_cdk::api::time;
use std::collections::{HashMap, HashSet};

use crate::{Order, OrderError};

// ============================================================================
// MEMORY MANAGEMENT - Simplified Storage
// ============================================================================

thread_local! {
    static ORDERS: std::cell::RefCell<HashMap<Vec<u8>, Order>> = std::cell::RefCell::new(HashMap::new());
    static FILLED_ORDERS: std::cell::RefCell<HashSet<Vec<u8>>> = std::cell::RefCell::new(HashSet::new());
    static CANCELLED_ORDERS: std::cell::RefCell<HashSet<Vec<u8>>> = std::cell::RefCell::new(HashSet::new());
    static REMAINING_AMOUNTS: std::cell::RefCell<HashMap<Vec<u8>, u64>> = std::cell::RefCell::new(HashMap::new());
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn validate_order_parameters(order: &Order) -> Result<(), OrderError> {
    if order.making_amount == 0 || order.taking_amount == 0 {
        return Err(OrderError::InvalidAmount);
    }

    if order.expiration <= time() {
        return Err(OrderError::OrderExpired);
    }

    Ok(())
}

pub fn validate_order_signature(_order: &Order, signature: &[u8]) -> Result<(), OrderError> {
    // ICP adaptation: For now, accept any signature since we use principal authentication
    // In production, this would verify EIP-712 equivalent signature
    if signature.is_empty() {
        return Err(OrderError::InvalidSignature);
    }

    Ok(())
}

pub fn validate_order_fillable(order: &Order, order_hash: &[u8]) -> Result<(), OrderError> {
    // Check if order is cancelled
    CANCELLED_ORDERS.with(|cancelled| {
        if cancelled.borrow().contains(order_hash) {
            return Err(OrderError::OrderCancelled);
        }
        Ok(())
    })?;

    // Check if order is filled
    FILLED_ORDERS.with(|filled| {
        if filled.borrow().contains(order_hash) {
            return Err(OrderError::OrderAlreadyFilled);
        }
        Ok(())
    })?;

    // Check if order is expired
    if order.expiration <= time() {
        return Err(OrderError::OrderExpired);
    }

    Ok(())
}

pub fn compute_escrow_address(order_hash: &[u8], taker: &Principal) -> Vec<u8> {
    // Deterministic escrow address computation
    // In ICP, this would be a canister ID or similar identifier
    let salt_data = format!("{}{}", hex::encode(order_hash), taker.to_string());

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(salt_data.as_bytes());
    hasher.finalize().to_vec()
}

pub fn validate_fill_amount(order: &Order, amount: u64) -> Result<(), OrderError> {
    if amount == 0 {
        return Err(OrderError::InvalidAmount);
    }

    if amount > order.taking_amount {
        return Err(OrderError::InsufficientAmount);
    }

    Ok(())
}

pub async fn execute_atomic_fill(order: &Order, amount: u64) -> Result<(u64, u64), OrderError> {
    // Calculate proportional amounts
    let making_amount = (order.making_amount * amount) / order.taking_amount;
    let taking_amount = amount;

    // In a real implementation, this would:
    // 1. Transfer maker's tokens to escrow address
    // 2. Transfer taker's tokens to maker
    // 3. Handle any fees

    // For now, just return the calculated amounts
    Ok((making_amount, taking_amount))
}

pub fn update_order_state(order_hash: &[u8], making_amount: u64) -> Result<(), OrderError> {
    // Update remaining amounts
    REMAINING_AMOUNTS.with(|amounts| {
        let mut amounts = amounts.borrow_mut();
        let current = amounts.get(order_hash).copied().unwrap_or(0);
        amounts.insert(order_hash.to_vec(), current + making_amount);
    });

    // If order is fully filled, mark as filled
    ORDERS.with(|orders| {
        if let Some(order) = orders.borrow().get(order_hash) {
            let total_filled = REMAINING_AMOUNTS
                .with(|amounts| amounts.borrow().get(order_hash).copied().unwrap_or(0));

            if total_filled >= order.making_amount {
                FILLED_ORDERS.with(|filled| {
                    filled.borrow_mut().insert(order_hash.to_vec());
                });
            }
        }
    });

    Ok(())
}

pub fn parse_extension_args(_args: &[u8]) -> Result<ExtensionData, OrderError> {
    // Parse extension data for cross-chain parameters
    // This would decode hashlock, timelock, chain_id, etc.
    Ok(ExtensionData { hashlock: None, timelock: None, chain_id: None })
}

pub async fn execute_cross_chain_fill(
    order: &Order,
    amount: u64,
    _extension_data: &ExtensionData,
) -> Result<(u64, u64), OrderError> {
    // Cross-chain fill implementation
    // This would coordinate with escrow manager for cross-chain swaps

    // For now, just return the same as atomic fill
    execute_atomic_fill(order, amount).await
}

pub fn hash_order(order: Order) -> Vec<u8> {
    // Create deterministic hash of order data
    let order_data = format!(
        "{}{}{}{}{}{}{}{}",
        order.salt,
        order.maker.to_string(),
        order.receiver.to_string(),
        order.maker_asset.to_string(),
        order.taker_asset.to_string(),
        order.making_amount,
        order.taking_amount,
        order.expiration
    );

    // Use SHA256 for ICP compatibility
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(order_data.as_bytes());
    hasher.finalize().to_vec()
}

// ============================================================================
// MEMORY ACCESS FUNCTIONS
// ============================================================================

pub fn get_remaining_amount(order_hash: &[u8]) -> u64 {
    REMAINING_AMOUNTS.with(|amounts| amounts.borrow().get(order_hash).copied().unwrap_or(0))
}

pub fn is_order_cancelled(order_hash: &[u8]) -> bool {
    CANCELLED_ORDERS.with(|cancelled| cancelled.borrow().contains(order_hash))
}

pub fn is_order_filled(order_hash: &[u8]) -> bool {
    FILLED_ORDERS.with(|filled| filled.borrow().contains(order_hash))
}

pub fn store_order(order_hash: Vec<u8>, order: Order) {
    ORDERS.with(|orders| {
        orders.borrow_mut().insert(order_hash, order);
    });
}

pub fn get_order(order_hash: &[u8]) -> Option<Order> {
    ORDERS.with(|orders| orders.borrow().get(order_hash).cloned())
}

pub fn cancel_order_internal(order_hash: Vec<u8>) {
    CANCELLED_ORDERS.with(|cancelled| {
        cancelled.borrow_mut().insert(order_hash);
    });
}

// ============================================================================
// HELPER TYPES
// ============================================================================

#[derive(Debug)]
pub struct ExtensionData {
    pub hashlock: Option<Vec<u8>>,
    pub timelock: Option<u64>,
    pub chain_id: Option<u32>,
}
