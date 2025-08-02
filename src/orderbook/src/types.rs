use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Represents a cross-chain fusion order (compatible with 1inch LOP)
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FusionOrder {
    // Core Order Data (ICP-specific)
    pub id: String,
    pub maker_eth_address: String,
    pub maker_icp_principal: Principal,
    pub resolver_eth_address: Option<String>,
    pub resolver_icp_principal: Option<Principal>,

    // Legacy fields (kept for backward compatibility)
    pub from_token: Token,
    pub to_token: Token,
    pub from_amount: u64,
    pub to_amount: u64,

    // 1inch LOP Order Compatibility
    pub salt: String,         // uint256 salt for uniqueness
    pub maker_asset: String,  // Address makerAsset (token being sold)
    pub taker_asset: String,  // Address takerAsset (token being bought)
    pub making_amount: u64,   // uint256 makingAmount (amount maker is selling)
    pub taking_amount: u64,   // uint256 takingAmount (amount maker wants)
    pub maker_traits: String, // MakerTraits encoded as hex string
    pub order_hash: String,   // bytes32 orderHash from LOP

    // Cross-Chain Parameters (Enhanced for Cross-Chain SDK compatibility)
    pub src_chain_id: u64,    // uint256 srcChainId (ICP chain ID)
    pub src_token: String,    // Address srcToken (ICP token address)
    pub src_amount: u64,      // Amount on source chain (ICP)
    pub dst_chain_id: u64,    // uint256 dstChainId
    pub dst_token: String,    // Address dstToken
    pub dst_amount: u64,      // Amount on destination chain
    pub safety_deposits: u64, // Combined safety deposits (src + dst)

    // Secret Management (Enhanced for Cross-Chain SDK)
    pub hashlock: String, // bytes32 hashlock (hash of secret) - primary
    pub secret_hashes: Vec<String>, // Multiple secret hashes for partial fills
    pub merkle_tree_root: Option<String>, // Merkle tree root for complex scenarios

    // Cross-Chain Escrow Immutables Compatibility (Legacy)
    pub maker_address: String,         // Address maker (from LOP order)
    pub taker_address: Option<String>, // Address taker (resolver)
    pub token_address: String,         // Address token (for this chain)
    pub amount: u64,                   // uint256 amount (for this chain)
    pub safety_deposit: u64,           // uint256 safetyDeposit

    // State Management (ICP-specific)
    pub status: OrderStatus,
    pub fusion_state: FusionState,
    pub created_at: u64,
    pub expires_at: u64,
    pub accepted_at: Option<u64>,
    pub completed_at: Option<u64>,

    // Cross-Chain Coordination
    pub escrow_src_address: Option<String>, // Set when escrow factory creates source escrow
    pub escrow_dst_address: Option<String>, // Set when escrow factory creates destination escrow

    // Fusion+ Protocol Data
    pub eip712_signature: Option<EIP712Signature>,
    pub partial_fill_data: Option<PartialFillData>, // For multiple fills

    // Dutch Auction Parameters (Fusion+ Whitepaper 2.3)
    pub auction_start_timestamp: u64, // When Dutch auction begins
    pub auction_start_rate: u64,      // Maximum exchange rate
    pub minimum_return_amount: u64,   // Lowest acceptable exchange rate
    pub decrease_rate: u64,           // Rate at which price decreases
    pub waiting_period: u64,          // Delay before auction starts
    pub current_price: u64,           // Current auction price
    pub price_curve: PriceCurve,      // Piecewise linear function

    // Legacy fields (kept for backward compatibility)
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

/// Fusion+ protocol state machine
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum FusionState {
    AnnouncementPhase, // Order created, waiting for resolver
    DepositPhase,      // Escrows being created/funded
    WithdrawalPhase,   // Secret revealed, withdrawals in progress
    RecoveryPhase,     // Timelock expired, recovery possible
    Completed,         // Swap successfully completed
    Cancelled,         // Order cancelled or failed
}

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

/// Partial fill data for multiple-fill orders
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PartialFillData {
    pub merkle_root: String,     // Merkle tree root of secrets
    pub parts_amount: u64,       // Number of parts the order can be split into
    pub filled_amount: u64,      // Amount already filled
    pub remaining_amount: u64,   // Amount remaining to be filled
    pub fills: Vec<PartialFill>, // Record of all partial fills
}

/// Escrow type enumeration
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum EscrowType {
    Source,      // Source chain escrow (locks maker's tokens)
    Destination, // Destination chain escrow (locks resolver's tokens)
}

/// Dutch Auction Price Curve Structure
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PriceCurve {
    pub segments: Vec<PriceSegment>,
    pub total_duration: u64,
    pub spot_price: u64, // X/6 as per whitepaper
}

/// Price segment for piecewise linear function
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PriceSegment {
    pub start_time: u64,
    pub end_time: u64,
    pub start_price: u64,
    pub end_price: u64,
    pub decrease_rate: u64,
}

/// Partial fill record for multiple-fill orders
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PartialFill {
    pub resolver_address: String,
    pub fill_amount: u64,
    pub secret_index: u32,
    pub timestamp: u64,
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
    // Order Management Errors
    OrderNotFound,
    OrderNotPending,
    OrderExpired,
    OrderAlreadyAccepted,
    OrderNotCancellable,

    // Validation Errors
    InvalidAmount,
    InvalidExpiration,
    InvalidSecretHash,
    InvalidSecret,
    InvalidEIP712Signature,
    InvalidSalt,
    InvalidMakerTraits,
    InvalidOrderHash,

    // Authorization Errors
    Unauthorized,
    InsufficientBalance,
    ResolverNotWhitelisted,

    // System Errors
    SystemError,
    EscrowCreationFailed,
    EscrowCoordinationFailed,
    NotImplemented,

    // State Management Errors
    InvalidStateTransition,
    TimelockNotExpired,
    FinalityLockActive,

    // Cross-Chain Errors
    ChainIdMismatch,
    TokenAddressInvalid,
    CrossChainVerificationFailed,

    // Dutch Auction Errors
    AuctionNotStarted,
    AuctionExpired,
    PriceNotProfitable,
    InvalidPriceCurve,

    // Partial Fill Errors
    PartialFillsNotSupported,
    InvalidFillAmount,
    InsufficientRemainingAmount,

    // Escrow Coordination Errors
    EscrowsNotReady,
    EscrowNotificationFailed,
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
            // Order Management Errors
            FusionError::OrderNotFound => "Order not found".to_string(),
            FusionError::OrderNotPending => "Order is not in pending state".to_string(),
            FusionError::OrderExpired => "Order has expired".to_string(),
            FusionError::OrderAlreadyAccepted => "Order already accepted".to_string(),
            FusionError::OrderNotCancellable => "Order cannot be cancelled".to_string(),

            // Validation Errors
            FusionError::InvalidAmount => "Invalid amount".to_string(),
            FusionError::InvalidExpiration => "Invalid expiration time".to_string(),
            FusionError::InvalidSecretHash => "Invalid secret hash format".to_string(),
            FusionError::InvalidSecret => "Invalid secret or hash mismatch".to_string(),
            FusionError::InvalidEIP712Signature => "Invalid EIP-712 signature".to_string(),
            FusionError::InvalidSalt => "Invalid salt value".to_string(),
            FusionError::InvalidMakerTraits => "Invalid maker traits".to_string(),
            FusionError::InvalidOrderHash => "Invalid order hash".to_string(),

            // Authorization Errors
            FusionError::Unauthorized => "Unauthorized".to_string(),
            FusionError::InsufficientBalance => "Insufficient balance".to_string(),
            FusionError::ResolverNotWhitelisted => "Resolver is not whitelisted".to_string(),

            // System Errors
            FusionError::SystemError => "System error occurred".to_string(),
            FusionError::EscrowCreationFailed => "Failed to create escrow".to_string(),
            FusionError::EscrowCoordinationFailed => {
                "Failed to coordinate with escrow factory".to_string()
            }
            FusionError::NotImplemented => "Feature not yet implemented".to_string(),

            // State Management Errors
            FusionError::InvalidStateTransition => "Invalid state transition".to_string(),
            FusionError::TimelockNotExpired => "Timelock has not expired yet".to_string(),
            FusionError::FinalityLockActive => "Finality lock is still active".to_string(),

            // Cross-Chain Errors
            FusionError::ChainIdMismatch => "Chain ID mismatch".to_string(),
            FusionError::TokenAddressInvalid => "Invalid token address".to_string(),
            FusionError::CrossChainVerificationFailed => {
                "Cross-chain verification failed".to_string()
            }

            // Dutch Auction Errors
            FusionError::AuctionNotStarted => "Dutch auction has not started yet".to_string(),
            FusionError::AuctionExpired => "Dutch auction has expired".to_string(),
            FusionError::PriceNotProfitable => "Current price is not profitable".to_string(),
            FusionError::InvalidPriceCurve => "Invalid price curve configuration".to_string(),

            // Partial Fill Errors
            FusionError::PartialFillsNotSupported => {
                "Order does not support partial fills".to_string()
            }
            FusionError::InvalidFillAmount => "Invalid fill amount".to_string(),
            FusionError::InsufficientRemainingAmount => {
                "Insufficient remaining amount for fill".to_string()
            }

            // Escrow Coordination Errors
            FusionError::EscrowsNotReady => "Escrows are not ready for withdrawal".to_string(),
            FusionError::EscrowNotificationFailed => "Failed to notify escrow factory".to_string(),
        }
    }
}
impl FusionState {
    /// Check if transition to new state is valid
    pub fn can_transition_to(&self, new_state: &FusionState) -> bool {
        use FusionState::*;
        match (self, new_state) {
            (AnnouncementPhase, DepositPhase) => true,
            (DepositPhase, WithdrawalPhase) => true,
            (WithdrawalPhase, Completed) => true,
            (AnnouncementPhase, Cancelled) => true,
            (DepositPhase, RecoveryPhase) => true,
            (RecoveryPhase, Cancelled) => true,
            _ => false,
        }
    }
}

