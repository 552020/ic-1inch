use crate::types::{CrossChainIdentity, FusionError, Order, OrderStatus};
use candid::Principal;
use std::cell::RefCell;
use std::collections::HashMap;

// Global state using thread_local! for safety
thread_local! {
    static ORDERS: RefCell<HashMap<String, Order>> = RefCell::new(HashMap::new());
    static CROSS_CHAIN_IDENTITIES: RefCell<HashMap<String, CrossChainIdentity>> = RefCell::new(HashMap::new());
}

/// Store an order (create or update)
pub fn store_order(order: Order) -> Result<(), FusionError> {
    ORDERS.with(|orders| {
        orders.borrow_mut().insert(order.id.clone(), order);
        Ok(())
    })
}

/// Update an existing order
pub fn update_order(order: Order) -> Result<(), FusionError> {
    ORDERS.with(|orders| {
        let mut orders_map = orders.borrow_mut();
        if orders_map.contains_key(&order.id) {
            orders_map.insert(order.id.clone(), order);
            Ok(())
        } else {
            Err(FusionError::OrderNotFound)
        }
    })
}

/// Get an order by ID
pub fn get_order(order_id: &str) -> Result<Order, FusionError> {
    ORDERS.with(|orders| orders.borrow().get(order_id).cloned().ok_or(FusionError::OrderNotFound))
}

/// Get all orders
pub fn get_all_orders() -> Vec<Order> {
    ORDERS.with(|orders| orders.borrow().values().cloned().collect())
}

/// Get orders by status (for queries)
pub fn get_orders_by_status(status: OrderStatus) -> Vec<Order> {
    ORDERS.with(|orders| {
        orders.borrow().values().filter(|order| order.status == status).cloned().collect()
    })
}

/// Get orders by maker principal (for queries)
pub fn get_orders_by_maker(maker_principal: Principal) -> Vec<Order> {
    ORDERS.with(|orders| {
        orders
            .borrow()
            .values()
            .filter(|order| order.maker_icp_principal == maker_principal)
            .cloned()
            .collect()
    })
}

/// Get active orders (pending and accepted)
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

/// Get cross-chain identity by ICP principal (reverse lookup)
pub fn get_cross_chain_identity_by_principal(principal: Principal) -> Option<CrossChainIdentity> {
    CROSS_CHAIN_IDENTITIES.with(|identities| {
        identities.borrow().values().find(|identity| identity.icp_principal == principal).cloned()
    })
}

/// Get all cross-chain identities
pub fn get_all_cross_chain_identities() -> Vec<CrossChainIdentity> {
    CROSS_CHAIN_IDENTITIES.with(|identities| identities.borrow().values().cloned().collect())
}

/// Serialize relayer state for canister upgrades
pub fn serialize_relayer_state() -> (Vec<(String, Order)>, Vec<(String, CrossChainIdentity)>) {
    let orders =
        ORDERS.with(|orders| orders.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect());
    let identities = CROSS_CHAIN_IDENTITIES.with(|identities| {
        identities.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    });
    (orders, identities)
}

/// Deserialize relayer state after canister upgrades
pub fn deserialize_relayer_state(
    orders: Vec<(String, Order)>,
    identities: Vec<(String, CrossChainIdentity)>,
) {
    ORDERS.with(|order_map| {
        order_map.borrow_mut().clear();
        for (id, order) in orders {
            order_map.borrow_mut().insert(id, order);
        }
    });

    CROSS_CHAIN_IDENTITIES.with(|identity_map| {
        identity_map.borrow_mut().clear();
        for (eth_address, identity) in identities {
            identity_map.borrow_mut().insert(eth_address, identity);
        }
    });
}

/// Clear all order data (for testing purposes)
#[cfg(test)]
pub fn clear_order_data() {
    ORDERS.with(|orders| orders.borrow_mut().clear());
    CROSS_CHAIN_IDENTITIES.with(|identities| identities.borrow_mut().clear());
}
