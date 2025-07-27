use candid::CandidType;
use sha2::{Digest, Sha256};

// TODO: Add unit tests and integration tests for hashlock verification
// TODO: Add test cases for valid/invalid secrets, edge cases, etc.

// This module will contain escrow business logic functions
// Re-export the safe memory functions
pub use crate::memory::with_escrows;

/// Verify that a preimage (secret) matches the expected hashlock
///
/// This is the core security mechanism of HTLC. The function:
/// 1. Takes a preimage (the secret revealed by the maker)
/// 2. Computes its SHA256 hash
/// 3. Compares it with the stored hashlock
/// 4. Returns true if they match, false otherwise
///
/// This function is used when the maker reveals their secret to claim tokens.
pub fn verify_hashlock(preimage: &[u8], expected_hashlock: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let computed_hash = hasher.finalize();
    computed_hash.as_slice() == expected_hashlock
}

/// Timelock status for HTLC escrows
#[derive(Debug, Clone, PartialEq, CandidType)]
pub enum TimelockStatus {
    Active,  // Timelock not expired
    Expired, // Timelock has expired
}

/// Get the current status of a timelock
///
/// This function compares the current ICP time with the timelock
/// to determine if the escrow has expired.
pub fn get_timelock_status(timelock: u64) -> TimelockStatus {
    let current_time = ic_cdk::api::time();

    if current_time < timelock {
        TimelockStatus::Active
    } else {
        TimelockStatus::Expired
    }
}

/// Validate that a timelock is in the future
///
/// This function ensures that escrows cannot be created with
/// timelocks that have already expired.
pub fn validate_timelock(timelock: u64) -> Result<(), crate::types::EscrowError> {
    let current_time = ic_cdk::api::time();

    if timelock <= current_time {
        return Err(crate::types::EscrowError::InvalidTimelock);
    }

    Ok(())
}