impl Default for FusionState {
    fn default() -> Self {
        FusionState::AnnouncementPhase
    }
}

impl EIP712Signature {
    /// Validate EIP-712 signature format
    pub fn validate_format(&self) -> Result<(), FusionError> {
        // Validate signature components are 64 hex characters
        if self.signature_r.len() != 64 || !self.signature_r.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err(FusionError::InvalidEIP712Signature);
        }
        if self.signature_s.len() != 64 || !self.signature_s.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err(FusionError::InvalidEIP712Signature);
        }

        // Validate signer address format (0x + 40 hex characters)
        if !self.signer_address.starts_with("0x") || self.signer_address.len() != 42 {
            return Err(FusionError::InvalidEIP712Signature);
        }

        // Validate v value (27 or 28, or 0 or 1)
        if self.signature_v != 27
            && self.signature_v != 28
            && self.signature_v != 0
            && self.signature_v != 1
        {
            return Err(FusionError::InvalidEIP712Signature);
        }

        Ok(())
    }
}

impl FusionOrder {
    /// Create a new FusionOrder with default values
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
        let auction_duration = 3600; // 1 hour default

        Self {
            // Core Order Data
            id,
            maker_eth_address: maker_eth_address.clone(),
            maker_icp_principal,
            resolver_eth_address: None,
            resolver_icp_principal: None,

            // Legacy fields (backward compatibility)
            from_token: Token::ICP, // Default, will be updated based on maker_asset
            to_token: Token::ETH,   // Default, will be updated based on taker_asset
            from_amount: making_amount,
            to_amount: taking_amount,

            // 1inch LOP Order Compatibility
            salt,
            maker_asset: maker_asset.clone(),
            taker_asset: taker_asset.clone(),
            making_amount,
            taking_amount,
            maker_traits: "0x".to_string(), // Default empty traits
            order_hash: "".to_string(),     // Will be computed

            // Cross-Chain Parameters (Enhanced for Cross-Chain SDK compatibility)
            src_chain_id: 0, // ICP chain ID (will be set)
            src_token: maker_asset.clone(),
            src_amount: making_amount,
            dst_chain_id: 1, // Default to Ethereum mainnet
            dst_token: taker_asset.clone(),
            dst_amount: taking_amount,
            safety_deposits: (making_amount * 10) / 100, // 10% combined default

            // Secret Management (Enhanced for Cross-Chain SDK)
            hashlock: hashlock.clone(),
            secret_hashes: vec![hashlock.clone()], // Start with primary secret
            merkle_tree_root: None,

            // Cross-Chain Escrow Immutables Compatibility (Legacy)
            maker_address: maker_eth_address.clone(),
            taker_address: None,
            token_address: maker_asset,
            amount: making_amount,
            safety_deposit: (making_amount * 5) / 100, // 5% default

            // State Management
            status: OrderStatus::Pending,
            fusion_state: FusionState::AnnouncementPhase,
            created_at: current_time,
            expires_at: current_time + auction_duration * 1_000_000_000, // Convert to nanoseconds
            accepted_at: None,
            completed_at: None,

            // Cross-Chain Coordination
            escrow_src_address: None,
            escrow_dst_address: None,

            // Fusion+ Protocol Data
            eip712_signature: None,
            partial_fill_data: None,

            // Dutch Auction Parameters (Fusion+ Whitepaper 2.3)
            auction_start_timestamp: current_time + 300_000_000_000, // 5 minutes waiting period
            auction_start_rate: taking_amount + (taking_amount * 20) / 100, // 20% above spot
            minimum_return_amount: (taking_amount * 95) / 100,       // 95% of spot price
            decrease_rate: (taking_amount * 5) / 100 / auction_duration, // Linear decrease
            waiting_period: 300_000_000_000,                         // 5 minutes in nanoseconds
            current_price: taking_amount + (taking_amount * 20) / 100, // Start at max
            price_curve: PriceCurve::default_curve(taking_amount, auction_duration),

            // Legacy fields (backward compatibility)
            secret_hash: hashlock.clone(),
            timelock_duration: 3600, // 1 hour default
            safety_deposit_amount: (making_amount * 5) / 100,
        }
    }
}
impl PriceCurve {
    /// Create a default price curve for Dutch auction
    pub fn default_curve(spot_price: u64, duration: u64) -> Self {
        let segments = vec![
            PriceSegment {
                start_time: 0,
                end_time: duration / 3,
                start_price: spot_price + (spot_price * 20) / 100, // Start 20% above spot
                end_price: spot_price,
                decrease_rate: (spot_price * 20) / 100 / (duration / 3),
            },
            PriceSegment {
                start_time: duration / 3,
                end_time: duration,
                start_price: spot_price,
                end_price: (spot_price * 95) / 100, // End 5% below spot
                decrease_rate: (spot_price * 5) / 100 / (duration * 2 / 3),
            },
        ];

        Self {
            segments,
            total_duration: duration,
            spot_price: spot_price / 6, // X/6 as per whitepaper
        }
    }

