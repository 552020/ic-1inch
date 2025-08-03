# Resolver-LOP Order Completion Integration

## Overview

This document provides a comprehensive analysis of how **resolvers interact with the Limit Order Protocol (LOP)** to complete cross-chain swap orders. It builds upon the existing `fusion_resolver_integration.md` and examines the actual EVM escrow factory implementation to provide a complete picture of the order completion flow.

## Core Architecture

### **1. Three-Way Integration Pattern**

The cross-chain swap system involves three key components working together:

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Resolver      │    │ Limit Order      │    │ EscrowFactory   │
│   (Taker)       │◄──►│ Protocol (LOP)   │◄──►│ (Post-Interaction)│
│                 │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### **2. Order Completion Flow**

#### **Phase 1: Order Creation & Signing**

```typescript
// 1. Maker creates and signs order with EIP-712
const order = FusionOrder.new(extension, orderParams, {
  whitelist: [resolverAddress], // Resolver must be whitelisted
  auction: auctionDetails,
  resolvingStartTime: 0n,
});

// 2. Maker signs with EIP-712
const signature = maker.signFusionOrder(order);
```

#### **Phase 2: Resolver Execution**

```solidity
// 3. Resolver calls LOP to execute the order
function settleOrders(bytes calldata calldata) external {
    // LOP validates maker's signature
    // LOP transfers maker's tokens to computed escrow address
    // LOP calls postInteraction on EscrowFactory
    // LOP calls takerInteraction on Resolver
}
```

#### **Phase 3: Escrow Creation**

```solidity
// 4. EscrowFactory.postInteraction() creates source escrow
function _postInteraction(
    IOrderMixin.Order calldata order,
    bytes calldata extension,
    bytes32 orderHash,
    address taker,
    uint256 makingAmount,
    uint256 takingAmount,
    uint256 remainingMakingAmount,
    bytes calldata extraData
) internal override {
    // Create deterministic escrow with maker's tokens
    // Emit SrcEscrowCreated event
}
```

#### **Phase 4: Resolver Callback**

```solidity
// 5. LOP calls resolver.takerInteraction()
function takerInteraction(
    IOrderMixin.Order calldata order,
    bytes calldata extension,
    bytes32 orderHash,
    address taker,
    uint256 makingAmount,
    uint256 takingAmount,
    uint256 remainingMakingAmount,
    bytes calldata extraData
) external {
    // Resolver creates destination escrow
    // Cross-chain swap begins
}
```

## Detailed Integration Points

### **1. Resolver → LOP Integration**

#### **A. Whitelisting Requirement**

```solidity
// Resolver must be whitelisted in LOP system
struct Whitelist {
    address resolver;
    uint256 allowFrom;
}

// Order includes whitelist
FusionOrder.new(extension, orderParams, {
    whitelist: [{ address: resolverAddress, allowFrom: 0n }],
    // ... other params
});
```

#### **B. Order Execution Call**

```solidity
// Resolver calls LOP to execute signed order
function settleOrders(bytes calldata calldata) external {
    // LOP validates:
    // 1. Maker's EIP-712 signature
    // 2. Resolver is whitelisted
    // 3. Order parameters are valid

    // LOP executes:
    // 1. Transfer maker's tokens to escrow
    // 2. Call postInteraction on EscrowFactory
    // 3. Call takerInteraction on Resolver
}
```

### **2. LOP → EscrowFactory Integration**

#### **A. Post-Interaction Hook**

```solidity
// LOP calls this after transferring maker's tokens
function _postInteraction(
    IOrderMixin.Order calldata order,
    bytes calldata extension,
    bytes32 orderHash,
    address taker,
    uint256 makingAmount,
    uint256 takingAmount,
    uint256 remainingMakingAmount,
    bytes calldata extraData
) internal override {
    // 1. Decode cross-chain parameters from extraData
    ExtraDataArgs calldata extraDataArgs = abi.decode(extraData, (ExtraDataArgs));

    // 2. Create immutable escrow parameters
    IBaseEscrow.Immutables memory immutables = IBaseEscrow.Immutables({
        orderHash: orderHash,
        hashlock: extraDataArgs.hashlockInfo,
        maker: order.maker,
        taker: Address.wrap(uint160(taker)),
        token: order.makerAsset,
        amount: makingAmount,
        safetyDeposit: extraDataArgs.deposits >> 128,
        timelocks: extraDataArgs.timelocks.setDeployedAt(block.timestamp)
    });

    // 3. Deploy deterministic source escrow
    bytes32 salt = immutables.hashMem();
    address escrow = _deployEscrow(salt, 0, ESCROW_SRC_IMPLEMENTATION);

    // 4. Emit event for off-chain tracking
    emit SrcEscrowCreated(immutables, dstImmutablesComplement);
}
```

