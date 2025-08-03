# LimitOrderProtocol API Documentation

## TL;DR - Quick Overview

The **LimitOrderProtocol** is a smart contract for creating and executing limit orders on Ethereum and compatible networks.

### 📊 **Function Count: 15 Core Functions**

#### **🔍 Query Functions (4)**

1. `DOMAIN_SEPARATOR()` - Get EIP-712 domain separator
2. `hashOrder(order)` - Calculate order hash
3. `bitInvalidatorForOrder(maker, slot)` - Check bit invalidator status
4. `remainingInvalidatorForOrder(maker, orderHash)` - Get remaining order amount

#### **📝 Order Filling Functions (4)**

5. `fillOrder(order, r, vs, amount, takerTraits)` - Fill order with EOA signature
6. `fillOrderArgs(order, r, vs, amount, takerTraits, args)` - Fill order with additional args
7. `fillContractOrder(order, signature, amount, takerTraits)` - Fill order with contract signature
8. `fillContractOrderArgs(order, signature, amount, takerTraits, args)` - Fill contract order with args

#### **❌ Order Cancellation Functions (3)**

9. `cancelOrder(makerTraits, orderHash)` - Cancel single order
10. `cancelOrders(makerTraits[], orderHashes[])` - Cancel multiple orders
11. `bitsInvalidateForOrder(makerTraits, additionalMask)` - Mass invalidate orders

#### **⚡ Utility Functions (2)**

12. `checkPredicate(predicate)` - Check if predicate condition is met
13. `simulate(target, data)` - Simulate arbitrary code execution

#### **👑 Administrative Functions (2)**

14. `pause()` - Pause all trading (owner only)
15. `unpause()` - Unpause all trading (owner only)

### **🎯 Key Features**

- **Two Order Types:** Regular Limit Orders vs RFQ Orders
- **Multiple Signatures:** EOA signatures vs Contract signatures (ERC-1271)
- **Advanced Features:** Predicates, Interactions, Permit2 support
- **Gas Optimization:** Bit invalidators, batch operations
- **Security:** Reentrancy protection, pause mechanism

---

## Overview

The **LimitOrderProtocol** is a smart contract that enables the creation and execution of limit orders on Ethereum and compatible networks. It supports two main order types:

1. **Regular Limit Orders** - Full-featured orders with customization options
2. **RFQ Orders** - Gas-efficient orders with basic functionality

## Contract Address

**Base Sepolia:** `0xdfC365795F146a6755998C5e916a592A9706eDC6`

## Core Data Structures

### Order Structure

```solidity
struct Order {
    uint256 salt;           // Unique identifier for the order
    Address maker;          // Address of the order creator
    Address receiver;       // Address to receive the order proceeds
    Address makerAsset;     // Token being offered by the maker
    Address takerAsset;     // Token being requested by the maker
    uint256 makingAmount;   // Amount of makerAsset being offered
    uint256 takingAmount;   // Amount of takerAsset being requested
    MakerTraits makerTraits; // Order configuration flags
}
```

### MakerTraits

```solidity
struct MakerTraits {
    uint256 info; // Packed configuration flags
}
```

### TakerTraits

```solidity
struct TakerTraits {
    uint256 info; // Packed configuration flags
}
```

## Core Functions

### 1. Domain Separator

```solidity
function DOMAIN_SEPARATOR() external view returns(bytes32)
```

**Purpose:** Returns the EIP-712 domain separator for order signing
**Returns:** Domain separator hash
**Access:** Public

### 2. Order Hashing

```solidity
function hashOrder(IOrderMixin.Order calldata order) external view returns(bytes32)
```

**Purpose:** Calculates the EIP-712 hash of an order
**Parameters:**

- `order` - The order structure to hash
  **Returns:** Order hash
  **Access:** Public

### 3. Order Filling (EOA Signatures)

#### Basic Order Fill

```solidity
function fillOrder(
    IOrderMixin.Order calldata order,
    bytes32 r,
    bytes32 vs,
    uint256 amount,
    TakerTraits takerTraits
) external payable returns(uint256 makingAmount, uint256 takingAmount, bytes32 orderHash)
```

**Purpose:** Fills an order using EOA signature
**Parameters:**

- `order` - The order to fill
- `r` - R component of the signature
- `vs` - VS component of the signature (S + V packed)
- `amount` - Amount to fill (interpreted based on takerTraits)
- `takerTraits` - Taker's preferences and thresholds
  **Returns:** Actual amounts transferred and order hash
  **Access:** Public

#### Order Fill with Arguments

```solidity
function fillOrderArgs(
    IOrderMixin.Order calldata order,
    bytes32 r,
    bytes32 vs,
    uint256 amount,
    TakerTraits takerTraits,
    bytes calldata args
) external payable returns(uint256 makingAmount, uint256 takingAmount, bytes32 orderHash)
```

