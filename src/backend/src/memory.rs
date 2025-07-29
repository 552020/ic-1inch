use crate::types::{
    DestinationEscrow, Escrow, Order, OrderId, ProtocolFees, SourceEscrow, SystemStats, TakerInfo,
    TakerWhitelist,
};
use candid::Principal;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

// Global state using thread_local! for safety
thread_local! {
    // Existing escrow storage
    static ESCROWS: RefCell<HashMap<String, Escrow>> = RefCell::new(HashMap::new());
    static SOURCE_ESCROWS: RefCell<HashMap<String, SourceEscrow>> = RefCell::new(HashMap::new());
    static DESTINATION_ESCROWS: RefCell<HashMap<String, DestinationEscrow>> = RefCell::new(HashMap::new());

    // Limit order storage
    static ORDERS: RefCell<HashMap<OrderId, Order>> = RefCell::new(HashMap::new());
    static FILLED_ORDERS: RefCell<HashSet<OrderId>> = RefCell::new(HashSet::new());
    static CANCELLED_ORDERS: RefCell<HashSet<OrderId>> = RefCell::new(HashSet::new());
    static ORDER_COUNTER: RefCell<u64> = RefCell::new(0);
    static SYSTEM_STATS: RefCell<SystemStats> = RefCell::new(SystemStats::default());

    // Taker whitelist storage
    static TAKER_WHITELIST: RefCell<TakerWhitelist> = RefCell::new(TakerWhitelist {
        whitelisted_takers: vec![],
    });

    // Taker registry storage (MVP - non-enforced)
    static TAKER_REGISTRY: RefCell<HashMap<Principal, TakerInfo>> = RefCell::new(HashMap::new());
    static PROTOCOL_FEES: RefCell<ProtocolFees> = RefCell::new(ProtocolFees {
        flat_fee_icp: 1000000,     // 0.001 ICP (8 decimals)
        percentage_fee_bps: 10,     // 0.1%
        min_balance_threshold: 10000000, // 0.01 ICP
    });
}

// Safe access to escrows (legacy support)
pub fn with_escrows<T>(f: impl FnOnce(&mut HashMap<String, Escrow>) -> T) -> T {
    ESCROWS.with(|escrows| f(&mut escrows.borrow_mut()))
}

// Safe access to source escrows
pub fn with_source_escrows<T>(f: impl FnOnce(&mut HashMap<String, SourceEscrow>) -> T) -> T {
    SOURCE_ESCROWS.with(|escrows| f(&mut escrows.borrow_mut()))
}

// Safe access to destination escrows
pub fn with_destination_escrows<T>(
    f: impl FnOnce(&mut HashMap<String, DestinationEscrow>) -> T,
) -> T {
    DESTINATION_ESCROWS.with(|escrows| f(&mut escrows.borrow_mut()))
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

/// Safe access to taker whitelist
pub fn with_taker_whitelist<T>(f: impl FnOnce(&mut TakerWhitelist) -> T) -> T {
    TAKER_WHITELIST.with(|whitelist| f(&mut whitelist.borrow_mut()))
}

/// Safe read-only access to taker whitelist
pub fn with_taker_whitelist_read<T>(f: impl FnOnce(&TakerWhitelist) -> T) -> T {
    TAKER_WHITELIST.with(|whitelist| f(&whitelist.borrow()))
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
        with_filled_orders_read(|filled| {
            with_cancelled_orders_read(|cancelled| {
                let current_time = ic_cdk::api::time();
                orders
                    .values()
                    .filter(|order| {
                        !filled.contains(&order.id)
                            && !cancelled.contains(&order.id)
                            && order.expiration > current_time
                    })
                    .cloned()
                    .collect()
            })
        })
    })
}

/// Check if an order exists and is active
pub fn is_order_active(order_id: OrderId) -> bool {
    with_orders_read(|orders| {
        if let Some(order) = orders.get(&order_id) {
            with_filled_orders_read(|filled| {
                with_cancelled_orders_read(|cancelled| order.is_active(filled, cancelled))
            })
        } else {
            false
        }
    })
}

/// Get order by ID
pub fn get_order(order_id: OrderId) -> Option<Order> {
    with_orders_read(|orders| orders.get(&order_id).cloned())
}

