use crate::types::{CreateEscrowParams, Escrow, EscrowError, EscrowState};
use candid::CandidType;
use ic_cdk::api::time;
use ic_cdk::caller;
use sha2::{Digest, Sha256};

// TODO: Add integration tests for end-to-end escrow lifecycle

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

/// Validate that the caller is authorized to create an escrow
///
/// Authorization rules:
/// 1. Only authorized resolvers can create escrows (1inch Fusion+ model)
/// 2. Makers cannot create escrows directly - they create intents
/// 3. Resolvers create escrows on behalf of makers during execution
pub fn validate_create_escrow_authorization(caller: candid::Principal) -> Result<(), EscrowError> {
    // Only resolvers can create escrows
    if is_authorized_resolver(caller) {
        return Ok(());
    }

    // Makers and other callers are unauthorized
    Err(EscrowError::Unauthorized)
}

/// Check if a principal is an authorized resolver
///
/// TODO: In production, this should check against a maintained list
/// of authorized resolvers or use a resolver registry
pub fn is_authorized_resolver(_resolver: candid::Principal) -> bool {
    // For MVP: Allow any resolver (insecure, but functional)
    // In production: Check against authorized resolver list
    true
}

/// Create a new escrow with the specified parameters
///
/// State Transition: None → Created
///
/// Authorization: Only authorized resolvers can create escrows
/// (Makers create intents, resolvers create escrows during execution)
pub async fn create_escrow(params: CreateEscrowParams) -> Result<String, EscrowError> {
    // Get the caller's principal
    let caller = caller();

    // Validate authorization
    validate_create_escrow_authorization(caller)?;

    // Validate parameters
    if params.amount <= 0 {
        return Err(EscrowError::InvalidAmount);
    }

    // Validate timelock is in the future
    validate_timelock(params.timelock)?;

    // Generate unique escrow ID
    // TODO: Replace with UUID for collision resistance
    // let escrow_id = uuid::Uuid::new_v4().to_string();
    let escrow_id = format!("escrow_{}", time());

    // Create escrow
    let escrow = Escrow {
        id: escrow_id.clone(),
        hashlock: params.hashlock,
        timelock: params.timelock,
        token_canister: params.token_canister,
        amount: params.amount,
        maker: params.maker,
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
        let escrow = escrows.get_mut(&escrow_id).ok_or(EscrowError::EscrowNotFound)?;

        // Check escrow state
        if escrow.state != EscrowState::Created {
            return Err(EscrowError::InvalidState);
        }

        // Check amount matches
        if amount != escrow.amount {
            return Err(EscrowError::InvalidAmount);
        }

        // Verify caller is authorized (only taker/resolver can deposit)
        if !is_authorized_resolver(caller()) {
            return Err(EscrowError::Unauthorized);
        }

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
        let escrow = escrows.get_mut(&escrow_id).ok_or(EscrowError::EscrowNotFound)?;

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

/// Refund tokens to maker after timelock expiration
///
/// State Transition: Funded → Refunded
pub async fn refund_escrow(escrow_id: String) -> Result<(), EscrowError> {
    crate::memory::with_escrows(|escrows| {
        let escrow = escrows.get_mut(&escrow_id).ok_or(EscrowError::EscrowNotFound)?;

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

        // TODO: Transfer tokens back to maker via ICRC-1
        // For MVP, just update state

        Ok(())
    })
}

/// Get the current status and details of an escrow
pub fn get_escrow_status(escrow_id: String) -> Result<Escrow, EscrowError> {
    crate::memory::with_escrows(|escrows| {
        escrows.get(&escrow_id).cloned().ok_or(EscrowError::EscrowNotFound)
    })
}

/// List all escrows (for debugging/testing)
pub fn list_escrows() -> Vec<Escrow> {
    crate::memory::with_escrows(|escrows| escrows.values().cloned().collect())
}
