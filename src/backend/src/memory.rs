use crate::types::{DestinationEscrow, Escrow, SourceEscrow};
use std::cell::RefCell;
use std::collections::HashMap;

// Global state using thread_local! for safety
thread_local! {
    static ESCROWS: RefCell<HashMap<String, Escrow>> = RefCell::new(HashMap::new());
    static SOURCE_ESCROWS: RefCell<HashMap<String, SourceEscrow>> = RefCell::new(HashMap::new());
    static DESTINATION_ESCROWS: RefCell<HashMap<String, DestinationEscrow>> = RefCell::new(HashMap::new());
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
