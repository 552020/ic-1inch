# Rust Backend Architecture: ICP <> EVM Fusion

## Executive Summary

This document outlines the **Rust-based backend architecture** for the ICP <> EVM cross-chain fusion protocol, including canister development, backend services, and cross-chain coordination.

## 1. Technology Stack

### 1.1 Core Technologies

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   ICP Canisters │    │  Rust Backend   │    │  EVM Contracts  │
│   (Rust/Motoko) │◄──►│   Services      │◄──►│   (Solidity)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Frontend       │    │  Relayer        │    │  Resolver       │
│  (TypeScript)   │    │  Service        │    │  Network        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### 1.2 Rust Components

- **ICP Orderbook Canister**: Rust canister for order management
- **ICP Escrow Canisters**: Rust canisters for asset locking
- **Backend Services**: Rust services for coordination
- **Relayer Service**: Rust service for order routing
- **Resolver SDK**: Rust library for resolver integration

## 2. ICP Canister Development (Rust)

### 2.1 Orderbook Canister

```rust
// src/orderbook_canister.rs
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::call::call_with_payment;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct VirtualEscrow {
    pub id: String,
    pub order_id: String,
    pub escrow_type: EscrowType,
    pub target_chain: Chain,
    pub target_address: Option<String>,
    pub status: EscrowStatus,

    // Escrow parameters
    pub maker: Principal,
    pub taker: Principal,
    pub token: String,
    pub amount: u128,
    pub hashlock: String,
    pub timelocks: TimelockConfig,
    pub safety_deposit: u128,

    // Migration tracking
    pub migration_tx_hash: Option<String>,
    pub deployed_at: Option<u64>,
    pub error: Option<String>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct OrderWithVirtualEscrows {
    pub id: String,
    pub order_type: OrderType,
    pub source_chain: String,
    pub destination_chain: String,
    pub source_token: String,
    pub destination_token: String,
    pub source_amount: u128,
    pub destination_amount: u128,
    pub hashlock: String,
    pub timelocks: TimelockConfig,
    pub status: OrderStatus,

    // Virtual escrows
    pub source_escrow: VirtualEscrow,
    pub destination_escrow: VirtualEscrow,

    // Order metadata
    pub created_at: u64,
    pub evm_signature: Option<String>,
    pub resolver: Option<Principal>,
    pub fill_tx_hash: Option<String>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum EscrowType {
    Source,
    Destination,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Chain {
    ICP,
    ETH,
    POLYGON,
    ARBITRUM,
    BASE,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum EscrowStatus {
    Virtual,
    Migrating,
    Deployed,
    Failed,
    Cancelled,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum OrderType {
    EVMToICP,
    ICPToEVM,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
    Expired,
}

pub struct OrderbookCanister {
    orders: HashMap<String, OrderWithVirtualEscrows>,
    virtual_escrows: HashMap<String, VirtualEscrow>,
}

impl OrderbookCanister {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
            virtual_escrows: HashMap::new(),
        }
    }

    // Create order with virtual escrows
    pub async fn create_order_with_virtual_escrows(
        &mut self,
        order: Order,
    ) -> Result<String, String> {
        let order_id = self.generate_order_id();

        // Create virtual source escrow
        let source_escrow = VirtualEscrow {
            id: self.generate_escrow_id(),
            order_id: order_id.clone(),
            escrow_type: EscrowType::Source,
            target_chain: order.source_chain.parse().unwrap(),
            target_address: None,
            status: EscrowStatus::Virtual,
            maker: order.maker,
            taker: order.taker,
            token: order.source_token,
            amount: order.source_amount,
            hashlock: order.hashlock.clone(),
            timelocks: order.timelocks.clone(),
            safety_deposit: self.calculate_safety_deposit(&order.source_chain),
            migration_tx_hash: None,
            deployed_at: None,
            error: None,
        };

        // Create virtual destination escrow
        let destination_escrow = VirtualEscrow {
            id: self.generate_escrow_id(),
            order_id: order_id.clone(),
            escrow_type: EscrowType::Destination,
            target_chain: order.destination_chain.parse().unwrap(),
            target_address: None,
            status: EscrowStatus::Virtual,
            maker: order.maker,
            taker: order.taker,
            token: order.destination_token,
            amount: order.destination_amount,
            hashlock: order.hashlock,
            timelocks: order.timelocks,
            safety_deposit: self.calculate_safety_deposit(&order.destination_chain),
            migration_tx_hash: None,
            deployed_at: None,
            error: None,
        };

        // Store order with virtual escrows
        let order_with_escrows = OrderWithVirtualEscrows {
            id: order_id.clone(),
            order_type: order.order_type,
            source_chain: order.source_chain,
            destination_chain: order.destination_chain,
            source_token: order.source_token,
            destination_token: order.destination_token,
            source_amount: order.source_amount,
            destination_amount: order.destination_amount,
            hashlock: order.hashlock,
            timelocks: order.timelocks,
            status: OrderStatus::Pending,
            source_escrow,
            destination_escrow,
            created_at: ic_cdk::api::time(),
            evm_signature: order.evm_signature,
            resolver: None,
            fill_tx_hash: None,
        };

        self.orders.insert(order_id.clone(), order_with_escrows);
        self.virtual_escrows.insert(source_escrow.id.clone(), source_escrow);
        self.virtual_escrows.insert(destination_escrow.id.clone(), destination_escrow);

        Ok(order_id)
    }

    // Migrate virtual escrow to actual chain
    pub async fn migrate_escrow(
        &mut self,
        escrow_id: &str,
        target_address: &str,
    ) -> Result<bool, String> {
        let virtual_escrow = self.virtual_escrows.get_mut(escrow_id)
            .ok_or("Virtual escrow not found")?;

        if !matches!(virtual_escrow.status, EscrowStatus::Virtual) {
            return Err("Escrow is not in virtual state".to_string());
        }

        virtual_escrow.status = EscrowStatus::Migrating;
        virtual_escrow.target_address = Some(target_address.to_string());

        match virtual_escrow.target_chain {
            Chain::ICP => {
                self.migrate_to_icp_escrow(virtual_escrow).await?;
            }
            _ => {
                self.migrate_to_evm_escrow(virtual_escrow).await?;
            }
        }

        virtual_escrow.status = EscrowStatus::Deployed;
        virtual_escrow.deployed_at = Some(ic_cdk::api::time());

        Ok(true)
    }

    // Get order with virtual escrows
    pub fn get_order_with_escrows(&self, order_id: &str) -> Option<&OrderWithVirtualEscrows> {
        self.orders.get(order_id)
    }

    // Get pending orders for resolvers
    pub fn get_pending_orders(&self) -> Vec<&OrderWithVirtualEscrows> {
        self.orders.values()
            .filter(|order| matches!(order.status, OrderStatus::Pending))
            .collect()
    }

    // Helper methods
    fn generate_order_id(&self) -> String {
        format!("order_{}", ic_cdk::api::time())
    }

    fn generate_escrow_id(&self) -> String {
        format!("escrow_{}", ic_cdk::api::time())
    }

    fn calculate_safety_deposit(&self, chain: &str) -> u128 {
        match chain {
            "ICP" => 1_000_000, // 0.001 ICP
            "ETH" => 10_000_000_000_000_000, // 0.01 ETH
            "POLYGON" => 10_000_000_000_000_000, // 0.01 MATIC
            _ => 1_000_000_000_000_000, // Default
        }
    }
}
```

