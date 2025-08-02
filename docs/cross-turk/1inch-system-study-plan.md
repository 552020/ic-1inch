# 1inch Fusion+ System Study Plan

> **Purpose**: Understand the existing 1inch Fusion+ system before integrating with our ICP Mechanical Turk
> **Goal**: Learn the interfaces, data structures, and workflows to build compatible ICP components
> **Timeline**: 1-2 weeks of focused study before implementation

## 🎯 **Core Study Areas**

### **1. Contract Architecture & Interfaces**

#### **A. EscrowFactory Interface**

```solidity
// Key methods to understand:
interface IEscrowFactory {
    function createDstEscrow(IBaseEscrow.Immutables calldata dstImmutables, uint256 srcCancellationTimestamp) external payable;
    function addressOfEscrowSrc(IBaseEscrow.Immutables calldata immutables) external view returns (address);
    function addressOfEscrowDst(IBaseEscrow.Immutables calldata immutables) external view returns (address);
}
```

**Study Tasks:**

- [ ] Understand `IBaseEscrow.Immutables` structure
- [ ] Learn factory pattern and deterministic address computation
- [ ] Study safety deposit requirements
- [ ] Understand timelock mechanisms

#### **B. Base Escrow Interface**

```solidity
interface IBaseEscrow {
    struct Immutables {
        bytes32 orderHash;
        bytes32 hashlock;  // Hash of the secret
        Address maker;
        Address taker;
        Address token;
        uint256 amount;
        uint256 safetyDeposit;
        Timelocks timelocks;
    }

    function withdraw(bytes32 secret, Immutables calldata immutables) external;
    function cancel(Immutables calldata immutables) external;
    function rescueFunds(address token, uint256 amount, Immutables calldata immutables) external;
}
```

**Study Tasks:**

- [ ] Understand withdrawal mechanism with secret
- [ ] Learn cancellation logic and timelocks
- [ ] Study rescue funds functionality
- [ ] Understand event emission patterns

### **2. Limit Order Protocol Integration**

#### **A. EIP-712 Order Structure**

```solidity
struct Order {
    uint256 salt;
    Address maker;
    Address receiver;
    Address makerAsset;
    Address takerAsset;
    uint256 makingAmount;
    uint256 takingAmount;
    MakerTraits makerTraits;
}
```

**Study Tasks:**

- [ ] Learn EIP-712 signing process
- [ ] Understand order validation
- [ ] Study extension data (hashlock, timelocks)
- [ ] Learn postInteraction callback mechanism

#### **B. Order Execution Flow**

```
1. Maker signs EIP-712 order (off-chain)
2. Resolver calls fillOrder() with extension data
3. LOP validates order and calls postInteraction
4. postInteraction creates EscrowSrc
5. Resolver creates EscrowDst manually
6. Maker reveals secret to resolver
7. Resolver withdraws from both escrows
```

**Study Tasks:**

- [ ] Trace complete order execution flow
- [ ] Understand extension data encoding
- [ ] Learn postInteraction callback timing
- [ ] Study cross-chain coordination

### **3. Data Structures & Types**

#### **A. Timelocks Structure**

```solidity
struct Timelocks {
    uint32 srcWithdrawal;
    uint32 srcPublicWithdrawal;
    uint32 srcCancellation;
    uint32 srcPublicCancellation;
    uint32 dstWithdrawal;
    uint32 dstPublicWithdrawal;
    uint32 dstCancellation;
    uint32 dstPublicCancellation;
}
```

**Study Tasks:**

- [ ] Understand each timelock period
- [ ] Learn withdrawal vs cancellation logic
- [ ] Study public vs private periods
- [ ] Understand time-based security

#### **B. Address & Token Handling**

```solidity
struct Address {
    uint256 chainId;
    address addr;
}
```

**Study Tasks:**

- [ ] Learn cross-chain address representation
- [ ] Understand chain ID handling
- [ ] Study token address validation
- [ ] Learn multi-chain token support

### **4. Security Mechanisms**

#### **A. Safety Deposits**

- [ ] Study safety deposit requirements
- [ ] Understand economic security model
- [ ] Learn deposit calculation logic
- [ ] Study refund mechanisms

#### **B. Hashlock & Secret Management**

- [ ] Understand hashlock computation
- [ ] Learn secret revelation timing
- [ ] Study secret validation
- [ ] Understand replay protection

