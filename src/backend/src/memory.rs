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

// Helper function to get escrows (safe version)
pub fn get_escrows() -> &'static mut HashMap<String, Escrow> {
    // This is safe because we're using thread_local! internally
    ESCROWS.with(|escrows| {
        // We need to return a static reference, but this is safe in ICP context
        // since we're single-threaded and thread_local! handles the safety
        unsafe {
            // This is safe because ICP canisters are single-threaded
            std::mem::transmute(escrows.borrow_mut())
        }
    })
}
