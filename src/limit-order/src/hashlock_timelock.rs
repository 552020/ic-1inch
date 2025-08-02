use crate::types::{HashlockInfo, OrderError, OrderId, OrderResult, TimelockInfo};
use candid::{CandidType, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;

// ============================================================================
// HASHLOCK & TIMELOCK IMPLEMENTATION - Cross-Chain Functionality
// ============================================================================

// Global state for hashlock and timelock storage
thread_local! {
    static HASHLOCK_STORE: RefCell<HashMap<Vec<u8>, HashlockInfo>> = RefCell::new(HashMap::new());
    static TIMELOCK_STORE: RefCell<HashMap<OrderId, TimelockInfo>> = RefCell::new(HashMap::new());
    static CROSS_CHAIN_ORDERS: RefCell<HashMap<OrderId, CrossChainOrder>> = RefCell::new(HashMap::new());
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CrossChainOrder {
    pub order_id: OrderId,
    pub hashlock: Vec<u8>,
    pub timelock: u64,
    pub target_chain: String,
    pub escrow_address: Option<String>,
    pub status: CrossChainStatus,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum CrossChainStatus {
    Pending,       // Order created, waiting for cross-chain coordination
    EscrowCreated, // EVM escrow created
    Filled,        // Cross-chain swap completed
    Expired,       // Timelock expired
    Failed,        // Cross-chain swap failed
}

/// Hashlock utilities for cross-chain atomic swaps
pub struct HashlockManager;

impl HashlockManager {
    /// Generate a hashlock from a secret (preimage)
    pub fn create_hashlock(preimage: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(preimage);
        hasher.finalize().to_vec()
    }

    /// Verify a hashlock against a preimage
    pub fn verify_hashlock(hashlock: &[u8], preimage: &[u8]) -> bool {
        let computed_hashlock = Self::create_hashlock(preimage);
        hashlock == computed_hashlock.as_slice()
    }

    /// Store hashlock information
    pub fn store_hashlock(order_id: OrderId, hashlock: Vec<u8>) -> OrderResult<()> {
        HASHLOCK_STORE.with(|store| {
            let mut store = store.borrow_mut();
            let info = HashlockInfo {
                order_id,
                hashlock: hashlock.clone(),
                preimage: None,
                created_at: ic_cdk::api::time(),
                revealed_at: None,
            };
            store.insert(hashlock, info);
        });
        Ok(())
    }

    /// Reveal preimage to complete cross-chain swap
    pub fn reveal_preimage(order_id: OrderId, preimage: Vec<u8>) -> OrderResult<()> {
        // Find hashlock for this order
        let hashlock = HASHLOCK_STORE
            .with(|store| {
                let store = store.borrow();
                for (h, info) in store.iter() {
                    if info.order_id == order_id {
                        return Some(h.clone());
                    }
                }
                None
            })
            .ok_or(OrderError::OrderNotFound)?;

        // Verify preimage
        if !Self::verify_hashlock(&hashlock, &preimage) {
            return Err(OrderError::InvalidHashlock);
        }

        // Update hashlock info
        HASHLOCK_STORE.with(|store| {
            let mut store = store.borrow_mut();
            if let Some(info) = store.get_mut(&hashlock) {
                info.preimage = Some(preimage);
                info.revealed_at = Some(ic_cdk::api::time());
            }
        });

        Ok(())
    }

    /// Get hashlock information
    pub fn get_hashlock_info(hashlock: &[u8]) -> Option<HashlockInfo> {
        HASHLOCK_STORE.with(|store| {
            let store = store.borrow();
            store.get(hashlock).cloned()
        })
    }
}

/// Timelock utilities for cross-chain coordination
pub struct TimelockManager;

impl TimelockManager {
    /// Calculate conservative timelock for cross-chain swaps
    pub fn calculate_timelock(target_chain: &str) -> u64 {
        let base_time = 3600_000_000_000; // 1 hour in nanoseconds
        let current_time = ic_cdk::api::time();

        // Add buffer for network delays and finality
        let buffer = match target_chain {
            "ethereum" => 180_000_000_000, // 3 minutes for EVM finality
            "polygon" => 120_000_000_000,  // 2 minutes for Polygon
            _ => 300_000_000_000,          // 5 minutes default
        };

        current_time + base_time + buffer
    }

    /// Store timelock information
    pub fn store_timelock(order_id: OrderId, timelock: u64) -> OrderResult<()> {
        TIMELOCK_STORE.with(|store| {
            let mut store = store.borrow_mut();
            let info = TimelockInfo {
                order_id,
                timelock,
                created_at: ic_cdk::api::time(),
                expires_at: timelock,
            };
            store.insert(order_id, info);
        });
        Ok(())
    }

    /// Check if timelock has expired
    pub fn is_timelock_expired(order_id: OrderId) -> bool {
        TIMELOCK_STORE.with(|store| {
            let store = store.borrow();
            if let Some(info) = store.get(&order_id) {
                return ic_cdk::api::time() > info.expires_at;
            }
            true // Consider expired if not found
        })
    }

    /// Get remaining time for timelock
    pub fn get_remaining_time(order_id: OrderId) -> Option<u64> {
        TIMELOCK_STORE.with(|store| {
            let store = store.borrow();
            if let Some(info) = store.get(&order_id) {
                let current_time = ic_cdk::api::time();
                if current_time < info.expires_at {
                    return Some(info.expires_at - current_time);
                }
            }
            None
        })
    }

    /// Get timelock information
    pub fn get_timelock_info(order_id: OrderId) -> Option<TimelockInfo> {
        TIMELOCK_STORE.with(|store| {
            let store = store.borrow();
            store.get(&order_id).cloned()
        })
    }
}

/// Cross-chain order management
pub struct CrossChainManager;

impl CrossChainManager {
    /// Store cross-chain order information
    pub fn store_cross_chain_order(
        order_id: OrderId,
        hashlock: Vec<u8>,
        timelock: u64,
        target_chain: String,
    ) -> OrderResult<()> {
        CROSS_CHAIN_ORDERS.with(|store| {
            let mut store = store.borrow_mut();
            let order = CrossChainOrder {
                order_id,
                hashlock,
                timelock,
                target_chain,
                escrow_address: None,
                status: CrossChainStatus::Pending,
            };
            store.insert(order_id, order);
        });
        Ok(())
    }

    /// Get cross-chain order information
    pub fn get_cross_chain_order(order_id: OrderId) -> Option<CrossChainOrder> {
        CROSS_CHAIN_ORDERS.with(|store| {
            let store = store.borrow();
            store.get(&order_id).cloned()
        })
    }

    /// Update cross-chain order status
    pub fn update_cross_chain_status(
        order_id: OrderId,
        status: CrossChainStatus,
    ) -> OrderResult<()> {
        CROSS_CHAIN_ORDERS.with(|store| {
            let mut store = store.borrow_mut();
            if let Some(order) = store.get_mut(&order_id) {
                order.status = status;
                Ok(())
            } else {
                Err(OrderError::OrderNotFound)
            }
        })
    }

    /// Get all cross-chain orders
    pub fn get_all_cross_chain_orders() -> Vec<CrossChainOrder> {
        CROSS_CHAIN_ORDERS.with(|store| {
            let store = store.borrow();
            store.values().cloned().collect()
        })
    }
}