#### **B. Deterministic Address Computation**

```solidity
// Escrow addresses are computed deterministically
function addressOfEscrowSrc(IBaseEscrow.Immutables calldata immutables)
    external view returns (address) {
    return Create2.computeAddress(immutables.hash(), _PROXY_SRC_BYTECODE_HASH);
}

function addressOfEscrowDst(IBaseEscrow.Immutables calldata immutables)
    external view returns (address) {
    return Create2.computeAddress(immutables.hash(), _PROXY_DST_BYTECODE_HASH);
}
```

### **3. LOP → Resolver Integration**

#### **A. Taker Interaction Callback**

```solidity
// LOP calls this after creating source escrow
function takerInteraction(
    IOrderMixin.Order calldata order,
    bytes calldata extension,
    bytes32 orderHash,
    address taker,
    uint256 makingAmount,
    uint256 takingAmount,
    uint256 remainingMakingAmount,
    bytes calldata extraData
) external {
    // Validate LOP call
    if (msg.sender != address(_LOP)) revert OnlyLOP();

    // Decode cross-chain parameters
    (IBaseEscrow.Immutables memory immutables, uint256 srcCancellationTimestamp) =
        abi.decode(extraData, (IBaseEscrow.Immutables, uint256));

    // Create destination escrow
    _ESCROW_FACTORY.createDstEscrow{value: msg.value}(
        immutables,
        srcCancellationTimestamp
    );
}
```

#### **B. Resolver → EscrowFactory Call**

```solidity
// Resolver calls EscrowFactory to create destination escrow
function createDstEscrow(
    IBaseEscrow.Immutables calldata dstImmutables,
    uint256 srcCancellationTimestamp
) external payable {
    // 1. Validate payment matches safety deposit
    if (msg.value != nativeAmount) revert InsufficientEscrowBalance();

    // 2. Create destination escrow
    IBaseEscrow.Immutables memory immutables = dstImmutables;
    immutables.timelocks = immutables.timelocks.setDeployedAt(block.timestamp);

    // 3. Validate timelock coordination
    if (immutables.timelocks.get(TimelocksLib.Stage.DstCancellation) > srcCancellationTimestamp)
        revert InvalidCreationTime();

    // 4. Deploy destination escrow
    bytes32 salt = immutables.hashMem();
    address escrow = _deployEscrow(salt, msg.value, ESCROW_DST_IMPLEMENTATION);

    // 5. Transfer destination tokens to escrow
    if (token != address(0)) {
        IERC20(token).safeTransferFrom(msg.sender, escrow, immutables.amount);
    }

    // 6. Emit event
    emit DstEscrowCreated(escrow, dstImmutables.hashlock, dstImmutables.taker);
}
```

## Cross-Chain Flow Patterns

### **1. SRC → DST Flow (Source to Destination)**

```
1. Maker creates order on source chain
   ↓
2. Resolver calls LOP.settleOrders()
   ↓
3. LOP validates signature & transfers maker's tokens
   ↓
4. LOP calls EscrowFactory._postInteraction()
   ↓
5. EscrowFactory creates source escrow with maker's tokens
   ↓
6. LOP calls Resolver.takerInteraction()
   ↓
7. Resolver calls EscrowFactory.createDstEscrow()
   ↓
8. EscrowFactory creates destination escrow with resolver's tokens
   ↓
9. Cross-chain swap begins (both escrows locked)
```

### **2. DST → SRC Flow (Destination to Source)**

