# Escrow Factory Fusion+ Requirements Document

## Introduction

This document defines the requirements for implementing an escrow factory canister that handles the actual HTLC (Hashed Timelock Contract) escrow creation and management for the Chain Fusion+ Protocol. The escrow factory is responsible for creating, managing, and executing escrow contracts that lock and release assets during cross-chain swaps.

**Critical Innovation - Atomic Escrow Creation:**
The escrow factory leverages ICP's Chain Fusion to atomically create escrows on both ICP and EVM chains simultaneously. This eliminates the coordination problem present in traditional cross-chain systems where escrow creation requires separate transactions.

**Escrow Factory Role in HTLC System:**

- **Atomic Escrow Creation**: Create HTLC escrows on both chains atomically via Chain Fusion
- **Asset Locking**: Lock user assets in escrows with hashlock and timelock mechanisms
- **Secret Revelation**: Release assets when correct secrets are revealed
- **Recovery Handling**: Handle timelock expirations and process refunds
- **Safety Deposits**: Manage safety deposit distribution to incentivize resolvers

**Orderbook Coordination:**

- Receives coordination requests from orderbook canister
- Does not manage order state or user identity
- Focuses purely on escrow lifecycle management

## Requirements

### Requirement 1: Atomic HTLC Escrow Creation via Chain Fusion

**User Story:** As the escrow factory, I want to atomically create HTLC escrows on both chains via Chain Fusion so that cross-chain coordination is eliminated.

#### Acceptance Criteria

1. WHEN escrow creation is requested THEN the escrow factory SHALL create both ICP and EVM escrows atomically
2. WHEN ICP escrow is created THEN the escrow factory SHALL lock ICP assets using reverse gas model
3. WHEN EVM escrow is created THEN the escrow factory SHALL use Chain Fusion for EVM contract deployment
4. WHEN atomic creation succeeds THEN the escrow factory SHALL return both escrow addresses
5. WHEN atomic creation fails THEN the escrow factory SHALL rollback both escrows and return error
6. WHEN escrow parameters are provided THEN the escrow factory SHALL validate hashlock and timelock values
7. WHEN deterministic addresses are needed THEN the escrow factory SHALL compute escrow addresses using Create2 algorithm
8. WHEN safety deposits are required THEN the escrow factory SHALL collect and manage ETH deposits for escrow creation
9. WHEN token approvals are needed THEN the escrow factory SHALL verify token transfer permissions before escrow creation

### Requirement 2: Asset Locking and Management

**User Story:** As the escrow factory, I want to lock assets securely so that they cannot be accessed until proper conditions are met.

#### Acceptance Criteria

1. WHEN assets are locked THEN the escrow factory SHALL verify sufficient balance before locking
2. WHEN ICP assets are locked THEN the escrow factory SHALL transfer tokens to escrow canister
3. WHEN EVM assets are locked THEN the escrow factory SHALL coordinate with EVM escrow contracts
4. WHEN locking succeeds THEN the escrow factory SHALL confirm assets are locked and inaccessible
5. WHEN locking fails THEN the escrow factory SHALL return assets to original owner
6. WHEN safety deposits are required THEN the escrow factory SHALL handle deposit collection and distribution
7. WHEN ERC20 tokens are involved THEN the escrow factory SHALL handle SafeERC20 transfers
8. WHEN native tokens are involved THEN the escrow factory SHALL handle native token transfers
9. WHEN partial fills are supported THEN the escrow factory SHALL handle Merkle tree of secrets validation
10. WHEN multiple fills occur THEN the escrow factory SHALL validate partial fill amounts and secrets

### Requirement 3: Secret Revelation and Asset Release

**User Story:** As the escrow factory, I want to release assets when correct secrets are revealed so that swaps can complete successfully.

#### Acceptance Criteria

