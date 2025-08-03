# Using Repository as EVM Foundation - Deployment Guide

## 🎯 **Overview**

This document provides a comprehensive guide for deploying the 1inch Fusion atomic swap contracts as the EVM foundation for your ICP<>EVM Fusion+ implementation.

## ⚠️ **The Deployment Struggle**

### **The Original Problem**

The deployment script `DeployEscrowFactory.s.sol` was trying to deploy to a **specific address** (`0x65B3Db8bAeF0215A1F9B14c506D2a3078b2C84AE`) that's already used on Base mainnet. This address is hardcoded in the script.

### **Why It Was Failing**

1. **CREATE3 Deployment Method**: The script uses `CREATE3` (a deterministic deployment method)
2. **Hardcoded Address**: It tries to deploy to the exact same address that's already deployed on Base mainnet
3. **Testnet Incompatibility**: On Base Sepolia, this address either doesn't exist or has different code, causing the revert

### **The Error You Encountered**

```bash
forge script script/DeployEscrowFactory.s.sol --rpc-url $BASE_SEPOLIA_RPC --broadcast
error: a value is required for '--fork-url <URL>' but none was supplied
```

This error occurred because the script was designed to work with a local Anvil fork that has access to mainnet addresses.

## 📋 **Deployment Options**

### **Option 1: Use Existing Base Mainnet Contract (Recommended for Production)**

The contracts are already deployed on Base mainnet:

- **EscrowFactory**: `0xfb742d35dd3a3ca8da4a79ac062064164845c6b9`
- **Chain ID**: 8453 (Base)
- **Status**: Production-ready

**Advantages:**

- ✅ No deployment needed
- ✅ Production-tested contracts
- ✅ Real 1inch integration
- ✅ CREATE3 deterministic deployment

**Usage:**

```json
{
  "escrowFactory": "0xfb742d35dd3a3ca8da4a79ac062064164845c6b9",
  "chainId": 8453,
  "rpcUrl": "https://mainnet.base.org"
}
```

> **⚠️ Note**: This option is for production use only. For testing, you'll need to use the original 1inch Fusion contracts on testnet, which requires either Option 2 (mock deployment) or Option 3/4 (local fork with real contracts).

### **Option 2: Deploy to Base Sepolia Testnet (Recommended for Testing)**

**This is the solution to the original problem!** Use the custom deployment script I created specifically to solve the CREATE3 deployment issues:

```bash
# Deploy with mock contracts (no 1inch dependency)
forge script script/DeployEscrowFactoryBaseSepolia.s.sol --rpc-url $BASE_SEPOLIA_RPC --broadcast
```

**What this script does (and why it solves the problem):**

- ✅ **No CREATE3**: Deploys `EscrowFactory` directly (avoids the hardcoded address issue)
- ✅ **Mock Tokens**: Creates mock DAI and Access tokens (no dependency on 1inch contracts)
- ✅ **No Hardcoded Addresses**: Uses deployer address as mock Limit Order Protocol
- ✅ **Standard Deployment**: Uses normal contract deployment instead of CREATE3
- ✅ **Testnet Compatible**: Works on any testnet without mainnet dependencies

**Output:**

```
=== Base Sepolia Deployment (Mock Mode) ===
EscrowFactory deployed at: 0x...
EscrowSrc implementation: 0x...
EscrowDst implementation: 0x...
Proxy Src bytecode hash: 0x...
Proxy Dst bytecode hash: 0x...
Mock Fee token (DAI): 0x...
Mock Access token: 0x...
Mock Limit Order Protocol: 0x...
Owner: 0x...
Rescue delay: 691200
==========================================
```

**Advantages:**

- ✅ Works on Base Sepolia testnet
- ✅ No dependency on 1inch contracts
- ✅ Perfect for testing core functionality
- ✅ Easy to upgrade to real contracts later

### **Option 3: Deploy to Local Fork (Best for Development)**

**What is a Local Fork?**

A local fork is like creating a **copy of mainnet** on your computer. Think of it as:

- **Mainnet**: The real Ethereum network with real contracts and real data
- **Local Fork**: A perfect copy of mainnet running on your local machine
- **Anvil**: The tool that creates this local copy (similar to Hardhat Network)

**Comparison with Hardhat:**

| Tool                | Purpose                   | Network Type                |
| ------------------- | ------------------------- | --------------------------- |
| **Hardhat Network** | Local development network | Fresh, empty blockchain     |
| **Anvil Fork**      | Local copy of mainnet     | Exact copy of mainnet state |

**Key Difference:**