**Purpose:** Fills an order with additional arguments for target, extension, and interaction
**Parameters:** Same as `fillOrder` plus:

- `args` - Packed arguments for target address, extension data, and interaction data
  **Returns:** Actual amounts transferred and order hash
  **Access:** Public

### 4. Order Filling (Contract Signatures)

#### Contract Order Fill

```solidity
function fillContractOrder(
    IOrderMixin.Order calldata order,
    bytes calldata signature,
    uint256 amount,
    TakerTraits takerTraits
) external returns(uint256 makingAmount, uint256 takingAmount, bytes32 orderHash)
```

**Purpose:** Fills an order using contract-based signature (ERC-1271)
**Parameters:**

- `order` - The order to fill
- `signature` - Contract signature bytes
- `amount` - Amount to fill
- `takerTraits` - Taker's preferences
  **Returns:** Actual amounts transferred and order hash
  **Access:** Public

#### Contract Order Fill with Arguments

```solidity
function fillContractOrderArgs(
    IOrderMixin.Order calldata order,
    bytes calldata signature,
    uint256 amount,
    TakerTraits takerTraits,
    bytes calldata args
) external returns(uint256 makingAmount, uint256 takingAmount, bytes32 orderHash)
```

**Purpose:** Fills a contract-signed order with additional arguments
**Parameters:** Same as `fillContractOrder` plus:

- `args` - Packed arguments for target, extension, and interaction
  **Returns:** Actual amounts transferred and order hash
  **Access:** Public

### 5. Order Cancellation

#### Cancel Single Order

```solidity
function cancelOrder(MakerTraits makerTraits, bytes32 orderHash) external
```

**Purpose:** Cancels a specific order
**Parameters:**

- `makerTraits` - Order configuration traits
- `orderHash` - Hash of the order to cancel
  **Access:** Public (only by order maker)

#### Cancel Multiple Orders

```solidity
function cancelOrders(
    MakerTraits[] calldata makerTraits,
    bytes32[] calldata orderHashes
) external
```

**Purpose:** Cancels multiple orders in a single transaction
**Parameters:**

- `makerTraits` - Array of order configuration traits
- `orderHashes` - Array of order hashes to cancel
  **Access:** Public (only by order makers)

#### Mass Invalidation (Bit Invalidator)

```solidity
function bitsInvalidateForOrder(MakerTraits makerTraits, uint256 additionalMask) external
```

**Purpose:** Invalidates multiple orders using bit invalidator (for gas efficiency)
**Parameters:**

- `makerTraits` - Order configuration traits
- `additionalMask` - Additional bitmask to invalidate
  **Access:** Public (only by order maker)

### 6. Order State Queries

#### Bit Invalidator Status

```solidity
function bitInvalidatorForOrder(address maker, uint256 slot) external view returns(uint256)
```

**Purpose:** Returns bitmask for double-spend invalidators
**Parameters:**

- `maker` - Maker address
- `slot` - Slot number to check
  **Returns:** Bitmask indicating invalidated orders
  **Access:** Public

#### Remaining Amount

```solidity
function remainingInvalidatorForOrder(address maker, bytes32 orderHash) external view returns(uint256)
```

**Purpose:** Returns remaining amount for a specific order
**Parameters:**

- `maker` - Maker address
- `orderHash` - Order hash
  **Returns:** Remaining amount to be filled
  **Access:** Public

#### Raw Remaining Amount

```solidity
function rawRemainingInvalidatorForOrder(address maker, bytes32 orderHash) external view returns(uint256)
```

**Purpose:** Returns raw remaining amount (inverse if filled, 0 if never filled)
**Parameters:**

- `maker` - Maker address
- `orderHash` - Order hash
  **Returns:** Raw remaining amount
  **Access:** Public

### 7. Predicate Checking

```solidity
function checkPredicate(bytes calldata predicate) public view returns(bool)
```

**Purpose:** Checks if a predicate condition is met
**Parameters:**

- `predicate` - Predicate data to evaluate
  **Returns:** True if predicate is satisfied
  **Access:** Public

### 8. Simulation

```solidity
function simulate(address target, bytes calldata data) external
```

**Purpose:** Simulates execution of arbitrary code (always reverts with results)
**Parameters:**

- `target` - Target address to simulate
- `data` - Calldata to simulate
  **Access:** Public

### 9. Administrative Functions

#### Pause Contract

```solidity
function pause() external onlyOwner
```

**Purpose:** Pauses all trading functionality
**Access:** Owner only

#### Unpause Contract

```solidity
function unpause() external onlyOwner
```

**Purpose:** Unpauses all trading functionality
**Access:** Owner only

