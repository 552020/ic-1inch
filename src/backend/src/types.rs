use candid::{CandidType, Deserialize, Principal};
use serde::{Deserialize as SerdeDeserialize, Serialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowParams {
    pub maker: Principal,          // Token maker (also the recipient)
    pub hashlock: Vec<u8>,         // Hash of the secret
    pub token_canister: Principal, // ICRC-1 token canister ID
    pub amount: u64,               // Token amount in smallest unit
    pub timelock: u64,             // Nanoseconds since epoch
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

// New structs for source and destination escrows
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SourceEscrow {
    pub id: String,
    pub hashlock: Vec<u8>,
    pub timelock: u64,
    pub token_canister: Principal,
    pub amount: u64,
    pub maker: Principal, // Who gets refunded on cancellation
    pub taker: Principal, // Who can withdraw (resolver)
    pub state: EscrowState,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DestinationEscrow {
    pub id: String,
    pub hashlock: Vec<u8>,
    pub timelock: u64,
    pub token_canister: Principal,
    pub amount: u64,
    pub maker: Principal, // Who receives tokens on withdrawal
    pub taker: Principal, // Who funds this escrow (resolver)
    pub state: EscrowState,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum EscrowState {
    Created,
    Funded,
    Claimed,
    Refunded,
    Expired,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum EscrowError {
    InvalidAmount,
    InsufficientBalance,
    EscrowNotFound,
    TimelockExpired,
    Unauthorized,
    InvalidTimelock,
    InvalidHashlock,
    TransferFailed,
    TimelockNotExpired,
    InvalidState,
    InvalidEscrowType,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TimelockStatus {
    Active,  // Timelock not expired
    Expired, // Timelock has expired
}

// Result types for different operations
pub type Result<T> = std::result::Result<T, EscrowError>;
pub type EscrowResult = Result<Escrow>;
pub type SourceEscrowResult = Result<SourceEscrow>;
pub type DestinationEscrowResult = Result<DestinationEscrow>;
pub type StringResult = Result<String>;
pub type TimelockStatusResult = Result<TimelockStatus>;

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
            EscrowError::InvalidEscrowType => write!(f, "Invalid escrow type"),
        }
    }
}
