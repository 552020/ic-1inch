# ICP <> EVM Cross-Chain Fusion Masterplan

## Overview

This document outlines the masterplan for building a cross-chain fusion protocol that enables seamless swaps between Internet Computer (ICP) and Ethereum Virtual Machine (EVM) chains using 1inch's Fusion+ technology.

## Project Architecture

### Core Components

1. **Frontend Application** - User interface for order creation and management
2. **Orderbook Canister** - ICP-based order storage and management
3. **EVM Contracts** - Fusion+ escrow contracts on EVM chains
4. **Relayer Service** - Order routing and resolver coordination
5. **Resolver Network** - Professional resolvers for order execution

## 1. Order Creation Flow Analysis

### 1.1 User Authentication & Frontend

**Frontend Application**

- User login/authentication system
- Order creation interface
- Order management dashboard
- Real-time orderbook display
- Transaction status tracking

**Key Features:**

- Web3 wallet integration (MetaMask, etc.)
- ICP identity integration
- Cross-chain balance display
- Order history and status

### 1.2 Order Types & Creation Logic

#### Type A: EVM → ICP Orders

```
User Flow:
1. User connects EVM wallet (MetaMask)
2. User specifies: EVM token → ICP token
3. User signs EIP-712 order with private key
4. Order stored in ICP orderbook canister
5. User locks assets on EVM chain (destination escrow)
```

**Technical Implementation:**

- EIP-712 signature for order validation
- Order stored in ICP canister with EVM signature
- User creates destination escrow on EVM chain
- Order remains pending until resolver fills

#### Type B: ICP → EVM Orders

```
User Flow:
1. User connects ICP identity
2. User specifies: ICP token → EVM token
3. Order stored in ICP orderbook canister
4. Source escrow automatically created on ICP
5. User assets locked in ICP escrow (reverse gas model)
```

**Technical Implementation:**

- ICP identity for user authentication
- Automatic source escrow creation on ICP
- Reverse gas model - no user gas fees
- Order ready for resolver execution

### 1.3 Orderbook Canister Design

#### Data Structure

```typescript
interface Order {
  id: string;
  maker: string; // User address/identity
  orderType: "EVM_TO_ICP" | "ICP_TO_EVM";
  sourceChain: string;
  destinationChain: string;
  sourceToken: string;
  destinationToken: string;
  sourceAmount: bigint;
  destinationAmount: bigint;
  hashlock: string;
  timelocks: {
    srcWithdrawal: number;
    srcPublicWithdrawal: number;
    srcCancellation: number;
    srcPublicCancellation: number;
    dstWithdrawal: number;
    dstPublicWithdrawal: number;
    dstCancellation: number;
    deployedAt: number;
  };
  status: "PENDING" | "FILLED" | "CANCELLED" | "EXPIRED";
  createdAt: number;
  evmSignature?: string; // For EVM → ICP orders
  escrowAddresses?: {
    source?: string;
    destination?: string;
  };
}
```

#### Canister Functions

```typescript
// Order Management
createOrder(order: Order): Promise<string>;
getOrder(orderId: string): Promise<Order>;
getOrders(filters: OrderFilters): Promise<Order[]>;
updateOrderStatus(orderId: string, status: OrderStatus): Promise<void>;

// Orderbook Queries
getOrderbook(sourceToken: string, destinationToken: string): Promise<Order[]>;
getUserOrders(userId: string): Promise<Order[]>;

// Escrow Management
createSourceEscrow(orderId: string): Promise<string>;
updateEscrowAddresses(orderId: string, addresses: EscrowAddresses): Promise<void>;
```

### 1.4 Reverse Gas Model Implementation

#### ICP Source Escrow Creation

```typescript
// Automatic source escrow creation for ICP → EVM orders
async function createICPSourceEscrow(order: Order): Promise<string> {
  // 1. Create escrow canister on ICP
  const escrowCanister = await createEscrowCanister();

  // 2. Lock user assets in ICP escrow
  await escrowCanister.lockAssets({
    user: order.maker,
    token: order.sourceToken,
    amount: order.sourceAmount,
    hashlock: order.hashlock,
    timelocks: order.timelocks,
  });

  // 3. Return escrow canister address
  return escrowCanister.address;
}
```

