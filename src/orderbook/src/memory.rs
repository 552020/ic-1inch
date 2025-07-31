use crate::types::{CrossChainIdentity, FusionError, FusionOrder};
use std::cell::RefCell;
use std::collections::HashMap;

// Global state using thread_local! for safety
thread_local! {
    static FUSION_ORDERS: RefCell<HashMap<String, FusionOrder>> = RefCell::new(HashMap::new());
    static CROSS_CHAIN_IDENTITIES: RefCell<HashMap<String, CrossChainIdentity>> = RefCell::new(HashMap::new());
}

/// Store a fusion order
pub fn store_fusion_order(order: FusionOrder) -> Result<(), FusionError> {
    FUSION_ORDERS.with(|orders| {
        orders.borrow_mut().insert(order.id.clone(), order);
        Ok(())
    })
}

/// Get a fusion order by ID
pub fn get_fusion_order(order_id: &str) -> Result<FusionOrder, FusionError> {
    FUSION_ORDERS
        .with(|orders| orders.borrow().get(order_id).cloned().ok_or(FusionError::OrderNotFound))
}

/// Get all fusion orders
pub fn get_all_fusion_orders() -> Vec<FusionOrder> {
    FUSION_ORDERS.with(|orders| orders.borrow().values().cloned().collect())
}

/// Store a cross-chain identity
pub fn store_cross_chain_identity(identity: CrossChainIdentity) -> Result<(), FusionError> {
    CROSS_CHAIN_IDENTITIES.with(|identities| {
        identities.borrow_mut().insert(identity.eth_address.clone(), identity);
        Ok(())
    })
}

/// Get a cross-chain identity by ETH address
pub fn get_cross_chain_identity(eth_address: &str) -> Result<CrossChainIdentity, FusionError> {
    CROSS_CHAIN_IDENTITIES.with(|identities| {
        identities.borrow().get(eth_address).cloned().ok_or(FusionError::OrderNotFound)
    })
}

/// Get all cross-chain identities
pub fn get_all_cross_chain_identities() -> Vec<CrossChainIdentity> {
    CROSS_CHAIN_IDENTITIES.with(|identities| identities.borrow().values().cloned().collect())
}

/// Clear all fusion data (for testing purposes)
#[cfg(test)]
pub fn clear_fusion_data() {
    FUSION_ORDERS.with(|orders| orders.borrow_mut().clear());
    CROSS_CHAIN_IDENTITIES.with(|identities| identities.borrow_mut().clear());
}
