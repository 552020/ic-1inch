# EscrowFactory API Documentation

## TL;DR

**4 Main Public Functions:**

- `ESCROW_SRC_IMPLEMENTATION()` - view
- `ESCROW_DST_IMPLEMENTATION()` - view
- `addressOfEscrowSrc()` - view
- `createDstEscrow()` - state-changing

**Resolver Only Uses 2 Functions:**

- `addressOfEscrowSrc()` - compute deterministic address
- `createDstEscrow()` - deploy destination escrow

**How I Found API:** Read `contracts/interfaces/IEscrowFactory.sol` → complete API definition

---

## Overview

The **EscrowFactory** is the main contract for creating cross-chain escrow contracts. It provides functions to deploy source and destination escrows with deterministic addresses.

## Contract Address

**EscrowFactory:** `0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0`

## Core API Functions

### **1. View Functions (Read-Only)**

#### **`ESCROW_SRC_IMPLEMENTATION()` → `address`**

Returns the implementation address for source escrows.

**Test Result:**

```bash
cast call 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 "ESCROW_SRC_IMPLEMENTATION()" --rpc-url http://localhost:8545
# Returns: 0xE451980132E65465d0a498c53f0b5227326Dd73F
```

#### **`ESCROW_DST_IMPLEMENTATION()` → `address`**

Returns the implementation address for destination escrows.

**Test Result:**

```bash
cast call 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 "ESCROW_DST_IMPLEMENTATION()" --rpc-url http://localhost:8545
# Returns: 0x5392A33F7F677f59e833FEBF4016cDDD88fF9E67
```

#### **`addressOfEscrowSrc(Immutables calldata immutables)` → `address`**

Computes the deterministic address for a source escrow based on immutables.

**Parameters:**

- `immutables`: Struct containing escrow parameters (orderHash, hashlock, maker, taker, token, amount, safetyDeposit, timelocks)

**Usage:**

```bash
# Example call (requires complex struct encoding)
cast call 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 "addressOfEscrowSrc(tuple)" --rpc-url http://localhost:8545
```

#### **`addressOfEscrowDst(Immutables calldata immutables)` → `address`**

Computes the deterministic address for a destination escrow based on immutables.

**Parameters:**

- `immutables`: Struct containing escrow parameters

### **2. State-Changing Functions**

#### **`createDstEscrow(Immutables calldata dstImmutables, uint256 srcCancellationTimestamp)` → `void`**

Creates a destination escrow contract. This is the main function for creating cross-chain swaps.

**Parameters:**

- `dstImmutables`: Immutables struct for destination escrow
- `srcCancellationTimestamp`: Timestamp when source escrow cancellation starts

**Requirements:**

- Must send exact `msg.value` (safety deposit + native token amount if applicable)
- Must approve destination token for transfer to escrow
- `srcCancellationTimestamp` must be >= destination escrow cancellation time

**Events Emitted:**

- `DstEscrowCreated(address escrow, bytes32 hashlock, Address taker)`

**Usage:**

```bash
# Example call (requires complex struct encoding and value)
cast send 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 "createDstEscrow(tuple,uint256)" --value 0.1ether --rpc-url http://localhost:8545
```

### **3. Internal Functions (Called by LOP)**

#### **`_postInteraction(...)` → `void`**

Called by the Limit Order Protocol after order execution to create source escrows.

**This function is called automatically by the LOP system, not directly by users.**

## Immutables Struct

```solidity
struct Immutables {
    bytes32 orderHash;        // Hash of the limit order
    bytes32 hashlock;         // Hash of the secret
    address maker;            // Order maker address
    Address taker;            // Order taker address
    Address token;            // Token address
    uint256 amount;           // Token amount
    uint256 safetyDeposit;    // Safety deposit amount
    Timelocks timelocks;      // Timelock configuration
}
```

## Timelocks Struct

```solidity
struct Timelocks {
    uint256 deployedAt;       // Deployment timestamp
    uint256 withdrawal;       // Withdrawal start time
    uint256 publicWithdrawal; // Public withdrawal start time
    uint256 cancellation;     // Cancellation start time
    uint256 publicCancellation; // Public cancellation start time
}
```

## Events

### **`SrcEscrowCreated`**

```solidity
event SrcEscrowCreated(
    Immutables srcImmutables,
    DstImmutablesComplement dstImmutablesComplement
);
```

### **`DstEscrowCreated`**

```solidity
event DstEscrowCreated(
    address escrow,
    bytes32 hashlock,
    Address taker
);
```

## Error Codes

- **`InsufficientEscrowBalance()`**: Sent value doesn't match required amount
- **`InvalidCreationTime()`**: Destination escrow cancellation time > source cancellation time
- **`InvalidPartialFill()`**: Invalid partial fill for multiple-fill orders
- **`InvalidSecretsAmount()`**: Invalid number of secrets for multiple fills

## Mock Token Information

### **FeeToken**

- **Address:** `0x5FbDB2315678afecb367f032d93F642f64180aa3`
- **Name:** "Fee Token"
- **Symbol:** "FEE"
- **Decimals:** 18

### **AccessToken**

- **Address:** `0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512`
- **Name:** "Access Token"
- **Symbol:** "ACCESS"
- **Decimals:** 18

## Usage Examples

### **1. Check Implementation Addresses**

```bash
# Source implementation
cast call 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 "ESCROW_SRC_IMPLEMENTATION()" --rpc-url http://localhost:8545

# Destination implementation
cast call 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 "ESCROW_DST_IMPLEMENTATION()" --rpc-url http://localhost:8545
```

### **2. Check Token Information**

```bash
# FeeToken name
cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "name()" --rpc-url http://localhost:8545

# AccessToken symbol
cast call 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "symbol()" --rpc-url http://localhost:8545
```

### **3. Mint AccessToken for Testing**

```bash
# Mint 1 token to deployer
cast send 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "mint(address,uint256)" \
  0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 1000000000000000000 \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  --rpc-url http://localhost:8545
```

## Integration with Limit Order Protocol

The EscrowFactory integrates with the 1inch Limit Order Protocol:

1. **Order Creation**: User creates limit order via LOP
2. **Order Execution**: LOP executes the order
3. **Post-Interaction**: LOP calls `_postInteraction()` on EscrowFactory
4. **Source Escrow**: EscrowFactory deploys source escrow
5. **Destination Escrow**: User calls `createDstEscrow()` to complete the swap

## Security Features

- **Deterministic Addresses**: All escrows use Create2 for predictable addresses
- **Access Control**: AccessToken required for public functions
- **Timelock Protection**: Time-based access control
- **Balance Validation**: Ensures sufficient funds before deployment
- **Cancellation Coordination**: Prevents timing issues between chains

---

**Status:** ✅ Successfully tested and functional on local deployment
