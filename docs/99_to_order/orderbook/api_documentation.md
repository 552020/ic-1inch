# Orderbook Canister API Documentation

## Overview

The Orderbook Canister implements the Chain Fusion+ Protocol between ICP and Ethereum (Base Sepolia). It serves as the central coordination hub for cross-chain swap orders, managing order lifecycle, Dutch auction mechanics, and coordinating between makers and resolvers.

## Core Concepts

### Order Directions

- **ICP→ETH**: Maker creates escrow, no EIP-712 signature required
- **ETH→ICP**: Resolver creates escrow, EIP-712 signature required

### Order Status Flow

```
Pending → Accepted → Completed
    ↓
  Failed/Cancelled
```

### Cross-Chain Identity

- Links Ethereum addresses to ICP principals
- Supports SIWE (Sign-In with Ethereum) integration
- Bidirectional lookup capabilities

## API Reference

### Core Order Management

#### `create_fusion_order`

Creates a new fusion order with 1inch LOP compatibility.

**Parameters:**

- `salt` (text): Unique salt for order identification
- `maker_asset` (text): Asset being sold (ICP or ETH address)
- `taker_asset` (text): Asset being bought (ICP or ETH address)
- `making_amount` (nat64): Amount maker is selling
- `taking_amount` (nat64): Amount maker wants to receive
- `maker_traits` (text): Maker traits encoded as hex string
- `hashlock` (text): Secret hash for atomic swap (64 hex chars)
- `expiration` (nat64): Order expiration timestamp (nanoseconds)
- `eip712_signature` (opt EIP712Signature): Optional EIP-712 signature for ETH→ICP orders

**Returns:** `Result_1` - Order ID on success, error on failure

**Example:**

```bash
# ICP→ETH order (no signature required)
dfx canister call orderbook create_fusion_order \
  '("0x1234567890abcdef", "ICP", "ETH", 1000000, 500000, "0x", "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234", 1700000000000000000, null)'

# ETH→ICP order (signature required)
dfx canister call orderbook create_fusion_order \
  '("0xabcdef1234567890", "ETH", "ICP", 500000, 1000000, "0x", "b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456", 1700000000000000000, opt record { domain_separator = "0x1234567890abcdef"; type_hash = "0xabcdef1234567890"; order_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_r = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_s = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_v = 27; signer_address = "0x1234567890abcdef1234567890abcdef1234567890" })'
```

#### `accept_fusion_order`

Accepts a pending fusion order as a resolver.

**Parameters:**

- `order_id` (text): ID of the order to accept
- `resolver_eth_address` (text): Resolver's Ethereum address

**Returns:** `Result_3` - JSON response with order data on success, error on failure

**Example:**

```bash
dfx canister call orderbook accept_fusion_order \
  '("order_123", "0xabcdef1234567890abcdef1234567890abcdef12")'
```

#### `complete_order_with_secret`

Completes an order by revealing the secret.

**Parameters:**

- `order_id` (text): ID of the order to complete
- `secret` (text): The secret that matches the hashlock

**Returns:** `Result` - Success or error

**Example:**

```bash
dfx canister call orderbook complete_order_with_secret \
  '("order_123", "my_secret_123")'
```

#### `cancel_order`

Cancels an order (only by maker or resolver).

**Parameters:**

- `order_id` (text): ID of the order to cancel

**Returns:** `Result` - Success or error

**Example:**

```bash
dfx canister call orderbook cancel_order '("order_123")'
```

### Order Queries

#### `get_active_fusion_orders`

Returns all active fusion orders.

**Returns:** `vec FusionOrder` - List of active orders

**Example:**

```bash
dfx canister call orderbook get_active_fusion_orders --query
```

#### `get_fusion_order_status`

Returns a specific order by ID.

**Parameters:**

- `order_id` (text): ID of the order to retrieve

**Returns:** `opt FusionOrder` - Order data or null if not found

**Example:**

```bash
dfx canister call orderbook get_fusion_order_status '("order_123")' --query
```

#### `get_orders_by_maker`

Returns all orders created by a specific maker.

**Parameters:**

- `maker_principal` (principal): ICP principal of the maker

**Returns:** `vec FusionOrder` - List of orders by the maker

**Example:**

```bash
dfx canister call orderbook get_orders_by_maker '(principal "2vxsx-fae")' --query
```

#### `get_orders_by_status`

Returns all orders with a specific status.

**Parameters:**

- `status` (OrderStatus): Status to filter by (Pending, Accepted, Completed, Failed, Cancelled)

**Returns:** `vec FusionOrder` - List of orders with the specified status

**Example:**

```bash
dfx canister call orderbook get_orders_by_status '(variant { Pending })' --query
```

#### `get_order_statistics`

Returns order statistics.

**Returns:** `OrderStatistics` - Statistics about all orders

**Example:**

```bash
dfx canister call orderbook get_order_statistics --query
```

### Direction-Specific Coordination

#### `get_order_direction_info`