- **Hardhat Network**: Starts empty, you deploy everything yourself
- **Anvil Fork**: Starts with all mainnet contracts already deployed (like 1inch contracts)

**Why We Need It:**

The original script uses real mainnet addresses (like 1inch contracts). A local fork gives us access to these real contracts without needing to deploy to actual mainnet.

**How It Works:**

```bash
# This command creates a local copy of mainnet
anvil --fork-url $MAINNET_RPC --chain-id 31337 --port 8545

# What happens:
# 1. Downloads the latest mainnet state
# 2. Creates a local blockchain with the same data
# 3. All mainnet contracts are now available locally
# 4. You can interact with them as if they were real
```

**⚠️ CRITICAL STEP: Fund Your Deployer Address**

Before deploying, you must fund your deployer address on the anvil fork:

```bash
# Fund the deployer address with ETH (required for gas fees)
cast send --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 YOUR_DEPLOYER_ADDRESS --value 10000000000000000000
```

**What this does:**

- Uses anvil's default account (which has unlimited ETH)
- Sends 10 ETH to your deployer address
- Uses the default anvil private key: `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80`

**Why this is needed:**

- Anvil fork starts with mainnet state but your address has no ETH
- The deployment needs ~0.0025 ETH for gas fees
- Without funding, you'll get "Insufficient funds for gas" error

**Then deploy using the fork:**

```bash
# Deploy using fork (uses real 1inch contracts)
forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast --sender YOUR_DEPLOYER_ADDRESS --private-key YOUR_PRIVATE_KEY
```

**Advantages:**

- ✅ **Real 1inch contracts** from mainnet (solves the missing contract issue)
- ✅ **Full functionality testing** (tests complete integration)
- ✅ **Fast local development** (no gas costs)
- ✅ **No testnet gas costs** (free testing)
- ✅ **Addresses the original error** (provides the fork that was missing)
- ✅ **Perfect copy of mainnet** (all real contracts available)

### **Option 4: Use Original Script on Fork**

**This is the original approach that was failing:** Now it works because we provide the required fork.

**Why This Failed Before:**

The original script was designed to work with mainnet addresses. When you tried to run it on Base Sepolia, it failed because:

1. The required addresses don't exist on Base Sepolia
2. The script expected a fork but you didn't provide one

**How the Fork Fixes It:**

```bash
# Step 1: Create a local copy of mainnet (this gives us the missing addresses)
anvil --fork-url $MAINNET_RPC --chain-id 31337 --port 8545

# Step 2: Now the original script works because it has access to mainnet addresses
forge script script/DeployEscrowFactory.s.sol --rpc-url http://localhost:8545 --broadcast
```

**What the Fork Provides:**