### 2.2 ICP Escrow Canister

```rust
// src/icp_escrow_canister.rs
use candid::{CandidType, Deserialize, Principal};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct LockedAsset {
    pub user: Principal,
    pub token: String,
    pub amount: u128,
    pub hashlock: String,
    pub timelocks: TimelockConfig,
    pub locked_at: u64,
}

pub struct ICPEscrowCanister {
    locked_assets: HashMap<String, LockedAsset>,
    canister_id: Principal,
}

impl ICPEscrowCanister {
    pub fn new(canister_id: Principal) -> Self {
        Self {
            locked_assets: HashMap::new(),
            canister_id,
        }
    }

    // Lock assets in ICP escrow
    pub async fn lock_assets(&mut self, params: LockParams) -> Result<(), String> {
        // 1. Transfer user assets to escrow canister
        self.transfer_from_user(params.user, &params.token, params.amount).await?;

        // 2. Lock assets with hashlock
        let locked_asset = LockedAsset {
            user: params.user,
            token: params.token,
            amount: params.amount,
            hashlock: params.hashlock.clone(),
            timelocks: params.timelocks,
            locked_at: ic_cdk::api::time(),
        };

        self.locked_assets.insert(params.hashlock, locked_asset);

        Ok(())
    }

    // Unlock assets with secret
    pub async fn unlock_assets(&mut self, secret: &str) -> Result<(), String> {
        let hashlock = self.generate_hashlock(secret);
        let locked_asset = self.locked_assets.get(&hashlock)
            .ok_or("No locked assets found for hashlock")?;

        // Transfer assets to resolver (who will complete the swap)
        self.transfer_to_resolver(locked_asset).await?;

        // Remove from locked assets
        self.locked_assets.remove(&hashlock);

        Ok(())
    }

    // Generate hashlock from secret
    fn generate_hashlock(&self, secret: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // Transfer assets from user to escrow
    async fn transfer_from_user(&self, user: Principal, token: &str, amount: u128) -> Result<(), String> {
        // Call ICP ledger or token canister to transfer assets
        // This is a simplified version - actual implementation would use ICP ledger
        Ok(())
    }

    // Transfer assets to resolver
    async fn transfer_to_resolver(&self, locked_asset: &LockedAsset) -> Result<(), String> {
        // Transfer assets to the resolver who will complete the swap
        // This is a simplified version - actual implementation would use ICP ledger
        Ok(())
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct LockParams {
    pub user: Principal,
    pub token: String,
    pub amount: u128,
    pub hashlock: String,
    pub timelocks: TimelockConfig,
}
```

