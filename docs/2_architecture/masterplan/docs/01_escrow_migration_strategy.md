# Escrow Migration Strategy: ICP Orderbook as Coordination Hub

## Executive Summary

This document outlines a **migration-based approach** for escrow creation where both source and destination escrows are initially created in the ICP orderbook canister, then "migrated" to their respective chains (EVM and ICP) when orders are filled. This approach solves many of the cross-chain coordination challenges identified in the THCL analysis.

## 1. Core Concept: ICP Orderbook as Coordination Hub

### 1.1 The Migration Strategy

Instead of creating escrows directly on their target chains, we create **virtual escrows** in the ICP orderbook first, then migrate them when needed:

```
Order Creation Flow:
1. User creates order → Stored in ICP orderbook
2. Virtual escrows created in ICP orderbook
3. Order remains in "virtual" state
4. When resolver fills order → Migrate escrows to actual chains
```

### 1.2 Benefits of This Approach

- **Atomic Coordination**: Both escrows created simultaneously in ICP
- **No Race Conditions**: No timing issues between chains
- **Simplified State Management**: Single source of truth in ICP
- **Rollback Capability**: Can cancel virtual escrows easily
- **Gas Optimization**: Only create real escrows when needed

## 2. Detailed Implementation

### 2.1 Virtual Escrow Structure

```typescript
// Virtual escrow representation in ICP orderbook
interface VirtualEscrow {
  id: string;
  orderId: string;
  escrowType: "SOURCE" | "DESTINATION";
  targetChain: "ICP" | "ETH" | "POLYGON" | "ARBITRUM" | "BASE";
  targetAddress?: string; // Will be set during migration
  status: "VIRTUAL" | "MIGRATING" | "DEPLOYED" | "FAILED";

  // Escrow parameters
  maker: string;
  taker: string;
  token: string;
  amount: bigint;
  hashlock: string;
  timelocks: TimelockConfig;
  safetyDeposit: bigint;

  // Migration tracking
  migrationTxHash?: string;
  deployedAt?: number;
  error?: string;
}

// Order with virtual escrows
interface OrderWithVirtualEscrows {
  id: string;
  orderType: "EVM_TO_ICP" | "ICP_TO_EVM";
  sourceChain: string;
  destinationChain: string;
  sourceToken: string;
  destinationToken: string;
  sourceAmount: bigint;
  destinationAmount: bigint;
  hashlock: string;
  timelocks: TimelockConfig;
  status: "PENDING" | "FILLED" | "CANCELLED" | "EXPIRED";

  // Virtual escrows
  sourceEscrow: VirtualEscrow;
  destinationEscrow: VirtualEscrow;

  // Order metadata
  createdAt: number;
  evmSignature?: string;
  resolver?: string;
  fillTxHash?: string;
}
```

### 2.2 ICP Orderbook Canister Implementation

