# Orderbook Canister Requirements & Specification

## Overview

The Orderbook Canister serves as the central coordination hub for the ICP <> EVM Cross-Chain Fusion protocol. It manages virtual escrow state, orchestrates cross-chain operations, and provides the primary interface for order creation, management, and execution.

**Key Architecture Insight:** The orderbook canister takes over most traditional relayer functions, but we maintain off-chain communication for EVM <> ETH interactions through the frontend, following the 1inch Fusion resolver pattern.

## Core Requirements

### 1. Order Management

#### 1.1 Order Creation

- **Function**: `create_order(order: Order) -> Result<String, OrderError>`
- **Purpose**: Accept new swap orders from users
- **Requirements**:
  - Validate order parameters (amounts, tokens, timelocks)
  - Generate unique order ID
  - Store order in canister state
  - Trigger automatic ICP escrow creation for ICP → EVM orders
  - Return order ID for tracking

#### 1.2 Order Retrieval

- **Function**: `get_order(order_id: String) -> Option<Order>`
- **Purpose**: Retrieve order details by ID
- **Requirements**:
  - Return complete order data
  - Include current state and escrow addresses
  - Handle non-existent orders gracefully

#### 1.3 Order Listing

- **Function**: `get_orders(filters: OrderFilters) -> Vec<Order>`
- **Purpose**: List orders with filtering capabilities
- **Requirements**:
  - Support filtering by maker, status, token pairs
  - Pagination support for large order sets
  - Efficient querying with indexed fields

#### 1.4 Order Status Updates

- **Function**: `update_order_status(order_id: String, status: OrderStatus) -> Result<(), OrderError>`
- **Purpose**: Update order state during execution
- **Requirements**:
  - Validate state transitions
  - Trigger side effects based on status changes
  - Maintain audit trail of state changes

### 2. Virtual Escrow Management

#### 2.1 Virtual Escrow Creation

- **Function**: `create_virtual_escrow(order_id: String, escrow_type: EscrowType) -> Result<String, EscrowError>`
- **Purpose**: Create virtual escrow representations
- **Requirements**:
  - Generate virtual escrow ID
  - Store escrow metadata
  - Link to actual escrow addresses when deployed
  - Support both ICP and EVM escrow types

#### 2.2 Escrow Address Management

- **Function**: `update_escrow_addresses(order_id: String, addresses: EscrowAddresses) -> Result<(), EscrowError>`
- **Purpose**: Update escrow addresses when deployed
- **Requirements**:
  - Validate escrow addresses
  - Update virtual escrow state
  - Trigger state transitions when both escrows ready

#### 2.3 Escrow State Tracking

- **Function**: `get_escrow_status(escrow_id: String) -> Result<EscrowInfo, EscrowError>`
- **Purpose**: Track escrow deployment and state
- **Requirements**:
  - Monitor deployment status
  - Track asset locking state
  - Provide state verification for frontend

### 3. Cross-Chain Coordination

#### 3.1 EVM Escrow Creation

- **Function**: `create_evm_escrow(order_id: String) -> Result<String, EscrowError>`
- **Purpose**: Coordinate EVM escrow creation via Chain Fusion
- **Requirements**:
  - Submit EVM transaction via EVM Coordinator
  - Track transaction status
  - Handle deployment failures
  - Update virtual escrow state

#### 3.2 ICP Escrow Creation

- **Function**: `create_icp_escrow(order_id: String) -> Result<String, EscrowError>`
- **Purpose**: Create ICP escrow for ICP-side assets
- **Requirements**:
  - Call ICP Escrow Factory
  - Lock user assets (reverse gas model)
  - Update virtual escrow state
  - Handle insufficient balance errors

#### 3.3 Secret Revelation

- **Function**: `reveal_secret(order_id: String, secret: Vec<u8>) -> Result<(), OrderError>`
- **Purpose**: Complete swap by revealing hashlock secret
- **Requirements**:
  - Validate secret against hashlock
  - Trigger asset release on both chains
  - Update order status to completed
  - Handle partial failures with rollback

#### 3.4 Off-Chain Communication (Frontend-Mediated)

