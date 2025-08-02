# ICP Fusion+ Mechanical Turk - Requirements Document

> **Purpose**: Define requirements for implementing a mechanical-turk version of 1inch Fusion+ protocol
> **Scope**: Cross-chain atomic swaps between ICP and Ethereum using existing 1inch infrastructure
> **Strategy**: Leverage deployed contracts on Base Sepolia, build ICP components, integrate via frontend

## 🎯 **Core Requirements**

### **Requirement 1: Leverage Existing 1inch Infrastructure**

**User Story:** As a developer, I want to use the existing 1inch Fusion+ contracts deployed on Base Sepolia so that I don't duplicate infrastructure and can focus on ICP integration.

#### Acceptance Criteria

1. WHEN the system is deployed THEN it SHALL use the existing EscrowFactory contract on Base Sepolia: `0xE53136D9De56672e8D2665C98653AC7b8A60Dc44`
2. WHEN Ethereum interactions occur THEN they SHALL use the Base Sepolia testnet for all contract calls
3. WHEN escrow contracts are needed THEN they SHALL be deployed through the existing EscrowFactory using the factory pattern
4. WHEN the system integrates with Ethereum THEN it SHALL use the existing Limit Order Protocol (LOP) contracts on Base
5. WHEN resolver functionality is needed THEN it SHALL use the existing ResolverExample contract patterns from the 1inch repo

### **Requirement 2: ICP-Specific Infrastructure Development**

**User Story:** As a developer, I want to build ICP-specific components that mirror the Ethereum infrastructure so that I can create a complete cross-chain system.

#### Acceptance Criteria

1. WHEN ICP escrow functionality is needed THEN the system SHALL implement EscrowSrc and EscrowDst canisters that mirror the Ethereum contract behavior
2. WHEN ICP order management is needed THEN the system SHALL implement an orderbook canister that stores and manages swap orders
3. WHEN ICP authentication is needed THEN the system SHALL implement SIWE (Sign-In with Ethereum) integration for cross-chain identity
4. WHEN ICP frontend is needed THEN the system SHALL serve a web interface from the ICP asset canister that communicates with both chains

### **Requirement 3: Cross-Chain Integration Architecture**

**User Story:** As a system architect, I want to create a unified frontend that coordinates between ICP and Ethereum so that users have a seamless experience.

#### Acceptance Criteria

1. WHEN users access the system THEN they SHALL interact with a single frontend served from ICP
2. WHEN Ethereum transactions are needed THEN the frontend SHALL use MetaMask to sign transactions on Base Sepolia
3. WHEN ICP transactions are needed THEN the frontend SHALL use Internet Identity or MetaMask-derived principals
4. WHEN cross-chain verification is needed THEN the system SHALL use off-chain relayer services to monitor both chains
5. WHEN state synchronization is needed THEN the system SHALL use event monitoring and HTTP outcalls for cross-chain communication

### **Requirement 4: Mechanical Turk Flow Implementation**

**User Story:** As a user, I want to experience atomic cross-chain swaps with manual coordination so that I can validate the core mechanics before full automation.

#### Acceptance Criteria

1. WHEN a maker creates an ICP → ETH order THEN they SHALL lock ICP tokens in the ICP escrow canister
2. WHEN a resolver accepts an order THEN they SHALL lock ETH tokens in the Base Sepolia escrow contract
3. WHEN both escrows are funded THEN the relayer SHALL manually verify cross-chain state
4. WHEN verification is complete THEN the maker SHALL share the secret with the resolver
5. WHEN the resolver completes the transfer THEN they SHALL use the receipt + secret to claim locked tokens
6. WHEN the PoC is active THEN all coordination SHALL be manual to simulate best-case scenarios

### **Requirement 5: Gasless User Experience**

**User Story:** As a user, I want a gasless experience on both chains so that I can trade without worrying about transaction fees.

#### Acceptance Criteria

1. WHEN users interact with ICP THEN the relayer SHALL pay all cycle costs for canister calls
2. WHEN users interact with Ethereum THEN the resolver SHALL pay gas fees for transactions they initiate
3. WHEN the frontend is served THEN it SHALL be hosted from ICP asset canister (no hosting fees)
4. WHEN authentication occurs THEN it SHALL use SIWE for seamless cross-chain identity

### **Requirement 6: Development and Testing Environment**

**User Story:** As a developer, I want a safe testing environment that mirrors production so that I can validate functionality without risk.

#### Acceptance Criteria

1. WHEN developing on ICP THEN the system SHALL use ICP mainnet with small test amounts
2. WHEN developing on Ethereum THEN the system SHALL use Base Sepolia testnet exclusively
3. WHEN testing cross-chain flows THEN the system SHALL use the existing Base Sepolia contracts
4. WHEN deploying ICP components THEN the system SHALL use DFX for canister deployment
5. WHEN managing Ethereum development THEN the system SHALL use Foundry for contract interaction

### **Requirement 7: Authentication and Identity Management**

**User Story:** As a user, I want secure authentication that works across both chains so that I can maintain consistent identity.

