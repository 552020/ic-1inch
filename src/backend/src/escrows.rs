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