- **Purpose**: Handle EVM <> ETH interactions that can't be done on-chain
- **Requirements**:
  - **EIP-712 Signature Validation**: Validate user signatures for EVM orders
  - **LOP Integration**: Interface with 1inch Limit Order Protocol
  - **Resolver Coordination**: Coordinate with Fusion resolvers
  - **Frontend Bridge**: Use frontend as communication bridge for off-chain parts
  - **Signature Verification**: Verify EIP-712 signatures for cross-chain parameters
  - **Whitelist Management**: Handle resolver whitelisting in LOP system

### 4. State Machine Management

#### 4.1 State Transitions

```rust
enum OrderState {
    Created,              // Order created, no escrows
    ICPEscrowCreated,     // ICP escrow created
    EVMEscrowCreated,     // EVM escrow created
    BothEscrowsReady,     // Both escrows deployed and ready
    SwapCompleted,        // Secret revealed, assets transferred
    Cancelled,           // Order cancelled by maker
    Refunded,            // Timelock expired, assets refunded
    Failed               // Error occurred during execution
}
```

#### 4.2 State Validation

- **Function**: `validate_state_transition(current: OrderState, new: OrderState) -> bool`
- **Purpose**: Ensure valid state transitions
- **Requirements**:
  - Define allowed transitions
  - Prevent invalid state changes
  - Log state transition events

### 5. Timer-Based Coordination

#### 5.1 Periodic State Checks

- **Function**: `check_order_timeouts() -> Result<(), TimerError>`
- **Purpose**: Handle timelock expirations
- **Requirements**:
  - Check all active orders for expired timelocks
  - Trigger refund processes
  - Update order states
  - Clean up completed orders

#### 5.2 Event-Driven Updates

- **Function**: `handle_evm_event(event: EVMEvent) -> Result<(), EventError>`
- **Purpose**: Process EVM events via Chain Fusion
- **Requirements**:
  - Parse EVM events from logs
  - Update order states based on events
  - Trigger appropriate actions

### 6. Data Structures

#### 6.1 Order Structure

```rust
#[derive(CandidType, Deserialize, Clone)]
pub struct Order {
    pub id: String,
    pub maker: Principal,
    pub taker: Option<Principal>,
    pub maker_asset: String,    // "ICP" or "ETH"
    pub taker_asset: String,    // "ETH" or "ICP"
    pub making_amount: u64,
    pub taking_amount: u64,
    pub hashlock: [u8; 32],
    pub timelock: u64,
    pub state: OrderState,
    pub icp_escrow_id: Option<String>,
    pub evm_escrow_address: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub evm_signature: Option<String>, // For EVM → ICP orders
    pub resolver: Option<Principal>,
    pub fill_tx_hash: Option<String>,
}
```

#### 6.2 Virtual Escrow Structure

```rust
#[derive(CandidType, Deserialize, Clone)]
pub struct VirtualEscrow {
    pub id: String,
    pub order_id: String,
    pub escrow_type: EscrowType,
    pub target_chain: Chain,
    pub target_address: Option<String>,
    pub status: EscrowStatus,

    // Single escrow parameters
    pub source_token: String,
    pub destination_token: String,
    pub source_amount: u128,
    pub destination_amount: u128,
    pub maker: Principal,
    pub taker: Principal,
    pub hashlock: String,
    pub timelocks: TimelockConfig,
    pub safety_deposit: u128,

    // Migration tracking
    pub migration_tx_hash: Option<String>,
    pub deployed_at: Option<u64>,
    pub error: Option<String>,
}
```

#### 6.3 Order Filters

```rust
#[derive(CandidType, Deserialize)]
pub struct OrderFilters {
    pub maker: Option<Principal>,
    pub status: Option<OrderState>,
    pub source_token: Option<String>,
    pub destination_token: Option<String>,
    pub min_amount: Option<u64>,
    pub max_amount: Option<u64>,
    pub created_after: Option<u64>,
    pub created_before: Option<u64>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
```

### 7. Error Handling

#### 7.1 Order Errors

```rust
#[derive(CandidType, Deserialize)]
pub enum OrderError {
    InvalidAssetPair,
    OrderCancelled,
    BalanceCheckFailed(String),
    InvalidAmount,
    TokenNotSupported(String),
    MemoryError(String),
    OrderInactive,
    OrderCreationRateLimited,
    NotOrderMaker,
    SystemError(String),
    OrderNotFound,
    InsufficientBalance,
    InvalidPrincipal,
    OrderExpired,
    InvalidReceiver,
    Unauthorized,
    OrderAlreadyFilled,
    InvalidExpiration,
    ConcurrencyError(String),
    TooManyOrders,
    TokenCallFailed(String),
    TransferFailed(String),
    InvalidOrderId,
    SystemOverloaded,
    AnonymousCaller,
}
```

