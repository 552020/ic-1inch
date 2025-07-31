use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Represents a cross-chain fusion order
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FusionOrder {
    pub id: String,
    pub maker_eth_address: String,
    pub maker_icp_principal: Principal,
    pub from_token: Token,
    pub to_token: Token,
    pub from_amount: u64,
    pub to_amount: u64,
    pub status: OrderStatus,
    pub created_at: u64,
    pub expires_at: u64,
}

/// Supported tokens for cross-chain swaps
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum Token {
    ICP,
    ETH,
}

/// Order status throughout the swap lifecycle
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum OrderStatus {
    Pending,   // Order created, waiting for resolver
    Accepted,  // Resolver accepted, coordinating swap
    Completed, // Swap successful
    Failed,    // Swap failed
}

/// Cross-chain identity linking ETH address to ICP principal
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CrossChainIdentity {
    pub eth_address: String,
    pub icp_principal: Principal,
    pub role: UserRole,
}

/// User roles in the fusion system
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum UserRole {
    Maker,
    Resolver,
}

/// Fusion-specific error types
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum FusionError {
    OrderNotFound,
    InsufficientBalance,
    Unauthorized,
    SystemError,
}

impl FusionError {
    pub fn user_message(&self) -> String {
        match self {
            FusionError::OrderNotFound => "Order not found".to_string(),
            FusionError::InsufficientBalance => "Insufficient balance".to_string(),
            FusionError::Unauthorized => "Unauthorized".to_string(),
            FusionError::SystemError => "System error occurred".to_string(),
        }
    }
}
