use crate::types::{
    CoordinationState, CrossChainEscrow, CrossChainEscrowEvent, EscrowError, EscrowStatus,
    HTLCEscrow,
};
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

/// Update an existing HTLC escrow
pub fn update_htlc_escrow(order_hash: &str, updated_escrow: HTLCEscrow) -> Result<(), EscrowError> {
    HTLC_ESCROWS.with(|escrows| {
        let mut escrows_map = escrows.borrow_mut();
        if escrows_map.contains_key(order_hash) {
            escrows_map.insert(order_hash.to_string(), updated_escrow);
            Ok(())
        } else {
            Err(EscrowError::EscrowNotFound)
        }
    })
}

/// Update HTLC escrow status
pub fn update_htlc_escrow_status(
    order_hash: &str,
    new_status: EscrowStatus,
) -> Result<(), EscrowError> {
    HTLC_ESCROWS.with(|escrows| {
        let mut escrows_map = escrows.borrow_mut();
        if let Some(escrow) = escrows_map.get_mut(order_hash) {
            escrow.status = new_status;
            escrow.updated_at = ic_cdk::api::time();
            Ok(())
        } else {
            Err(EscrowError::EscrowNotFound)
        }
    })
}

/// Add event to HTLC escrow
pub fn add_event_to_htlc_escrow(
    order_hash: &str,
    event: CrossChainEscrowEvent,
) -> Result<(), EscrowError> {
    HTLC_ESCROWS.with(|escrows| {
        let mut escrows_map = escrows.borrow_mut();
        if let Some(escrow) = escrows_map.get_mut(order_hash) {
            escrow.events.push(event);
            escrow.updated_at = ic_cdk::api::time();
            Ok(())
        } else {
            Err(EscrowError::EscrowNotFound)
        }
    })
}

/// Get an HTLC escrow by order hash
pub fn get_htlc_escrow(order_hash: &str) -> Result<HTLCEscrow, EscrowError> {
    HTLC_ESCROWS.with(|escrows| {
        escrows.borrow().get(order_hash).cloned().ok_or(EscrowError::EscrowNotFound)
    })
}

/// Get all HTLC escrows
pub fn get_all_htlc_escrows() -> Vec<HTLCEscrow> {
    HTLC_ESCROWS.with(|escrows| escrows.borrow().values().cloned().collect())
}

/// Get HTLC escrows by status
pub fn get_htlc_escrows_by_status(status: EscrowStatus) -> Vec<HTLCEscrow> {
    HTLC_ESCROWS.with(|escrows| {
        escrows.borrow().values().filter(|escrow| escrow.status == status).cloned().collect()
    })
}

/// Check if HTLC escrow exists
pub fn htlc_escrow_exists(order_hash: &str) -> bool {
    HTLC_ESCROWS.with(|escrows| escrows.borrow().contains_key(order_hash))
}

/// Store a cross-chain escrow
pub fn store_cross_chain_escrow(escrow: CrossChainEscrow) -> Result<(), EscrowError> {
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        escrows.borrow_mut().insert(escrow.order_id.clone(), escrow);
        Ok(())
    })
}

/// Update an existing cross-chain escrow
pub fn update_cross_chain_escrow(
    order_id: &str,
    updated_escrow: CrossChainEscrow,
) -> Result<(), EscrowError> {
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        let mut escrows_map = escrows.borrow_mut();
        if escrows_map.contains_key(order_id) {
            escrows_map.insert(order_id.to_string(), updated_escrow);
            Ok(())
        } else {
            Err(EscrowError::EscrowNotFound)
        }
    })
}

/// Update cross-chain escrow coordination state
pub fn update_cross_chain_coordination_state(
    order_id: &str,
    new_state: CoordinationState,
) -> Result<(), EscrowError> {
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        let mut escrows_map = escrows.borrow_mut();
        if let Some(escrow) = escrows_map.get_mut(order_id) {
            escrow.coordination_state = new_state;
            escrow.updated_at = ic_cdk::api::time();
            Ok(())
        } else {
            Err(EscrowError::EscrowNotFound)
        }
    })
}