#### 7.2 Escrow Errors

```rust
#[derive(CandidType, Deserialize)]
pub enum EscrowError {
    InvalidAmount,
    InsufficientBalance,
    InvalidEscrowType,
    EscrowNotFound,
    TimelockExpired,
    Unauthorized,
    InvalidTimelock,
    InvalidHashlock,
    TransferFailed,
    TimelockNotExpired,
    InvalidState,
}
```

### 8. Integration Requirements

#### 8.1 ICP Escrow Factory Integration

- **Dependency**: `icp_escrow_factory` canister
- **Functions**:
  - `create_escrow()` - Create ICP escrow
  - `release_escrow()` - Release funds with secret
  - `refund_escrow()` - Refund after timelock
  - `get_escrow_state()` - Query escrow state

#### 8.2 EVM Coordinator Integration

- **Dependency**: `evm_coordinator` canister
- **Functions**:
  - `create_evm_escrow()` - Submit EVM escrow creation
  - `verify_evm_escrow()` - Verify EVM escrow state
  - `submit_evm_transaction()` - Generic EVM transaction
  - `poll_evm_events()` - Poll EVM logs

#### 8.3 Token Ledger Integration

- **Dependency**: `token_ledger` canister
- **Functions**:
  - `transfer_to_escrow()` - Transfer tokens to escrow
  - `transfer_from_escrow()` - Transfer from escrow
  - `get_balance()` - Query user balance
  - `mint_tokens()` - Mint tokens (testing)

#### 8.4 Frontend Integration (Off-Chain Bridge)

- **Purpose**: Handle EVM <> ETH interactions through frontend
- **Functions**:
  - `validate_evm_signature()` - Verify EIP-712 signatures
  - `submit_lop_order()` - Submit orders to 1inch LOP
  - `coordinate_resolver()` - Coordinate with Fusion resolvers
  - `handle_whitelist()` - Manage resolver whitelisting
  - `process_offchain_events()` - Process off-chain events

### 9. Hybrid Architecture: On-Chain vs Off-Chain

#### 9.1 On-Chain Components (Orderbook Canister)

- **Order Management**: Create, retrieve, update orders
- **Virtual Escrow State**: Track escrow deployment and state
- **ICP Escrow Creation**: Direct canister-to-canister calls
- **State Machine**: Manage order state transitions
- **Timer Coordination**: Handle timelock expirations
- **Secret Revelation**: Complete swaps with hashlock secrets

#### 9.2 Off-Chain Components (Frontend-Mediated)

- **EIP-712 Signatures**: User signature validation for EVM orders
- **LOP Integration**: Interface with 1inch Limit Order Protocol
- **Resolver Coordination**: Coordinate with Fusion resolvers
- **Whitelist Management**: Handle resolver whitelisting in LOP
- **Cross-Chain Communication**: Bridge EVM <> ETH interactions

#### 9.3 Communication Flow

```
User → Frontend → Orderbook Canister (On-Chain)
                ↓
            Frontend → LOP (Off-Chain)
                ↓
            Frontend → Resolver (Off-Chain)
                ↓
            Orderbook Canister → EVM (On-Chain via Chain Fusion)
```

### 10. Security Requirements

#### 10.1 Access Control

- **Maker Authorization**: Only order maker can modify their orders
- **Resolver Authorization**: Only assigned resolver can execute orders
- **Admin Functions**: Restricted to authorized principals
- **Rate Limiting**: Prevent spam and DoS attacks

#### 10.2 Input Validation

- **Order Parameters**: Validate amounts, tokens, timelocks
- **Principal Validation**: Ensure valid ICP principals
- **Hashlock Validation**: Verify hashlock format and uniqueness
- **Timelock Validation**: Ensure reasonable timelock durations
- **EIP-712 Validation**: Verify EVM order signatures

#### 10.3 State Consistency

- **Atomic Operations**: Ensure state changes are atomic
- **Rollback Mechanisms**: Handle partial failures
- **Idempotent Operations**: Safe to retry failed operations
- **Audit Trail**: Log all state changes for debugging

