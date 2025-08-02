use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Supported tokens for cross-chain swaps
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum Token {
    ICP,
    ETH,
}

/// Enhanced escrow status with HTLC coordination states
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum EscrowStatus {
    Created,
    Funded,
    Claimed,
    Refunded,
}

/// HTLC coordination state for cross-chain escrow management
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum CoordinationState {
    Pending,
    EscrowsCreated,
    Active,
    SecretRevealed,
    Completed,
    Expired,
    Failed,
}

/// Escrow type for source vs destination escrows
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum EscrowType {
    Source,
    Destination,
}

/// Threshold ECDSA health status for EVM operations
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum ThresholdECDSAHealth {
    Healthy,
    Degraded,
    Unavailable,
}

/// Chain-specific health status for monitoring
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ChainHealthStatus {
    pub icp_finality_lag: u64,
    pub evm_finality_lag: u64,
    pub failed_transactions: u32,
    pub last_health_check: u64,
}

/// Partial fill information for Fusion+ compatibility
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PartialFillInfo {
    pub is_partial_fill: bool,
    pub expected_amount: u64,
    pub executed_amount: u64,
    pub fill_percentage: f64,
}

/// Cross-chain escrow event for auditable coordination history
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum CrossChainEscrowEvent {
    EscrowCreated { escrow_id: String, chain: String },
    EscrowFunded { escrow_id: String, chain: String },
    SecretRevealed { escrow_id: String, secret_hash: String },
    EscrowCompleted { escrow_id: String, chain: String },
    EscrowCancelled { escrow_id: String, chain: String },
    NetworkPartitionDetected { chain: String, lag: u64 },
    HealthCheckFailed { chain: String, error: String },
}

/// Enhanced HTLC escrow structure with cross-chain compatibility
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct HTLCEscrow {
    // Core HTLC fields
    pub order_hash: String,
    pub hashlock: String,
    pub maker: String,
    pub taker: String,
    pub token: String,
    pub amount: u64,
    pub safety_deposit: u64,
    pub timelock: u64,

    // Cross-chain parameters
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub src_token: String,
    pub dst_token: String,
    pub src_amount: u64,
    pub dst_amount: u64,

    // Enhanced state management
    pub escrow_type: EscrowType,
    pub status: EscrowStatus,
    pub address: String,

    // Timelock configuration
    pub timelock_config: TimelockConfig,

    // Chain Fusion integration
    pub threshold_ecdsa_key_id: Option<String>,

    // Health monitoring
    pub chain_health_status: Option<ChainHealthStatus>,

    // Partial fill support
    pub partial_fill_info: Option<PartialFillInfo>,

    // Event tracking
    pub events: Vec<CrossChainEscrowEvent>,

    // Metadata
    pub created_at: u64,
    pub updated_at: u64,
}

/// Timelock configuration for conservative coordination
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TimelockConfig {
    pub deployed_at: u64,
    pub src_withdrawal: u32,
    pub src_public_withdrawal: u32,
    pub src_cancellation: u32,
    pub src_public_cancellation: u32,
    pub dst_withdrawal: u32,
    pub dst_public_withdrawal: u32,
    pub dst_cancellation: u32,
    pub conservative_buffer: u32, // 3-minute buffer (180 seconds)
}

impl TimelockConfig {
    pub fn default_config() -> Self {
        Self {
            deployed_at: 0,                 // Set during escrow creation
            src_withdrawal: 3600,           // 1 hour
            src_public_withdrawal: 7200,    // 2 hours
            src_cancellation: 10800,        // 3 hours
            src_public_cancellation: 14400, // 4 hours
            dst_withdrawal: 1800,           // 30 minutes
            dst_public_withdrawal: 3600,    // 1 hour
            dst_cancellation: 5400,         // 1.5 hours
            conservative_buffer: 180,       // 3 minutes
        }
    }
}

