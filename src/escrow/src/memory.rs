use crate::types::{FusionEscrow, EscrowError};
use std::cell::RefCell;
use std::collections::HashMap;

// Global state using thread_local! for safety
thread_local! {
    static FUSION_ESCROWS: RefCell<HashMap<String, FusionEscrow>> = RefCell::new(HashMap::new());
}

/// Store a fusion escrow
pub fn store_fusion_escrow(escrow: FusionEscrow) -> Result<(), EscrowError> {
    FUSION_ESCROWS.with(|escrows| {
        escrows.borrow_mut().insert(escrow.id.clone(), escrow);
        Ok(())
    })
}

/// Get a fusion escrow by ID
pub fn get_fusion_escrow(escrow_id: &str) -> Result<FusionEscrow, EscrowError> {
    FUSION_ESCROWS.with(|escrows| {
        escrows.borrow()
            .get(escrow_id)
            .cloned()
            .ok_or(EscrowError::EscrowNotFound)
    })
}

/// Get all fusion escrows
pub fn get_all_fusion_escrows() -> Vec<FusionEscrow> {
    FUSION_ESCROWS.with(|escrows| {
        escrows.borrow().values().cloned().collect()
    })
}

/// Clear all fusion data (for testing purposes)
#[cfg(test)]
pub fn clear_fusion_data() {
    FUSION_ESCROWS.with(|escrows| escrows.borrow_mut().clear());
}