    /// Calculate current price based on elapsed time
    pub fn calculate_price(&self, elapsed_time: u64) -> u64 {
        if elapsed_time >= self.total_duration {
            return self.segments.last().map(|s| s.end_price).unwrap_or(0);
        }

        for segment in &self.segments {
            if elapsed_time >= segment.start_time && elapsed_time < segment.end_time {
                let segment_elapsed = elapsed_time - segment.start_time;
                let segment_duration = segment.end_time - segment.start_time;

                if segment_duration == 0 {
                    return segment.start_price;
                }

                let price_decrease =
                    (segment.start_price - segment.end_price) * segment_elapsed / segment_duration;
                return segment.start_price - price_decrease;
            }
        }

        self.segments.first().map(|s| s.start_price).unwrap_or(0)
    }
}

impl Default for PriceCurve {
    fn default() -> Self {
        Self::default_curve(1000, 3600) // Default 1000 units, 1 hour
    }
}

impl PartialFillData {
    /// Create new partial fill data
    pub fn new(parts_amount: u64, total_amount: u64, merkle_root: String) -> Self {
        Self {
            merkle_root,
            parts_amount,
            filled_amount: 0,
            remaining_amount: total_amount,
            fills: Vec::new(),
        }
    }

    /// Add a new partial fill
    pub fn add_fill(&mut self, fill: PartialFill) -> Result<(), FusionError> {
        if fill.fill_amount > self.remaining_amount {
            return Err(FusionError::InsufficientRemainingAmount);
        }

        self.filled_amount += fill.fill_amount;
        self.remaining_amount -= fill.fill_amount;
        self.fills.push(fill);

        Ok(())
    }