/// Cross-chain escrow coordination structure
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CrossChainEscrow {
    pub order_id: String,
    pub icp_escrow: HTLCEscrow,
    pub evm_escrow: HTLCEscrow,
    pub coordination_state: CoordinationState,
    pub events: Vec<CrossChainEscrowEvent>,
    pub icp_finality_lag: u64,
    pub evm_finality_lag: u64,
    pub failed_transactions: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Enhanced escrow-specific error types with Chain Fusion and ECDSA support
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EscrowError {
    // Basic escrow errors
    EscrowNotFound,
    InsufficientBalance,
    Unauthorized,
    InvalidState,
    TimelockNotExpired,
    TimelockExpired,
    InvalidReceipt,
    TransferFailed,
    OrderNotFound,
    SystemError,

    // Chain Fusion integration errors
    ChainFusionRequestFailed,
    ThresholdECDSAUnavailable,
    EVMAddressDerivationFailed,
    EVMEscrowCreationFailed,

    // Network partition and health errors
    NetworkPartitionDetected,
    ChainHealthDegraded,
    InsufficientConfirmations,

    // HTLC-specific errors
    InvalidHashlock,
    InvalidTimelockCoordination,
    SecretVerificationFailed,

    // Enhanced coordination errors
    CrossChainCoordinationFailed,
    StateTransitionInvalid,
    EventLoggingFailed,

    // Slippage protection errors
    SlippageProtectionViolation,
    ExecutionAmountMismatch,

    // Partial fill errors
    InvalidPartialFill,
    PartialFillValidationFailed,
}

impl EscrowError {
    pub fn user_message(&self) -> String {
        match self {
            // Basic escrow error messages
            EscrowError::EscrowNotFound => "Escrow not found".to_string(),
            EscrowError::InsufficientBalance => "Insufficient balance".to_string(),
            EscrowError::Unauthorized => "Unauthorized".to_string(),
            EscrowError::InvalidState => "Invalid escrow state".to_string(),
            EscrowError::TimelockNotExpired => "Timelock has not expired".to_string(),
            EscrowError::TimelockExpired => "Timelock has expired".to_string(),
            EscrowError::InvalidReceipt => "Invalid cross-chain receipt".to_string(),
            EscrowError::TransferFailed => "Token transfer failed".to_string(),
            EscrowError::OrderNotFound => "Associated order not found".to_string(),
            EscrowError::SystemError => "System error occurred".to_string(),

            // Chain Fusion error messages
            EscrowError::ChainFusionRequestFailed => "Chain Fusion request failed".to_string(),
            EscrowError::ThresholdECDSAUnavailable => {
                "Threshold ECDSA signing unavailable".to_string()
            }
            EscrowError::EVMAddressDerivationFailed => "Failed to derive EVM address".to_string(),
            EscrowError::EVMEscrowCreationFailed => "Failed to create EVM escrow".to_string(),

            // Network partition error messages
            EscrowError::NetworkPartitionDetected => "Network partition detected".to_string(),
            EscrowError::ChainHealthDegraded => "Chain health is degraded".to_string(),
            EscrowError::InsufficientConfirmations => {
                "Insufficient confirmations received".to_string()
            }

            // HTLC-specific error messages
            EscrowError::InvalidHashlock => "Invalid hashlock provided".to_string(),
            EscrowError::InvalidTimelockCoordination => "Invalid timelock coordination".to_string(),
            EscrowError::SecretVerificationFailed => "Secret verification failed".to_string(),

            // Enhanced coordination error messages
            EscrowError::CrossChainCoordinationFailed => {
                "Cross-chain coordination failed".to_string()
            }
            EscrowError::StateTransitionInvalid => "Invalid state transition".to_string(),
            EscrowError::EventLoggingFailed => "Failed to log coordination event".to_string(),

            // Slippage protection error messages
            EscrowError::SlippageProtectionViolation => "Slippage protection violation".to_string(),
            EscrowError::ExecutionAmountMismatch => "Execution amount mismatch".to_string(),

            // Partial fill error messages
            EscrowError::InvalidPartialFill => "Invalid partial fill".to_string(),
            EscrowError::PartialFillValidationFailed => {
                "Partial fill validation failed".to_string()
            }
        }
    }
}
