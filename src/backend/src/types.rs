use candid::{CandidType, Deserialize, Principal};
use std::collections::HashMap;

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

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
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
// ============================================================================
// LIMIT ORDER PROTOCOL TYPES
// ============================================================================

/// Unique identifier for limit orders
pub type OrderId = u64;

/// System constants for limit order protocol
pub const MAX_ACTIVE_ORDERS: usize = 10_000;
pub const MAX_HISTORICAL_ORDERS: usize = 100_000;
pub const MAX_EXPIRATION_DAYS: u64 = 30;

/// Core limit order structure
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: OrderId,
    pub maker: Principal,
    pub receiver: Principal,    // Can be different from maker
    pub maker_asset: Principal, // ICRC token canister ID
    pub taker_asset: Principal, // ICRC token canister ID
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expiration: u64, // Nanoseconds since epoch
    pub created_at: u64,
    pub allowed_taker: Option<Principal>, // Private orders

    // Extension fields for future ChainFusion+ compatibility
    pub metadata: Option<OrderMetadata>,
}

/// Optional metadata for future cross-chain functionality
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct OrderMetadata {
    // Reserved for future cross-chain fields
    pub hashlock: Option<Vec<u8>>,
    pub timelock: Option<u64>,
    pub target_chain: Option<String>,
}

/// Order state enumeration
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderState {
    Active,    // Order is available for filling
    Filled,    // Order has been completely filled
    Cancelled, // Order has been cancelled by maker
    Expired,   // Order has passed expiration time
}

/// Comprehensive error types for limit order operations
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum OrderError {
    // Validation Errors
    InvalidAmount,
    InvalidExpiration,
    InvalidAssetPair,
    InvalidReceiver,
    InvalidOrderId,
    InvalidPrincipal,

    // State Errors
    OrderNotFound,
    OrderAlreadyFilled,
    OrderCancelled,
    OrderExpired,
    OrderInactive,

    // Authorization Errors
    Unauthorized,
    InsufficientBalance,
    AnonymousCaller,
    NotOrderMaker,

    // Token Integration Errors
    TokenCallFailed(String),
    TransferFailed(String),
    BalanceCheckFailed(String),
    TokenNotSupported(String),

    // System Errors
    SystemError(String),
    MemoryError(String),
    ConcurrencyError(String),

    // Rate Limiting & DoS Protection
    TooManyOrders,
    OrderCreationRateLimited,
    SystemOverloaded,
}

/// System statistics for monitoring
#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct SystemStats {
    pub orders_created: u64,
    pub orders_filled: u64,
    pub orders_cancelled: u64,
    pub total_volume: HashMap<Principal, u64>, // Per token
    pub error_counts: HashMap<String, u64>,    // Error frequency tracking
}

/// Parameters for creating a new limit order
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateOrderParams {
    pub receiver: Principal,
    pub maker_asset: Principal,
    pub taker_asset: Principal,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expiration: u64,
    pub allowed_taker: Option<Principal>,
}

