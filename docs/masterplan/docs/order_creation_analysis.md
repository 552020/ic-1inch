# Order Creation Flow Analysis: ICP <> EVM Cross-Chain Fusion

## Executive Summary

This document provides a detailed analysis of the order creation flow for the ICP <> EVM cross-chain fusion protocol, focusing on the two distinct order types and their technical implementations.

## 1. Order Creation Overview

### 1.1 Two Order Types

The protocol supports two main order types, each with different user flows and technical requirements:

#### Type A: EVM → ICP Orders

- **Direction**: User wants to swap EVM tokens for ICP tokens
- **Authentication**: EVM wallet (MetaMask, etc.)
- **Signature**: EIP-712 signature required
- **Escrow**: User creates destination escrow on EVM chain

#### Type B: ICP → EVM Orders

- **Direction**: User wants to swap ICP tokens for EVM tokens
- **Authentication**: ICP identity
- **Signature**: No signature required (trusted environment)
- **Escrow**: Automatic source escrow creation on ICP

## 2. Detailed Flow Analysis

### 2.1 EVM → ICP Order Flow

#### User Journey

```
1. User connects EVM wallet (MetaMask)
2. User selects: EVM token → ICP token
3. User enters amounts and confirms
4. User signs EIP-712 order with private key
5. Order stored in ICP orderbook canister
6. User creates destination escrow on EVM chain
7. Order becomes available for resolvers
```

#### Technical Implementation

```typescript
// 1. Order Creation
interface EVMToICPOrder {
  maker: string; // EVM address
  orderType: "EVM_TO_ICP";
  sourceChain: "ethereum" | "polygon" | "arbitrum" | "base";
  destinationChain: "icp";
  sourceToken: string; // EVM token address
  destinationToken: string; // ICP token ID
  sourceAmount: bigint;
  destinationAmount: bigint;
  hashlock: string;
  timelocks: TimelockConfig;
  evmSignature: string; // EIP-712 signature
}

// 2. EIP-712 Signature
const domain = {
  name: "ICP-EVM Fusion Protocol",
  version: "1.0",
  chainId: chainId,
  verifyingContract: orderbookCanisterId,
};

const types = {
  Order: [
    { name: "maker", type: "address" },
    { name: "orderType", type: "string" },
    { name: "sourceChain", type: "string" },
    { name: "destinationChain", type: "string" },
    { name: "sourceToken", type: "address" },
    { name: "destinationToken", type: "string" },
    { name: "sourceAmount", type: "uint256" },
    { name: "destinationAmount", type: "uint256" },
    { name: "hashlock", type: "bytes32" },
    { name: "timelocks", type: "TimelockConfig" },
  ],
};

// 3. Order Validation
function validateEVMOrder(order: EVMToICPOrder): boolean {
  const recoveredAddress = ethers.utils.verifyTypedData(domain, types, order, order.evmSignature);
  return recoveredAddress === order.maker;
}
```

### 2.2 ICP → EVM Order Flow

#### User Journey

```
1. User connects ICP identity
2. User selects: ICP token → EVM token
3. User enters amounts and confirms
4. Order stored in ICP orderbook canister
5. Source escrow automatically created on ICP
6. User assets locked in ICP escrow (reverse gas model)
7. Order becomes available for resolvers
```

#### Technical Implementation

```typescript
// 1. Order Creation
interface ICPToEVMOrder {
  maker: string; // ICP principal
  orderType: "ICP_TO_EVM";
  sourceChain: "icp";
  destinationChain: "ethereum" | "polygon" | "arbitrum" | "base";
  sourceToken: string; // ICP token ID
  destinationToken: string; // EVM token address
  sourceAmount: bigint;
  destinationAmount: bigint;
  hashlock: string;
  timelocks: TimelockConfig;
  // No signature required - trusted ICP environment
}

// 2. Automatic Source Escrow Creation
async function createICPSourceEscrow(order: ICPToEVMOrder): Promise<string> {
  // Create escrow canister on ICP
  const escrowCanister = await createEscrowCanister({
    user: order.maker,
    token: order.sourceToken,
    amount: order.sourceAmount,
    hashlock: order.hashlock,
    timelocks: order.timelocks,
  });

  // Lock user assets immediately
  await escrowCanister.lockAssets({
    user: order.maker,
    token: order.sourceToken,
    amount: order.sourceAmount,
    hashlock: order.hashlock,
  });

  return escrowCanister.address;
}
```

## 3. Orderbook Canister Design

### 3.1 Data Structure

