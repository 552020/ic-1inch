use candid::{CandidType, Deserialize, Principal};
use icrc_ledger_types::icrc1::account::Account;
use std::collections::HashMap;

// ============================================================================
// CORE LOP TYPES - Order Management and Token Swaps
// ============================================================================

pub type OrderId = u64;

// System limits
pub const MAX_ACTIVE_ORDERS: usize = 10_000;
pub const MAX_EXPIRATION_DAYS: u64 = 30;

// Core order structure
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

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct OrderMetadata {
    // Reserved for future cross-chain fields
    pub hashlock: Option<Vec<u8>>,
    pub timelock: Option<u64>,
    pub target_chain: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderState {
    Active,    // Order is available for filling
    Filled,    // Order has been completely filled
    Cancelled, // Order has been cancelled by maker
    Expired,   // Order has passed expiration time
}

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

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SystemStats {
    pub orders_created: u64,
    pub orders_filled: u64,
    pub orders_cancelled: u64,
    pub total_volume: HashMap<Principal, u64>, // Per token
    pub error_counts: HashMap<String, u64>,    // Error frequency tracking
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateOrderParams {
    pub receiver: Principal,
    pub maker_asset: Principal,
    pub taker_asset: Principal,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expiration: u64,
}

pub type OrderResult<T> = std::result::Result<T, OrderError>;

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OrderError::InvalidAmount => write!(f, "Invalid amount"),
            OrderError::InvalidExpiration => write!(f, "Invalid expiration time"),
            OrderError::InvalidAssetPair => write!(f, "Invalid asset pair"),
            OrderError::InvalidReceiver => write!(f, "Invalid receiver"),
            OrderError::InvalidOrderId => write!(f, "Invalid order ID"),
            OrderError::InvalidPrincipal => write!(f, "Invalid principal"),
            OrderError::OrderNotFound => write!(f, "Order not found"),
            OrderError::OrderAlreadyFilled => write!(f, "Order already filled"),
            OrderError::OrderCancelled => write!(f, "Order cancelled"),
            OrderError::OrderExpired => write!(f, "Order expired"),
            OrderError::OrderInactive => write!(f, "Order inactive"),
            OrderError::Unauthorized => write!(f, "Unauthorized"),
            OrderError::InsufficientBalance => write!(f, "Insufficient balance"),
            OrderError::AnonymousCaller => write!(f, "Anonymous caller not allowed"),
            OrderError::NotOrderMaker => write!(f, "Not the order maker"),
            OrderError::TokenCallFailed(msg) => write!(f, "Token call failed: {}", msg),
            OrderError::TransferFailed(msg) => write!(f, "Transfer failed: {}", msg),
            OrderError::BalanceCheckFailed(msg) => write!(f, "Balance check failed: {}", msg),
            OrderError::TokenNotSupported(msg) => write!(f, "Token not supported: {}", msg),
            OrderError::SystemError(msg) => write!(f, "System error: {}", msg),
            OrderError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            OrderError::ConcurrencyError(msg) => write!(f, "Concurrency error: {}", msg),
            OrderError::TooManyOrders => write!(f, "Too many orders"),
            OrderError::OrderCreationRateLimited => write!(f, "Order creation rate limited"),
            OrderError::SystemOverloaded => write!(f, "System overloaded"),
        }
    }
}

impl Order {
    /// Get the current state of the order
    pub fn get_state(
        &self,
        filled_orders: &std::collections::HashSet<OrderId>,
        cancelled_orders: &std::collections::HashSet<OrderId>,
    ) -> OrderState {
        if filled_orders.contains(&self.id) {
            OrderState::Filled
        } else if cancelled_orders.contains(&self.id) {
            OrderState::Cancelled
        } else if self.expiration <= ic_cdk::api::time() {
            OrderState::Expired
        } else {
            OrderState::Active
        }
    }

    /// Check if the order is active (can be filled)
    pub fn is_active(
        &self,
        filled_orders: &std::collections::HashSet<OrderId>,
        cancelled_orders: &std::collections::HashSet<OrderId>,
    ) -> bool {
        self.get_state(filled_orders, cancelled_orders) == OrderState::Active
    }
}

impl SystemStats {
    pub fn default() -> Self {
        Self {
            orders_created: 0,
            orders_filled: 0,
            orders_cancelled: 0,
            total_volume: HashMap::new(),
            error_counts: HashMap::new(),
        }
    }

    pub fn increment_orders_created(&mut self) {
        self.orders_created += 1;
    }

    pub fn increment_orders_filled(&mut self, token: Principal, volume: u64) {
        self.orders_filled += 1;
        *self.total_volume.entry(token).or_insert(0) += volume;
    }

    pub fn increment_orders_cancelled(&mut self) {
        self.orders_cancelled += 1;
    }

    pub fn track_error(&mut self, error_type: &str) {
        *self.error_counts.entry(error_type.to_string()).or_insert(0) += 1;
    }
}

// ============================================================================
// TOKEN INTERFACE - ICRC Integration
// ============================================================================

#[derive(Clone, Debug)]
pub struct TokenInterface {
    pub canister_id: Principal,
}

impl TokenInterface {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    pub async fn balance_of(&self, account: Principal) -> OrderResult<u64> {
        let account_arg = Account { owner: account, subaccount: None };

        let result: std::result::Result<(candid::Nat,), _> =
            ic_cdk::api::call::call(self.canister_id, "icrc1_balance_of", (account_arg,)).await;

        match result {
            Ok((balance,)) => {
                // Convert candid::Nat to u64
                match balance.0.try_into() {
                    Ok(balance_u64) => Ok(balance_u64),
                    Err(_) => {
                        Err(OrderError::BalanceCheckFailed("Balance too large for u64".to_string()))
                    }
                }
            }
            Err(e) => Err(OrderError::TokenCallFailed(format!("Balance check failed: {:?}", e))),
        }
    }

    pub async fn transfer(&self, from: Principal, to: Principal, amount: u64) -> OrderResult<u64> {
        let transfer_arg = icrc_ledger_types::icrc1::transfer::TransferArg {
            from_subaccount: None,
            to: Account { owner: to, subaccount: None },
            amount: candid::Nat::from(amount),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let result: std::result::Result<
            (std::result::Result<candid::Nat, icrc_ledger_types::icrc1::transfer::TransferError>,),
            _,
        > = ic_cdk::call(self.canister_id, "icrc1_transfer", (transfer_arg,)).await;

        match result {
            Ok((Ok(block_index),)) => {
                // Convert candid::Nat to u64
                match block_index.0.try_into() {
                    Ok(block_index_u64) => Ok(block_index_u64),
                    Err(_) => {
                        Err(OrderError::TransferFailed("Block index too large for u64".to_string()))
                    }
                }
            }
            Ok((Err(transfer_error),)) => {
                Err(OrderError::TransferFailed(format!("Transfer failed: {:?}", transfer_error)))
            }
            Err(call_error) => {
                Err(OrderError::TokenCallFailed(format!("Transfer call failed: {:?}", call_error)))
            }
        }
    }
}
