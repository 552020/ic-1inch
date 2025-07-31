use crate::types::{FusionError, FusionOrder};
use std::cell::RefCell;
use std::collections::HashMap;

// Global state using thread_local! for safety
thread_local! {
    static FUSION_ORDERS: RefCell<HashMap<String, FusionOrder>> = RefCell::new(HashMap::new());
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

/// Clear all fusion data (for testing purposes)
#[cfg(test)]
pub fn clear_fusion_data() {
    FUSION_ORDERS.with(|orders| orders.borrow_mut().clear());
}