```typescript
// ICP Orderbook Canister (Motoko/Rust)
class OrderbookCanister {
  private orders: Map<string, OrderWithVirtualEscrows> = new Map();
  private virtualEscrows: Map<string, VirtualEscrow> = new Map();

  // Create order with virtual escrows
  async createOrderWithVirtualEscrows(order: Order): Promise<string> {
    const orderId = generateOrderId();

    // Create virtual source escrow
    const sourceEscrow: VirtualEscrow = {
      id: generateEscrowId(),
      orderId: orderId,
      escrowType: "SOURCE",
      targetChain: order.sourceChain,
      status: "VIRTUAL",
      maker: order.maker,
      taker: order.taker,
      token: order.sourceToken,
      amount: order.sourceAmount,
      hashlock: order.hashlock,
      timelocks: order.timelocks,
      safetyDeposit: calculateSafetyDeposit(order.sourceChain),
    };

    // Create virtual destination escrow
    const destinationEscrow: VirtualEscrow = {
      id: generateEscrowId(),
      orderId: orderId,
      escrowType: "DESTINATION",
      targetChain: order.destinationChain,
      status: "VIRTUAL",
      maker: order.maker,
      taker: order.taker,
      token: order.destinationToken,
      amount: order.destinationAmount,
      hashlock: order.hashlock,
      timelocks: order.timelocks,
      safetyDeposit: calculateSafetyDeposit(order.destinationChain),
    };

    // Store order with virtual escrows
    const orderWithEscrows: OrderWithVirtualEscrows = {
      ...order,
      id: orderId,
      sourceEscrow: sourceEscrow,
      destinationEscrow: destinationEscrow,
      status: "PENDING",
      createdAt: Date.now(),
    };

    this.orders.set(orderId, orderWithEscrows);
    this.virtualEscrows.set(sourceEscrow.id, sourceEscrow);
    this.virtualEscrows.set(destinationEscrow.id, destinationEscrow);

    return orderId;
  }

  // Migrate virtual escrow to actual chain
  async migrateEscrow(escrowId: string, targetAddress: string): Promise<boolean> {
    const virtualEscrow = this.virtualEscrows.get(escrowId);
    if (!virtualEscrow || virtualEscrow.status !== "VIRTUAL") {
      return false;
    }

    try {
      // Update status to migrating
      virtualEscrow.status = "MIGRATING";
      virtualEscrow.targetAddress = targetAddress;

      // Perform chain-specific migration
      if (virtualEscrow.targetChain === "ICP") {
        await this.migrateToICPEscrow(virtualEscrow);
      } else {
        await this.migrateToEVMEscrow(virtualEscrow);
      }

      virtualEscrow.status = "DEPLOYED";
      virtualEscrow.deployedAt = Date.now();

      return true;
    } catch (error) {
      virtualEscrow.status = "FAILED";
      virtualEscrow.error = error.message;
      return false;
    }
  }

  // Get order with virtual escrows
  async getOrderWithEscrows(orderId: string): Promise<OrderWithVirtualEscrows | null> {
    return this.orders.get(orderId) || null;
  }

  // Get pending orders for resolvers
  async getPendingOrders(): Promise<OrderWithVirtualEscrows[]> {
    return Array.from(this.orders.values()).filter((order) => order.status === "PENDING");
  }
}
```

### 2.3 Migration Process

#### Step 1: Virtual Escrow Creation

```typescript
// When user creates order
async function createOrderWithVirtualEscrows(order: Order): Promise<string> {
  // 1. Validate order parameters
  await validateOrder(order);

  // 2. Create virtual escrows in ICP orderbook
  const orderId = await orderbookCanister.createOrderWithVirtualEscrows(order);

  // 3. For ICP → EVM orders, lock assets in virtual source escrow
  if (order.orderType === "ICP_TO_EVM") {
    await lockAssetsInVirtualEscrow(orderId, "SOURCE");
  }

  return orderId;
}
```

#### Step 2: Resolver Fills Order

```typescript
// When resolver decides to fill order
async function fillOrder(orderId: string, resolver: string): Promise<boolean> {
  const order = await orderbookCanister.getOrderWithEscrows(orderId);
  if (!order || order.status !== "PENDING") {
    return false;
  }

  try {
    // 1. Update order status
    order.status = "FILLED";
    order.resolver = resolver;

    // 2. Migrate source escrow
    const sourceMigrationSuccess = await migrateEscrow(order.sourceEscrow.id, order.sourceEscrow.targetChain);

    // 3. Migrate destination escrow
    const destMigrationSuccess = await migrateEscrow(order.destinationEscrow.id, order.destinationEscrow.targetChain);

    // 4. Verify both migrations succeeded
    if (sourceMigrationSuccess && destMigrationSuccess) {
      order.fillTxHash = generateFillTxHash();
      return true;
    } else {
      // Rollback if either migration failed
      await rollbackOrder(orderId);
      return false;
    }
  } catch (error) {
    await rollbackOrder(orderId);
    return false;
  }
}
```

#### Step 3: Chain-Specific Migration

##### ICP Escrow Migration

```typescript
// Migrate virtual escrow to actual ICP escrow canister
async function migrateToICPEscrow(virtualEscrow: VirtualEscrow): Promise<void> {
  // 1. Create actual ICP escrow canister
  const icpEscrowCanister = await createICPEscrowCanister({
    maker: virtualEscrow.maker,
    taker: virtualEscrow.taker,
    token: virtualEscrow.token,
    amount: virtualEscrow.amount,
    hashlock: virtualEscrow.hashlock,
    timelocks: virtualEscrow.timelocks,
  });

  // 2. Transfer assets from virtual escrow to actual escrow
  await transferAssetsToICPEscrow(virtualEscrow.id, icpEscrowCanister.address);

  // 3. Update virtual escrow with actual address
  virtualEscrow.targetAddress = icpEscrowCanister.address;
}
```