## 3. Rust Backend Services

### 3.1 Relayer Service

```rust
// src/relayer_service.rs
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub order_type: OrderType,
    pub source_chain: String,
    pub destination_chain: String,
    pub source_token: String,
    pub destination_token: String,
    pub source_amount: u128,
    pub destination_amount: u128,
    pub hashlock: String,
    pub timelocks: TimelockConfig,
    pub maker: String,
    pub taker: String,
    pub evm_signature: Option<String>,
}

pub struct RelayerService {
    orderbook_canister: OrderbookCanisterClient,
    resolvers: HashMap<String, ResolverInfo>,
    order_queue: mpsc::Sender<Order>,
}

impl RelayerService {
    pub fn new(orderbook_canister: OrderbookCanisterClient) -> Self {
        let (tx, mut rx) = mpsc::channel(100);

        // Spawn order processing task
        tokio::spawn(async move {
            while let Some(order) = rx.recv().await {
                Self::process_order(order).await;
            }
        });

        Self {
            orderbook_canister,
            resolvers: HashMap::new(),
            order_queue: tx,
        }
    }

    // Broadcast order to resolvers
    pub async fn broadcast_order(&self, order: Order) -> Result<(), String> {
        // Send order to processing queue
        self.order_queue.send(order).await
            .map_err(|_| "Failed to queue order".to_string())?;

        Ok(())
    }

    // Process order and broadcast to resolvers
    async fn process_order(order: Order) {
        // 1. Validate order
        if let Err(e) = Self::validate_order(&order).await {
            log::error!("Order validation failed: {}", e);
            return;
        }

        // 2. Broadcast to resolvers
        Self::broadcast_to_resolvers(&order).await;

        // 3. Simulate Dutch auction
        Self::simulate_dutch_auction(&order).await;
    }

    // Validate order parameters
    async fn validate_order(order: &Order) -> Result<(), String> {
        // Check order parameters
        if order.source_amount == 0 || order.destination_amount == 0 {
            return Err("Invalid amounts".to_string());
        }

        // Validate EVM signature if present
        if let Some(signature) = &order.evm_signature {
            Self::validate_evm_signature(order, signature).await?;
        }

        Ok(())
    }

    // Validate EIP-712 signature
    async fn validate_evm_signature(order: &Order, signature: &str) -> Result<(), String> {
        // Implement EIP-712 signature validation
        // This would use ethers-rs or similar library
        Ok(())
    }

    // Broadcast order to all resolvers
    async fn broadcast_to_resolvers(order: &Order) {
        // Send order to all registered resolvers
        log::info!("Broadcasting order {} to resolvers", order.id);
    }

    // Simulate Dutch auction
    async fn simulate_dutch_auction(order: &Order) {
        // Implement Dutch auction logic
        log::info!("Starting Dutch auction for order {}", order.id);
    }
}

pub struct OrderbookCanisterClient {
    canister_id: Principal,
}

impl OrderbookCanisterClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    // Get pending orders from canister
    pub async fn get_pending_orders(&self) -> Result<Vec<Order>, String> {
        // Call ICP canister to get pending orders
        Ok(vec![])
    }

    // Update order status
    pub async fn update_order_status(&self, order_id: &str, status: OrderStatus) -> Result<(), String> {
        // Call ICP canister to update order status
        Ok(())
    }
}
```