- ✅ **All mainnet contracts** (including the missing CREATE3_DEPLOYER)
- ✅ **Real 1inch addresses** (the ones that don't exist on testnets)
- ✅ **Exact mainnet state** (perfect for testing production scenarios)

**Advantages:**

- ✅ **Original CREATE3 deployment** (exact same as production)
- ✅ **Exact same as production** (tests the real deployment method)
- ✅ **Real mainnet addresses** (uses the actual 1inch contracts)
- ✅ **Now works** (because we provide the required fork)

## 🔧 **Environment Setup**

### **Required Environment Variables**

Create a `.env` file:

```bash
# For Base Sepolia deployment
DEPLOYER_ADDRESS=0xYourAddress
BASE_SEPOLIA_RPC=https://base-sepolia.g.alchemy.com/v2/YOUR_KEY

# For local fork
MAINNET_RPC=https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY

# Private keys (for testing only)
DEPLOYER_PRIVATE_KEY=0xYourPrivateKey
MAKER_PRIVATE_KEY=0xYourPrivateKey
```

### **Required Dependencies**

```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
source ~/.zshenv
foundryup

# Install dependencies
forge install
```

## 📊 **Deployment Comparison**

| Option       | Network      | 1inch Integration | Gas Cost | Complexity | Use Case    |
| ------------ | ------------ | ----------------- | -------- | ---------- | ----------- |
| **Option 1** | Base Mainnet | ✅ Real           | High     | Low        | Production  |
| **Option 2** | Base Sepolia | ❌ Mock           | Low      | Medium     | Testing     |
| **Option 3** | Local Fork   | ✅ Real           | None     | Medium     | Development |
| **Option 4** | Local Fork   | ✅ Real           | None     | High       | Development |

## 🚀 **Recommended Deployment Strategy**

### **Phase 1: Development & Testing**

```bash
# Use Option 2 for Base Sepolia testing
forge script script/DeployEscrowFactoryBaseSepolia.s.sol --rpc-url $BASE_SEPOLIA_RPC --broadcast
```

### **Phase 2: Integration Testing**

```bash
# Use Option 3 for full functionality testing
anvil --fork-url $MAINNET_RPC --chain-id 31337 --port 8545
forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast
```

### **Phase 3: Production**

```json
{
  "escrowFactory": "0xfb742d35dd3a3ca8da4a79ac062064164845c6b9",
  "chainId": 8453
}
```

## 🔄 **Upgrade Path**

### **From Mock to Real Contracts**

When ready for production, replace mock addresses:

```solidity
// In DeployEscrowFactoryBaseSepolia.s.sol
// Replace these lines:
address limitOrderProtocol = deployer;  // Mock
ERC20 mockFeeToken = new ERC20("Mock DAI", "DAI");  // Mock
ERC20 mockAccessToken = new ERC20("Mock Access", "ACC");  // Mock

// With real addresses:
address public constant LOP = 0x111111125421cA6dc452d289314280a0f8842A65;
address public constant FEE_TOKEN = 0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb;
address public constant ACCESS_TOKEN = 0xACCe550000159e70908C0499a1119D04e7039C28;
```

## 🧪 **Testing Your Deployment**

### **1. Update Config**

After deployment, update your config:

```json
{
  "escrowFactory": "0x...", // From deployment output
  "limitOrderProtocol": "0x111111125421cA6dc452d289314280a0f8842A65",
  "srcToken": "0x4200000000000000000000000000000000000006", // WETH
  "dstToken": "0x0000000000000000000000000000000000000000", // ETH
  "srcAmount": 1000000000000000000, // 1 WETH
  "dstAmount": 1000000000000000000, // 1 ETH
  "stages": ["deployMocks", "deployEscrowSrc", "deployEscrowDst"]
}
```

### **2. Test Escrow Creation**

```bash
# Run the testing script
./scripts/mechanical-turk/create_order.sh
```

### **3. Verify Events**

Check that escrow creation events are emitted:

```solidity
event SrcEscrowCreated(IBaseEscrow.Immutables srcImmutables, DstImmutablesComplement dstImmutablesComplement);
event DstEscrowCreated(address escrow, bytes32 hashlock, Address taker);
```

## ⚠️ **Important Considerations**

### **CREATE3 Deployment Issues (The Root Cause)**

The original deployment script uses CREATE3 which:

- **Requires specific addresses** that exist on mainnet (`0x65B3Db8bAeF0215A1F9B14c506D2a3078b2C84AE`)
- **Fails on testnets** where those addresses don't exist or have different code
- **Uses deterministic deployment** for predictable addresses (but only works on mainnet)
- **Causes the original error** you encountered when trying to deploy to Base Sepolia

### **Why the Original Script Failed**

```solidity
// From the original DeployEscrowFactory.s.sol
ICreate3Deployer public constant CREATE3_DEPLOYER = ICreate3Deployer(0x65B3Db8bAeF0215A1F9B14c506D2a3078b2C84AE);

// This address exists on mainnet but not on Base Sepolia
// When you tried to deploy to Base Sepolia, it failed because:
// 1. The address doesn't exist on Base Sepolia
// 2. Or it exists but has different code
// 3. The CREATE3 deployment method requires this exact address
```

### **Mock vs Real Contracts**

**Mock Contracts (Option 2) - The Solution:**

- ✅ **Work on any testnet** (solves the CREATE3 issue)
- ✅ **No external dependencies** (no 1inch contract requirements)
- ✅ **Perfect for testing core functionality** (escrow creation, withdrawals, cancellations)
- ✅ **Solves the original problem** (no hardcoded addresses)
- ❌ **Don't test real 1inch integration** (but that's not needed for core testing)

**Real Contracts (Options 1, 3, 4) - The Original Approach:**

- ✅ **Full 1inch integration** (when working with mainnet)
- ✅ **Production-like behavior** (exact same as production)
- ✅ **Test complete functionality** (including 1inch integration)
- ❌ **Require mainnet addresses or forking** (this was the original problem)
- ❌ **Don't work on testnets** (without forking)

### **Gas Costs**

| Network      | Approximate Gas Cost |
| ------------ | -------------------- |
| Base Mainnet | ~0.001 ETH           |
| Base Sepolia | ~0.0001 ETH          |
| Local Fork   | 0 ETH                |

## 🔍 **Troubleshooting**

### **Common Issues (Including the Original Problem)**

