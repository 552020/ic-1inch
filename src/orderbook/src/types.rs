use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Simplified cross-chain fusion order for MVP (1inch LOP compatible)
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FusionOrder {
    // Core Order Data
    pub id: String,
    pub maker_eth_address: String,
    pub maker_icp_principal: Principal,
    pub resolver_eth_address: Option<String>,
    pub resolver_icp_principal: Option<Principal>,

    // 1inch LOP Order Compatibility (Essential Fields Only)
    pub salt: String,         // uint256 salt for uniqueness
    pub maker_asset: String,  // Address makerAsset (token being sold)
    pub taker_asset: String,  // Address takerAsset (token being bought)
    pub making_amount: u64,   // uint256 makingAmount (amount maker is selling)
    pub taking_amount: u64,   // uint256 takingAmount (amount maker wants)
    pub maker_traits: String, // MakerTraits encoded as hex string

    // Secret Management (Simplified)
    pub hashlock: String, // bytes32 hashlock (hash of secret)

    // State Management
    pub status: OrderStatus,
    pub created_at: u64,
    pub expires_at: u64,
    pub accepted_at: Option<u64>,
    pub completed_at: Option<u64>,

    // EIP-712 Support for ETH→ICP orders
    pub eip712_signature: Option<EIP712Signature>,

    // Legacy fields (backward compatibility)
    pub from_token: Token,
    pub to_token: Token,
    pub from_amount: u64,
    pub to_amount: u64,
    pub secret_hash: String,        // Deprecated: use hashlock instead
    pub timelock_duration: u64,     // Deprecated: use timelocks instead
    pub safety_deposit_amount: u64, // Deprecated: use safety_deposit instead
}

/// Supported tokens for cross-chain swaps
#[derive(Clone, Copy, Debug, CandidType, Deserialize, Serialize, PartialEq)]
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
    Cancelled, // Order cancelled
}

// Removed: Complex Fusion+ state machine - using simple OrderStatus instead

/// EIP-712 signature data for ETH→ICP orders
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EIP712Signature {
    pub domain_separator: String,
    pub type_hash: String,
    pub order_hash: String,
    pub signature_r: String,
    pub signature_s: String,
    pub signature_v: u8,
    pub signer_address: String,
}

// Removed: Complex Dutch auction and partial fill structures for MVP simplicity

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

/// Simplified error types for MVP (8 core error types)
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum FusionError {
    // Order Management Errors
    OrderNotFound,
    OrderNotPending,
    OrderExpired,
    OrderNotCancellable,

    // Validation Errors
    InvalidAmount,
    InvalidSecretHash,
    InvalidEIP712Signature,

    // Authorization Errors
    Unauthorized,

    // System Errors
    SystemError,

    // Legacy errors (for backward compatibility)
    InvalidExpiration,
    InvalidSecret,
    InvalidSalt,
    InvalidMakerTraits,
    TokenAddressInvalid,
    NotImplemented,
}

/// Order statistics for analytics
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct OrderStatistics {
    pub total_orders: u64,
    pub pending_orders: u64,
    pub accepted_orders: u64,
    pub completed_orders: u64,
    pub failed_orders: u64,
    pub cancelled_orders: u64,
    pub total_icp_volume: u64,
    pub total_eth_volume: u64,
}

impl FusionError {
    pub fn user_message(&self) -> String {
        match self {
            // Core Order Management Errors
            FusionError::OrderNotFound => "Order not found".to_string(),
            FusionError::OrderNotPending => "Order is not in pending state".to_string(),
            FusionError::OrderExpired => "Order has expired".to_string(),
            FusionError::OrderNotCancellable => "Order cannot be cancelled".to_string(),

            // Core Validation Errors
            FusionError::InvalidAmount => "Invalid amount".to_string(),
            FusionError::InvalidSecretHash => "Invalid secret hash format".to_string(),
            FusionError::InvalidEIP712Signature => "Invalid EIP-712 signature".to_string(),

            // Authorization Errors
            FusionError::Unauthorized => "Unauthorized".to_string(),

            // System Errors
            FusionError::SystemError => "System error occurred".to_string(),

            // Legacy errors (for backward compatibility)
            FusionError::InvalidExpiration => "Invalid expiration time".to_string(),
            FusionError::InvalidSecret => "Invalid secret or hash mismatch".to_string(),
            FusionError::InvalidSalt => "Invalid salt value".to_string(),
            FusionError::InvalidMakerTraits => "Invalid maker traits".to_string(),
            FusionError::TokenAddressInvalid => "Invalid token address".to_string(),
            FusionError::NotImplemented => "Feature not yet implemented".to_string(),
        }
    }
}
// Removed: Complex FusionState implementation - using simple OrderStatus instead

// Removed: Complex EIP712Signature validation - keeping simple for MVP

impl FusionOrder {
    /// Create a new simplified FusionOrder for MVP
    pub fn new(
        id: String,
        maker_eth_address: String,
        maker_icp_principal: Principal,
        salt: String,
        maker_asset: String,
        taker_asset: String,
        making_amount: u64,
        taking_amount: u64,
        hashlock: String,
    ) -> Self {
        let current_time = 0; // Will be set by caller

        Self {
            // Core Order Data
            id,
            maker_eth_address: maker_eth_address.clone(),
            maker_icp_principal,
            resolver_eth_address: None,
            resolver_icp_principal: None,

            // 1inch LOP Order Compatibility (Essential Fields Only)
            salt,
            maker_asset: maker_asset.clone(),
            taker_asset: taker_asset.clone(),
            making_amount,
            taking_amount,
            maker_traits: "0x".to_string(), // Default empty traits

            // Secret Management (Simplified)
            hashlock: hashlock.clone(),

            // State Management
            status: OrderStatus::Pending,
            created_at: current_time,
            expires_at: current_time + 3600_000_000_000, // 1 hour default
            accepted_at: None,
            completed_at: None,

            // EIP-712 Support for ETH→ICP orders
            eip712_signature: None,

            // Legacy fields (backward compatibility)
            from_token: Token::ICP, // Default, will be updated based on maker_asset
            to_token: Token::ETH,   // Default, will be updated based on taker_asset
            from_amount: making_amount,
            to_amount: taking_amount,
            secret_hash: hashlock.clone(),
            timelock_duration: 3600, // 1 hour default
            safety_deposit_amount: (making_amount * 5) / 100,
        }
    }
}
// Removed: Complex Dutch auction and partial fill implementations for MVP simplicity

// Removed: Complex FusionOrder methods for Dutch auction and partial fills - keeping simple for MVP
// Removed: Complex OrderCreationParams - using simple function parameters for MVP