/// Add event to cross-chain escrow
pub fn add_event_to_cross_chain_escrow(
    order_id: &str,
    event: CrossChainEscrowEvent,
) -> Result<(), EscrowError> {
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        let mut escrows_map = escrows.borrow_mut();
        if let Some(escrow) = escrows_map.get_mut(order_id) {
            escrow.events.push(event);
            escrow.updated_at = ic_cdk::api::time();
            Ok(())
        } else {
            Err(EscrowError::EscrowNotFound)
        }
    })
}

/// Get a cross-chain escrow by order ID
pub fn get_cross_chain_escrow(order_id: &str) -> Result<CrossChainEscrow, EscrowError> {
    CROSS_CHAIN_ESCROWS
        .with(|escrows| escrows.borrow().get(order_id).cloned().ok_or(EscrowError::EscrowNotFound))
}

/// Get all cross-chain escrows
pub fn get_all_cross_chain_escrows() -> Vec<CrossChainEscrow> {
    CROSS_CHAIN_ESCROWS.with(|escrows| escrows.borrow().values().cloned().collect())
}

/// Get cross-chain escrows by coordination state
pub fn get_cross_chain_escrows_by_state(state: CoordinationState) -> Vec<CrossChainEscrow> {
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        escrows
            .borrow()
            .values()
            .filter(|escrow| escrow.coordination_state == state)
            .cloned()
            .collect()
    })
}

/// Check if cross-chain escrow exists
pub fn cross_chain_escrow_exists(order_id: &str) -> bool {
    CROSS_CHAIN_ESCROWS.with(|escrows| escrows.borrow().contains_key(order_id))
}

/// Get memory statistics for monitoring
pub fn get_memory_stats() -> MemoryStats {
    let htlc_count = HTLC_ESCROWS.with(|escrows| escrows.borrow().len());
    let cross_chain_count = CROSS_CHAIN_ESCROWS.with(|escrows| escrows.borrow().len());

    MemoryStats {
        htlc_escrows_count: htlc_count,
        cross_chain_escrows_count: cross_chain_count,
        total_escrows: htlc_count + cross_chain_count,
    }
}

/// Memory statistics structure
#[derive(Clone, Debug)]
pub struct MemoryStats {
    pub htlc_escrows_count: usize,
    pub cross_chain_escrows_count: usize,
    pub total_escrows: usize,
}

/// Canister upgrade support - export data for backup
pub fn export_escrow_data() -> EscrowBackup {
    let htlc_escrows = get_all_htlc_escrows();
    let cross_chain_escrows = get_all_cross_chain_escrows();

    EscrowBackup { htlc_escrows, cross_chain_escrows, exported_at: ic_cdk::api::time() }
}

/// Canister upgrade support - import data from backup
pub fn import_escrow_data(backup: EscrowBackup) -> Result<(), EscrowError> {
    // Clear existing data
    clear_escrow_data();

    // Import HTLC escrows
    HTLC_ESCROWS.with(|escrows| {
        let mut escrows_map = escrows.borrow_mut();
        for escrow in backup.htlc_escrows {
            escrows_map.insert(escrow.order_hash.clone(), escrow);
        }
    });

    // Import cross-chain escrows
    CROSS_CHAIN_ESCROWS.with(|escrows| {
        let mut escrows_map = escrows.borrow_mut();
        for escrow in backup.cross_chain_escrows {
            escrows_map.insert(escrow.order_id.clone(), escrow);
        }
    });

    Ok(())
}

/// Backup structure for canister upgrades
#[derive(Clone, Debug)]
pub struct EscrowBackup {
    pub htlc_escrows: Vec<HTLCEscrow>,
    pub cross_chain_escrows: Vec<CrossChainEscrow>,
    pub exported_at: u64,
}

/// Clear all escrow data (for testing purposes)
#[cfg(test)]
pub fn clear_escrow_data() {
    HTLC_ESCROWS.with(|escrows| escrows.borrow_mut().clear());
    CROSS_CHAIN_ESCROWS.with(|escrows| escrows.borrow_mut().clear());
}

/// Clear all escrow data (for production use during upgrades)
#[cfg(not(test))]
pub fn clear_escrow_data() {
    HTLC_ESCROWS.with(|escrows| escrows.borrow_mut().clear());
    CROSS_CHAIN_ESCROWS.with(|escrows| escrows.borrow_mut().clear());
}