#### Get Owner

```solidity
function owner() external view returns(address)
```

**Purpose:** Returns the contract owner
**Returns:** Owner address
**Access:** Public

#### Check Pause Status

```solidity
function paused() external view returns(bool)
```

**Purpose:** Returns whether the contract is paused
**Returns:** True if paused
**Access:** Public

## Events

### OrderFilled

```solidity
event OrderFilled(bytes32 orderHash, uint256 remainingAmount)
```

**Emitted when:** An order is successfully filled
**Parameters:**

- `orderHash` - Hash of the filled order
- `remainingAmount` - Remaining amount to be filled

### OrderCancelled

```solidity
event OrderCancelled(bytes32 orderHash)
```

**Emitted when:** An order is cancelled
**Parameters:**

- `orderHash` - Hash of the cancelled order

### BitInvalidatorUpdated

```solidity
event BitInvalidatorUpdated(address indexed maker, uint256 slotIndex, uint256 slotValue)
```

**Emitted when:** Bit invalidator is updated for mass cancellation
**Parameters:**

- `maker` - Maker address
- `slotIndex` - Slot index that was updated
- `slotValue` - New slot value

## Error Codes

### Order Validation Errors

- `InvalidatedOrder()` - Order has been invalidated
- `TakingAmountExceeded()` - Taking amount exceeds allowed limit
- `PrivateOrder()` - Order is private and caller is not authorized
- `BadSignature()` - Order signature is invalid
- `OrderExpired()` - Order has expired
- `WrongSeriesNonce()` - Series nonce is incorrect
- `SwapWithZeroAmount()` - Attempting to swap zero amount
- `PartialFillNotAllowed()` - Partial fills not allowed for this order
- `OrderIsNotSuitableForMassInvalidation()` - Order cannot be mass invalidated

### Amount Validation Errors

- `TakingAmountTooHigh()` - Taking amount exceeds threshold
- `MakingAmountTooLow()` - Making amount below threshold
- `MismatchArraysLengths()` - Array lengths don't match

### Transfer Errors

- `TransferFromMakerToTakerFailed()` - Failed to transfer from maker to taker
- `TransferFromTakerToMakerFailed()` - Failed to transfer from taker to maker
- `InvalidPermit2Transfer()` - Invalid Permit2 transfer

### Other Errors

- `ReentrancyDetected()` - Reentrancy attack detected
- `PredicateIsNotTrue()` - Predicate condition not met
- `EpochManagerAndBitInvalidatorsAreIncompatible()` - Incompatible configuration

## Usage Examples

### 1. Basic Order Fill

```javascript
// Fill an order with EOA signature
const result = await lop.fillOrder(order, signature.r, signature.vs, amount, takerTraits);
```

### 2. Check Order Status

```javascript
// Check remaining amount
const remaining = await lop.remainingInvalidatorForOrder(maker, orderHash);

// Check bit invalidator
const invalidator = await lop.bitInvalidatorForOrder(maker, slot);
```

### 3. Cancel Order

```javascript
// Cancel a single order
await lop.cancelOrder(makerTraits, orderHash);

// Cancel multiple orders
await lop.cancelOrders(makerTraitsArray, orderHashesArray);
```

### 4. Get Domain Separator

```javascript
// Get domain separator for order signing
const domainSeparator = await lop.DOMAIN_SEPARATOR();
```

## Network-Specific Information

### Base Sepolia

- **Contract Address:** `0xdfC365795F146a6755998C5e916a592A9706eDC6`
- **WETH Address:** `0x4200000000000000000000000000000000000006`
- **Chain ID:** 84532
- **RPC URL:** `https://sepolia.base.org`
- **Block Explorer:** `https://sepolia.basescan.org`

## Security Considerations

1. **Signature Verification:** Always verify order signatures before filling
2. **Reentrancy Protection:** Contract includes reentrancy protection
3. **Pause Mechanism:** Contract can be paused by owner in emergencies
4. **Order Validation:** Orders are validated for expiration and conditions
5. **Amount Limits:** Thresholds prevent unfavorable trades

## Gas Optimization

1. **Bit Invalidator:** Use for gas-efficient mass cancellation
2. **Batch Operations:** Use `cancelOrders` for multiple cancellations
3. **Partial Fills:** Enable partial fills for better liquidity
4. **Contract Signatures:** Use for gas-efficient contract-based orders

## Integration Notes

1. **EIP-712:** Orders use EIP-712 for structured data signing
2. **ERC-1271:** Supports contract-based signatures
3. **Permit2:** Supports Permit2 for gasless approvals
4. **WETH Integration:** Native ETH/WETH wrapping/unwrapping support
5. **Predicates:** Custom execution conditions supported
6. **Interactions:** Pre/post execution callbacks supported