### 11. Fusion Resolver Integration Requirements

#### 11.1 Resolver Contract Integration

- **ITakerInteraction Interface**: Implement required interface for LOP integration
- **Whitelist Management**: Handle resolver whitelisting in LOP system
- **Order Execution**: Coordinate with LOP for order settlement
- **Escrow Creation**: Trigger escrow creation via resolver callbacks

#### 11.2 EIP-712 Signature Handling

- **Signature Validation**: Verify EIP-712 signatures for EVM orders
- **Cross-Chain Parameters**: Ensure signed parameters match order details
- **Signature Storage**: Store validated signatures for order execution
- **Replay Protection**: Prevent signature reuse across orders

#### 11.3 LOP Integration

- **Order Submission**: Submit orders to 1inch Limit Order Protocol
- **Order Settlement**: Handle order settlement through LOP
- **Callback Processing**: Process takerInteraction callbacks from LOP
- **Whitelist Coordination**: Manage resolver whitelist status

### 12. Performance Requirements

#### 12.1 Scalability

- **Order Storage**: Efficient storage for thousands of orders
- **Query Performance**: Fast order retrieval and filtering
- **Memory Management**: Optimize memory usage for large datasets
- **Batch Operations**: Support batch processing for efficiency

#### 12.2 Reliability

- **Fault Tolerance**: Handle canister upgrades gracefully
- **Error Recovery**: Automatic recovery from transient failures
- **State Persistence**: Ensure state survives canister restarts
- **Monitoring**: Comprehensive logging and metrics

### 13. Testing Requirements

#### 13.1 Unit Testing

- **Order Creation**: Test order creation with various parameters
- **State Transitions**: Test all valid state transitions
- **Error Handling**: Test error conditions and edge cases
- **Integration**: Test canister interactions
- **EIP-712 Validation**: Test signature validation for EVM orders

#### 13.2 Integration Testing

- **Cross-Chain Flow**: Test complete EVM ↔ ICP flows
- **Escrow Creation**: Test escrow creation on both chains
- **Secret Revelation**: Test swap completion with secret
- **Timeout Handling**: Test timelock expiration scenarios
- **LOP Integration**: Test 1inch Limit Order Protocol integration
- **Resolver Coordination**: Test Fusion resolver integration

#### 13.3 Performance Testing

- **Load Testing**: Test with high order volumes
- **Stress Testing**: Test under failure conditions
- **Memory Testing**: Test memory usage under load
- **Gas Testing**: Test gas efficiency of operations

### 14. Deployment Requirements

#### 14.1 Configuration

- **EVM RPC Endpoints**: Configure multiple RPC providers
- **Threshold ECDSA**: Configure signing keys for EVM
- **Timelock Settings**: Configure default timelock durations
- **Gas Limits**: Configure gas limits for EVM transactions
- **LOP Configuration**: Configure 1inch Limit Order Protocol endpoints
- **Resolver Whitelist**: Configure resolver whitelist settings

#### 14.2 Monitoring

- **Metrics Collection**: Track order creation, execution, failures
- **Alert System**: Alert on critical failures
- **Logging**: Comprehensive logging for debugging
- **Health Checks**: Monitor canister health and performance
- **LOP Integration**: Monitor LOP integration status
- **Resolver Status**: Monitor resolver whitelist and execution status

### 15. Documentation Requirements

#### 15.1 API Documentation

- **Function Signatures**: Complete Candid interface
- **Parameter Descriptions**: Detailed parameter documentation
- **Return Values**: Document all return types and error codes
- **Examples**: Provide usage examples for common operations
- **EIP-712 Integration**: Document signature validation requirements
- **LOP Integration**: Document Limit Order Protocol integration

#### 15.2 Integration Guide

- **Frontend Integration**: Guide for frontend developers
- **Resolver Integration**: Guide for resolver services
- **LOP Integration**: Guide for 1inch Limit Order Protocol integration
- **Fusion Resolver**: Guide for Fusion resolver pattern implementation
- **Testing Guide**: Guide for testing the canister
- **Deployment Guide**: Guide for deploying and configuring

This comprehensive specification provides the foundation for implementing the Orderbook Canister as the central coordination hub for the ICP <> EVM Cross-Chain Fusion protocol, incorporating both on-chain canister functionality and off-chain communication through the frontend following the 1inch Fusion resolver pattern.