#### Acceptance Criteria

1. WHEN users first access the system THEN they SHALL authenticate using MetaMask (SIWE)
2. WHEN SIWE authentication completes THEN the system SHALL derive a deterministic ICP principal
3. WHEN users return THEN they SHALL use the same MetaMask wallet to access their ICP principal
4. WHEN principal derivation occurs THEN it SHALL be consistent and reproducible for the same ETH address
5. WHEN cross-chain operations occur THEN the system SHALL maintain consistent user identity

### **Requirement 8: Order Management and State Tracking**

**User Story:** As a user, I want to track my orders and their status so that I know when swaps are in progress or completed.

#### Acceptance Criteria

1. WHEN orders are created THEN they SHALL be stored in the ICP orderbook canister
2. WHEN order status changes THEN the system SHALL update state and notify relevant parties
3. WHEN resolvers scan for orders THEN they SHALL see all available orders with current status
4. WHEN orders timeout or fail THEN the system SHALL handle cleanup and fund recovery
5. WHEN cross-chain state changes THEN the system SHALL synchronize status across both chains

### **Requirement 9: Error Handling and Recovery**

**User Story:** As a user, I want clear error messages and recovery options so that I can understand and resolve issues.

#### Acceptance Criteria

1. WHEN transactions fail THEN the system SHALL display clear, non-technical error messages
2. WHEN funds are locked but swaps don't complete THEN users SHALL have clear recovery instructions
3. WHEN cross-chain verification fails THEN the system SHALL provide diagnostic information
4. WHEN timeouts occur THEN the system SHALL handle cleanup automatically where possible
5. WHEN manual intervention is needed THEN the relayer SHALL have override capabilities for testing

### **Requirement 10: Monitoring and Observability**

**User Story:** As a developer/relayer, I want comprehensive monitoring so that I can track system health and debug issues.

#### Acceptance Criteria

1. WHEN swaps occur THEN all state changes SHALL be logged with timestamps
2. WHEN cross-chain events happen THEN they SHALL be tracked in a queryable format
3. WHEN errors occur THEN they SHALL be logged with sufficient context for debugging
4. WHEN the system is running THEN health metrics SHALL be available for monitoring
5. WHEN manual coordination is needed THEN the relayer SHALL have clear visibility into both chains

## 🎯 **Technical Architecture**

### **Infrastructure Components**

```
ICP Chain:
├── Orderbook Canister (stores orders)
├── EscrowSrc Canister (locks ICP tokens)
├── EscrowDst Canister (locks ICP tokens for ETH→ICP)
├── Asset Canister (serves frontend)
└── Relayer Service (monitors both chains)

Ethereum Chain (Base Sepolia):
├── EscrowFactory (0xE53136D9De56672e8D2665C98653AC7b8A60Dc44)
├── EscrowSrc contracts (deployed per swap)
├── EscrowDst contracts (deployed per swap)
└── Limit Order Protocol (existing)

Frontend:
├── Single web interface (served from ICP)
├── MetaMask integration (Ethereum transactions)
├── SIWE authentication (cross-chain identity)
└── Manual relayer coordination (PoC only)
```

### **Key Design Decisions**

1. **Use Existing Infrastructure**: Leverage 1inch's deployed contracts on Base Sepolia
2. **ICP-First Development**: Build ICP components that mirror Ethereum patterns
3. **Single Frontend**: Serve everything from ICP asset canister
4. **Manual Coordination**: Human relayer for PoC validation
5. **Gasless Experience**: Relayer pays cycles, resolver pays gas

## 🚀 **Success Criteria**

### **Technical**

- ✅ Successful ICP ↔ ETH atomic swaps
- ✅ Integration with existing Base Sepolia contracts
- ✅ Cross-chain state synchronization
- ✅ Gasless user experience on both chains

### **User Experience**

- ✅ Single frontend for all operations
- ✅ Clear order tracking and status updates
- ✅ Seamless cross-chain authentication
- ✅ Error handling and recovery mechanisms

### **Development**

- ✅ Reusable codebase for future enhancements
- ✅ Clear separation of concerns
- ✅ Comprehensive monitoring and logging
- ✅ Safe testing environment

## 📋 **Implementation Phases**

### **Phase 1: Foundation**

1. Set up DFX project with ICP canisters
2. Integrate with existing Base Sepolia contracts
3. Implement basic frontend with MetaMask integration
4. Create SIWE authentication flow

### **Phase 2: Core Mechanics**

1. Implement ICP escrow canisters
2. Build orderbook canister
3. Create cross-chain verification system
4. Implement manual relayer coordination

### **Phase 3: User Experience**

1. Build complete frontend interface
2. Implement order management and tracking
3. Add error handling and recovery
4. Create monitoring and observability

### **Phase 4: Testing and Validation**

1. End-to-end testing of both swap directions
2. Error scenario testing
3. Performance optimization
4. Documentation and deployment guides

---

> **Note**: This requirements document builds upon the existing 1inch Fusion+ infrastructure while adding ICP-specific components for a complete cross-chain atomic swap system.
