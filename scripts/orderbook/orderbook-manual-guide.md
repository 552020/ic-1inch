# Orderbook Canister Manual Testing Guide

## Overview

This guide provides step-by-step instructions for manually testing the Orderbook Canister functionality. The orderbook implements the Chain Fusion+ Protocol between ICP and Ethereum, serving as the central coordination hub for cross-chain swap orders.

## Prerequisites

### Required Tools

- **DFX**: Internet Computer development kit
- **Bash**: For running test scripts
- **bc**: For calculations (usually pre-installed)

### Network Configuration

```bash
# Set network (local for development, ic for mainnet)
export DFX_NETWORK=local

# Start local network (if using local)
dfx start --clean --background
```

## Quick Start

### 1. Deploy the Canister

```bash
# Deploy orderbook canister
dfx deploy orderbook

# Verify deployment
dfx canister status orderbook
```

### 2. Run Comprehensive Tests

```bash
# Run all tests
./scripts/orderbook/run_all_tests.sh

# Or run individual tests
./scripts/orderbook/test_directions.sh
./scripts/orderbook/test_order_acceptance.sh
```

## Manual Testing Scenarios

### Scenario 1: ICP→ETH Order Flow (Maker Creates Escrow)

#### Step 1: Create ICP→ETH Order

```bash
# Create an ICP→ETH order (no EIP-712 signature required)
ORDER_ID=$(dfx canister call orderbook create_fusion_order \
  '("0x1234567890abcdef", "ICP", "ETH", 1000000, 500000, "0x", "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234", 1700000000000000000, null)' \
  | grep -o '"[^"]*"' | head -1 | tr -d '"')

echo "Created order: $ORDER_ID"
```

#### Step 2: Verify Order Direction

```bash
# Check order direction info
dfx canister call orderbook get_order_direction_info "(\"$ORDER_ID\")" --query
```

**Expected Output:**

```
{
  "order_id": "order_123",
  "direction": "ICP_TO_ETH",
  "escrow_creator": "maker",
  "maker_asset": "ICP",
  "taker_asset": "ETH",
  "status": "Pending"
}
```

#### Step 3: Check Order Status

```bash
# Get order details
dfx canister call orderbook get_fusion_order_status "(\"$ORDER_ID\")" --query
```

#### Step 4: Accept Order as Resolver

```bash
# Accept the order (this will be done by a different identity)
dfx canister call orderbook accept_fusion_order \
  "(\"$ORDER_ID\", \"0xabcdef1234567890abcdef1234567890abcdef12\")"
```

### Scenario 2: ETH→ICP Order Flow (Resolver Creates Escrow)

#### Step 1: Create ETH→ICP Order with EIP-712 Signature

```bash
# Create an ETH→ICP order (EIP-712 signature required)
ORDER_ID=$(dfx canister call orderbook create_fusion_order \
  '("0xabcdef1234567890", "ETH", "ICP", 500000, 1000000, "0x", "b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456", 1700000000000000000, opt record { domain_separator = "0x1234567890abcdef"; type_hash = "0xabcdef1234567890"; order_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_r = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_s = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"; signature_v = 27; signer_address = "0x1234567890abcdef1234567890abcdef1234567890" })' \
  | grep -o '"[^"]*"' | head -1 | tr -d '"')

echo "Created order: $ORDER_ID"
```

#### Step 2: Verify Order Direction

```bash
# Check order direction info
dfx canister call orderbook get_order_direction_info "(\"$ORDER_ID\")" --query
```

**Expected Output:**

```
{
  "order_id": "order_456",
  "direction": "ETH_TO_ICP",
  "escrow_creator": "resolver",
  "maker_asset": "ETH",
  "taker_asset": "ICP",
  "status": "Pending"
}
```

#### Step 3: Accept Order as Resolver

```bash
# Accept the order (resolver will create escrow)
dfx canister call orderbook accept_fusion_order \
  "(\"$ORDER_ID\", \"0xabcdef1234567890abcdef1234567890abcdef12\")"
```

### Scenario 3: Cross-Chain Identity Management

#### Step 1: Register Cross-Chain Identity

```bash
# Register a maker identity
dfx canister call orderbook register_cross_chain_identity \
  '("0x1234567890abcdef1234567890abcdef1234567890", principal "2vxsx-fae", variant { Maker })'

# Register a resolver identity
dfx canister call orderbook register_cross_chain_identity \
  '("0xabcdef1234567890abcdef1234567890abcdef12", principal "2vxsx-fae", variant { Resolver })'
```

#### Step 2: Query Identity by Ethereum Address

```bash
# Look up identity by ETH address
dfx canister call orderbook get_cross_chain_identity \
  '("0x1234567890abcdef1234567890abcdef1234567890")' --query
```

#### Step 3: Query Identity by ICP Principal

```bash
# Look up identity by ICP principal
dfx canister call orderbook get_cross_chain_identity_by_principal \
  '(principal "2vxsx-fae")' --query
```

### Scenario 4: Order Queries and Filtering

#### Step 1: Get All Active Orders

```bash
# Get all active orders
dfx canister call orderbook get_active_fusion_orders --query
```

#### Step 2: Filter Orders by Direction

```bash
# Get ICP→ETH orders
dfx canister call orderbook get_orders_by_direction '("ICP_TO_ETH")' --query

# Get ETH→ICP orders
dfx canister call orderbook get_orders_by_direction '("ETH_TO_ICP")' --query
```

#### Step 3: Get Orders for Escrow Creation

```bash
# Get orders where caller is responsible for escrow creation
dfx canister call orderbook get_orders_for_escrow_creation --query
```

#### Step 4: Get Order Statistics

