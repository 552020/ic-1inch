# Limit Order Protocol (LOP) - Base Sepolia Deployment Guide

## Overview

This document describes the process of deploying the Limit Order Protocol (LOP) to Base Sepolia testnet for integration with the Cross-Chain Swap contracts.

## Prerequisites

- Node.js and npm/pnpm installed
- A wallet with ETH on Base Sepolia testnet
- Access to the limit-order-protocol repository

## Deployment Steps

### 1. Repository Setup

```bash
# Navigate to the limit-order-protocol repository
cd /Users/stefano/Documents/Code/Unite_DeFi/ic-1inch/internal/1inch/repos/limit-order-protocol

# Install dependencies
pnpm install
```

### 2. Environment Configuration

Create a `.env` file in the repository root:

```bash
# Create .env file
echo "PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" > .env
echo "DEPLOYER_ADDRESS=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266" >> .env
```

**Important:** Make sure the private key and deployer address are correctly mapped:

- `PRIVATE_KEY`: The private key of your funded wallet
- `DEPLOYER_ADDRESS`: The public address of your funded wallet

### 3. Hardhat Configuration Updates

#### 3.1 Add Base Sepolia Network

Edit `hardhat.config.js` to include Base Sepolia network:

```javascript
// Add Base Sepolia network manually
networks["base-sepolia"] = {
  chainId: 84532,
  url: "https://sepolia.base.org",
  accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
  verify: {
    etherscan: {
      apiUrl: "https://api-sepolia.basescan.org",
    },
  },
};
```

#### 3.2 Update Deployment Script

Edit `deploy/deploy.js`:

1. **Remove skip flag:**

```javascript
// Change from:
module.exports.skip = async () => true;
// To:
module.exports.skip = async () => false;
```

2. **Add Base Sepolia WETH address:**

```javascript
const wethByNetwork = {
  hardhat: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
  mainnet: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
  "base-sepolia": "0x4200000000000000000000000000000000000006", // WETH on Base Sepolia
};
```

3. **Add debugging logs:**

```javascript
console.log("network name:", network.name);
console.log("wethByNetwork keys:", Object.keys(wethByNetwork));
console.log("wethByNetwork[network.name]:", wethByNetwork[network.name]);
```

### 4. Fund Your Wallet

Ensure your deployer wallet has ETH on Base Sepolia:

- **Faucet:** https://www.coinbase.com/faucets/base-ethereum-sepolia-faucet
- **Required:** At least 0.01 ETH for deployment

### 5. Deploy the Contract

```bash
# Deploy to Base Sepolia
npx hardhat deploy --network base-sepolia --reset
```

### 6. Verify Deployment

- **Block Explorer:** https://sepolia.basescan.org/address/[CONTRACT_ADDRESS]
- **Expected Output:** Contract deployed successfully with address

## Issues Encountered and Solutions

### Issue 1: Missing Dependencies

**Error:** `Cannot find module '@matterlabs/hardhat-zksync-deploy'`
**Solution:** Run `pnpm install` to install all dependencies

### Issue 2: Network Not Registered

**Error:** `Network 'base-sepolia' not registered`
**Solution:** Manually add Base Sepolia network configuration to `hardhat.config.js`

### Issue 3: Deployment Skipped

**Error:** No deployment output after compilation
**Solution:** Change `module.exports.skip = async () => true;` to `false` in `deploy/deploy.js`

### Issue 4: Invalid WETH Address

**Error:** `Error: invalid address (argument="_weth", value=undefined...)`
**Solution:** Add Base Sepolia WETH address to `wethByNetwork` mapping

### Issue 5: Insufficient Funds

**Error:** `insufficient funds for intrinsic transaction cost`
**Solution:** Fund the deployer wallet with ETH from Base Sepolia faucet

### Issue 6: Contract Verification Error

**Error:** `MissingApiKeyError: You are trying to verify a contract in 'baseSepolia'`
**Solution:** This is optional - deployment succeeds even if verification fails

## Deployment Results

### Successful Deployment

- **Contract Address:** `0xdfC365795F146a6755998C5e916a592A9706eDC6`
- **Network:** Base Sepolia (Chain ID: 84532)
- **Deployer:** `0x8CB80b37...03D888D3A`
- **Status:** Active and ready for integration

### Block Explorer Details

- **URL:** https://sepolia.basescan.org/address/0xdfC365795F146a6755998C5e916a592A9706eDC6
- **Balance:** 0 ETH (expected for this contract type)
- **Creation:** 3 minutes ago
- **Verification:** Not verified (optional for testing)

## Next Steps

1. Use this LOP address in the Cross-Chain Swap deployment
2. Deploy Cross-Chain Swap contracts to Base Sepolia
3. Test the integration between LOP and Cross-Chain Swap

## Important Notes

### Repository Choice

**Current Approach:** Working directly in the limit-order-protocol repository
**Better Approach:** Fork the repository first for better version control

### Environment Variables

- Keep private keys secure
- Use environment variables for sensitive data
- Consider using `.env.example` for documentation

### Network Configuration

- Base Sepolia uses different RPC endpoints than mainnet
- WETH addresses vary by network
- Always verify network-specific configurations

## Troubleshooting

### Common Issues

1. **Wrong private key/address mapping:** Double-check `.env` file
2. **Insufficient funds:** Use faucet to get testnet ETH
3. **Network configuration:** Verify `hardhat.config.js` settings
4. **Dependencies:** Run `pnpm install` if modules are missing

### Verification (Optional)

To verify the contract on Base Sepolia:

1. Get API key from https://basescan.org/
2. Add to `hardhat.config.js`:

```javascript
etherscan: {
  apiKey: {
    baseSepolia: "your-api-key";
  }
}
```

## Resources

- **Base Sepolia Faucet:** https://www.coinbase.com/faucets/base-ethereum-sepolia-faucet
- **Base Sepolia Explorer:** https://sepolia.basescan.org/
- **Base Sepolia RPC:** https://sepolia.base.org/
- **WETH Address:** `0x4200000000000000000000000000000000000006`
