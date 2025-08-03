# Fusion Auction Mechanism

> **Important**: The auction mechanism described in this document is **NOT implemented in the cross-chain-swap repository**. This is external infrastructure that works alongside the escrow contracts.

## Overview

The Fusion protocol uses a **Dutch auction mechanism** where resolvers compete to execute cross-chain swaps. The auction starts with high fees and decreases over time until a resolver finds it profitable to execute.

## How the Auction Works

### **Key Concept: Winning = Successful Execution**

**The resolver "wins" the auction by successfully executing the swap first!**

```
Multiple Resolvers Monitor Order
         ↓
    Fee Becomes Profitable
         ↓
All Resolvers Try to Execute Simultaneously
         ↓
First Successful Transaction Wins
         ↓
Order Becomes Unavailable to Others
```

### **The Race to Execute**

```javascript
// Multiple resolvers running this simultaneously
async function tryToWin(order) {
  if (isProfitable(order)) {
    try {
      // This is the "auction bid" - trying to execute
      const result = await executeSwap(order);

      if (result.success) {
        // 🏆 WE WON! Order is locked to us
        return "WINNER";
      }
    } catch (error) {
      // Someone else got there first
      return "LOST - Order already taken";
    }
  }
}
```

### 1. Order Creation with Auction Parameters

When a user creates a cross-chain swap order, they include auction parameters:

```solidity
struct AuctionDetails {
    uint24 gasBumpEstimate;     // Gas cost estimation
    uint32 gasPriceEstimate;    // Gas price estimation
    uint32 startTime;           // When auction begins
    uint24 duration;            // How long auction runs (e.g., 10 minutes)
    uint32 delay;               // Delay before auction starts
    uint24 initialRateBump;     // Starting fee premium (e.g., 100 = 1%)
    bytes auctionPoints;        // Price decay curve data
}
```

### 2. Dutch Auction Price Decay

The auction follows a **Dutch auction** model where the resolver fee decreases over time:

```
Timeline Example (10-minute auction):
┌─────────────────────────────────────────────────────────────┐
│ Time 0min:  Resolver Fee = 1.0% (high - less competition)   │
│ Time 2min:  Resolver Fee = 0.8%                            │
│ Time 4min:  Resolver Fee = 0.6%                            │
│ Time 6min:  Resolver Fee = 0.4%                            │
│ Time 8min:  Resolver Fee = 0.2%                            │
│ Time 10min: Resolver Fee = 0.1% (low - more competition)   │
└─────────────────────────────────────────────────────────────┘
```

### 3. Resolver Competition Strategy

Resolvers monitor orders and make strategic decisions:

```javascript
class ResolverStrategy {
  async monitorOrder(order) {
    while (auction.isActive()) {
      const currentFee = this.calculateCurrentFee(order.auctionDetails);
      const profitability = this.estimateProfitability(order, currentFee);

      if (profitability > this.minimumProfit) {
        // Execute immediately - first come, first served
        return await this.executeSwap(order);
      }

      await this.sleep(1000); // Wait 1 second and check again
    }
  }

  calculateCurrentFee(auctionDetails) {
    const elapsed = now() - auctionDetails.startTime;
    const progress = elapsed / auctionDetails.duration;

    // Linear decay from initialRateBump to minimum
    return auctionDetails.initialRateBump * (1 - progress);
  }
}
```

## Auction Components

### 1. Order Distribution System

```
┌─────────────────────────────────────────────────────────────┐
│                    1INCH BACKEND                           │
├─────────────────────────────────────────────────────────────┤
│ • Receives user orders with auction parameters            │
│ • Validates order parameters                              │
│ • Distributes to resolver network                         │
│ • Manages auction lifecycle                               │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│                 RESOLVER NETWORK                           │
├─────────────────────────────────────────────────────────────┤
│ Resolver A ──┐                                            │
│ Resolver B ──┤ All monitor the same order                 │
│ Resolver C ──┤ Wait for profitable fee level              │
│ Resolver D ──┤ First to execute wins                      │
│ ...          │                                            │
│ Resolver Z ──┘                                            │
└─────────────────────────────────────────────────────────────┘
```

### 2. Auction Parameters in Code

From the repository's test files, we can see the auction parameter structure:

```solidity
// From CrossChainTestLib.sol
function buildAuctionDetails(
    uint24 gasBumpEstimate,      // Extra gas cost buffer
    uint32 gasPriceEstimate,     // Expected gas price
    uint32 startTime,            // Auction start timestamp
    uint24 duration,             // Auction duration in seconds
    uint32 delay,                // Delay before auction starts
    uint24 initialRateBump,      // Starting fee (basis points)
    bytes memory auctionPoints   // Price curve data points
) internal pure returns (bytes memory auctionDetails) {
    auctionDetails = abi.encodePacked(
        gasBumpEstimate,
        gasPriceEstimate,
        startTime + delay,
        duration,
        initialRateBump,
        auctionPoints
    );
}
```

### 3. Resolver Whitelisting

```solidity
// From test files - resolver whitelist format
bytes memory whitelist = abi.encodePacked(uint32(block.timestamp)); // auction start time
for (uint256 i = 0; i < resolvers.length; i++) {
    whitelist = abi.encodePacked(
        whitelist,
        uint80(uint160(resolvers[i])),  // resolver address
        uint16(0)                       // time delta (priority)
    );
}
```

## Auction Lifecycle

### Phase 1: Order Creation

