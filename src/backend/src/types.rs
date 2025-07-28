use candid::{CandidType, Deserialize, Principal};

// Core HTLC data structures
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowParams {
    pub hashlock: Vec<u8>,         // SHA256 hash of secret
    pub timelock: u64,             // Nanoseconds since epoch
    pub token_canister: Principal, // ICRC-1 token canister ID
    pub amount: u64,               // Token amount in smallest unit
    pub maker: Principal,          // Token maker (also the recipient)
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Escrow {
    pub id: String,
    pub hashlock: Vec<u8>,
    pub timelock: u64,
    pub token_canister: Principal,
    pub amount: u64,
    pub maker: Principal, // Token maker (also the recipient)
    pub state: EscrowState,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum EscrowState {
    Created,  // Escrow created, not funded
    Funded,   // Tokens deposited, waiting for secret
    Claimed,  // Secret revealed, tokens transferred
    Refunded, // Timelock expired, tokens returned
    Expired,  // Escrow expired, cleanup needed
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum TimelockStatus {
    Active,  // Timelock not expired
    Expired, // Timelock has expired
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum EscrowError {
    EscrowNotFound,
    InvalidState,
    InvalidHashlock,
    TimelockNotExpired,
    TimelockExpired,
    InsufficientBalance,
    TransferFailed,
    Unauthorized,
    InvalidAmount,
    InvalidTimelock,
}

impl std::fmt::Display for EscrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EscrowError::EscrowNotFound => write!(f, "Escrow not found"),
            EscrowError::InvalidState => write!(f, "Invalid escrow state"),
            EscrowError::InvalidHashlock => write!(f, "Invalid secret provided"),
            EscrowError::TimelockNotExpired => write!(f, "Timelock has not expired"),
            EscrowError::TimelockExpired => write!(f, "Timelock has expired"),
            EscrowError::InsufficientBalance => write!(f, "Insufficient token balance"),
            EscrowError::TransferFailed => write!(f, "Token transfer failed"),
            EscrowError::Unauthorized => write!(f, "Unauthorized operation"),
            EscrowError::InvalidAmount => write!(f, "Invalid amount"),
            EscrowError::InvalidTimelock => write!(f, "Invalid timelock"),
        }
    }
}