##### EVM Escrow Migration

```typescript
// Migrate virtual escrow to actual EVM escrow contract
async function migrateToEVMEscrow(virtualEscrow: VirtualEscrow): Promise<void> {
  // 1. Prepare EVM escrow parameters
  const immutables = {
    orderHash: virtualEscrow.orderId,
    hashlock: virtualEscrow.hashlock,
    maker: virtualEscrow.maker,
    taker: virtualEscrow.taker,
    token: virtualEscrow.token,
    amount: virtualEscrow.amount,
    safetyDeposit: virtualEscrow.safetyDeposit,
    timelocks: virtualEscrow.timelocks,
  };

  // 2. Calculate escrow address
  const escrowAddress = await escrowFactory.addressOfEscrowDst(immutables);

  // 3. Create actual EVM escrow
  const tx = await escrowFactory.createDstEscrow(immutables, virtualEscrow.timelocks.srcCancellation, { value: virtualEscrow.safetyDeposit });

  // 4. Wait for confirmation
  await tx.wait();

  // 5. Update virtual escrow with actual address
  virtualEscrow.targetAddress = escrowAddress;
  virtualEscrow.migrationTxHash = tx.hash;
}
```

## 3. Advantages of Migration Strategy

### 3.1 **Solves Cross-Chain Coordination**

- **No Race Conditions**: Both escrows created atomically in ICP
- **Synchronized State**: Single source of truth in orderbook
- **Atomic Operations**: Either both escrows migrate or neither does
- **Rollback Capability**: Can cancel virtual escrows easily

### 3.2 **Simplifies Complex Scenarios**

#### ETH → ICP Orders

```
1. User creates order → Virtual escrows created in ICP
2. User locks ETH in virtual destination escrow
3. Resolver fills order → Migrate destination escrow to EVM
4. Resolver creates ICP escrow → Migrate source escrow to ICP
```

#### ICP → ETH Orders

```
1. User creates order → Virtual escrows created in ICP
2. User locks ICP tokens in virtual source escrow (reverse gas model)
3. Resolver fills order → Migrate source escrow to ICP canister
4. Resolver creates ETH escrow → Migrate destination escrow to EVM
```

### 3.3 **Gas Optimization**

- **Lazy Deployment**: Only create real escrows when needed
- **Batch Operations**: Can batch multiple escrow migrations
- **Failed Order Handling**: No gas wasted on unfilled orders

## 4. Implementation Details

### 4.1 **Virtual Escrow State Machine**

```typescript
enum VirtualEscrowStatus {
  VIRTUAL = "VIRTUAL", // Created in orderbook
  MIGRATING = "MIGRATING", // Being migrated to actual chain
  DEPLOYED = "DEPLOYED", // Successfully deployed on target chain
  FAILED = "FAILED", // Migration failed
  CANCELLED = "CANCELLED", // Cancelled before migration
}
```

### 4.2 **Migration Coordination**

```typescript
// Migration coordinator
class EscrowMigrationCoordinator {
  async migrateOrderEscrows(orderId: string): Promise<MigrationResult> {
    const order = await orderbookCanister.getOrderWithEscrows(orderId);

    // 1. Start both migrations concurrently
    const sourceMigration = this.migrateEscrow(order.sourceEscrow.id);
    const destMigration = this.migrateEscrow(order.destinationEscrow.id);

    // 2. Wait for both to complete
    const [sourceResult, destResult] = await Promise.allSettled([sourceMigration, destMigration]);

    // 3. Handle results
    if (sourceResult.status === "fulfilled" && destResult.status === "fulfilled") {
      return { success: true, sourceAddress: sourceResult.value, destAddress: destResult.value };
    } else {
      // Rollback successful migration if one failed
      await this.rollbackSuccessfulMigration(orderId, sourceResult, destResult);
      return { success: false, error: "Migration failed" };
    }
  }
}
```