1. **"CREATE3_DEPLOYER not found"** (This was your original issue!)

   - **Root Cause**: The hardcoded address `0x65B3Db8bAeF0215A1F9B14c506D2a3078b2C84AE` doesn't exist on testnets
   - **Solution**: Use Option 2 (mock deployment) for testnets
   - **Alternative**: Use Option 3 or 4 (fork) for local testing

2. **"a value is required for '--fork-url <URL>'"** (This was your exact error!)

   - **Root Cause**: The original script requires a fork but you didn't provide one
   - **What is a fork?**: A local copy of mainnet that provides access to real contracts
   - **Solution**: Use Option 2 (no fork needed) or Option 3/4 (with fork)
   - **Command**: `anvil --fork-url $MAINNET_RPC --chain-id 31337 --port 8545`
   - **Why this works**: The fork gives you access to mainnet addresses that don't exist on testnets

3. **"1inch contracts not found"**

   - **Root Cause**: 1inch contracts aren't deployed on testnets
   - **Solution**: Use Option 2 (mock deployment)
   - **Alternative**: Use Option 3/4 with mainnet fork

4. **"Insufficient gas"**

   - **Solution**: Increase gas limit in deployment command
   - **Command**: `--gas-limit 5000000`

5. **"Private key not found"**

   - **Solution**: Ensure `.env` file exists with correct variables
   - **Check**: That private keys are properly formatted

### **Verification Commands**

```bash
# Check if contract is deployed
cast call 0x... "ESCROW_SRC_IMPLEMENTATION()" --rpc-url $RPC_URL

# Check implementation addresses
cast call 0x... "ESCROW_DST_IMPLEMENTATION()" --rpc-url $RPC_URL

# Verify on Etherscan
# Base Sepolia: https://sepolia.basescan.org/address/0x...
# Base Mainnet: https://basescan.org/address/0x...
```

## 📚 **Next Steps**

After successful deployment:

1. **Update your relayer configuration** with new contract addresses
2. **Test escrow creation** with the testing scripts
3. **Integrate with your ICP components**
4. **Deploy to production** using the mainnet contract address

## 🎯 **Summary**

For your ICP<>EVM Fusion+ implementation:

- **Development**: Use Option 2 (Base Sepolia with mocks)
- **Testing**: Use Option 3 (Local fork with real contracts)
- **Production**: Use Option 1 (Existing Base mainnet contract)

This approach gives you a complete testing and deployment pipeline while maintaining compatibility with the existing 1inch infrastructure.

## APPENDIX I

```bash
 cd eth && source .env && forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast
[⠊] Compiling...
[⠃] Compiling 86 files with Solc 0.8.23
[⠊] Solc 0.8.23 finished in 24.67s
Compiler run successful!
Script ran successfully.

== Logs ==
  === Local Fork Deployment ===
  EscrowFactory deployed at: 0x5b73C5498c1E3b4dbA84de0F1833c4a029d90519
  EscrowSrc implementation: 0xdaE97900D4B184c5D2012dcdB658c008966466DD
  EscrowDst implementation: 0x238213078DbD09f2D15F4c14c02300FA1b2A81BB
  Fee token (DAI): 0x6B175474E89094C44Da98b954EedeAC495271d0F
  Access token: 0xACCe550000159e70908C0499a1119D04e7039C28
  Limit Order Protocol: 0x111111125421cA6dc452d289314280a0f8842A65
  Owner: 0x8CB80b37cc7193D0f055b1189F25eB903D888D3A
  =============================

## Setting up 1 EVM.

==========================

Chain 31337

Estimated gas price: 0.458200113 gwei

Estimated total gas used for script: 5561585

Estimated amount required: 0.002548318875459105 ETH

==========================

Transactions saved to: /Users/stefano/Documents/Code/Unite_DeFi/ic-1inch/eth/broadcast/DeployEscrowFactoryLocal.s.sol/31337/run-latest.json

Sensitive values saved to: /Users/stefano/Documents/Code/Unite_DeFi/ic-1inch/eth/cache/DeployEscrowFactoryLocal.s.sol/31337/run-latest.json

Error: You seem to be using Foundry's default sender. Be sure to set your own --sender.
╭─stefano@kurt ~/Documents/Code/Unite_DeFi/ic-1inch/eth  ‹master*›
╰─➤                                                                                 1 ↵

## 🔧 **Fixing the Sender Error**

### **What is the "sender"?**

The **sender** is the Ethereum address that will be used to send transactions. In Foundry, this is the address that pays for gas and signs transactions.

### **The Error:**

```

Error: You seem to be using Foundry's default sender. Be sure to set your own --sender.