```
1. Maker creates order on destination chain
   ↓
2. Resolver calls LOP.settleOrders()
   ↓
3. LOP validates signature & transfers maker's tokens
   ↓
4. LOP calls EscrowFactory._postInteraction()
   ↓
5. EscrowFactory creates destination escrow with maker's tokens
   ↓
6. LOP calls Resolver.takerInteraction()
   ↓
7. Resolver calls EscrowFactory.addressOfEscrowSrc()
   ↓
8. Resolver computes existing source escrow address
   ↓
9. Cross-chain swap begins (destination escrow + existing source escrow)
```

## Key Technical Details

### **1. EIP-712 Signature Validation**

```typescript
// Maker signs order with EIP-712
const signedOrder = maker.signFusionOrder(order);

// LOP validates signature
const calldata = LimitOrderContract.getFillOrderArgsCalldata(
  order.build(),
  signedOrder, // EIP-712 signature
  takerTraits,
  amount
);
```

### **2. Deterministic Escrow Addresses**

```solidity
// Escrow addresses computed from immutable parameters
bytes32 salt = immutables.hashMem();
address escrow = Create2.computeAddress(salt, bytecodeHash);

// Same parameters = same address across chains
```

### **3. Cross-Chain Parameter Encoding**

```solidity
// ExtraDataArgs contains all cross-chain parameters
struct ExtraDataArgs {
    bytes32 hashlockInfo;     // Secret hash or Merkle root
    uint256 dstChainId;       // Destination chain ID
    Address dstToken;         // Destination token address
    uint256 deposits;         // Safety deposits (packed)
    Timelocks timelocks;      // Timelock configuration
}
```

### **4. Timelock Coordination**

```solidity
// Destination escrow timelock must be <= source cancellation time
if (immutables.timelocks.get(TimelocksLib.Stage.DstCancellation) > srcCancellationTimestamp)
    revert InvalidCreationTime();
```

## Security Considerations

### **1. Whitelisting Protection**

- Resolvers must be explicitly whitelisted in LOP
- Prevents unauthorized resolvers from executing orders
- Whitelist managed by LOP governance

### **2. Signature Validation**

- All orders must be signed with EIP-712
- LOP validates maker's signature before execution
- Prevents order tampering and replay attacks

### **3. Deterministic Addresses**

- Escrow addresses computed deterministically
- Same parameters produce same address across chains
- Prevents address manipulation attacks

### **4. Timelock Coordination**

- Destination escrow timelock ≤ source cancellation time
- Ensures proper cross-chain timing coordination
- Prevents timing-based attacks

## Implementation Requirements

### **✅ What We Have:**

1. **EscrowFactory Contract** - ✅ Complete implementation
2. **BaseEscrowFactory** - ✅ Abstract base with postInteraction
3. **Cross-chain parameter encoding** - ✅ ExtraDataArgs structure
4. **Deterministic address computation** - ✅ Create2 pattern
5. **Timelock coordination** - ✅ Validation logic

### **🔧 What We Need:**

1. **Cross-chain resolver** implementing `ITakerInteraction`
2. **Fusion SDK integration** for order creation
3. **Whitelisting** in LOP system
4. **Testing** with real LOP integration
5. **Production deployment** and monitoring

## Testing Strategy

### **1. Unit Testing**

```bash
# Test resolver with mock LOP
forge test --match-test testCrossChainResolver
```

### **2. Integration Testing**

```bash
# Test with real LOP on mainnet fork
NODE_URL=https://mainnet-fork-url yarn test
```

### **3. End-to-End Testing**

```bash
# Test complete cross-chain flow
yarn test:integration
```

## Conclusion

The resolver-LOP integration provides a **robust, secure foundation** for cross-chain atomic swaps. The three-way integration pattern (Resolver ↔ LOP ↔ EscrowFactory) ensures:

- ✅ **Secure order execution** via EIP-712 signatures
- ✅ **Deterministic escrow creation** via Create2
- ✅ **Proper cross-chain coordination** via timelocks
- ✅ **Flexible resolver architecture** via ITakerInteraction interface

This architecture aligns perfectly with 1inch's proven Fusion resolver pattern and provides a production-ready foundation for cross-chain swap protocols.

## References

- [Fusion Resolver Integration Pattern](fusion_resolver_integration.md)
- [EIP-712 Order Execution](cross-turk/eip712-order-execution.md)
- [Limit Order Protocol Documentation](eth/lib/limit-order-protocol/)
- [EscrowFactory Implementation](eth/contracts/EscrowFactory.sol)
