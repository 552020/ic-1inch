use crate::types::{Order, OrderId, SystemStats};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

// Global state using thread_local! for safety
thread_local! {
    // Limit order storage
    static ORDERS: RefCell<HashMap<OrderId, Order>> = RefCell::new(HashMap::new());
    static FILLED_ORDERS: RefCell<HashSet<OrderId>> = RefCell::new(HashSet::new());
    static CANCELLED_ORDERS: RefCell<HashSet<OrderId>> = RefCell::new(HashSet::new());
    static ORDER_COUNTER: RefCell<u64> = RefCell::new(0);
    static SYSTEM_STATS: RefCell<SystemStats> = RefCell::new(SystemStats::default());
}

// ============================================================================
// LIMIT ORDER STORAGE ACCESS FUNCTIONS
// ============================================================================

/// Safe access to orders storage
pub fn with_orders<T>(f: impl FnOnce(&mut HashMap<OrderId, Order>) -> T) -> T {
    ORDERS.with(|orders| f(&mut orders.borrow_mut()))
}

/// Safe read-only access to orders storage
pub fn with_orders_read<T>(f: impl FnOnce(&HashMap<OrderId, Order>) -> T) -> T {
    ORDERS.with(|orders| f(&orders.borrow()))
}

/// Safe access to filled orders tracking
pub fn with_filled_orders<T>(f: impl FnOnce(&mut HashSet<OrderId>) -> T) -> T {
    FILLED_ORDERS.with(|filled| f(&mut filled.borrow_mut()))
}

/// Safe read-only access to filled orders tracking
pub fn with_filled_orders_read<T>(f: impl FnOnce(&HashSet<OrderId>) -> T) -> T {
    FILLED_ORDERS.with(|filled| f(&filled.borrow()))
}

/// Safe access to cancelled orders tracking
pub fn with_cancelled_orders<T>(f: impl FnOnce(&mut HashSet<OrderId>) -> T) -> T {
    CANCELLED_ORDERS.with(|cancelled| f(&mut cancelled.borrow_mut()))
}

/// Safe read-only access to cancelled orders tracking
pub fn with_cancelled_orders_read<T>(f: impl FnOnce(&HashSet<OrderId>) -> T) -> T {
    CANCELLED_ORDERS.with(|cancelled| f(&cancelled.borrow()))
}

/// Safe access to order counter
pub fn with_order_counter<T>(f: impl FnOnce(&mut u64) -> T) -> T {
    ORDER_COUNTER.with(|counter| f(&mut counter.borrow_mut()))
}

/// Safe read-only access to order counter
pub fn with_order_counter_read<T>(f: impl FnOnce(&u64) -> T) -> T {
    ORDER_COUNTER.with(|counter| f(&counter.borrow()))
}

/// Safe access to system statistics
pub fn with_system_stats<T>(f: impl FnOnce(&mut SystemStats) -> T) -> T {
    SYSTEM_STATS.with(|stats| f(&mut stats.borrow_mut()))
}

/// Safe read-only access to system statistics
pub fn with_system_stats_read<T>(f: impl FnOnce(&SystemStats) -> T) -> T {
    SYSTEM_STATS.with(|stats| f(&stats.borrow()))
}

/// Generate next unique order ID
pub fn generate_order_id() -> OrderId {
    with_order_counter(|counter| {
        *counter += 1;
        *counter
    })
}

/// Get all active orders (not filled, cancelled, or expired)
pub fn get_active_orders() -> Vec<Order> {
    with_orders_read(|orders| {
        let current_time = ic_cdk::api::time();
        orders
            .values()
            .filter(|order| {
                order.expiration > current_time
                    && !with_filled_orders_read(|filled| filled.contains(&order.id))
                    && !with_cancelled_orders_read(|cancelled| cancelled.contains(&order.id))
            })
            .cloned()
            .collect()
    })
}

/// Check if an order is active (not filled, cancelled, or expired)
pub fn is_order_active(order_id: OrderId) -> bool {
    with_orders_read(|orders| {
        if let Some(order) = orders.get(&order_id) {
            let current_time = ic_cdk::api::time();
            order.expiration > current_time
                && !with_filled_orders_read(|filled| filled.contains(&order_id))
                && !with_cancelled_orders_read(|cancelled| cancelled.contains(&order_id))
        } else {
            false
        }
    })
}

/// Get a specific order by ID
pub fn get_order(order_id: OrderId) -> Option<Order> {
    with_orders_read(|orders| orders.get(&order_id).cloned())
}

/// Mark an order as filled
pub fn mark_order_filled(order_id: OrderId) {
    with_filled_orders(|filled| {
        filled.insert(order_id);
    });
}

/// Mark an order as cancelled
pub fn mark_order_cancelled(order_id: OrderId) {
    with_cancelled_orders(|cancelled| {
        cancelled.insert(order_id);
    });
}

/// Track order creation in statistics
pub fn track_order_created() {
    with_system_stats(|stats| {
        stats.increment_orders_created();
    });
}

/// Track order filling in statistics
pub fn track_order_filled(token: candid::Principal, volume: u64) {
    with_system_stats(|stats| {
        stats.increment_orders_filled(token, volume);
    });
}

/// Track order cancellation in statistics
pub fn track_order_cancelled() {
    with_system_stats(|stats| {
        stats.increment_orders_cancelled();
    });
}

/// Track error occurrence in statistics
pub fn track_error(error_type: &str) {
    with_system_stats(|stats| {
        stats.track_error(error_type);
    });
}

// ============================================================================
// CANISTER UPGRADE SUPPORT
// ============================================================================

/// Serialize limit order state for canister upgrade
pub fn serialize_limit_order_state(
) -> (Vec<(OrderId, Order)>, Vec<OrderId>, Vec<OrderId>, u64, SystemStats) {
    let orders = with_orders_read(|orders| orders.clone());
    let filled = with_filled_orders_read(|filled| filled.iter().cloned().collect());
    let cancelled = with_cancelled_orders_read(|cancelled| cancelled.iter().cloned().collect());
    let counter = with_order_counter_read(|counter| *counter);
    let stats = with_system_stats_read(|stats| stats.clone());

    (orders.into_iter().collect(), filled, cancelled, counter, stats)
}

/// Deserialize limit order state after canister upgrade
pub fn deserialize_limit_order_state(
    orders: Vec<(OrderId, Order)>,
    filled: Vec<OrderId>,
    cancelled: Vec<OrderId>,
    counter: u64,
    stats: SystemStats,
) {
    // Restore orders
    with_orders(|orders_map| {
        orders_map.clear();
        for (order_id, order) in orders {
            orders_map.insert(order_id, order);
        }
    });

    // Restore filled orders
    with_filled_orders(|filled_set| {
        filled_set.clear();
        for order_id in filled {
            filled_set.insert(order_id);
        }
    });

    // Restore cancelled orders
    with_cancelled_orders(|cancelled_set| {
        cancelled_set.clear();
        for order_id in cancelled {
            cancelled_set.insert(order_id);
        }
    });

    // Restore counter
    with_order_counter(|counter_ref| {
        *counter_ref = counter;
    });

    // Restore statistics
    with_system_stats(|stats_ref| {
        *stats_ref = stats;
    });
}

/// Clear all limit order data (for testing)
pub fn clear_limit_order_data() {
    with_orders(|orders| orders.clear());
    with_filled_orders(|filled| filled.clear());
    with_cancelled_orders(|cancelled| cancelled.clear());
    with_order_counter(|counter| *counter = 0);
    with_system_stats(|stats| *stats = SystemStats::default());
}