```bash
# Get order statistics
dfx canister call orderbook get_order_statistics --query
```

### Scenario 5: Order Completion and Cancellation

#### Step 1: Complete Order with Secret

```bash
# Complete an order by revealing the secret
dfx canister call orderbook complete_order_with_secret \
  '("order_123", "my_secret_123")'
```

#### Step 2: Cancel Order

```bash
# Cancel an order (only by maker or resolver)
dfx canister call orderbook cancel_order '("order_123")'
```

### Scenario 6: Escrow Factory Integration

#### Step 1: Notify Escrow Completion

```bash
# Notify orderbook of escrow completion
dfx canister call orderbook notify_escrow_completed \
  '("order_123", "0xabcdef1234567890abcdef1234567890abcdef12")'
```

#### Step 2: Notify Escrow Cancellation

```bash
# Notify orderbook of escrow cancellation
dfx canister call orderbook notify_escrow_cancelled \
  '("order_123", "0xabcdef1234567890abcdef1234567890abcdef12")'
```

## Error Testing

### Test 1: Invalid Order Creation

```bash
# Try to create ETH→ICP order without EIP-712 signature (should fail)
dfx canister call orderbook create_fusion_order \
  '("0xfedcba0987654321", "ETH", "ICP", 500000, 1000000, "0x", "c3d4e5f6789012345678901234567890abcdef1234567890abcdef12345678", 1700000000000000000, null)'
```

### Test 2: Invalid Order Acceptance

```bash
# Try to accept non-existent order (should fail)
dfx canister call orderbook accept_fusion_order \
  '("non_existent_order", "0xabcdef1234567890abcdef1234567890abcdef12")'
```

### Test 3: Invalid Secret Hash

```bash
# Try to create order with invalid hashlock (should fail)
dfx canister call orderbook create_fusion_order \
  '("0xinvalidhash", "ICP", "ETH", 1000000, 500000, "0x", "invalid_hash", 1700000000000000000, null)'
```

## Performance Testing

### Load Test: Create Multiple Orders

```bash
# Create 10 orders in sequence
for i in {1..10}; do
  echo "Creating order $i..."
  dfx canister call orderbook create_fusion_order \
    "(\"0x$(printf '%016x' $i)\", \"ICP\", \"ETH\", 1000000, 500000, \"0x\", \"a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234\", 1700000000000000000, null)" \
    >/dev/null 2>&1 || echo "Failed to create order $i"
done
```

### Query Performance Test

```bash
# Test query performance
time dfx canister call orderbook get_active_fusion_orders --query >/dev/null
```

## Troubleshooting

### Common Issues

#### Issue 1: Canister Not Deployed

**Symptoms:** `dfx canister call` returns "canister not found"
**Solution:**

```bash
dfx deploy orderbook
```

#### Issue 2: Network Connection Error

**Symptoms:** Timeout or connection refused
**Solution:**

```bash
# Check network status
dfx ping

# Restart local network
dfx stop
dfx start --clean --background
```

#### Issue 3: Invalid Arguments

**Symptoms:** "Invalid argument" errors
**Solution:**

- Check argument format (quotes, brackets)
- Verify data types match Candid interface
- Use `--help` to see function signatures

#### Issue 4: Order Not Found

**Symptoms:** "OrderNotFound" error
**Solution:**

- Verify order ID is correct
- Check if order was actually created
- Use `get_active_fusion_orders` to list all orders

### Debug Commands

#### Check Canister Status

```bash
dfx canister status orderbook
```

#### View Canister Logs

```bash
dfx canister call orderbook get_active_fusion_orders --query
```

#### Check Network Status

```bash
dfx ping
```

#### Verify Deployment

```bash
dfx canister info orderbook
```

## Best Practices

### 1. Order Creation

- Use unique salts for each order
- Set reasonable expiration times (minimum 1 hour)
- Provide EIP-712 signatures for ETH→ICP orders
- Use valid Ethereum addresses for asset addresses

### 2. Testing

- Test both order directions (ICP→ETH and ETH→ICP)
- Verify EIP-712 signature requirements
- Test error conditions and edge cases
- Use different identities for maker and resolver roles

### 3. Monitoring

- Check order status regularly
- Monitor for expired orders
- Verify cross-chain identity mappings
- Track order statistics

### 4. Integration

- Test escrow factory notifications
- Verify order completion flows
- Test cancellation scenarios
- Validate cross-chain coordination

## API Reference

### Core Functions

- `create_fusion_order`: Create new fusion order
- `accept_fusion_order`: Accept pending order as resolver
- `complete_order_with_secret`: Complete order with secret
- `cancel_order`: Cancel order

### Query Functions

- `get_active_fusion_orders`: Get all active orders
- `get_fusion_order_status`: Get specific order
- `get_orders_by_direction`: Filter by direction
- `get_order_statistics`: Get order statistics

### Identity Functions

- `register_cross_chain_identity`: Register identity mapping
- `get_cross_chain_identity`: Lookup by ETH address
- `get_cross_chain_identity_by_principal`: Lookup by ICP principal

### Notification Functions

- `notify_escrow_completed`: Notify escrow completion
- `notify_escrow_cancelled`: Notify escrow cancellation

## Conclusion

This manual testing guide covers all major functionality of the Orderbook Canister. Use these scenarios to verify that the canister is working correctly before deploying to production.

For automated testing, use the provided test scripts:

- `run_all_tests.sh`: Comprehensive test suite
- `test_directions.sh`: Direction-specific coordination tests
- `test_order_acceptance.sh`: Order acceptance tests
- `test_identity.sh`: Identity management tests

The orderbook canister is designed to be robust and handle various edge cases while maintaining simplicity for the hackathon MVP.