#### **C. Whitelisting & Access Control**

- [ ] Study resolver whitelisting
- [ ] Learn access control patterns
- [ ] Understand permission management
- [ ] Study upgrade mechanisms

## 📋 **Implementation Study Plan**

### **Week 1: Core Understanding**

#### **Day 1-2: Contract Interfaces**

- [ ] Read `IEscrowFactory.sol` and `IBaseEscrow.sol`
- [ ] Study factory pattern implementation
- [ ] Understand deterministic address computation
- [ ] Learn safety deposit mechanisms

#### **Day 3-4: Order Protocol**

- [ ] Study EIP-712 order structure
- [ ] Learn Limit Order Protocol integration
- [ ] Understand postInteraction callback
- [ ] Study extension data encoding

#### **Day 5-7: Data Structures**

- [ ] Study `Immutables` structure
- [ ] Learn `Timelocks` configuration
- [ ] Understand `Address` cross-chain format
- [ ] Study event emission patterns

### **Week 2: Integration Planning**

#### **Day 1-3: ICP Component Design**

- [ ] Design ICP escrow canisters (mirror Ethereum contracts)
- [ ] Plan orderbook canister structure
- [ ] Design cross-chain verification system
- [ ] Plan frontend integration

#### **Day 4-5: Testing Strategy**

- [ ] Plan integration testing with Base Sepolia
- [ ] Design cross-chain verification tests
- [ ] Plan error handling and recovery
- [ ] Design monitoring and observability

#### **Day 6-7: Implementation Roadmap**

- [ ] Create detailed implementation plan
- [ ] Define ICP-specific interfaces
- [ ] Plan development phases
- [ ] Set up development environment

## 🎯 **Key Questions to Answer**

### **1. Contract Integration**

- [ ] How do we call `createDstEscrow` from ICP?
- [ ] How do we monitor Base Sepolia events from ICP?
- [ ] How do we handle cross-chain address validation?
- [ ] How do we manage safety deposits across chains?

### **2. Order Management**

- [ ] How do we store orders in ICP orderbook?
- [ ] How do we validate EIP-712 signatures?
- [ ] How do we handle order state transitions?
- [ ] How do we coordinate cross-chain order execution?

### **3. Security & Validation**

- [ ] How do we validate cross-chain state?
- [ ] How do we handle secret revelation?
- [ ] How do we manage timelocks across chains?
- [ ] How do we handle error recovery?

### **4. User Experience**

- [ ] How do we provide gasless experience?
- [ ] How do we handle MetaMask integration?
- [ ] How do we manage cross-chain identity?
- [ ] How do we provide clear status updates?

## 🚀 **Study Resources**

### **Primary Sources**

- [ ] `contracts/interfaces/IEscrowFactory.sol`
- [ ] `contracts/interfaces/IBaseEscrow.sol`
- [ ] `contracts/EscrowFactory.sol`
- [ ] `docs/eip712-order-execution.md`
- [ ] `docs/order-data-structure.md`

### **Secondary Sources**

- [ ] Base Sepolia deployment: `0xE53136D9De56672e8D2665C98653AC7b8A60Dc44`
- [ ] Limit Order Protocol documentation
- [ ] EIP-712 specification
- [ ] Foundry testing examples

### **Testing & Validation**

- [ ] Set up Foundry environment
- [ ] Deploy test contracts to Base Sepolia
- [ ] Test order creation and execution
- [ ] Validate cross-chain coordination

## 📊 **Success Metrics**

### **Understanding Level**

- [ ] Can explain factory pattern and deterministic addresses
- [ ] Can trace complete order execution flow
- [ ] Can design ICP components that mirror Ethereum contracts
- [ ] Can implement cross-chain verification

### **Implementation Readiness**

- [ ] Have clear interface specifications for ICP components
- [ ] Have testing strategy for cross-chain integration
- [ ] Have error handling and recovery plans
- [ ] Have monitoring and observability design

### **Documentation Quality**

- [ ] Have detailed integration guides
- [ ] Have clear development roadmaps
- [ ] Have comprehensive testing plans
- [ ] Have deployment and maintenance guides

---

> **Note**: This study plan ensures we understand the existing 1inch system thoroughly before building our ICP integration. The goal is to create compatible components that work seamlessly with the existing infrastructure.
