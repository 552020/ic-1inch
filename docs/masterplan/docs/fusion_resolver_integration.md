# Fusion Resolver Integration Pattern

## TL;DR Overview

### **Resolver Interactions:**

**With LOP (Limit Order Protocol):**

- **1 interaction:** `takerInteraction()` - Called by LOP when order is filled
- **1 setup:** `settleOrders()` - Resolver calls LOP to execute orders

**With EscrowFactory:**

- **2 interactions:**
  1. `addressOfEscrowSrc()` - View function to compute escrow address
  2. `createDstEscrow()` - Deploy destination escrow contract

### **Flow Summary:**

**SRC → DST (Source to Destination):**

1. **User creates order** → LOP stores order
2. **Resolver calls LOP** → `settleOrders()` to execute order
3. **LOP calls resolver** → `takerInteraction()` callback when filled
4. **Resolver executes** → Calls EscrowFactory `createDstEscrow()` to deploy escrow
5. **Cross-chain swap begins** → Escrow handles the rest

**DST → SRC (Destination to Source):**

1. **User creates order** → LOP stores order
2. **Resolver calls LOP** → `settleOrders()` to execute order
3. **LOP calls resolver** → `takerInteraction()` callback when filled
4. **Resolver executes** → Calls EscrowFactory `addressOfEscrowSrc()` to compute address
5. **Cross-chain swap begins** → Escrow handles the rest

**Key Difference:** SRC→DST deploys new escrow, DST→SRC computes existing escrow address

### **EIP-712 Role in Cross-Chain Flow:**

**EIP-712 is explicitly used** in the Fusion resolver pattern for order signing and validation:

**Found in:** `../fusion-resolver-example/test/helpers/user.ts:35`

```typescript
signFusionOrder(order: FusionOrder): string {
    return signTypedData({
        privateKey: this.privateKey,
        data: order.getTypedData(NetworkEnum.ETHEREUM), // ← EIP-712 typed data
        version: SignTypedDataVersion.V4 // ← EIP-712 v4
    })
}
```

**Found in:** `../fusion-resolver-example/test/Settlement.ts:148`

```typescript
// User signs order with EIP-712
const signedOrder = userA.signFusionOrder(orderA);

// LOP validates the EIP-712 signature
const calldata = LimitOrderContract.getFillOrderArgsCalldata(
  orderA.build(),
  signedOrder, // ← EIP-712 signature here
  takerTraits,
  amount
);
```

**EIP-712 ensures** that cross-chain parameters (escrow details, timelocks, etc.) are **cryptographically signed** and **cannot be tampered with** during the cross-chain process.

### **Key Requirements:**

- Resolver must be **whitelisted** in LOP
- Resolver implements `ITakerInteraction` interface
- Resolver needs **approval** to spend user's tokens

---

## Overview

This document analyzes the **official 1inch Fusion resolver example** and explains how it applies to our cross-chain swap protocol integration with the Limit Order Protocol (LOP).

## Key Discovery

The `fusion-resolver-example` repository provides the **official pattern** for integrating external protocols with 1inch's Limit Order Protocol. This is exactly what we need for our cross-chain swap integration.

## Core Architecture

### **1. Resolver Contract Pattern**

```solidity
contract ResolverExample is ITakerInteraction {
    IOrderMixin private immutable _LOPV4;
    address private immutable _OWNER;

    function takerInteraction(
        IOrderMixin.Order calldata order,
        bytes calldata extension,
        bytes32 orderHash,
        address taker,
        uint256 makingAmount,
        uint256 takingAmount,
        uint256 remainingMakingAmount,
        bytes calldata extraData
    ) public {
        if (msg.sender != address(_LOPV4)) revert OnlyLOP();
        if (taker != address(this)) revert NotTaker();

        (Address[] memory targets, bytes[] memory calldatas) = abi.decode(extraData, (Address[], bytes[]));

        // Execute arbitrary contract calls
        for (uint256 i = 0; i < targets.length; ++i) {
            (bool success, bytes memory reason) = targets[i].get().call(calldatas[i]);
            if (!success) revert FailedExternalCall(i, reason);
        }
    }
}
```

**Key Insights:**

- Resolvers implement `ITakerInteraction` interface
- LOP calls `takerInteraction()` when orders are filled
- `extraData` contains target contracts and calldata to execute
- Resolvers can execute **arbitrary contract calls**

### **2. Whitelisting Requirement**

```typescript
// Resolver must be whitelisted
const whitelist = [
  {
    address: new Address(resolverAddress),
    allowFrom: 0n,
  },
];

// Order includes whitelist
const order = FusionOrder.new(extension, orderParams, {
  whitelist, // ← Resolver is whitelisted here
  auction: auctionDetails,
  resolvingStartTime: 0n,
});
```

**Critical:** Resolvers must be **whitelisted** in the LOP system to execute orders.

### **3. Interaction Setup Pattern**

```typescript
// Define what the resolver should execute
const targets = [TOKENS.WETH, swapContract];
const callDataList = [encodeInfinityApprove(swapContract), swapCalldata];

const resolverExecutionBytes = AbiCoder.defaultAbiCoder().encode(["address[]", "bytes[]"], [targets, callDataList]);

// Attach to order
const takerTraits = TakerTraits.default()
  .setExtension(order.extension)
  .setInteraction(
    new Interaction(
      new Address(resolverAddress),
      resolverExecutionBytes // ← Passed to takerInteraction()
    )
  );
```

## Integration Flow

### **Complete Order Execution Flow:**