    /// Check if order is fully filled
    pub fn is_fully_filled(&self) -> bool {
        self.remaining_amount == 0
    }
}

impl FusionOrder {
    /// Calculate current Dutch auction price
    pub fn calculate_current_price(&self) -> u64 {
        let current_time = ic_cdk::api::time();

        // Check if auction has started
        if current_time < self.auction_start_timestamp {
            return self.auction_start_rate;
        }

        // Calculate elapsed time since auction start
        let elapsed_time = current_time - self.auction_start_timestamp;

        // Use price curve to calculate current price
        let calculated_price = self.price_curve.calculate_price(elapsed_time);

        // Ensure price doesn't go below minimum
        std::cmp::max(calculated_price, self.minimum_return_amount)
    }

    /// Check if order is profitable for resolver at current price
    pub fn is_profitable_for_resolver(&self, resolver_fee: u64) -> bool {
        let current_price = self.calculate_current_price();
        let total_cost = current_price + resolver_fee;
        total_cost <= self.taking_amount
    }

    /// Add a secret hash for partial fills
    pub fn add_secret_hash(&mut self, secret_hash: String) {
        self.secret_hashes.push(secret_hash);
    }

    /// Get secret hash by index
    pub fn get_secret_hash(&self, index: usize) -> Option<&String> {
        self.secret_hashes.get(index)
    }