```typescript
// Core Order Interface
interface Order {
  id: string;
  maker: string;
  orderType: "EVM_TO_ICP" | "ICP_TO_EVM";
  sourceChain: string;
  destinationChain: string;
  sourceToken: string;
  destinationToken: string;
  sourceAmount: bigint;
  destinationAmount: bigint;
  hashlock: string;
  timelocks: TimelockConfig;
  status: OrderStatus;
  createdAt: number;
  evmSignature?: string; // Only for EVM → ICP orders
  escrowAddresses?: {
    source?: string;
    destination?: string;
  };
}

// Timelock Configuration
interface TimelockConfig {
  srcWithdrawal: number;
  srcPublicWithdrawal: number;
  srcCancellation: number;
  srcPublicCancellation: number;
  dstWithdrawal: number;
  dstPublicWithdrawal: number;
  dstCancellation: number;
  deployedAt: number;
}

// Order Status
type OrderStatus = "PENDING" | "FILLED" | "CANCELLED" | "EXPIRED";
```

### 3.2 Canister Functions

```typescript
// Order Management
createOrder(order: Order): Promise<string>;
getOrder(orderId: string): Promise<Order>;
updateOrderStatus(orderId: string, status: OrderStatus): Promise<void>;
cancelOrder(orderId: string, user: string): Promise<void>;

// Orderbook Queries
getOrderbook(sourceToken: string, destinationToken: string): Promise<Order[]>;
getUserOrders(userId: string): Promise<Order[]>;
getPendingOrders(): Promise<Order[]>;

// Escrow Management
createSourceEscrow(orderId: string): Promise<string>;
updateEscrowAddresses(orderId: string, addresses: EscrowAddresses): Promise<void>;
getEscrowAddresses(orderId: string): Promise<EscrowAddresses>;

// Validation
validateEVMOrder(order: Order, signature: string): boolean;
validateICPOrder(order: Order, identity: Identity): boolean;
```

## 4. Reverse Gas Model Implementation

### 4.1 Benefits

The reverse gas model for ICP → EVM orders provides several key advantages:

- **No User Gas Fees**: Users don't pay for ICP transactions
- **Automatic Locking**: Assets locked immediately on order creation
- **Trustless**: No intermediary needed for asset locking
- **Seamless UX**: One-click order creation
- **Instant Orders**: Orders ready for resolver execution immediately

### 4.2 Technical Implementation

```typescript
// ICP Source Escrow Canister
class ICPSourceEscrow {
  private lockedAssets: Map<string, LockedAsset> = new Map();

  async lockAssets(params: { user: string; token: string; amount: bigint; hashlock: string; timelocks: TimelockConfig }): Promise<void> {
    // 1. Transfer user assets to escrow canister
    await this.transferFromUser(params.user, params.token, params.amount);

    // 2. Lock assets with hashlock
    this.lockedAssets.set(params.hashlock, {
      user: params.user,
      token: params.token,
      amount: params.amount,
      timelocks: params.timelocks,
      lockedAt: Date.now(),
    });
  }

  async unlockAssets(secret: string): Promise<void> {
    const hashlock = this.generateHashlock(secret);
    const lockedAsset = this.lockedAssets.get(hashlock);

    if (!lockedAsset) {
      throw new Error("No locked assets found for hashlock");
    }

    // Transfer assets to resolver (who will complete the swap)
    await this.transferToResolver(lockedAsset);

    // Remove from locked assets
    this.lockedAssets.delete(hashlock);
  }

  private generateHashlock(secret: string): string {
    // SHA-256 hash of secret
    return crypto.createHash("sha256").update(secret).digest("hex");
  }
}
```

## 5. Security Considerations

### 5.1 EVM Order Security

- **EIP-712 Signatures**: Cryptographic proof of order authenticity
- **Address Validation**: Verify signature matches maker address
- **Replay Protection**: Include nonce or timestamp in signature
- **Chain ID Validation**: Prevent cross-chain signature reuse

### 5.2 ICP Order Security

- **Identity Validation**: Verify ICP principal matches maker
- **Canister Authentication**: Use ICP's built-in authentication
- **Asset Locking**: Immediate asset transfer to escrow
- **Timelock Protection**: Prevent indefinite asset locking

### 5.3 Cross-Chain Security

- **Hashlock Verification**: Ensure hashlock matches across chains
- **Timelock Synchronization**: Coordinate timelocks between chains
- **Escrow Validation**: Verify escrow addresses and parameters
- **Status Consistency**: Maintain order status across systems

## 6. Frontend Implementation

