use crate::types::{HTLCEscrow, CrossChainEscrow, EscrowError};
use std::cell::RefCell;
use std::collections::HashMap;

// Global state using thread_local! for safety
thread_local! {
    static HTLC_ESCROWS: RefCell<HashMap<String, HTLCEscrow>> = RefCell::new(HashMap::new());
    static CROSS_CHAIN_ESCROWS: RefCell<HashMap<String, CrossChainEscrow>> = RefCell::new(HashMap::new());
}

/// Store an HTLC escrow
pub fn store_htlc_escrow(escrow: HTLCEscrow) -> Result<(), EscrowError> {
    HTLC_ESCROWS.with(|escrows| {
        escrows.borrow_mut().insert(escrow.order_hash.clone(), escrow);
        Ok(())
    })
}

/// Get an HTLC escrow by order hash
pub fn get_htlc_escrow(order_hash: &str) -> Result<HTLCEscrow, EscrowError> {
    HTLC_ESCROWS.with(|escrows| {
        escrows.borrow()
            .get(order_hash)
            .cloned()
            .ok_or(EscrowError::EscrowNotFound)
    })
}

/// Get all HTLC escrows
pub fn get_all_htlc_escrows() -> Vec<HTLCEscrow> {
    HTLC_ESCROWS.with(|escrows| {
        escrows.borrow().values().cloned().collect()
    })
}

/// Store a cross-chain escrow
pub fn store_cross_chain_escrow(escrow: CrossChainEscrow) -> Result<(), EscrowError> {
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        escrows.borrow_mut().insert(escrow.order_id.clone(), escrow);
        Ok(())
    })
}

/// Get a cross-chain escrow by order ID
pub fn get_cross_chain_escrow(order_id: &str) -> Result<CrossChainEscrow, EscrowError> {
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        escrows.borrow()
            .get(order_id)
            .cloned()
            .ok_or(EscrowError::EscrowNotFound)
    })
}

/// Get all cross-chain escrows
pub fn get_all_cross_chain_escrows() -> Vec<CrossChainEscrow> {
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        escrows.borrow().values().cloned().collect()
    })
}

/// Clear all escrow data (for testing purposes)
#[cfg(test)]
pub fn clear_escrow_data() {
    HTLC_ESCROWS.with(|escrows| escrows.borrow_mut().clear());
    CROSS_CHAIN_ESCROWS.with(|escrows| escrows.borrow_mut().clear());
}