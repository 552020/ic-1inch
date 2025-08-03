use crate::types::{FusionError, Order, OrderStatus};
use std::cell::RefCell;
use std::collections::HashMap;

// Global state using thread_local! for safety
thread_local! {
    static ORDERS: RefCell<HashMap<String, Order>> = RefCell::new(HashMap::new());
}

/// Store an order (create or update)
pub fn store_order(order: Order) -> Result<(), FusionError> {
    ORDERS.with(|orders| {
        orders.borrow_mut().insert(order.id.clone(), order);
        Ok(())
    })
}

/// Get an order by ID
pub fn get_order(order_id: &str) -> Result<Order, FusionError> {
    ORDERS.with(|orders| orders.borrow().get(order_id).cloned().ok_or(FusionError::OrderNotFound))
}

/// Get all active orders (Pending and Accepted status)
pub fn get_active_orders() -> Vec<Order> {
    ORDERS.with(|orders| {
        orders
            .borrow()
            .values()
            .filter(|order| matches!(order.status, OrderStatus::Pending | OrderStatus::Accepted))
            .cloned()
            .collect()
    })
}

/// Serialize the entire relayer state for upgrade
pub fn serialize_relayer_state() -> (Vec<(String, Order)>, Vec<(String, String)>) {
    let orders =
        ORDERS.with(|orders| orders.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect());

    // Return empty identities vector since we removed CrossChainIdentity
    let identities = vec![];

    (orders, identities)
}

/// Deserialize the relayer state after upgrade
pub fn deserialize_relayer_state(
    orders: Vec<(String, Order)>,
    _identities: Vec<(String, String)>, // Ignored since we removed CrossChainIdentity
) {
    ORDERS.with(|order_map| {
        let mut map = order_map.borrow_mut();
        map.clear();
        for (id, order) in orders {
            map.insert(id, order);
        }
    });
}


