# 1inch Intent Swaps (Fusion) API - Introduction

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/introduction)_

---

## Overview

**1inch intent-based swap (Fusion mode)**

1inch intent swaps (Fusion mode) offer users a way to execute swaps without spending gas or risking being front-run. To the user, Fusion mode looks like a swap, but technically it is a limit order with a variable exchange rate filled by a third party called a resolver. An order's exchange rate decreases from the desired rate to the minimal return amount (Dutch auction) until it becomes profitable for resolvers to fill the order. Multiple resolvers compete for the order to ensure it is filled before the rate falls to the minimal return amount.

---

## Key Benefits

### **For Users**

- **Gasless execution** - No upfront gas costs for users
- **MEV protection** - Protection against front-running and sandwich attacks
- **Better rates** - Dutch auction ensures competitive pricing
- **Simplified experience** - Looks like a regular swap but with enhanced features

### **For Resolvers**

Here are some examples of opportunities for resolvers to gain profit:

- **Dutch auction dynamics** - The Dutch auction constantly decreases the order rate
- **Gas economy** - When filling matching orders
- **Batch filling efficiency** - Gas economy due to batch filling
- **Competitive advantage** - Multiple resolvers compete for orders

For resolvers and integrators there is a **Fusion SDK** available to help with the integration.

---

## Dutch Auction Filling Rate

Each order starts with an auction timestamp, calculated as the order's signature timestamp plus a waiting period to account for different network speeds. Before the auction begins, an order can be filled at the maximum rate. After the auction starts, the filling rate gradually decreases.

### **Rate Calculation Factors**

The filling rate depends on several factors, including:

- **Swap volume** - Larger orders may have different rate curves
- **Gas costs** - Current network gas prices affect profitability
- **Chosen preset** - Different presets (e.g., fast, fair, auction) have different rate curves

### **Price Impact Optimization**

To minimize price impact, the source token volume is divided into parts, creating multiple price points. This benefits users with better rates and allows resolvers to profit.

### **Partial Fill Functionality**

The partial fill functionality optimizes efficiency by allowing large swaps to be executed at better rates. Different resolvers can fill different parts of the order, making it profitable at various points.

### **Dynamic Price Curve**

The price curve is also dynamic and adapts to gas market conditions to manage gas price volatility. This reduces the chance of order expiration and speeds up execution by 75%.

#### **Gas Price Adaptation**

- **If gas prices increase**: The resolver might delay fulfillment
- **If gas prices decrease**: The resolver benefits from lower costs
- **Adjusted price curve**: Ensures users receive more tokens when base fees decline and corrects execution costs when they rise

---

## Best Practices

### **For Resolvers**

We recommend resolvers split orders into **6-10 parts** and check if at least one part can be filled.

### **Order Optimization**

- Monitor gas prices and adjust filling strategies accordingly
- Use batch filling to optimize gas costs
- Implement partial fill strategies for large orders
- Monitor auction timestamps and rate curves

---

## Technical Architecture

### **Order Flow**

1. **User signs intent** - Creates a limit order with variable exchange rate
2. **Order broadcast** - Order is distributed to resolver network
3. **Dutch auction begins** - Rate decreases over time
4. **Resolver competition** - Multiple resolvers compete to fill order
5. **Order execution** - Winning resolver executes the swap
6. **Gasless completion** - User receives tokens without paying gas

### **Key Components**

- **Intent-based orders** - Users sign off-chain intents
- **Dutch auction mechanism** - Dynamic pricing based on time
- **Resolver network** - Professional market makers competing for orders
- **Partial fill support** - Large orders split into multiple parts
- **Gas cost optimization** - Resolvers handle all gas costs

---

## API Reference

For detailed information about each endpoint, refer to the [Intent swap API Swagger section](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/swagger).

### **Available Endpoints**

- **Get gasless swap active orders** - Retrieve active Fusion orders
- **Create gasless swap order** - Submit new Fusion orders
- **Get order status** - Check order execution status
- **Cancel order** - Cancel pending orders

---

## Supported Networks

Fusion mode is available on multiple blockchain networks, including:

- Ethereum Mainnet
- Arbitrum
- Avalanche
- BNB Chain
- Gnosis
- Solana
- Sonic
- Optimism
- Polygon
- zkSync Era
- Base
- Unichain

---

## Integration Resources

### **SDK and Tools**

- **Fusion SDK** - Official SDK for resolver integration
- **Swagger Documentation** - Interactive API documentation
- **Code Examples** - Language-specific implementation guides

### **Development Support**

- **API Documentation** - Complete endpoint reference
- **Best Practices Guide** - Optimization strategies
- **Community Support** - Developer forums and Discord

---

## Comparison with Other Swap Types

| Feature             | Fusion (Intent Swaps) | Classic Swap       | Fusion+ (Cross-Chain)   |
| ------------------- | --------------------- | ------------------ | ----------------------- |
| **Gas Costs**       | User pays no gas      | User pays gas      | User pays no gas        |
| **MEV Protection**  | Built-in protection   | Limited protection | Built-in protection     |
| **Cross-Chain**     | No                    | No                 | Yes                     |
| **Execution Speed** | Fast (Dutch auction)  | Immediate          | Variable (escrow-based) |
| **Complexity**      | Medium                | Low                | High                    |
| **Use Case**        | Same-chain swaps      | Simple swaps       | Cross-chain swaps       |

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
