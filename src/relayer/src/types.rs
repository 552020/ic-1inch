use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

// ============================================================================
// 1INCH API STRUCTURES (Essential Only)
// ============================================================================

/// Core order data - matches 1inch LOP exactly (for /submit endpoint)
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CrossChainOrderDto {
    pub salt: String,
    pub maker: String,
    pub receiver: String,
    #[serde(rename = "makerAsset")]
    pub maker_asset: String,
    #[serde(rename = "takerAsset")]
    pub taker_asset: String,
    #[serde(rename = "makingAmount")]
    pub making_amount: String,
    #[serde(rename = "takingAmount")]
    pub taking_amount: String,
    #[serde(rename = "makerTraits")]
    pub maker_traits: String,
}

/// Internal order storage and API responses
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Order {
    // Core Order Data
    pub id: String,
    pub maker_eth_address: String,
    pub maker_icp_principal: Principal,
    pub resolver_eth_address: Option<String>,
    pub resolver_icp_principal: Option<Principal>,

    // 1inch LOP Order Compatibility
    pub salt: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub making_amount: String,
    pub taking_amount: String,
    pub maker_traits: String,

    // Secret Management
    pub hashlock: String,

    // State Management
    pub status: OrderStatus,
    pub created_at: u64,
    pub expires_at: u64,
    pub accepted_at: Option<u64>,
    pub completed_at: Option<u64>,

    // 1inch API fields
    pub signature: String,
    pub deadline: u64,
    pub auction_start_date: u64,
    pub auction_end_date: u64,
    pub quote_id: String,
    pub remaining_maker_amount: String,
    pub maker_balance: String,
    pub maker_allowance: String,
    pub is_maker_contract: bool,
    pub extension: String,
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub secret_hashes: Vec<String>,
    pub fills: Vec<String>,
}

/// Order status
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum OrderStatus {
    Pending,   // Order created, waiting for resolver
    Accepted,  // Resolver accepted, coordinating swap
    Completed, // Swap successful
    Failed,    // Swap failed
    Cancelled, // Order cancelled
}

/// Cross-chain identity linking ETH address to ICP principal
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CrossChainIdentity {
    pub eth_address: String,
    pub icp_principal: Principal,
    pub role: UserRole,
}

/// User roles
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum UserRole {
    Maker,
    Resolver,
}

/// Error types (simplified)
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum FusionError {
    // Order Management Errors
    OrderNotFound,
    OrderNotPending,
    OrderExpired,

    // Validation Errors
    InvalidAmount,
    InvalidSecretHash,
    InvalidEIP712Signature,
    InvalidSalt,
    TokenAddressInvalid,

    // System Errors
    SystemError,
    Unauthorized,
}

// ============================================================================
// IMPLEMENTATIONS
// ============================================================================

impl Order {
    /// Create a new Order compatible with 1inch API
    pub fn new(
        id: String,
        maker_eth_address: String,
        maker_icp_principal: Principal,
        salt: String,
        maker_asset: String,
        taker_asset: String,
        making_amount: String,
        taking_amount: String,
        hashlock: String,
        signature: String,
        quote_id: String,
        extension: String,
        src_chain_id: u64,
        dst_chain_id: u64,
    ) -> Self {
        let current_time = ic_cdk::api::time();

        Self {
            // Core Order Data
            id,
            maker_eth_address: maker_eth_address.clone(),
            maker_icp_principal,
            resolver_eth_address: None,
            resolver_icp_principal: None,

            // 1inch LOP Order Compatibility
            salt,
            maker_asset: maker_asset.clone(),
            taker_asset: taker_asset.clone(),
            making_amount: making_amount.clone(),
            taking_amount: taking_amount.clone(),
            maker_traits: "0x".to_string(), // Default empty traits

            // Secret Management
            hashlock: hashlock.clone(),

            // State Management
            status: OrderStatus::Pending,
            created_at: current_time,
            expires_at: current_time + 3600_000_000_000, // 1 hour default
            accepted_at: None,
            completed_at: None,

            // 1inch API fields
            signature,
            deadline: current_time + 3600_000_000_000, // 1 hour default
            auction_start_date: current_time,
            auction_end_date: current_time + 3600_000_000_000,
            quote_id,
            remaining_maker_amount: making_amount.clone(),
            maker_balance: "0".to_string(), // Will be fetched from chain
            maker_allowance: "0".to_string(), // Will be fetched from chain
            is_maker_contract: false,       // Default assumption
            extension,
            src_chain_id,
            dst_chain_id,
            secret_hashes: vec![hashlock.clone()],
            fills: vec![],
        }
    }
}