```
User → 1inch Frontend → Backend
  ↓
Order with auction parameters created
  ↓
Distributed to resolver network
```

### Phase 2: Auction Period

```
Resolvers monitor order:
┌─────────────────────────────────────────────────────────────┐
│ Time 0: Fee = 1.0% → Too expensive, wait                   │
│ Time 2: Fee = 0.8% → Still too expensive, wait             │
│ Time 4: Fee = 0.6% → Getting better, wait                  │
│ Time 6: Fee = 0.4% → Profitable! Resolver C executes      │
└─────────────────────────────────────────────────────────────┘
```

### Phase 3: Execution

```
Winning resolver:
1. Calls Limit Order Protocol to fill order
2. Deploys EscrowSrc on source chain
3. Deploys EscrowDst on destination chain
4. Coordinates cross-chain withdrawal
```

## Economic Incentives

### For Users

- **Lower fees over time**: If users can wait, they get better rates
- **Guaranteed execution**: Eventually someone will execute at low fee
- **No gas costs**: Users don't pay gas for cross-chain coordination

### For Resolvers

- **Competition drives efficiency**: Must be fast and efficient to win
- **Risk vs reward**: Early execution = higher fees but guaranteed win
- **Specialization opportunities**: Can focus on specific chains/tokens

### Fee Structure Example

```
Total Swap Fee: 0.5%
├── User pays: 0.5%
├── 1inch platform: 0.1% (fixed)
└── Winning resolver: 0.4% (variable based on auction)
```

## Implementation Requirements

### For Your Own Auction System

If you want to implement a similar system, you'll need:

#### 1. Order Management System

```javascript
class OrderManager {
  createOrder(userOrder) {
    const auctionDetails = this.generateAuctionParams(userOrder);
    const order = { ...userOrder, auctionDetails };
    this.distributeToResolvers(order);
    return order;
  }

  generateAuctionParams(userOrder) {
    return {
      startTime: Date.now(),
      duration: 600, // 10 minutes
      initialRateBump: 100, // 1%
      // ... other parameters
    };
  }
}
```

#### 2. Resolver Network Infrastructure

```javascript
class ResolverNetwork {
  constructor() {
    this.resolvers = new Map();
    this.activeOrders = new Map();
  }

  registerResolver(resolver) {
    this.resolvers.set(resolver.address, resolver);
  }

  distributeOrder(order) {
    for (const resolver of this.resolvers.values()) {
      resolver.notifyNewOrder(order);
    }
  }
}
```

#### 3. Auction Price Calculator

```javascript
class AuctionPriceCalculator {
  calculateCurrentFee(auctionDetails) {
    const now = Date.now();
    const elapsed = now - auctionDetails.startTime;
    const progress = Math.min(elapsed / (auctionDetails.duration * 1000), 1);

    // Linear decay from initial to minimum
    const initialFee = auctionDetails.initialRateBump / 10000; // Convert basis points
    const minimumFee = 0.001; // 0.1%

    return initialFee * (1 - progress) + minimumFee * progress;
  }

  isAuctionActive(auctionDetails) {
    const now = Date.now();
    const endTime = auctionDetails.startTime + auctionDetails.duration * 1000;
    return now >= auctionDetails.startTime && now <= endTime;
  }
}
```

## Security Considerations

### 1. Front-Running Protection

- **Time-based execution**: First valid transaction wins
- **Resolver whitelisting**: Only approved resolvers can participate
- **Minimum time delays**: Prevent immediate execution

### 2. Auction Manipulation Prevention

- **Fixed auction parameters**: Cannot be changed once order is created
- **Transparent pricing**: All resolvers see same price curve
- **Economic penalties**: Resolvers lose safety deposits for failures

### 3. Resolver Reliability

- **Safety deposits**: Resolvers must stake tokens
- **Performance tracking**: Poor performers get removed
- **Redundancy**: Multiple resolvers ensure execution

## Integration with Escrow Contracts

The auction system integrates with the escrow contracts from this repository:

```
Auction Winner → Limit Order Protocol → EscrowFactory.postInteraction()
                                              ↓
                                        Creates EscrowSrc proxy
                                              ↓
                                        Resolver deploys EscrowDst
                                              ↓
                                        Cross-chain swap execution
```

## Monitoring and Analytics

### Key Metrics to Track

1. **Auction Efficiency**

   - Average execution time
   - Fee levels at execution
   - Number of competing resolvers

2. **Resolver Performance**

   - Success rate per resolver
   - Average execution speed
   - Profitability analysis

3. **User Experience**
   - Order fulfillment rate
   - Time to execution
   - Fee satisfaction

### Example Monitoring Dashboard

```javascript
class AuctionMonitor {
  trackAuctionMetrics(order, executionResult) {
    const metrics = {
      orderId: order.id,
      executionTime: executionResult.timestamp - order.auctionDetails.startTime,
      finalFee: executionResult.fee,
      winningResolver: executionResult.resolver,
      competitorCount: this.getCompetitorCount(order.id),
    };

    this.analytics.record(metrics);
  }
}
```

## Conclusion

The auction mechanism is a sophisticated system that operates **outside** the escrow contracts but is **essential** for the Fusion protocol's operation. It creates a competitive marketplace where resolvers compete to provide the best service to users while maintaining economic incentives for all participants.

For anyone implementing a similar system, the auction mechanism requires significant infrastructure beyond just the smart contracts - including order management, resolver networks, and real-time price calculations.
