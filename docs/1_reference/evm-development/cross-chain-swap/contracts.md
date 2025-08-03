# Cross-Chain Swap Contracts Documentation

## Overview

This document describes the main contracts in the 1inch Cross-Chain Swap protocol, their purposes, and interdependencies. The protocol enables atomic cross-chain swaps using escrow contracts on both source and destination chains.

## Contract Architecture

### Core Contracts

#### 1. **EscrowFactory** (`contracts/EscrowFactory.sol`)

**Purpose**: Main factory contract that creates escrow contracts for cross-chain atomic swaps.

**Key Features**:

- Creates both source and destination escrow contracts
- Integrates with 1inch Limit Order Protocol
- Manages deterministic address computation for escrows
- Handles safety deposits and token transfers

**Dependencies**:

- `BaseEscrowFactory` (inherits)
- `EscrowSrc` (creates instances)
- `EscrowDst` (creates instances)
- `MerkleStorageInvalidator` (inherits)
- `ResolverValidationExtension` (inherits)

#### 2. **BaseEscrowFactory** (`contracts/BaseEscrowFactory.sol`)

**Purpose**: Abstract base contract providing core factory functionality.

**Key Features**:

- Handles `_postInteraction` from Limit Order Protocol
- Manages escrow creation with deterministic addresses
- Validates partial fills for multiple-fill orders
- Computes escrow addresses using Create2

**Dependencies**:

- `IEscrowFactory` (implements)
- `MerkleStorageInvalidator` (inherits)
- `ResolverValidationExtension` (inherits)
- `ImmutablesLib` (uses)
- `TimelocksLib` (uses)

#### 3. **EscrowSrc** (`contracts/EscrowSrc.sol`)

**Purpose**: Source chain escrow contract that locks funds initially.

**Key Features**:

- Locks funds on source chain
- Allows withdrawal with secret verification
- Supports public withdrawal and cancellation
- Manages timelock-based access control

**Dependencies**:

- `Escrow` (inherits)
- `IEscrowSrc` (implements)
- `BaseEscrow` (inherits)
- `TimelocksLib` (uses)
- `ImmutablesLib` (uses)

#### 4. **EscrowDst** (`contracts/EscrowDst.sol`)

**Purpose**: Destination chain escrow contract that locks funds for the swap.

**Key Features**:

- Locks funds on destination chain
- Allows withdrawal with secret verification
- Supports public withdrawal and cancellation
- Manages timelock-based access control

**Dependencies**:

- `Escrow` (inherits)
- `IEscrowDst` (implements)
- `BaseEscrow` (inherits)
- `TimelocksLib` (uses)

#### 5. **BaseEscrow** (`contracts/BaseEscrow.sol`)

**Purpose**: Abstract base contract providing core escrow functionality.

**Key Features**:

- Manages access control modifiers
- Handles token transfers (ERC20 and native)
- Validates secrets and immutables
- Provides rescue functionality
- Manages timelock-based operations

**Dependencies**:

- `IBaseEscrow` (implements)
- `ImmutablesLib` (uses)
- `TimelocksLib` (uses)
- OpenZeppelin contracts (SafeERC20, IERC20)

#### 6. **Escrow** (`contracts/Escrow.sol`)

**Purpose**: Abstract contract that adds proxy validation to BaseEscrow.

**Key Features**:

- Validates that computed escrow address matches contract address
- Uses Create2 for deterministic address computation
- Stores proxy bytecode hash

**Dependencies**:

- `BaseEscrow` (inherits)
- `IEscrow` (implements)
- `ImmutablesLib` (uses)
- `ProxyHashLib` (uses)
- OpenZeppelin Create2

#### 7. **MerkleStorageInvalidator** (`contracts/MerkleStorageInvalidator.sol`)

**Purpose**: Handles invalidation of hashed secrets for multiple-fill orders.

**Key Features**:

- Validates Merkle proofs for secret hashes
- Stores validation data for partial fills
- Integrates with Limit Order Protocol
- Manages access control for LOP calls

**Dependencies**:

- `IMerkleStorageInvalidator` (implements)
- `ITakerInteraction` (implements)
- OpenZeppelin MerkleProof
- Limit Order Protocol contracts

### Interface Contracts

#### **IBaseEscrow** (`contracts/interfaces/IBaseEscrow.sol`)

- Defines core escrow functionality interface
- Used by BaseEscrow implementation

#### **IEscrow** (`contracts/interfaces/IEscrow.sol`)

- Defines escrow-specific interface
- Used by Escrow implementation

#### **IEscrowSrc** (`contracts/interfaces/IEscrowSrc.sol`)

- Defines source escrow specific functions
- Used by EscrowSrc implementation

#### **IEscrowDst** (`contracts/interfaces/IEscrowDst.sol`)

- Defines destination escrow specific functions
- Used by EscrowDst implementation

#### **IEscrowFactory** (`contracts/interfaces/IEscrowFactory.sol`)

- Defines factory functionality interface
- Used by BaseEscrowFactory implementation

#### **IMerkleStorageInvalidator** (`contracts/interfaces/IMerkleStorageInvalidator.sol`)

- Defines Merkle storage invalidator interface
- Used by MerkleStorageInvalidator implementation

### Library Contracts

#### **TimelocksLib** (`contracts/libraries/TimelocksLib.sol`)

- Manages timelock stages and calculations
- Used by all escrow contracts for time-based access control

#### **ImmutablesLib** (`contracts/libraries/ImmutablesLib.sol`)

- Handles immutable data structures and validation
- Used by all contracts for data management

#### **ProxyHashLib** (`contracts/libraries/ProxyHashLib.sol`)

- Computes proxy bytecode hashes for deterministic addresses
- Used by Escrow and EscrowFactory contracts

## Contract Dependencies Diagram

```
EscrowFactory
├── BaseEscrowFactory
│   ├── IEscrowFactory
│   ├── MerkleStorageInvalidator
│   │   └── IMerkleStorageInvalidator
│   ├── ResolverValidationExtension
│   ├── ImmutablesLib
│   └── TimelocksLib
├── EscrowSrc
│   ├── Escrow
│   │   ├── BaseEscrow
│   │   │   ├── IBaseEscrow
│   │   │   ├── ImmutablesLib
│   │   │   └── TimelocksLib
│   │   ├── IEscrow
│   │   ├── ImmutablesLib
│   │   └── ProxyHashLib
│   ├── IEscrowSrc
│   ├── ImmutablesLib
│   └── TimelocksLib
└── EscrowDst
    ├── Escrow (same as above)
    ├── IEscrowDst
    ├── ImmutablesLib
    └── TimelocksLib
```

## Key Workflows

### 1. **Source Chain Escrow Creation**

1. User creates limit order via 1inch LOP
2. `EscrowFactory._postInteraction` is called
3. Source escrow is deployed with deterministic address
4. Funds are locked in source escrow

### 2. **Destination Chain Escrow Creation**

1. Taker calls `EscrowFactory.createDstEscrow`
2. Destination escrow is deployed with deterministic address
3. Funds are locked in destination escrow

### 3. **Cross-Chain Swap Execution**

1. Taker reveals secret on source chain
2. Source escrow releases funds to taker
3. Taker reveals same secret on destination chain
4. Destination escrow releases funds to maker

### 4. **Cancellation/Rescue**

1. If swap fails, cancellation functions can be called
2. Funds are returned to original owners
3. Rescue functions allow recovery of stuck funds

## Security Features

- **Deterministic Addresses**: All escrows use Create2 for predictable addresses
- **Timelock Protection**: Time-based access control prevents premature actions
- **Secret Verification**: Cryptographic secrets ensure atomic execution
- **Access Control**: Multiple layers of authorization checks
- **Merkle Proof Validation**: Secure validation for partial fills
- **Rescue Mechanisms**: Emergency fund recovery capabilities

## External Dependencies

- **OpenZeppelin Contracts**: IERC20, SafeERC20, Create2, MerkleProof
- **1inch Limit Order Protocol**: Order management and settlement
- **1inch Limit Order Settlement**: Extension framework
- **Solidity Utils**: AddressLib, SafeERC20 utilities
