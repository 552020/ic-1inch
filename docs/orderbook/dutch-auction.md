# Dutch Auction Implementation Details

## Overview

This document captures the existing Dutch auction implementation details from the 1inch Fusion protocol codebase. These implementations are derived from the official 1inch codebase and provide reliable reference for future development.

## 1inch Fusion Dutch Auction Implementation

### Core Contract: DutchAuctionCalculator.sol

**Location**: `eth/lib/limit-order-protocol/contracts/extensions/DutchAuctionCalculator.sol`

**Key Features**:

- Implements price decay over time from max to min
- Used by 1inch Fusion for Dutch auction price calculation
- Supports both `getMakingAmount` and `getTakingAmount` calculations

### Price Calculation Algorithm

```solidity
function _calculateAuctionTakingAmount(
    uint256 startTimeEndTime,
    uint256 takingAmountStart,
    uint256 takingAmountEnd
) private view returns(uint256) {
    uint256 startTime = startTimeEndTime >> 128;
    uint256 endTime = startTimeEndTime & _LOW_128_BITS;
    uint256 currentTime = Math.max(startTime, Math.min(endTime, block.timestamp));

    return (takingAmountStart * (endTime - currentTime) +
            takingAmountEnd * (currentTime - startTime)) /
           (endTime - startTime);
}
```

**Key Implementation Details**:

- Uses bit manipulation to pack start/end times efficiently
- Linear interpolation between start and end amounts
- Clamps current time between start and end times
- Returns calculated taking amount based on elapsed time

### Auction Parameters Structure

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

### Price Curve Implementation

**From 1inch Fusion+ Whitepaper**:

The Dutch auction does **NOT** decrease linearly. The auction duration is divided into several segments, each having its own decrease speed:

1. **Grid Approach**: Instead of starting at market price, auction commences at X/6 (SpotPrice)
2. **Six Equal Parts**: Outgoing amount X is divided into: X/6, 2X/6, 3X/6, 4X/6, 5X/6, 6X/6
3. **Descending Adjustment**: Over ~2/3 of auction duration, price descends from SpotPrice to market price
4. **Partial Fills**: Combined with partial fills for better prices and quicker fulfillment

### Advanced Features from 1inch Implementation

#### 1. Piecewise Linear Price Curves

```solidity
// Multiple segments with different decrease rates
struct PriceSegment {
    uint256 startTime;
    uint256 endTime;
    uint256 startPrice;
    uint256 endPrice;
    uint256 decreaseRate;
}
```

#### 2. Volatility Adjustments

```solidity
// Market condition-based adjustments
function calculateVolatilityAdjustment(
    uint256 basePrice,
    uint256 marketVolatility
) internal pure returns (uint256) {
    // Adjust price based on market volatility
    return basePrice * (1000 + marketVolatility) / 1000;
}
```

#### 3. Partial Fill Support

```solidity
// Merkle tree verification for partial fills
function verifyPartialFill(
    bytes32[] memory proof,
    bytes32 root,
    bytes32 leaf
) internal pure returns (bool) {
    // Verify partial fill using Merkle proof
}
```

### Testing Implementation

**Location**: `eth/lib/limit-order-settlement/test/Settlement.js`

**Key Test Patterns**:

- Single order preparation with auction parameters
- Time-based price calculations
- Rate bump calculations based on remaining duration
- Integration testing with resolver execution

### Integration with Fusion Protocol

#### Auction Flow:

1. **Order Creation**: User creates order with auction parameters
2. **Resolver Monitoring**: Multiple resolvers monitor for profitability
3. **Race to Execute**: First successful execution wins the auction
4. **Order Locking**: Winning resolver locks the order

#### Key Concepts:

- **Winning = Successful Execution**: Resolver wins by executing first
- **Fee Competition**: Resolvers compete on execution fees
- **Time-based Pricing**: Price decreases over time
- **Partial Fills**: Orders can be filled partially with Merkle verification

### Future Development Considerations

#### Preserved for Advanced Implementation:

1. **Piecewise Linear Curves**: Multiple segments with different rates
2. **Volatility Adjustments**: Market condition-based pricing
3. **Partial Fill Support**: Merkle tree verification
4. **Advanced State Management**: Complex auction states
5. **Real-time Updates**: Market data integration
6. **Multi-phase Auctions**: Complex auction mechanics

#### MVP Simplification Applied:

1. **Linear Interpolation**: Simple start-to-end price decrease
2. **Fixed Intervals**: 5-minute update intervals
3. **Basic States**: 4 simple auction states
4. **No Partial Fills**: All-or-nothing execution
5. **No Volatility**: Fixed pricing parameters

### References

- **Official 1inch DutchAuctionCalculator**: `eth/lib/limit-order-protocol/contracts/extensions/DutchAuctionCalculator.sol`
- **1inch Fusion+ Whitepaper**: `docs/cross-turk/1inch-fusion-plus-whitepaper.md`
- **Auction Mechanism Documentation**: `docs/cross-turk/auction-mechanism.md`
- **Testing Implementation**: `eth/lib/limit-order-settlement/test/Settlement.js`