1. WHEN secret is provided THEN the escrow factory SHALL verify it matches stored hashlock
2. WHEN secret verification succeeds THEN the escrow factory SHALL release assets to designated recipients
3. WHEN ICP assets are released THEN the escrow factory SHALL transfer tokens from escrow to recipient
4. WHEN EVM assets are released THEN the escrow factory SHALL coordinate with EVM escrow contracts
5. WHEN release succeeds THEN the escrow factory SHALL confirm assets are transferred
6. WHEN release fails THEN the escrow factory SHALL maintain locked state and return error
7. WHEN safety deposits are involved THEN the escrow factory SHALL distribute deposits to executing resolver
8. WHEN public withdrawal is available THEN the escrow factory SHALL allow public withdrawal after timelock
9. WHEN withdrawal authorization is needed THEN the escrow factory SHALL verify caller permissions
10. WHEN rescue operations are required THEN the escrow factory SHALL provide rescue functionality for stuck funds

### Requirement 4: Timelock Expiration and Recovery

**User Story:** As the escrow factory, I want to handle timelock expirations so that users can recover assets if swaps fail.

#### Acceptance Criteria

1. WHEN timelock expires THEN the escrow factory SHALL allow authorized parties to cancel escrow
2. WHEN cancellation is requested THEN the escrow factory SHALL return assets to original owners
3. WHEN ICP escrow is cancelled THEN the escrow factory SHALL return ICP assets to maker
4. WHEN EVM escrow is cancelled THEN the escrow factory SHALL coordinate with EVM contracts
5. WHEN safety deposits are involved THEN the escrow factory SHALL distribute deposits to cancellation executor
6. WHEN recovery succeeds THEN the escrow factory SHALL confirm assets are returned
7. WHEN recovery fails THEN the escrow factory SHALL maintain locked state and log error
8. WHEN public cancellation is available THEN the escrow factory SHALL allow public cancellation after timelock
9. WHEN cancellation authorization is needed THEN the escrow factory SHALL verify caller permissions
10. WHEN timelock coordination occurs THEN the escrow factory SHALL ensure source and destination timelocks are synchronized

### Requirement 5: Fusion+ Protocol Compliance

**User Story:** As the escrow factory, I want to follow Fusion+ protocol specifications so that escrows are compatible with the 1inch Fusion+ design.

#### Acceptance Criteria

1. WHEN escrows are created THEN the escrow factory SHALL follow Fusion+ escrow structure
2. WHEN finality locks are active THEN the escrow factory SHALL prevent withdrawals until lock expires
3. WHEN timelock coordination occurs THEN the escrow factory SHALL follow Fusion+ timelock periods
4. WHEN safety deposits are managed THEN the escrow factory SHALL follow Fusion+ deposit mechanics
5. WHEN partial fills are supported THEN the escrow factory SHALL use Merkle tree of secrets (placeholder for future implementation)
6. WHEN escrow parameters are set THEN the escrow factory SHALL validate against Fusion+ specifications

### Requirement 6: Cross-Chain Coordination via Chain Fusion

**User Story:** As the escrow factory, I want to coordinate escrow operations across ICP and EVM chains using Chain Fusion so that cross-chain swaps work seamlessly.

#### Acceptance Criteria

1. WHEN cross-chain coordination is needed THEN the escrow factory SHALL use Chain Fusion for EVM operations
2. WHEN EVM operations are required THEN the escrow factory SHALL use threshold ECDSA signing
3. WHEN state verification is needed THEN the escrow factory SHALL verify escrow state on both chains
4. WHEN coordination fails THEN the escrow factory SHALL handle partial failures gracefully
5. WHEN both escrows are ready THEN the escrow factory SHALL signal readiness to orderbook
6. WHEN chain-specific operations occur THEN the escrow factory SHALL handle chain-specific requirements

### Requirement 7: Error Handling and Recovery

**User Story:** As the escrow factory, I want robust error handling so that escrow operations are reliable and recoverable.

#### Acceptance Criteria