````

This means Foundry is using a **default address** instead of your specified deployer address.

### **The Fix:**

Add the `--sender` flag to specify your deployer address:

```bash
cd eth && source .env && forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast --sender YOUR_DEPLOYER_ADDRESS
````

### **Where to find the sender:**

The sender address is already in your `.env` file:

```bash
DEPLOYER_ADDRESS=YOUR_DEPLOYER_ADDRESS
```

### **Why this happens:**

1. **Foundry default**: Uses a hardcoded default address when no `--sender` is specified
2. **Your script**: Reads `DEPLOYER_ADDRESS` from environment but doesn't pass it as `--sender`
3. **The fix**: Explicitly tell Foundry which address to use for transactions

### **Updated command:**

```bash
cd eth && source .env && forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast --sender YOUR_DEPLOYER_ADDRESS
```

This will use your deployer address as the sender for all transactions, eliminating the warning.

## ⚠️ **Important Note: Private Key Required**

When using the `--sender` flag, you also need to provide the private key for that address. The corrected command is:

```bash
cd eth && source .env && forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast --sender YOUR_DEPLOYER_ADDRESS --private-key YOUR_PRIVATE_KEY
```

**Alternative using environment variable:**

```bash
cd eth && source .env && forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast --account YOUR_DEPLOYER_ADDRESS
```

**Why this is needed:**

- `--sender` only specifies which address to use
- Foundry still needs the private key to sign transactions
- The private key can be provided via `--private-key` flag or `--account` flag (which uses environment variables)

```

cast send --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 YOUR_DEPLOYER_ADDRESS --value 10000000000000000000

blockHash            0x1cd66102da27349995ae490f215e873a76ce1b48f15f854476e6bce01185756a
blockNumber          23043872
contractAddress
cumulativeGasUsed    21000
effectiveGasPrice    239791686
from                 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
gasUsed              21000
logs                 []
logsBloom            0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
root
status               1 (success)
transactionHash      0x9a4a43b335629baedf48042b41c589f22e7bb48f4ecf0fc2d6074650deb4357d
transactionIndex     0
type                 2
blobGasPrice         1
blobGasUsed
to                   0x8CB80b37cc7193D0f055b1189F25eB903D888D3A
╭─stefano@kurt ~/Docum

```

## APPENDIX II - SUCCESS

source .env && forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast --sender YOUR_DEPLOYER_ADDRESS --private-key YOUR_PRIVATE_KEY
[⠒] Compiling...
No files changed, compilation skipped
Script ran successfully.

== Logs ==
=== Local Fork Deployment ===
EscrowFactory deployed at: 0xB469BeD842eA1760cC4b85087b7623a10289Ef2A
EscrowSrc implementation: 0x7935469dcbe01dAD2889df9806Cc4059A17BF42e
EscrowDst implementation: 0x683Cce7C0C7406d25E5F16D0Eb609a20Baa19b31
Fee token (DAI): 0x6B175474E89094C44Da98b954EedeAC495271d0F
Access token: 0xACCe550000159e70908C0499a1119D04e7039C28
Limit Order Protocol: 0x111111125421cA6dc452d289314280a0f8842A65
Owner: 0x8CB80b37cc7193D0f055b1189F25eB903D888D3A
=============================

## Setting up 1 EVM.

==========================

Chain 31337

Estimated gas price: 0.477621573 gwei

Estimated total gas used for script: 5561585

Estimated amount required: 0.002656332976073205 ETH

==========================

##### anvil-hardhat

✅ [Success] Hash: 0xc132f316fc5a6a9361000a52c9032e9e5c3e7a556bc3d6f047f4963eb560c571
Contract Address: 0xB469BeD842eA1760cC4b85087b7623a10289Ef2A
Block: 23043873
Paid: 0.000898798044735671 ETH (4278143 gas \* 0.210090697 gwei)

✅ Sequence #1 on anvil-hardhat | Total Paid: 0.000898798044735671 ETH (4278143 gas \* avg 0.210090697 gwei)

==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.

Transactions saved to: /Users/stefano/Documents/Code/Unite_DeFi/ic-1inch/eth/broadcast/DeployEscrowFactoryLocal.s.sol/31337/run-latest.json

Sensitive values saved to: /Users/stefano/Documents/Code/Unite_DeFi/ic-1inch/eth/cache/DeployEscrowFactoryLocal.s.sol/31337/run-latest.json

╭─stefano@kurt ~/Documents/Code/Unite_DeFi/ic-1inch/eth ‹master\*›
╰─➤
