# 1inch Fusion SDK

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/fusion-sdk)_

---

## Overview

The 1inch Fusion SDK provides tools and libraries for integrating with the Intent Swaps (Fusion) API across different blockchain platforms.

---

## Available SDKs

### **Solana SDK**

Specialized SDK for integrating with the Solana blockchain.

- **Installation**: `npm install @1inch/fusion-solana-sdk`
- **Documentation**: [Solana SDK Guide](solana-sdk.md)
- **Features**:
  - Solana-specific optimizations
  - Native Solana transaction handling
  - SPL token support
  - Solana wallet integration

### **Fusion SDK for EVMs**

SDK for Ethereum Virtual Machine (EVM) compatible blockchains.

- **Installation**: `npm install @1inch/fusion-evm-sdk`
- **Documentation**: [EVM SDK Guide](evm-sdk.md)
- **Features**:
  - Multi-chain EVM support
  - Gas optimization
  - Web3 wallet integration
  - Batch transaction support

---

## Quick Start

### **Installation**

```bash
# For Solana
npm install @1inch/fusion-solana-sdk

# For EVM chains
npm install @1inch/fusion-evm-sdk
```

### **Basic Usage**

```javascript
// EVM Example
import { FusionSDK } from "@1inch/fusion-evm-sdk";

const sdk = new FusionSDK({
  apiKey: "YOUR_API_KEY",
  chainId: 1, // Ethereum mainnet
});

// Create a gasless swap order
const order = await sdk.createOrder({
  fromToken: "0xA0b86a33E6441b8c4C8C1C1B9C9C9C9C9C9C9C9C9C9",
  toToken: "0xB0b86a33E6441b8c4C8C1C1B9C9C9C9C9C9C9C9C9C9C9",
  amount: "1000000000000000000",
  fromAddress: "0xYourWalletAddress",
});
```

---

## Features

### **Cross-Platform Support**

- **Solana**: Native Solana integration
- **EVM Chains**: Ethereum, BSC, Polygon, Arbitrum, etc.
- **Multi-Chain**: Unified API across platforms

### **Developer Tools**

- **TypeScript Support**: Full type definitions
- **Code Examples**: Comprehensive examples
- **Error Handling**: Robust error management
- **Testing**: Built-in testing utilities

### **Integration Features**

- **Wallet Integration**: Support for popular wallets
- **Gas Optimization**: Automatic gas cost optimization
- **Batch Operations**: Efficient batch processing
- **Real-time Updates**: WebSocket support for order updates

---

## Documentation

- **[Solana SDK Guide](solana-sdk.md)** - Complete Solana integration guide
- **[EVM SDK Guide](evm-sdk.md)** - Complete EVM integration guide
- **[API Reference](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/swagger)** - Full API documentation
- **[Examples](https://github.com/1inch/fusion-sdk-examples)** - Code examples and tutorials

---

## Support

- **Documentation**: [1inch Developer Portal](https://portal.1inch.dev)
- **GitHub**: [Fusion SDK Repository](https://github.com/1inch/fusion-sdk)
- **Discord**: [Community Support](https://discord.gg/1inch)
- **Email**: [Developer Support](mailto:dev@1inch.io)

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
