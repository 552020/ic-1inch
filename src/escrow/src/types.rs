use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Supported tokens for cross-chain swaps
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum Token {
    ICP,
    ETH,
}

/// Escrow status
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum EscrowStatus {
    Created,
    Funded,
    Claimed,
    Refunded,
}

/// Escrow data for cross-chain atomic swaps
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FusionEscrow {
    pub id: String,
    pub order_id: String,
    pub token: Token,
    pub amount: u64,
    pub locked_by: Principal,
    pub locked_at: u64,
    pub status: EscrowStatus,
}

/// Escrow-specific error types
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EscrowError {
    EscrowNotFound,
    InsufficientBalance,
    Unauthorized,
    InvalidState,
    SystemError,
}

impl EscrowError {
    pub fn user_message(&self) -> String {
        match self {
            EscrowError::EscrowNotFound => "Escrow not found".to_string(),
            EscrowError::InsufficientBalance => "Insufficient balance".to_string(),
            EscrowError::Unauthorized => "Unauthorized".to_string(),
            EscrowError::InvalidState => "Invalid escrow state".to_string(),
            EscrowError::SystemError => "System error occurred".to_string(),
        }
    }
}