// Result types for limit order operations
pub type OrderResult<T> = std::result::Result<T, OrderError>;
pub type OrderQueryResult = OrderResult<Order>;
pub type OrderIdResult = OrderResult<OrderId>;
pub type OrderListResult = OrderResult<Vec<Order>>;
pub type SystemStatsResult = OrderResult<SystemStats>;

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Validation Errors
            OrderError::InvalidAmount => write!(f, "Amount must be greater than zero"),
            OrderError::InvalidExpiration => {
                write!(f, "Expiration must be in the future and within allowed range")
            }
            OrderError::InvalidAssetPair => write!(f, "Maker and taker assets cannot be the same"),
            OrderError::InvalidReceiver => write!(f, "Invalid receiver principal"),
            OrderError::InvalidOrderId => write!(f, "Invalid order ID"),
            OrderError::InvalidPrincipal => write!(f, "Invalid principal format"),

            // State Errors
            OrderError::OrderNotFound => write!(f, "Order not found"),
            OrderError::OrderAlreadyFilled => write!(f, "Order has already been filled"),
            OrderError::OrderCancelled => write!(f, "Order has been cancelled"),
            OrderError::OrderExpired => write!(f, "Order has expired"),
            OrderError::OrderInactive => write!(f, "Order is not active"),

            // Authorization Errors
            OrderError::Unauthorized => write!(f, "Unauthorized to perform this operation"),
            OrderError::InsufficientBalance => write!(f, "Insufficient token balance"),
            OrderError::AnonymousCaller => write!(f, "Anonymous callers are not allowed"),
            OrderError::NotOrderMaker => {
                write!(f, "Only the order maker can perform this operation")
            }

            // Token Integration Errors
            OrderError::TokenCallFailed(msg) => write!(f, "Token canister call failed: {}", msg),
            OrderError::TransferFailed(msg) => write!(f, "Token transfer failed: {}", msg),
            OrderError::BalanceCheckFailed(msg) => write!(f, "Balance check failed: {}", msg),
            OrderError::TokenNotSupported(msg) => write!(f, "Token not supported: {}", msg),

            // System Errors
            OrderError::SystemError(msg) => write!(f, "System error: {}", msg),
            OrderError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            OrderError::ConcurrencyError(msg) => write!(f, "Concurrency error: {}", msg),

            // Rate Limiting & DoS Protection
            OrderError::TooManyOrders => write!(
                f,
                "Too many active orders. Please cancel some orders before creating new ones"
            ),
            OrderError::OrderCreationRateLimited => {
                write!(f, "Order creation rate limited. Please wait before creating another order")
            }
            OrderError::SystemOverloaded => {
                write!(f, "System is currently overloaded. Please try again later")
            }
        }
    }
}

impl Order {
    /// Get the current state of the order based on global state and expiration
    pub fn get_state(
        &self,
        filled_orders: &std::collections::HashSet<OrderId>,
        cancelled_orders: &std::collections::HashSet<OrderId>,
    ) -> OrderState {
        let current_time = ic_cdk::api::time();

        // Check expiration first
        if self.expiration <= current_time {
            return OrderState::Expired;
        }

        // Check if filled
        if filled_orders.contains(&self.id) {
            return OrderState::Filled;
        }

        // Check if cancelled
        if cancelled_orders.contains(&self.id) {
            return OrderState::Cancelled;
        }

        OrderState::Active
    }

    /// Check if the order is active (not filled, cancelled, or expired)
    pub fn is_active(
        &self,
        filled_orders: &std::collections::HashSet<OrderId>,
        cancelled_orders: &std::collections::HashSet<OrderId>,
    ) -> bool {
        self.get_state(filled_orders, cancelled_orders) == OrderState::Active
    }

    /// Validate order parameters
    pub fn validate(&self) -> OrderResult<()> {
        // Amount validation
        if self.making_amount == 0 || self.taking_amount == 0 {
            return Err(OrderError::InvalidAmount);
        }

        // Asset pair validation
        if self.maker_asset == self.taker_asset {
            return Err(OrderError::InvalidAssetPair);
        }

        // Expiration validation
        let current_time = ic_cdk::api::time();
        if self.expiration <= current_time {
            return Err(OrderError::InvalidExpiration);
        }

        // Maximum expiration validation (30 days)
        let max_expiration = current_time + (MAX_EXPIRATION_DAYS * 24 * 3600 * 1_000_000_000);
        if self.expiration > max_expiration {
            return Err(OrderError::InvalidExpiration);
        }

        Ok(())
    }
}

impl SystemStats {
    /// Increment order creation counter
    pub fn increment_orders_created(&mut self) {
        self.orders_created += 1;
    }

    /// Increment order filled counter and update volume
    pub fn increment_orders_filled(&mut self, token: Principal, volume: u64) {
        self.orders_filled += 1;
        *self.total_volume.entry(token).or_insert(0) += volume;
    }

    /// Increment order cancelled counter
    pub fn increment_orders_cancelled(&mut self) {
        self.orders_cancelled += 1;
    }

    /// Track error occurrence
    pub fn track_error(&mut self, error_type: &str) {
        *self.error_counts.entry(error_type.to_string()).or_insert(0) += 1;
    }
}