    /// Set Merkle tree root for complex partial fill scenarios
    pub fn set_merkle_tree_root(&mut self, root: String) {
        self.merkle_tree_root = Some(root);
    }

    /// Verify Merkle proof for partial fill
    pub fn verify_merkle_proof(&self, leaf: &str, proof: &[String]) -> Result<bool, FusionError> {
        // Simplified Merkle proof verification for MVP
        // In production, this would use proper cryptographic verification
        if let Some(ref root) = self.merkle_tree_root {
            // For MVP, just check if leaf is in the proof array
            Ok(proof.iter().any(|p| p == leaf) || leaf == root)
        } else {
            Ok(false)
        }
    }

    /// Check if order supports partial fills
    pub fn supports_partial_fills(&self) -> bool {
        self.partial_fill_data.is_some()
    }

    /// Enable partial fills for this order
    pub fn enable_partial_fills(&mut self, parts_amount: u64, merkle_root: String) {
        self.partial_fill_data =
            Some(PartialFillData::new(parts_amount, self.making_amount, merkle_root));
    }

    /// Add a partial fill to this order
    pub fn add_partial_fill(&mut self, fill: PartialFill) -> Result<(), FusionError> {
        if let Some(ref mut partial_data) = self.partial_fill_data {
            partial_data.add_fill(fill)
        } else {
            Err(FusionError::PartialFillsNotSupported)
        }
    }

    /// Check if order is fully filled
    pub fn is_fully_filled(&self) -> bool {
        if let Some(ref partial_data) = self.partial_fill_data {
            partial_data.is_fully_filled()
        } else {
            self.status == OrderStatus::Completed
        }
    }

    /// Get remaining amount for partial fills
    pub fn get_remaining_amount(&self) -> u64 {
        if let Some(ref partial_data) = self.partial_fill_data {
            partial_data.remaining_amount
        } else {
            if self.status == OrderStatus::Completed {
                0
            } else {
                self.making_amount
            }
        }
    }
}
/// Order creation parameters for enhanced fusion orders
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct OrderCreationParams {
    // LOP Order Parameters
    pub salt: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub maker_traits: String,

    // Cross-Chain Parameters
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub safety_deposits: u64,

    // Secret Management
    pub hashlock: String,
    pub secret_hashes: Vec<String>,
    pub merkle_tree_root: Option<String>,

    // Dutch Auction Parameters
    pub auction_start_rate: u64,
    pub minimum_return_amount: u64,
    pub decrease_rate: u64,
    pub waiting_period: u64,

    // Optional EIP-712 signature for ETH→ICP orders
    pub eip712_signature: Option<EIP712Signature>,
}