Returns order direction and escrow creator information.

**Parameters:**

- `order_id` (text): ID of the order

**Returns:** `Result_3` - JSON with direction and escrow creator info

**Example:**

```bash
dfx canister call orderbook get_order_direction_info '("order_123")' --query
```

#### `get_orders_by_direction`

Returns orders filtered by direction.

**Parameters:**

- `direction` (text): Direction to filter by ("ICP_TO_ETH" or "ETH_TO_ICP")

**Returns:** `vec FusionOrder` - List of orders in the specified direction

**Example:**

```bash
dfx canister call orderbook get_orders_by_direction '("ICP_TO_ETH")' --query
```

#### `get_orders_for_escrow_creation`

Returns orders where the caller is responsible for escrow creation.

**Returns:** `vec FusionOrder` - List of orders for escrow creation

**Example:**

```bash
dfx canister call orderbook get_orders_for_escrow_creation --query
```

### Cross-Chain Identity Management

#### `register_cross_chain_identity`

Registers a cross-chain identity mapping.

**Parameters:**

- `eth_address` (text): Ethereum address
- `icp_principal` (principal): ICP principal
- `role` (UserRole): User role (Maker or Resolver)

**Returns:** `Result` - Success or error

**Example:**

```bash
dfx canister call orderbook register_cross_chain_identity \
  '("0x1234567890abcdef1234567890abcdef1234567890", principal "2vxsx-fae", variant { Maker })'
```

#### `get_cross_chain_identity`

Returns cross-chain identity by Ethereum address.

**Parameters:**

- `eth_address` (text): Ethereum address to lookup

**Returns:** `opt CrossChainIdentity` - Identity data or null if not found

**Example:**

```bash
dfx canister call orderbook get_cross_chain_identity \
  '("0x1234567890abcdef1234567890abcdef1234567890")' --query
```

#### `get_cross_chain_identity_by_principal`

Returns cross-chain identity by ICP principal.

**Parameters:**

- `principal` (principal): ICP principal to lookup

**Returns:** `opt CrossChainIdentity` - Identity data or null if not found

**Example:**

```bash
dfx canister call orderbook get_cross_chain_identity_by_principal \
  '(principal "2vxsx-fae")' --query
```

#### `store_siwe_identity`

Stores SIWE (Sign-In with Ethereum) identity.

**Parameters:**

- `eth_address` (text): Ethereum address
- `icp_principal` (principal): ICP principal
- `role` (UserRole): User role

**Returns:** `Result` - Success or error

**Example:**

```bash
dfx canister call orderbook store_siwe_identity \
  '("0x1234567890abcdef1234567890abcdef1234567890", principal "2vxsx-fae", variant { Maker })'
```

### Order Status Management

#### `update_order_status`

Updates the status of an order.

**Parameters:**

- `order_id` (text): ID of the order
- `status` (OrderStatus): New status

**Returns:** `Result` - Success or error

**Example:**

```bash
dfx canister call orderbook update_order_status \
  '("order_123", variant { Completed })'
```

### Escrow Factory Notifications

#### `notify_escrow_completed`

Called by escrow factory to notify of escrow completion.

**Parameters:**

- `order_id` (text): ID of the completed order
- `escrow_address` (text): Address of the completed escrow

**Returns:** `Result` - Success or error

**Example:**

```bash
dfx canister call orderbook notify_escrow_completed \
  '("order_123", "0xabcdef1234567890abcdef1234567890abcdef12")'
```

#### `notify_escrow_cancelled`

Called by escrow factory to notify of escrow cancellation.

**Parameters:**

- `order_id` (text): ID of the cancelled order
- `escrow_address` (text): Address of the cancelled escrow

**Returns:** `Result` - Success or error

**Example:**

```bash
dfx canister call orderbook notify_escrow_cancelled \
  '("order_123", "0xabcdef1234567890abcdef1234567890abcdef12")'
```

## Data Types

### FusionOrder

Complete order structure with all fields:

```candid
type FusionOrder = record {
  id : text;                           // Unique order ID
  maker_eth_address : text;            // Maker's Ethereum address
  maker_icp_principal : principal;     // Maker's ICP principal
  resolver_eth_address : opt text;     // Resolver's Ethereum address (if accepted)
  resolver_icp_principal : opt principal; // Resolver's ICP principal (if accepted)
  salt : text;                         // Unique salt for order identification
  maker_asset : text;                  // Asset being sold
  taker_asset : text;                  // Asset being bought
  making_amount : nat64;               // Amount maker is selling
  taking_amount : nat64;               // Amount maker wants to receive
  maker_traits : text;                 // Maker traits encoded as hex
  hashlock : text;                     // Secret hash for atomic swap
  status : OrderStatus;                // Current order status
  created_at : nat64;                  // Creation timestamp
  expires_at : nat64;                  // Expiration timestamp
  accepted_at : opt nat64;             // Acceptance timestamp (if accepted)
  completed_at : opt nat64;            // Completion timestamp (if completed)
  eip712_signature : opt EIP712Signature; // EIP-712 signature (for ETH→ICP orders)
  // Legacy fields for backward compatibility
  from_token : Token;
  to_token : Token;
  from_amount : nat64;
  to_amount : nat64;
  secret_hash : text;
  timelock_duration : nat64;
  safety_deposit_amount : nat64;
};
```

