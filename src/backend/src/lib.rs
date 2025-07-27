mod escrows;
mod memory;
mod types;

use escrows::{get_timelock_status, TimelockStatus};

// Keep the hello world function for testing
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Test function for timelock enforcement
#[ic_cdk::query]
fn test_timelock(timelock: u64) -> TimelockStatus {
    get_timelock_status(timelock)
}

ic_cdk::export_candid!();