### 3.2 Migration Coordinator

```rust
// src/migration_coordinator.rs
use tokio::sync::mpsc;
use std::collections::HashMap;

pub struct MigrationCoordinator {
    orderbook_canister: OrderbookCanisterClient,
    evm_client: EVMClient,
    icp_client: ICPClient,
    migration_queue: mpsc::Sender<MigrationRequest>,
}

#[derive(Debug, Clone)]
pub struct MigrationRequest {
    pub order_id: String,
    pub source_escrow_id: String,
    pub destination_escrow_id: String,
    pub resolver: String,
}

#[derive(Debug, Clone)]
pub struct MigrationResult {
    pub success: bool,
    pub source_address: Option<String>,
    pub destination_address: Option<String>,
    pub error: Option<String>,
}

impl MigrationCoordinator {
    pub fn new(
        orderbook_canister: OrderbookCanisterClient,
        evm_client: EVMClient,
        icp_client: ICPClient,
    ) -> Self {
        let (tx, mut rx) = mpsc::channel(50);

        // Spawn migration processing task
        tokio::spawn(async move {
            while let Some(request) = rx.recv().await {
                Self::process_migration(request).await;
            }
        });

        Self {
            orderbook_canister,
            evm_client,
            icp_client,
            migration_queue: tx,
        }
    }

    // Queue migration request
    pub async fn queue_migration(&self, request: MigrationRequest) -> Result<(), String> {
        self.migration_queue.send(request).await
            .map_err(|_| "Failed to queue migration".to_string())
    }

    // Process migration request
    async fn process_migration(request: MigrationRequest) {
        log::info!("Processing migration for order {}", request.order_id);

        // 1. Start both migrations concurrently
        let source_migration = Self::migrate_source_escrow(&request);
        let dest_migration = Self::migrate_destination_escrow(&request);

        // 2. Wait for both to complete
        let (source_result, dest_result) = tokio::join!(source_migration, dest_migration);

        // 3. Handle results
        match (source_result, dest_result) {
            (Ok(source_addr), Ok(dest_addr)) => {
                log::info!("Migration successful for order {}", request.order_id);
                // Update order status
            }
            (Err(e), _) | (_, Err(e)) => {
                log::error!("Migration failed for order {}: {}", request.order_id, e);
                // Rollback successful migration if any
            }
        }
    }

    // Migrate source escrow
    async fn migrate_source_escrow(request: &MigrationRequest) -> Result<String, String> {
        // Implement source escrow migration
        Ok("source_address".to_string())
    }

    // Migrate destination escrow
    async fn migrate_destination_escrow(request: &MigrationRequest) -> Result<String, String> {
        // Implement destination escrow migration
        Ok("destination_address".to_string())
    }
}

pub struct EVMClient {
    rpc_url: String,
    private_key: String,
}

impl EVMClient {
    pub fn new(rpc_url: String, private_key: String) -> Self {
        Self { rpc_url, private_key }
    }

    // Create EVM escrow
    pub async fn create_escrow(&self, immutables: EVMImmutables) -> Result<String, String> {
        // Use ethers-rs to create EVM escrow
        Ok("evm_escrow_address".to_string())
    }
}

pub struct ICPClient {
    canister_id: Principal,
}

impl ICPClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    // Create ICP escrow
    pub async fn create_escrow(&self, params: ICPEscrowParams) -> Result<String, String> {
        // Call ICP canister to create escrow
        Ok("icp_escrow_address".to_string())
    }
}
```

## 4. Configuration and Dependencies

### 4.1 Cargo.toml

```toml
[package]
name = "icp-evm-fusion"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core dependencies
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"
env_logger = "0.10"

# ICP dependencies
candid = "0.9"
ic-cdk = "0.10"
ic-cdk-macros = "0.7"

# EVM dependencies
ethers = { version = "2.0", features = ["legacy"] }
web3 = "0.19"

# Crypto dependencies
sha2 = "0.10"
hex = "0.4"

# Async dependencies
futures = "0.3"
async-trait = "0.1"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
tokio-test = "0.4"
```