### EIP712Signature

EIP-712 signature structure for ETH→ICP orders:

```candid
type EIP712Signature = record {
  domain_separator : text;     // EIP-712 domain separator
  type_hash : text;            // EIP-712 type hash
  order_hash : text;           // Order hash
  signature_r : text;          // Signature r component
  signature_s : text;          // Signature s component
  signature_v : nat8;          // Signature v component (27 or 28)
  signer_address : text;       // Signer's Ethereum address
};
```

### OrderStatus

Order status enumeration:

```candid
type OrderStatus = variant {
  Pending;    // Order created, waiting for resolver
  Accepted;   // Resolver accepted, coordinating swap
  Completed;  // Swap successful
  Failed;     // Swap failed
  Cancelled;  // Order cancelled
};
```

### FusionError

Error types for the orderbook:

```candid
type FusionError = variant {
  InvalidAmount;              // Invalid amount
  OrderNotPending;            // Order is not in pending state
  SystemError;                // System error
  OrderNotFound;              // Order not found
  InsufficientBalance;        // Insufficient balance
  OrderExpired;               // Order has expired
  Unauthorized;               // Unauthorized operation
  InvalidExpiration;          // Invalid expiration time
  OrderAlreadyAccepted;       // Order already accepted
  ResolverNotWhitelisted;     // Resolver not whitelisted
  InvalidSecretHash;          // Invalid secret hash format
  InvalidEIP712Signature;     // Invalid EIP-712 signature
  OrderNotCancellable;        // Order cannot be cancelled
  InvalidSecret;              // Invalid secret or hash mismatch
  InvalidSalt;                // Invalid salt value
  InvalidMakerTraits;         // Invalid maker traits
  TokenAddressInvalid;        // Invalid token address
  NotImplemented;             // Feature not yet implemented
};
```

## Error Handling

All functions return `Result` types that can be either:

- `Ok` with the expected return value
- `Err` with a `FusionError` describing what went wrong

Common error scenarios:

- **OrderNotFound**: Order ID doesn't exist
- **OrderNotPending**: Trying to accept an already accepted/completed order
- **OrderExpired**: Order has passed its expiration time
- **Unauthorized**: Caller doesn't have permission for the operation
- **InvalidEIP712Signature**: ETH→ICP order missing or invalid signature

## Best Practices

### Order Creation

1. Use unique salts for each order
2. Set reasonable expiration times (minimum 1 hour)
3. Provide EIP-712 signatures for ETH→ICP orders
4. Use valid Ethereum addresses for asset addresses

### Order Acceptance

1. Verify order is still pending before accepting
2. Check expiration time before accepting
3. Provide valid Ethereum address for resolver

### Cross-Chain Identity

1. Register identities before creating orders
2. Use consistent Ethereum addresses across operations
3. Link both ETH address and ICP principal for full functionality

### Error Handling

1. Always check return values for errors
2. Handle expired orders appropriately
3. Validate input parameters before calling functions

## Integration Examples

### Frontend Integration

```javascript
// Create ICP→ETH order
const orderId = await canister.create_fusion_order(
  "0x1234567890abcdef", // salt
  "ICP", // maker_asset
  "ETH", // taker_asset
  1000000, // making_amount
  500000, // taking_amount
  "0x", // maker_traits
  "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234", // hashlock
  Date.now() + 3600000, // expiration (1 hour from now)
  null // no EIP-712 signature needed for ICP→ETH
);

// Accept order as resolver
const response = await canister.accept_fusion_order(
  orderId,
  "0xabcdef1234567890abcdef1234567890abcdef12" // resolver ETH address
);
```

### Escrow Factory Integration

```javascript
// Notify orderbook of escrow completion
await canister.notify_escrow_completed(orderId, escrowAddress);
```

## Deployment

### Local Development

```bash
# Start local network
dfx start --clean

# Deploy orderbook canister
dfx deploy orderbook

# Check canister status
dfx canister status orderbook
```

### Production Deployment

```bash
# Deploy to IC mainnet
dfx deploy --network ic orderbook

# Get canister ID
dfx canister --network ic id orderbook
```

## Testing

Use the provided test scripts for comprehensive testing:

```bash
# Test order creation and acceptance
./scripts/orderbook/test_order_acceptance.sh

# Test direction-specific coordination
./scripts/orderbook/test_directions.sh

# Test identity management
./scripts/orderbook/test_identity.sh
```

## Support

For issues or questions:

1. Check the test scripts for usage examples
2. Review the error messages for debugging
3. Verify canister deployment and network connectivity
4. Ensure proper authentication and authorization
