# Limit Order Protocol API Specification

**Task B1 Deliverable - Team B**

## Overview

This document defines the API interface for limit order functions that will be added to `src/backend/src/lib.rs`. The API follows existing escrow function patterns for consistency.

## Function Signatures for lib.rs

### Order Management Functions

```rust
// Add to lib.rs imports
mod limit_orders;
use types::{Order, OrderError, OrderId, CreateOrderParams, SystemStats};

/// Create a new limit order - Used by: Makers
#[ic_cdk::update]
async fn create_order(
    receiver: candid::Principal,
    maker_asset: candid::Principal,
    taker_asset: candid::Principal,
    making_amount: u64,
    taking_amount: u64,
    expiration: u64,
    allowed_taker: Option<candid::Principal>,
) -> Result<OrderId, OrderError> {
    limit_orders::create_order(receiver, maker_asset, taker_asset, making_amount, taking_amount, expiration, allowed_taker).await
}

/// Fill an existing limit order - Used by: Takers/Resolvers
#[ic_cdk::update]
async fn fill_order(order_id: OrderId) -> Result<(), OrderError> {
    limit_orders::fill_order(order_id).await
}

/// Cancel an unfilled order - Used by: Makers
#[ic_cdk::update]
fn cancel_order(order_id: OrderId) -> Result<(), OrderError> {
    limit_orders::cancel_order(order_id)
}
```

### Order Discovery Functions

```rust
/// Get all active orders - Used by: Resolvers/Frontend
#[ic_cdk::query]
fn get_active_orders() -> Vec<Order> {
    limit_orders::get_active_orders()
}

/// Get specific order details - Used by: Everyone
#[ic_cdk::query]
fn get_order(order_id: OrderId) -> Option<Order> {
    limit_orders::get_order(order_id)
}

/// Get orders by maker - Used by: Makers/Frontend
#[ic_cdk::query]
fn get_orders_by_maker(maker: candid::Principal) -> Vec<Order> {
    limit_orders::get_orders_by_maker(maker)
}

/// Get orders by asset pair - Used by: Resolvers/Frontend
#[ic_cdk::query]
fn get_orders_by_asset_pair(
    maker_asset: candid::Principal,
    taker_asset: candid::Principal,
) -> Vec<Order> {
    limit_orders::get_orders_by_asset_pair(maker_asset, taker_asset)
}
```

### System Monitoring Functions

```rust
/// Get system statistics - Used by: Developers/Monitoring
#[ic_cdk::query]
fn get_system_stats() -> SystemStats {
    limit_orders::get_system_stats()
}

/// List all orders (for debugging) - Used by: Developers
#[ic_cdk::query]
fn list_all_orders() -> Vec<Order> {
    limit_orders::list_all_orders()
}
```

## API Design Patterns

### Naming Convention

- **Action + Subject**: `create_order`, `fill_order`, `cancel_order`
- **Get + Target**: `get_active_orders`, `get_order`, `get_orders_by_maker`
- **Consistent with escrow APIs**: `create_escrow` → `create_order`

### Parameter Patterns

- **Individual parameters** for create operations (following `create_source_escrow`)
- **ID-based lookup** for operations on existing entities
- **Principal types** explicitly as `candid::Principal`
- **Optional parameters** using `Option<T>`

### Return Type Patterns

- **State-changing operations**: `Result<T, OrderError>`
- **Queries**: Direct types (`Vec<Order>`, `Option<Order>`, `SystemStats`)
- **ID generation**: Return `OrderId` for created orders

### ic_cdk Annotations

- **`#[ic_cdk::update]`**: For `create_order`, `fill_order`, `cancel_order`
- **`#[ic_cdk::query]`**: For all get/list functions

## Error Handling Strategy

### OrderError Integration

```rust
// Already defined in types.rs - follows EscrowError patterns
pub enum OrderError {
    InvalidAmount,
    InvalidExpiration,
    InvalidAssetPair,
    OrderNotFound,
    OrderAlreadyFilled,
    OrderCancelled,
    OrderExpired,
    Unauthorized,
    InsufficientBalance,
    TokenCallFailed(String),
    TransferFailed(String),
    SystemError(String),
}
```

### Frontend Error Handling

- **Structured errors** with specific enum variants
- **String details** for TokenCallFailed/TransferFailed
- **Display implementation** provides user-friendly messages
- **Candid serialization** for frontend consumption

## Frontend Integration Points

### TypeScript Generation

After implementing functions in lib.rs, run:

```bash
./scripts/generate_declarations.sh
```

This generates:

- `src/declarations/backend/backend.did` (Candid interface)
- `src/declarations/backend/index.ts` (TypeScript bindings)

### Frontend Usage Pattern

```typescript
import { backend } from "../declarations/backend";

// Create order
const orderId = await backend.create_order(
  receiver,
  makerAsset,
  takerAsset,
  makingAmount,
  takingAmount,
  expiration,
  allowedTaker
);

// Get active orders
const orders = await backend.get_active_orders();

// Fill order
await backend.fill_order(orderId);
```

## Implementation Priority

### Phase 2 (Core Functions)

1. `create_order` - Task A4
2. `fill_order` - Task A5
3. `cancel_order` - Task A6

### Phase 2 (Discovery Functions)

4. `get_active_orders` - Task B4
5. `get_order` - Task B4
6. `get_orders_by_maker` - Task B4
7. `get_orders_by_asset_pair` - Task B4

### Phase 3 (System Functions)

8. `get_system_stats` - Task C3
9. `list_all_orders` - Task C3

## Candid Interface Preview

The auto-generated .did file will contain:

```candid
type OrderId = nat64;

type Order = record {
  id: OrderId;
  maker: principal;
  receiver: principal;
  maker_asset: principal;
  taker_asset: principal;
  making_amount: nat64;
  taking_amount: nat64;
  expiration: nat64;
  created_at: nat64;
  allowed_taker: opt principal;
  metadata: opt OrderMetadata;
};

type OrderError = variant {
  InvalidAmount;
  InvalidExpiration;
  InvalidAssetPair;
  OrderNotFound;
  OrderAlreadyFilled;
  OrderCancelled;
  OrderExpired;
  Unauthorized;
  InsufficientBalance;
  TokenCallFailed: text;
  TransferFailed: text;
  SystemError: text;
};

service : {
  create_order: (principal, principal, principal, nat64, nat64, nat64, opt principal) -> (Result<OrderId, OrderError>);
  fill_order: (OrderId) -> (Result<(), OrderError>);
  cancel_order: (OrderId) -> (Result<(), OrderError>);
  get_active_orders: () -> (vec Order) query;
  get_order: (OrderId) -> (opt Order) query;
  get_orders_by_maker: (principal) -> (vec Order) query;
  get_orders_by_asset_pair: (principal, principal) -> (vec Order) query;
  get_system_stats: () -> (SystemStats) query;
  list_all_orders: () -> (vec Order) query;
}
```

## Task B1 Completion Checklist

- [x] **Function signatures defined** following escrow patterns
- [x] **Candid interface preview** documented for frontend
- [x] **Error handling patterns** specified with OrderError
- [x] **Integration examples** provided for frontend usage
- [x] **Implementation priority** aligned with task dependencies

**Next Steps**: Teams A can implement these functions in Phase 2 (A4, A5, A6) and Team B can implement discovery functions (B4).