### 6.1 User Interface Components

```typescript
// Order Creation Component
interface OrderCreationProps {
  orderType: "EVM_TO_ICP" | "ICP_TO_EVM";
  onOrderCreated: (order: Order) => void;
}

// EVM Wallet Integration
const EVMOrderCreation: React.FC<OrderCreationProps> = ({ orderType, onOrderCreated }) => {
  const [wallet, setWallet] = useState<Web3Provider>();
  const [order, setOrder] = useState<Partial<Order>>();

  const createOrder = async () => {
    // 1. Create order data
    const orderData = createOrderData(order);

    // 2. Sign with EIP-712
    const signature = await signOrder(orderData, wallet);

    // 3. Submit to canister
    const orderId = await submitOrderToCanister(orderData, signature);

    // 4. Create destination escrow
    await createDestinationEscrow(orderData);

    onOrderCreated({ ...orderData, id: orderId });
  };

  return <div>{/* Order form UI */}</div>;
};

// ICP Identity Integration
const ICPOrderCreation: React.FC<OrderCreationProps> = ({ orderType, onOrderCreated }) => {
  const [identity, setIdentity] = useState<Identity>();
  const [order, setOrder] = useState<Partial<Order>>();

  const createOrder = async () => {
    // 1. Create order data
    const orderData = createOrderData(order);

    // 2. Submit to canister (no signature needed)
    const orderId = await submitOrderToCanister(orderData);

    // 3. Source escrow created automatically
    // 4. Assets locked immediately

    onOrderCreated({ ...orderData, id: orderId });
  };

  return <div>{/* Order form UI */}</div>;
};
```

## 7. Testing Strategy

### 7.1 Unit Tests

```typescript
// Test EVM order validation
describe("EVM Order Validation", () => {
  it("should validate correct EIP-712 signature", () => {
    const order = createTestOrder();
    const signature = signOrder(order, testWallet);
    expect(validateEVMOrder(order, signature)).toBe(true);
  });

  it("should reject invalid signature", () => {
    const order = createTestOrder();
    const invalidSignature = "0xinvalid";
    expect(validateEVMOrder(order, invalidSignature)).toBe(false);
  });
});

// Test ICP order creation
describe("ICP Order Creation", () => {
  it("should create source escrow automatically", async () => {
    const order = createICPOrder();
    const orderId = await createOrder(order);
    const escrowAddress = await getSourceEscrow(orderId);
    expect(escrowAddress).toBeDefined();
  });

  it("should lock assets immediately", async () => {
    const order = createICPOrder();
    const orderId = await createOrder(order);
    const lockedAmount = await getLockedAmount(orderId);
    expect(lockedAmount).toEqual(order.sourceAmount);
  });
});
```

### 7.2 Integration Tests

```typescript
// Test complete order flow
describe("Complete Order Flow", () => {
  it("should handle EVM → ICP order creation", async () => {
    // 1. Create EVM order
    const order = await createEVMOrder();

    // 2. Verify order stored in canister
    const storedOrder = await getOrder(order.id);
    expect(storedOrder).toEqual(order);

    // 3. Verify destination escrow created
    const escrowAddress = await getDestinationEscrow(order.id);
    expect(escrowAddress).toBeDefined();
  });

  it("should handle ICP → EVM order creation", async () => {
    // 1. Create ICP order
    const order = await createICPOrder();

    // 2. Verify order stored in canister
    const storedOrder = await getOrder(order.id);
    expect(storedOrder).toEqual(order);

    // 3. Verify source escrow created and assets locked
    const escrowAddress = await getSourceEscrow(order.id);
    expect(escrowAddress).toBeDefined();

    const lockedAmount = await getLockedAmount(order.id);
    expect(lockedAmount).toEqual(order.sourceAmount);
  });
});
```

## 8. Next Steps

### 8.1 Immediate Implementation

1. **Design Orderbook Canister**: Define data structures and functions
2. **Create Frontend Mockup**: Design user interface for both order types
3. **Implement Order Validation**: Build EVM signature validation
4. **Test Reverse Gas Model**: Validate ICP escrow creation

### 8.2 Research Requirements

1. **ICP Identity Integration**: Study authentication methods and best practices
2. **Canister Development**: Learn Motoko/Rust for ICP development
3. **Cross-Chain Communication**: Understand chain interaction patterns
4. **Security Auditing**: Plan security review for cross-chain operations

This analysis provides a comprehensive foundation for implementing the order creation flow in the ICP <> EVM cross-chain fusion protocol.