#### Benefits of Reverse Gas Model

- **No User Gas Fees**: Users don't pay for ICP transactions
- **Automatic Locking**: Assets locked immediately on order creation
- **Trustless**: No intermediary needed for asset locking
- **Seamless UX**: One-click order creation

### 1.5 Order Validation & Security

#### EVM Order Validation

```typescript
// Validate EIP-712 signature for EVM orders
function validateEVMOrder(order: Order, signature: string): boolean {
  const recoveredAddress = ethers.utils.verifyTypedData(domain, types, order, signature);
  return recoveredAddress === order.maker;
}
```

#### ICP Order Validation

```typescript
// Validate ICP identity for ICP orders
function validateICPOrder(order: Order, identity: Identity): boolean {
  return identity.getPrincipal().toText() === order.maker;
}
```

## 2. Technical Architecture

### 2.1 System Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │  Orderbook      │    │  EVM Contracts  │
│   Application   │◄──►│  Canister       │◄──►│  (Fusion+)      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Relayer        │    │  ICP Escrow     │    │  Resolver       │
│  Service        │    │  Canisters      │    │  Network        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### 2.2 Data Flow

#### Order Creation Flow

```
1. User → Frontend: Create order
2. Frontend → Canister: Store order
3. Canister → EVM/ICP: Create escrow (if needed)
4. Canister → Frontend: Order confirmation
5. Frontend → User: Order status update
```

#### Order Execution Flow

```
1. Relayer → Canister: Fetch pending orders
2. Relayer → Resolvers: Broadcast orders
3. Resolver → EVM/ICP: Execute swaps
4. Resolver → Canister: Update order status
5. Canister → Frontend: Status update
```

## 3. Implementation Phases

### Phase 1: Core Infrastructure

- [ ] Design and implement orderbook canister
- [ ] Create basic frontend application
- [ ] Implement order creation flows
- [ ] Set up EVM contract integration

### Phase 2: Escrow System

- [ ] Implement ICP source escrow canisters
- [ ] Integrate with EVM destination escrows
- [ ] Test reverse gas model
- [ ] Validate cross-chain locking

### Phase 3: Relayer & Resolver

- [ ] Build relayer service
- [ ] Integrate with resolver network
- [ ] Implement order execution logic
- [ ] Test complete flow

### Phase 4: Production Ready

- [ ] Security audits
- [ ] Performance optimization
- [ ] User experience improvements
- [ ] Production deployment

## 4. Key Advantages

### 4.1 User Experience

- **Seamless Cross-Chain**: One interface for both chains
- **No Gas Fees**: Reverse gas model for ICP transactions
- **Instant Orders**: Immediate order placement and locking
- **Real-Time Updates**: Live orderbook and status tracking

### 4.2 Technical Benefits

- **Trustless**: No intermediaries for asset locking
- **Scalable**: Canister-based order management
- **Secure**: EIP-712 signatures and ICP identity validation
- **Efficient**: Optimized for cross-chain operations

### 4.3 Business Model

- **Professional Resolvers**: Network of trusted resolvers
- **Competitive Pricing**: Dutch auction mechanism
- **Liquidity Aggregation**: Access to multiple DEXs
- **Revenue Sharing**: Fees distributed to resolvers

## 5. Next Steps

### Immediate Actions

1. **Design Orderbook Canister**: Define data structures and functions
2. **Create Frontend Mockup**: Design user interface
3. **Implement Order Creation**: Build basic order flow
4. **Test Reverse Gas Model**: Validate ICP escrow creation

### Research Requirements

1. **ICP Identity Integration**: Study authentication methods
2. **Canister Development**: Learn Motoko/Rust for ICP
3. **Cross-Chain Communication**: Understand chain interaction
4. **Security Considerations**: Audit cross-chain vulnerabilities

This masterplan provides a comprehensive framework for building the ICP <> EVM cross-chain fusion protocol, starting with the order creation flow as the foundation.