/// Mark order as filled
pub fn mark_order_filled(order_id: OrderId) {
    with_filled_orders(|filled| {
        filled.insert(order_id);
    });
}

/// Mark order as cancelled
pub fn mark_order_cancelled(order_id: OrderId) {
    with_cancelled_orders(|cancelled| {
        cancelled.insert(order_id);
    });
}

/// Update system statistics for order creation
pub fn track_order_created() {
    with_system_stats(|stats| {
        stats.increment_orders_created();
    });
}

/// Update system statistics for order filled
pub fn track_order_filled(token: candid::Principal, volume: u64) {
    with_system_stats(|stats| {
        stats.increment_orders_filled(token, volume);
    });
}

/// Update system statistics for order cancelled
pub fn track_order_cancelled() {
    with_system_stats(|stats| {
        stats.increment_orders_cancelled();
    });
}

/// Track error occurrence
pub fn track_error(error_type: &str) {
    with_system_stats(|stats| {
        stats.track_error(error_type);
    });
}

// ============================================================================
// STABLE MEMORY INTEGRATION FOR CANISTER UPGRADES
// ============================================================================

/// Serialize limit order state for canister upgrades
pub fn serialize_limit_order_state(
) -> (Vec<(OrderId, Order)>, Vec<OrderId>, Vec<OrderId>, u64, SystemStats) {
    let orders = with_orders_read(|orders| orders.iter().map(|(k, v)| (*k, v.clone())).collect());
    let filled = with_filled_orders_read(|filled| filled.iter().cloned().collect());
    let cancelled = with_cancelled_orders_read(|cancelled| cancelled.iter().cloned().collect());
    let counter = with_order_counter_read(|counter| *counter);
    let stats = with_system_stats_read(|stats| stats.clone());

    (orders, filled, cancelled, counter, stats)
}

/// Deserialize limit order state after canister upgrades
pub fn deserialize_limit_order_state(
    orders: Vec<(OrderId, Order)>,
    filled: Vec<OrderId>,
    cancelled: Vec<OrderId>,
    counter: u64,
    stats: SystemStats,
) {
    with_orders(|order_map| {
        order_map.clear();
        for (id, order) in orders {
            order_map.insert(id, order);
        }
    });

    with_filled_orders(|filled_set| {
        filled_set.clear();
        for id in filled {
            filled_set.insert(id);
        }
    });

    with_cancelled_orders(|cancelled_set| {
        cancelled_set.clear();
        for id in cancelled {
            cancelled_set.insert(id);
        }
    });

    with_order_counter(|order_counter| {
        *order_counter = counter;
    });

    with_system_stats(|system_stats| {
        *system_stats = stats;
    });
}

/// Clear all limit order data (for testing purposes)
#[cfg(test)]
pub fn clear_limit_order_data() {
    with_orders(|orders| orders.clear());
    with_filled_orders(|filled| filled.clear());
    with_cancelled_orders(|cancelled| cancelled.clear());
    with_order_counter(|counter| *counter = 0);
    with_system_stats(|stats| *stats = SystemStats::default());
}

// ============================================================================
// TAKER REGISTRY ACCESS FUNCTIONS (MVP - non-enforced)
// ============================================================================

/// Safe access to taker registry
pub fn with_taker_registry<T>(f: impl FnOnce(&mut HashMap<Principal, TakerInfo>) -> T) -> T {
    TAKER_REGISTRY.with(|registry| f(&mut registry.borrow_mut()))
}

/// Safe read-only access to taker registry
pub fn with_taker_registry_read<T>(f: impl FnOnce(&HashMap<Principal, TakerInfo>) -> T) -> T {
    TAKER_REGISTRY.with(|registry| f(&registry.borrow()))
}

/// Safe access to protocol fees
pub fn with_protocol_fees<T>(f: impl FnOnce(&mut ProtocolFees) -> T) -> T {
    PROTOCOL_FEES.with(|fees| f(&mut fees.borrow_mut()))
}

/// Safe read-only access to protocol fees
pub fn with_protocol_fees_read<T>(f: impl FnOnce(&ProtocolFees) -> T) -> T {
    PROTOCOL_FEES.with(|fees| f(&fees.borrow()))
}