1. **Setup Phase:**

   ```typescript
   // Deploy resolver
   const resolver = await ResolverExample.deploy(LOP_ADDRESS);

   // Whitelist resolver
   const whitelist = [{ address: resolver.address, allowFrom: 0n }];

   // Approve tokens
   await resolver.approve(token, LOP_ADDRESS);
   ```

2. **Order Creation:**

   ```typescript
   // Create Fusion order with resolver interaction
   const order = FusionOrder.new(extension, orderParams, { whitelist, auction, resolvingStartTime });

   // Define resolver actions
   const targets = [ESCROW_FACTORY];
   const callDataList = [createDstEscrowCalldata];

   const interaction = new Interaction(resolver.address, encode(["address[]", "bytes[]"], [targets, callDataList]));
   ```

3. **Order Execution:**

   ```typescript
   // Execute through resolver
   const calldata = LimitOrderContract.getFillOrderArgsCalldata(order.build(), signature, takerTraits, amount);

   await resolver.settleOrders(calldata);
   ```

4. **Callback Execution:**
   ```solidity
   // LOP calls resolver.takerInteraction()
   // Resolver executes our cross-chain logic
   // EscrowFactory.createDstEscrow() is called
   // Cross-chain swap begins!
   ```

## Application to Cross-Chain Swap

### **Our Integration Pattern:**

1. **Cross-Chain Resolver:**

   ```solidity
   contract CrossChainResolver is ITakerInteraction {
       IEscrowFactory private immutable _ESCROW_FACTORY;
       IOrderMixin private immutable _LOP;

       function takerInteraction(
           IOrderMixin.Order calldata order,
           bytes calldata extension,
           bytes32 orderHash,
           address taker,
           uint256 makingAmount,
           uint256 takingAmount,
           uint256 remainingMakingAmount,
           bytes calldata extraData
       ) public {
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
   }
   ```

2. **Order Creation with Cross-Chain Interaction:**

   ```typescript
   // Create order for cross-chain swap
   const order = FusionOrder.new(
     new Address(CROSS_CHAIN_EXTENSION),
     {
       makerAsset: WETH,
       takerAsset: USDC,
       makingAmount: "1",
       takingAmount: "1000",
       maker: userAddress,
     },
     {
       whitelist: [crossChainResolver],
       auction: auctionDetails,
       resolvingStartTime: 0n,
     }
   );

   // Define cross-chain interaction
   const targets = [ESCROW_FACTORY_ADDRESS];
   const callDataList = [escrowFactory.interface.encodeFunctionData("createDstEscrow", [immutables, srcCancellationTimestamp])];

   const crossChainInteraction = new Interaction(crossChainResolver.address, encode(["address[]", "bytes[]"], [targets, callDataList]));
   ```

## Key Components

### **1. Extension Contract**

- Handles protocol-specific logic
- Validates cross-chain parameters
- Manages whitelisting

### **2. Resolver Contract**

- Implements `ITakerInteraction`
- Executes cross-chain logic
- Must be whitelisted

### **3. Fusion SDK Integration**

- Creates Fusion orders
- Manages interactions
- Handles order execution

### **4. EscrowFactory Integration**

- Receives calls from resolver
- Deploys escrow contracts
- Manages cross-chain state

## Implementation Requirements

### **✅ What We Have:**

- **EscrowFactory** - ✅ Working and tested
- **Cross-chain logic** - ✅ Implemented
- **Contract architecture** - ✅ Solid

### **🔧 What We Need:**

1. **Cross-chain resolver** implementing `ITakerInteraction`
2. **Extension contract** for cross-chain validation
3. **Whitelisting** in LOP system
4. **Fusion SDK integration** for order creation
5. **Testing** with real LOP integration

## Testing Strategy

### **1. Local Testing:**

```bash
# Test resolver with mock LOP
forge test --match-test testCrossChainResolver
```

### **2. Mainnet Fork Testing:**

```bash
# Test with real LOP on mainnet fork
NODE_URL=https://mainnet-fork-url yarn test
```

### **3. Integration Testing:**

```bash
# Test complete cross-chain flow
yarn test:integration
```

## Next Steps

### **Phase 1: Resolver Development**

1. Create `CrossChainResolver` contract
2. Implement `ITakerInteraction` interface
3. Add cross-chain parameter validation
4. Test with mock LOP

### **Phase 2: Extension Contract**

1. Create `CrossChainExtension` contract
2. Implement protocol-specific logic
3. Add whitelisting management
4. Test extension functionality

### **Phase 3: Fusion Integration**

1. Integrate with Fusion SDK
2. Create order creation utilities
3. Test with real LOP on mainnet fork
4. Validate complete flow

### **Phase 4: Production Deployment**

1. Deploy to mainnet
2. Get whitelisted in LOP
3. Launch cross-chain swap protocol
4. Monitor and optimize

## Conclusion

The Fusion resolver pattern provides the **official, proven way** to integrate with 1inch's Limit Order Protocol. Our cross-chain swap architecture aligns perfectly with this pattern.

**Key Benefits:**

- ✅ **Official integration pattern**
- ✅ **Proven architecture**
- ✅ **Flexible interaction system**
- ✅ **Production-ready approach**

**Next Action:** Implement our cross-chain resolver following this pattern! 🚀

## References

- [Fusion Resolver Example](https://github.com/1inch/fusion-resolver-example)
- [Fusion SDK Documentation](https://github.com/1inch/fusion-sdk)
- [Limit Order Protocol](https://github.com/1inch/limit-order-protocol)
- [Cross-Chain Swap Protocol](../README.md)