### 4.3 **Asset Locking in Virtual Escrows**

```typescript
// For ICP → EVM orders, lock assets in virtual source escrow
async function lockAssetsInVirtualEscrow(orderId: string, escrowType: "SOURCE" | "DESTINATION"): Promise<void> {
  const order = await orderbookCanister.getOrderWithEscrows(orderId);
  const virtualEscrow = escrowType === "SOURCE" ? order.sourceEscrow : order.destinationEscrow;

  if (virtualEscrow.targetChain === "ICP") {
    // Lock ICP tokens in virtual escrow (reverse gas model)
    await transferICPToVirtualEscrow(virtualEscrow.id, virtualEscrow.amount);
  } else {
    // For EVM chains, user will lock assets when creating real escrow
    // Virtual escrow just tracks the requirement
  }
}
```

## 5. Error Handling and Rollback

### 5.1 **Migration Failure Handling**

```typescript
// Handle migration failures
async function handleMigrationFailure(orderId: string, failedEscrowId: string): Promise<void> {
  const order = await orderbookCanister.getOrderWithEscrows(orderId);

  // 1. Mark failed escrow
  const failedEscrow = order.sourceEscrow.id === failedEscrowId ? order.sourceEscrow : order.destinationEscrow;
  failedEscrow.status = "FAILED";

  // 2. Rollback successful migration if any
  const successfulEscrow = order.sourceEscrow.id === failedEscrowId ? order.destinationEscrow : order.sourceEscrow;

  if (successfulEscrow.status === "DEPLOYED") {
    await rollbackEscrowMigration(successfulEscrow);
  }

  // 3. Reset order status
  order.status = "PENDING";
  order.resolver = undefined;
  order.fillTxHash = undefined;
}
```

### 5.2 **Virtual Escrow Cancellation**

```typescript
// Cancel virtual escrows (for unfilled orders)
async function cancelVirtualEscrows(orderId: string): Promise<void> {
  const order = await orderbookCanister.getOrderWithEscrows(orderId);

  // 1. Return locked assets to users
  if (order.sourceEscrow.status === "VIRTUAL" && order.sourceEscrow.targetChain === "ICP") {
    await returnLockedAssets(order.sourceEscrow.id);
  }

  // 2. Mark escrows as cancelled
  order.sourceEscrow.status = "CANCELLED";
  order.destinationEscrow.status = "CANCELLED";

  // 3. Update order status
  order.status = "CANCELLED";
}
```

## 6. Testing Strategy

### 6.1 **Migration Testing**

```typescript
describe("Escrow Migration Testing", () => {
  it("should migrate both escrows successfully", async () => {
    // 1. Create order with virtual escrows
    const orderId = await createOrderWithVirtualEscrows(testOrder);

    // 2. Simulate resolver filling order
    const result = await fillOrder(orderId, testResolver);

    // 3. Verify both escrows migrated
    expect(result.success).toBe(true);
    expect(result.sourceAddress).toBeDefined();
    expect(result.destAddress).toBeDefined();
  });

  it("should rollback on migration failure", async () => {
    // Test partial migration failure scenarios
  });

  it("should handle virtual escrow cancellation", async () => {
    // Test cancellation of unfilled orders
  });
});
```

## 7. Conclusion

The **migration-based escrow creation strategy** provides an elegant solution to the cross-chain coordination challenges:

### **Key Benefits:**

1. **Atomic Coordination**: Both escrows created simultaneously in ICP
2. **Simplified State Management**: Single source of truth
3. **Rollback Capability**: Easy cancellation of virtual escrows
4. **Gas Optimization**: Only create real escrows when needed
5. **Error Handling**: Robust failure recovery mechanisms

### **Implementation Priority:**

1. **Design ICP orderbook canister** with virtual escrow support
2. **Implement migration coordination** logic
3. **Add chain-specific migration** handlers
4. **Create comprehensive testing** for migration scenarios
5. **Add rollback and error handling** mechanisms

This approach transforms the complex cross-chain escrow creation into a **two-phase process**: virtual creation in ICP, followed by migration to actual chains. This significantly reduces the complexity and risk of the cross-chain coordination problem.