### 4.2 Environment Configuration

```rust
// src/config.rs
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub icp_canister_id: String,
    pub evm_rpc_url: String,
    pub evm_private_key: String,
    pub resolver_timeout: u64,
    pub migration_timeout: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            icp_canister_id: env::var("ICP_CANISTER_ID")
                .map_err(|_| "ICP_CANISTER_ID not set")?,
            evm_rpc_url: env::var("EVM_RPC_URL")
                .map_err(|_| "EVM_RPC_URL not set")?,
            evm_private_key: env::var("EVM_PRIVATE_KEY")
                .map_err(|_| "EVM_PRIVATE_KEY not set")?,
            resolver_timeout: env::var("RESOLVER_TIMEOUT")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .map_err(|_| "Invalid RESOLVER_TIMEOUT")?,
            migration_timeout: env::var("MIGRATION_TIMEOUT")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .map_err(|_| "Invalid MIGRATION_TIMEOUT")?,
        })
    }
}
```

## 5. Testing Strategy

### 5.1 Unit Tests

```rust
// src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_order_with_virtual_escrows() {
        let mut orderbook = OrderbookCanister::new();

        let order = Order {
            id: "test_order".to_string(),
            order_type: OrderType::ICPToEVM,
            source_chain: "ICP".to_string(),
            destination_chain: "ETH".to_string(),
            source_token: "ICP".to_string(),
            destination_token: "0x...".to_string(),
            source_amount: 1_000_000_000, // 1 ICP
            destination_amount: 1_500_000_000, // 1500 USDC
            hashlock: "0x...".to_string(),
            timelocks: TimelockConfig::default(),
            maker: Principal::anonymous(),
            taker: Principal::anonymous(),
            evm_signature: None,
        };

        let order_id = orderbook.create_order_with_virtual_escrows(order).await.unwrap();

        assert!(!order_id.is_empty());
        assert_eq!(orderbook.orders.len(), 1);
        assert_eq!(orderbook.virtual_escrows.len(), 2);
    }

    #[tokio::test]
    async fn test_migrate_escrow() {
        // Test escrow migration
    }

    #[tokio::test]
    async fn test_migration_coordination() {
        // Test migration coordination
    }
}
```

### 5.2 Integration Tests

```rust
// tests/integration_tests.rs
use icp_evm_fusion::{OrderbookCanister, RelayerService, MigrationCoordinator};

#[tokio::test]
async fn test_full_order_flow() {
    // 1. Create order with virtual escrows
    let mut orderbook = OrderbookCanister::new();
    let order_id = create_test_order(&mut orderbook).await;

    // 2. Simulate resolver filling order
    let relayer = RelayerService::new(orderbook.clone());
    let result = relayer.fill_order(&order_id, "resolver_address").await;

    // 3. Verify migration
    assert!(result.is_ok());

    // 4. Check final state
    let order = orderbook.get_order_with_escrows(&order_id).unwrap();
    assert!(matches!(order.status, OrderStatus::Filled));
}
```

## 6. Deployment Strategy

### 6.1 ICP Canister Deployment

```bash
# Deploy orderbook canister
dfx deploy orderbook_canister

# Deploy escrow canisters
dfx deploy icp_escrow_canister

# Set canister permissions
dfx canister call orderbook_canister set_permissions '(principal "your-principal")'
```

### 6.2 Backend Service Deployment

```bash
# Build backend service
cargo build --release

# Run with environment variables
ICP_CANISTER_ID="your-canister-id" \
EVM_RPC_URL="https://sepolia.base.org" \
EVM_PRIVATE_KEY="your-private-key" \
./target/release/icp-evm-fusion
```

## 7. Conclusion

The **Rust backend architecture** provides:

### **Key Benefits:**

1. **Performance**: High-performance cross-chain coordination
2. **Safety**: Memory safety and thread safety
3. **Integration**: Native ICP canister development
4. **Reliability**: Strong error handling and testing
5. **Scalability**: Async/await for concurrent operations

### **Implementation Priority:**

1. **Develop ICP canisters** in Rust
2. **Build backend services** for coordination
3. **Implement migration logic** for escrow creation
4. **Add comprehensive testing** for all components
5. **Deploy and monitor** the complete system

This Rust-based architecture provides a robust foundation for the ICP <> EVM fusion protocol with excellent performance, safety, and reliability characteristics.
