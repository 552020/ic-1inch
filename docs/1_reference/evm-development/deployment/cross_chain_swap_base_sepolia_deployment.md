# Cross-Chain Swap - Base Sepolia Deployment Guide

## 🎯 **Overview**

This guide will help you deploy the **1inch Cross-Chain Swap** contracts to Base Sepolia testnet. This deployment will complete the EVM foundation for your Fusion+ protocol, working alongside your already deployed Limit Order Protocol.

## 📊 **Deployment Architecture**

```
Limit Order Protocol (Base Sepolia)
           ↓
Cross-Chain Swap Contracts (Base Sepolia)
           ↓
Fusion+ (EVM ↔ ICP Bridge)
```

## 🏗️ **Contracts to Deploy**

### **Main Contracts**
1. **`EscrowFactory`** - Factory contract for creating escrow instances
2. **`EscrowSrc`** - Source chain escrow logic
3. **`EscrowDst`** - Destination chain escrow logic

### **Integration Points**
- **LOP Address**: `0xdfC365795F146a6755998C5e916a592A9706eDC6` (your deployed LOP)
- **Network**: Base Sepolia (Chain ID: 84532)
- **Fee Token**: DAI (`0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb`)

---

## 🚀 **Deployment Steps**

### **1. Environment Setup**

Create a `.env` file in the project root:

```bash
# Required for deployment
DEPLOYER_ADDRESS=your_deployer_address_here
PRIVATE_KEY=your_private_key_here

# Optional for verification
ETHERSCAN_API_KEY=your_etherscan_api_key
```

### **2. Install Dependencies**

```bash
# Install Foundry (if not already installed)
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Install project dependencies
forge install
```

### **3. Build Contracts**

```bash
# Build all contracts
forge build
```

### **4. Deploy to Base Sepolia**

```bash
# Deploy EscrowFactory
forge script script/DeployEscrowFactory.s.sol --rpc-url https://sepolia.base.org --broadcast --verify
```

### **5. Verify Deployment**

After deployment, verify your contract:
- **Block Explorer**: https://sepolia.basescan.org/
- **Contract Address**: Will be output after deployment

---

## 🔧 **Configuration Details**

### **Deployment Script Analysis**

The `DeployEscrowFactory.s.sol` script includes:

```solidity
// Base Sepolia Configuration
FEE_TOKEN[8453] = 0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb; // Base (DAI)

// Constants
LOP = 0x111111125421cA6dc452d289314280a0f8842A65; // All chains
ACCESS_TOKEN = 0xACCe550000159e70908C0499a1119D04e7039C28; // All chains
CREATE3_DEPLOYER = 0x65B3Db8bAeF0215A1F9B14c506D2a3078b2C84AE; // All chains
```

### **Constructor Parameters**

The `EscrowFactory` constructor takes:
- **`LOP`**: Limit Order Protocol address
- **`feeToken`**: DAI address on Base Sepolia
- **`ACCESS_TOKEN`**: Access control token
- **`feeBankOwner`**: Fee collection address
- **`RESCUE_DELAY`**: 8 days (691200 seconds)

---

## 📋 **Integration with LOP**

### **Configuration Update**

After deployment, update your configuration to use the deployed contracts:

```json
{
  "escrowFactory": "YOUR_DEPLOYED_ESCROW_FACTORY_ADDRESS",
  "limitOrderProtocol": "0xdfC365795F146a6755998C5e916a592A9706eDC6",
  "deployer": "YOUR_DEPLOYER_ADDRESS",
  "maker": "YOUR_MAKER_ADDRESS",
  "srcToken": "0x4200000000000000000000000000000000000006", // WETH
  "dstToken": "0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb", // DAI
  "resolver": "YOUR_RESOLVER_ADDRESS",
  "srcAmount": 1000000000000000000, // 1 WETH
  "dstAmount": 2000000000000000000, // 2 DAI
  "safetyDeposit": 100000000000000000, // 0.1 ETH
  "secret": "your_secret_here",
  "stages": [
    "deployEscrowSrc",
    "deployEscrowDst",
    "withdrawSrc",
    "withdrawDst"
  ]
}
```

---

## 🧪 **Testing Deployment**

### **Local Testing**

```bash
# Test on local Anvil
anvil
forge script script/DeployEscrowFactory.s.sol --rpc-url http://localhost:8545 --broadcast
```

### **Integration Testing**

```bash
# Run the example script
cd examples
chmod +x scripts/create_order.sh
./scripts/create_order.sh
```

---

## 🔍 **Verification & Monitoring**

### **Contract Verification**

```bash
# Verify on Base Sepolia
forge verify-contract YOUR_CONTRACT_ADDRESS \
  --chain-id 84532 \
  --etherscan-api-key YOUR_API_KEY \
  --constructor-args $(cast abi-encode "constructor(address,address,address,address,uint32,uint32)" \
    "0xdfC365795F146a6755998C5e916a592A9706eDC6" \
    "0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb" \
    "0xACCe550000159e70908C0499a1119D04e7039C28" \
    "YOUR_DEPLOYER_ADDRESS" \
    "691200" \
    "691200")
```

### **Monitoring**

- **Block Explorer**: https://sepolia.basescan.org/
- **RPC Endpoint**: https://sepolia.base.org/
- **Faucet**: https://www.coinbase.com/faucets/base-ethereum-sepolia-faucet

---

## 🆘 **Troubleshooting**

### **Common Issues**

1. **Insufficient Funds**
   ```bash
   # Get testnet ETH from faucet
   # https://www.coinbase.com/faucets/base-ethereum-sepolia-faucet
   ```

2. **Network Issues**
   ```bash
   # Check RPC connectivity
   curl -X POST https://sepolia.base.org \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
   ```

3. **Deployment Failures**
   ```bash
   # Check gas estimation
   forge script script/DeployEscrowFactory.s.sol --rpc-url https://sepolia.base.org --dry-run
   ```

### **Debug Commands**

```bash
# Check contract compilation
forge build --force

# Check deployment script
forge script script/DeployEscrowFactory.s.sol --rpc-url https://sepolia.base.org --dry-run -vvv

# Check environment variables
echo $DEPLOYER_ADDRESS
echo $PRIVATE_KEY
```

---

## 📚 **Next Steps**

### **After Successful Deployment**

1. **Update Configuration**
   - Add deployed contract addresses to your config
   - Configure integration with LOP

2. **Test Integration**
   - Test order creation and execution
   - Verify cross-chain functionality

3. **Prepare for ICP Integration**
   - Document contract addresses
   - Plan cross-chain communication

### **Production Considerations**

- **Mainnet Deployment**: Use Base mainnet addresses
- **Security Audits**: Ensure contracts are audited
- **Monitoring**: Set up alerts and monitoring
- **Documentation**: Update all documentation

---

## 📝 **Notes**

### **Security Considerations**

- ✅ **Testnet Deployment**: Safe for development
- ✅ **Access Control**: Proper access token configuration
- ✅ **Timelocks**: 8-day rescue delay for safety
- ✅ **Fee Management**: DAI-based fee collection

### **Gas Optimization**

- **CREATE3 Deployment**: Deterministic addresses
- **Optimized Contracts**: Gas-efficient implementation
- **Batch Operations**: Support for multiple operations

### **Compatibility**

- **EVM Compatible**: Works with all EVM chains
- **Token Standards**: Supports ERC-20, ERC-721, ERC-1155
- **Cross-Chain**: Designed for multi-chain operations

---

**Last Updated**: August 2, 2024  
**Status**: Ready for Deployment  
**Network**: Base Sepolia (Chain ID: 84532) 