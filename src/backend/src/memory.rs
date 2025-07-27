use crate::types::Escrow;
use std::cell::RefCell;
use std::collections::HashMap;

// Global state using thread_local! for safety
thread_local! {
    static ESCROWS: RefCell<HashMap<String, Escrow>> = RefCell::new(HashMap::new());
}

// Safe access to escrows
pub fn with_escrows<T>(f: impl FnOnce(&mut HashMap<String, Escrow>) -> T) -> T {
    ESCROWS.with(|escrows| f(&mut escrows.borrow_mut()))
}
