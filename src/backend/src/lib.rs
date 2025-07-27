mod escrows;
mod memory;
mod types;

use types::{CreateEscrowParams, Escrow, EscrowState};

// Keep the hello world function for testing
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

ic_cdk::export_candid!();
