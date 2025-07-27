use sha2::{Sha256, Digest};
use candid::CandidType;
use ic_cdk::api::time;
use crate::types::{CreateEscrowParams, Escrow, EscrowState, EscrowError};

// TODO: Add integration tests for end-to-end escrow lifecycle

// This module will contain escrow business logic functions

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
    Active,     // Timelock not expired
    Expired,    // Timelock has expired
}

/// Get the current status of a timelock
/// 
/// This function compares the current ICP time with the timelock
/// to determine if the escrow has expired.
pub fn get_timelock_status(timelock: u64) -> TimelockStatus {
    let current_time = time();
    
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
pub fn validate_timelock(timelock: u64) -> Result<(), EscrowError> {
    let current_time = time();
    
    if timelock <= current_time {
        return Err(EscrowError::InvalidTimelock);
    }
    
    Ok(())
}

/// Create a new escrow with the specified parameters
/// 
/// State Transition: None → Created
pub async fn create_escrow(params: CreateEscrowParams) -> Result<String, EscrowError> {
    // Validate parameters
    if params.amount == 0 {
        return Err(EscrowError::InvalidAmount);
    }
    
    // Validate timelock is in the future
    validate_timelock(params.timelock)?;
    
    // Generate unique escrow ID
    let escrow_id = format!("escrow_{}", time());
    
    // Create escrow
    let escrow = Escrow {
        id: escrow_id.clone(),
        hashlock: params.hashlock,
        timelock: params.timelock,
        token_canister: params.token_canister,
        amount: params.amount,
        recipient: params.recipient,
        depositor: params.depositor,
        state: EscrowState::Created,
        created_at: time(),
        updated_at: time(),
    };
    
    // Store escrow
    crate::memory::with_escrows(|escrows| {
        escrows.insert(escrow_id.clone(), escrow);
    });
    
    Ok(escrow_id)
}

/// Deposit tokens into an existing escrow
/// 
/// State Transition: Created → Funded
pub async fn deposit_tokens(escrow_id: String, amount: u64) -> Result<(), EscrowError> {
    crate::memory::with_escrows(|escrows| {
        let escrow = escrows.get_mut(&escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;
        
        // Check escrow state
        if escrow.state != EscrowState::Created {
            return Err(EscrowError::InvalidState);
        }
        
        // Check amount matches
        if amount != escrow.amount {
            return Err(EscrowError::InvalidAmount);
        }
        
        // TODO: Verify caller is authorized (depositor or resolver)
        // For MVP, allow anyone to deposit
        
        // Update escrow state
        escrow.state = EscrowState::Funded;
        escrow.updated_at = time();
        
        Ok(())
    })
}

/// Claim tokens by revealing the secret preimage
/// 
/// State Transition: Funded → Claimed
pub async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    crate::memory::with_escrows(|escrows| {
        let escrow = escrows.get_mut(&escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;
        
        // Check escrow state
        if escrow.state != EscrowState::Funded {
            return Err(EscrowError::InvalidState);
        }
        
        // Verify hashlock
        if !verify_hashlock(&preimage, &escrow.hashlock) {
            return Err(EscrowError::InvalidHashlock);
        }
        
        // Check timelock hasn't expired
        if get_timelock_status(escrow.timelock) == TimelockStatus::Expired {
            return Err(EscrowError::TimelockExpired);
        }
        
        // Update escrow state
        escrow.state = EscrowState::Claimed;
        escrow.updated_at = time();
        
        // TODO: Transfer tokens to recipient via ICRC-1
        // For MVP, just update state
        
        Ok(())
    })
}

/// Refund tokens to depositor after timelock expiration
/// 
/// State Transition: Funded → Refunded
pub async fn refund_escrow(escrow_id: String) -> Result<(), EscrowError> {
    crate::memory::with_escrows(|escrows| {
        let escrow = escrows.get_mut(&escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;
        
        // Check escrow state
        if escrow.state != EscrowState::Funded {
            return Err(EscrowError::InvalidState);
        }
        
        // Check timelock has expired
        if get_timelock_status(escrow.timelock) != TimelockStatus::Expired {
            return Err(EscrowError::TimelockNotExpired);
        }
        
        // Update escrow state
        escrow.state = EscrowState::Refunded;
        escrow.updated_at = time();
        
        // TODO: Transfer tokens back to depositor via ICRC-1
        // For MVP, just update state
        
        Ok(())
    })
}

/// Get the current status and details of an escrow
pub fn get_escrow_status(escrow_id: String) -> Result<Escrow, EscrowError> {
    crate::memory::with_escrows(|escrows| {
        escrows.get(&escrow_id)
            .cloned()
            .ok_or(EscrowError::EscrowNotFound)
    })
}

/// List all escrows (for debugging/testing)
pub fn list_escrows() -> Vec<Escrow> {
    crate::memory::with_escrows(|escrows| {
        escrows.values().cloned().collect()
    })
}
