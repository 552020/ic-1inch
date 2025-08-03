# Base Sepolia Deployment Guide - Limit Order Protocol v4.3.2

## ⚠️ **Important: Production Version**

This deployment uses **version 4.3.2** which is the latest **audited and production-ready** version of the Limit Order Protocol.

**Why this version?**
- ✅ **Audited**: Passed security audits
- ✅ **Production-ready**: Deployed on multiple mainnets
- ✅ **Stable**: No work-in-progress features
- ✅ **Secure**: No known vulnerabilities

## 🚀 **Deployment Steps**

### **1. Environment Setup**

Create a `.env` file in the root directory:

```bash
# Base Sepolia RPC
BASE_SEPOLIA_RPC_URL=https://sepolia.base.org

# Deployer private key (keep secure!)
PRIVATE_KEY=your_private_key_here

# Etherscan API key for verification
ETHERSCAN_API_KEY=your_etherscan_api_key
```

### **2. Install Dependencies**

```bash
npm install
# or
yarn install
```

### **3. Compile Contracts**

```bash
npx hardhat compile
```

### **4. Deploy to Base Sepolia**

```bash
# Deploy the Limit Order Protocol
npx hardhat deploy --network base-sepolia
```

### **5. Verify Contract (Optional)**

The deployment script automatically verifies the contract on Etherscan if you have an API key configured.

## 📋 **Configuration Details**

### **Network Configuration**

The deployment uses the `@1inch/solidity-utils` network configuration which includes Base Sepolia:

- **Chain ID**: 84532 (Base Sepolia)
- **RPC URL**: https://sepolia.base.org
- **Explorer**: https://sepolia.basescan.org

### **Contract Details**

- **Contract**: `LimitOrderProtocol`
- **Constructor**: Takes WETH address as parameter
- **WETH Address**: Automatically configured for Base Sepolia

## 🔍 **Verification**

After deployment, verify your contract:

1. **Check deployment**: The script will output the deployed address
2. **Verify on Etherscan**: Visit https://sepolia.basescan.org/address/YOUR_CONTRACT_ADDRESS
3. **Test functionality**: Use the test scripts in the `scripts/` directory

## 📚 **Usage Examples**

### **Create a Limit Order**

```javascript
// Example order creation
const order = {
    maker: "0x...",
    taker: "0x...",
    makerAsset: "0x...", // Token address
    takerAsset: "0x...", // Token address
    makerAmount: "1000000000000000000", // 1 token (18 decimals)
    takerAmount: "2000000000000000000", // 2 tokens (18 decimals)
    expiration: Math.floor(Date.now() / 1000) + 3600, // 1 hour
    nonce: "123456789"
};
```

### **Fill an Order**

```javascript
// Example order filling
const fillOrder = async (order, signature) => {
    const limitOrderProtocol = await ethers.getContract("LimitOrderProtocol");
    await limitOrderProtocol.fillOrder(order, signature);
};
```

## ⚡ **Quick Test**

After deployment, test the contract:

```bash
# Run tests
npm test

# Or run specific test
npx hardhat test test/LimitOrderProtocol.test.js
```

## 🔗 **Integration with Cross-Chain Swap**

This deployed Limit Order Protocol will be used by the `cross-chain-swap` repository for:

1. **Order Management**: Creating and managing limit orders
2. **Order Execution**: Filling orders through resolvers
3. **Cross-Chain Integration**: Part of the Fusion+ protocol stack

## 📝 **Notes**

- **Gas Optimization**: The contract is optimized for gas efficiency
- **Security**: This version has passed multiple security audits
- **Compatibility**: Works with all EVM-compatible tokens
- **Extensibility**: Supports custom interactions and conditions

## 🆘 **Troubleshooting**

### **Common Issues**

1. **Insufficient Gas**: Base Sepolia may require higher gas limits
2. **RPC Issues**: Ensure your RPC URL is working
3. **Verification Failures**: Check your Etherscan API key

### **Support**

- **Documentation**: https://docs.1inch.io/docs/limit-order-protocol/
- **Audit Reports**: https://github.com/1inch/1inch-audits
- **GitHub**: https://github.com/1inch/limit-order-protocol

---

**Deployment Address**: Will be output after successful deployment
**Version**: 4.3.2 (Production-ready)
**Status**: ✅ Audited and Secure 