1. WHEN escrow operations fail THEN the escrow factory SHALL provide detailed error information
2. WHEN partial failures occur THEN the escrow factory SHALL maintain consistent state
3. WHEN recovery is needed THEN the escrow factory SHALL provide recovery mechanisms
4. WHEN critical errors occur THEN the escrow factory SHALL prevent asset loss
5. WHEN debugging is needed THEN the escrow factory SHALL provide comprehensive logging
6. WHEN state inconsistencies occur THEN the escrow factory SHALL detect and handle gracefully

### Requirement 8: Performance and Scalability

**User Story:** As the escrow factory, I want efficient escrow management so that the system can handle multiple concurrent swaps.

#### Acceptance Criteria

1. WHEN multiple escrows are created THEN the escrow factory SHALL handle them concurrently
2. WHEN escrow operations are performed THEN the escrow factory SHALL complete within reasonable time
3. WHEN storage grows THEN the escrow factory SHALL maintain acceptable performance
4. WHEN concurrent operations occur THEN the escrow factory SHALL prevent race conditions
5. WHEN system resources are limited THEN the escrow factory SHALL prioritize critical operations
6. WHEN load testing occurs THEN the escrow factory SHALL demonstrate stability under expected usage

### Requirement 9: Security and Validation

**User Story:** As the escrow factory, I want secure escrow operations so that assets are protected during cross-chain swaps.

#### Acceptance Criteria

1. WHEN escrow creation occurs THEN the escrow factory SHALL validate all parameters
2. WHEN asset locking occurs THEN the escrow factory SHALL verify authorization
3. WHEN secret revelation occurs THEN the escrow factory SHALL validate secret authenticity
4. WHEN timelock operations occur THEN the escrow factory SHALL enforce timelock constraints
5. WHEN safety deposits are handled THEN the escrow factory SHALL ensure proper distribution
6. WHEN cross-chain operations occur THEN the escrow factory SHALL verify chain state consistency

### Requirement 10: Core EscrowFactory API Functions

**User Story:** As the escrow factory, I want to provide the core API functions that resolvers need so that cross-chain swaps can be executed properly.

#### Acceptance Criteria

1. WHEN deterministic address computation is needed THEN the escrow factory SHALL provide `addressOfEscrowSrc(Immutables)` function
2. WHEN destination escrow creation is needed THEN the escrow factory SHALL provide `createDstEscrow(Immutables, timestamp)` function
3. WHEN source escrow creation is triggered THEN the escrow factory SHALL handle `_postInteraction()` calls from LOP
4. WHEN escrow parameters are provided THEN the escrow factory SHALL validate Immutables struct format
5. WHEN safety deposits are required THEN the escrow factory SHALL collect exact `msg.value` for escrow creation
6. WHEN token approvals are needed THEN the escrow factory SHALL verify token transfer permissions
7. WHEN timelock coordination occurs THEN the escrow factory SHALL validate `srcCancellationTimestamp` requirements
8. WHEN escrow creation succeeds THEN the escrow factory SHALL emit `DstEscrowCreated` event
9. WHEN escrow creation fails THEN the escrow factory SHALL return detailed error information
10. WHEN implementation addresses are requested THEN the escrow factory SHALL provide `ESCROW_SRC_IMPLEMENTATION()` and `ESCROW_DST_IMPLEMENTATION()` functions

### Requirement 11: Integration with Orderbook

**User Story:** As the escrow factory, I want to integrate with the orderbook canister so that escrow operations are coordinated with order management.

#### Acceptance Criteria

1. WHEN orderbook requests escrow creation THEN the escrow factory SHALL create escrow and report status
2. WHEN orderbook requests escrow completion THEN the escrow factory SHALL release assets and report success
3. WHEN orderbook requests escrow cancellation THEN the escrow factory SHALL cancel escrow and report status
4. WHEN escrow state changes THEN the escrow factory SHALL notify orderbook of state updates
5. WHEN coordination is needed THEN the escrow factory SHALL provide necessary data to orderbook
6. WHEN integration errors occur THEN the escrow factory SHALL provide clear error information to orderbook

This document defines the escrow factory as a separate component focused purely on HTLC escrow lifecycle management, while the orderbook handles order coordination and state management.
