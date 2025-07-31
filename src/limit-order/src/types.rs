use crate::memory::track_error;
use candid::{CandidType, Deserialize, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
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
}

// Result types for limit order operations
pub type OrderResult<T> = std::result::Result<T, OrderError>;

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

// ============================================================================
// ICRC TOKEN INTEGRATION
// ============================================================================

/// Token interface for ICRC-1 integration
pub struct TokenInterface {
    pub canister_id: Principal,
}

impl TokenInterface {
    /// Create a new token interface for the given canister
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    /// Check balance of an account using ICRC-1 balance_of method
    /// NOTE: icrc1_balance_of is a QUERY method, not an UPDATE method
    /// We use ic_cdk::api::call::call() for query methods, not ic_cdk::call() for update methods
    pub async fn balance_of(&self, account: Principal) -> OrderResult<u64> {
        let account_arg = Account { owner: account, subaccount: None };

        let result: std::result::Result<(candid::Nat,), _> =
            ic_cdk::api::call::call(self.canister_id, "icrc1_balance_of", (account_arg,)).await;

        match result {
            Ok((balance,)) => {
                // Convert candid::Nat to u64 (ICRC-1 tokens return nat)
                match balance.0.try_into() {
                    Ok(balance_u64) => Ok(balance_u64),
                    Err(_) => {
                        let error_msg = "Balance too large for u64".to_string();
                        track_error("balance_overflow");
                        Err(OrderError::BalanceCheckFailed(error_msg))
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Balance check failed: {:?}", e);
                track_error("balance_check_failed");
                Err(OrderError::TokenCallFailed(error_msg))
            }
        }
    }

    /// Transfer tokens using ICRC-2 transfer_from method
    /// This requires the user to have approved the backend canister to spend their tokens
    pub async fn transfer(&self, from: Principal, to: Principal, amount: u64) -> OrderResult<u64> {
        // Use ICRC-2 transfer_from for third-party transfers
        let transfer_from_args = TransferFromArgs {
            spender_subaccount: None,
            from: Account { owner: from, subaccount: None },
            to: Account { owner: to, subaccount: None },
            amount: candid::Nat::from(amount),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let result: std::result::Result<(std::result::Result<u64, TransferFromError>,), _> =
            ic_cdk::call(self.canister_id, "icrc2_transfer_from", (transfer_from_args,)).await;

        match result {
            Ok((Ok(block_index),)) => Ok(block_index),
            Ok((Err(transfer_error),)) => {
                let error_msg = format!("Transfer failed: {:?}", transfer_error);
                track_error("transfer_failed");
                Err(OrderError::TransferFailed(error_msg))
            }
            Err(call_error) => {
                let error_msg = format!("Transfer call failed: {:?}", call_error);
                track_error("transfer_call_failed");
                Err(OrderError::TokenCallFailed(error_msg))
            }
        }
    }
}
