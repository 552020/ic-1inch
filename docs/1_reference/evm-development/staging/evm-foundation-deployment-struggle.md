# EVM Foundation Deployment Struggle

## 🎯 **Objective**

Deploy the EscrowFactory contract to Base Sepolia testnet as part of the EVM foundation setup for the cross-chain swap implementation.

## 📋 **What We Accomplished**

### ✅ **Phase 1: Environment Setup**

- Successfully added `eth/` as git submodule (cross-chain-swap repository)
- Installed Foundry dependencies with `forge install`
- Created `.env` file with private keys and RPC URL
- Set up configuration files (`scripts/mechanical-turk/evm-test-config.json`)
- Made `create_order.sh` script executable

### ✅ **Phase 2: Configuration**

- Copied testing script to `scripts/mechanical-turk/create_order.sh`
- Copied test config to `scripts/mechanical-turk/evm-test-config.json`
- Updated config with actual addresses:
  - Deployer: `0x8CB80b37cc7193D0f055b1189F25eB903D888D3A`
  - Maker: `0x086153956EF36381bca361575EF7eF67886052A5`
  - Resolver: `0x086153956EF36381bca361575EF7eF67886052A5`

## 🚫 **What Failed**

### **Issue 1: Missing DEPLOYER_ADDRESS Environment Variable**

- **Problem**: `DeployEscrowFactory.s.sol` script requires `DEPLOYER_ADDRESS` env var
- **Error**: `vm.envAddress: environment variable "DEPLOYER_ADDRESS" not found`
- **Solution**: Added `DEPLOYER_ADDRESS=0x8CB80b37cc7193D0f055b1189F25eB903D888D3A` to `.env`

### **Issue 2: OutOfGas Errors**

- **Problem**: Deployment consistently failed with `EvmError: OutOfGas`
- **Attempts**:
  - Increased gas limit to 5M → Failed
  - Increased gas limit to 10M → Failed
  - Confirmed sufficient test ETH balance
  - Verified RPC URL functionality

### **Issue 3: CREATE3 Address Conflict**

- **Problem**: Script tries to deploy to hardcoded address `0x65B3Db8bAeF0215A1F9B14c506D2a3078b2C84AE`
- **Root Cause**: This address is already deployed on Base mainnet
- **Error**: `EvmError: Revert` with `<empty revert data>`
- **Analysis**: The deployment script is designed for Base mainnet, not Base Sepolia

## 🔍 **Technical Analysis**

### **Deployment Script Issues**

```solidity
// DeployEscrowFactory.s.sol
address deployer = vm.envAddress("DEPLOYER_ADDRESS");
// ... tries to deploy to hardcoded CREATE3 address
```

### **CREATE3 Deployment Problem**

- The script uses deterministic deployment via CREATE3
- Target address `0x65B3Db8bAeF0215A1F9B14c506D2a3078b2C84AE` is already occupied
- Base Sepolia needs a different deployment strategy

## 🛠️ **Attempted Solutions**

### **Solution 1: Fix Environment Variables**

- ✅ Added `DEPLOYER_ADDRESS` to `.env`
- ✅ Verified address derivation from private key

### **Solution 2: Increase Gas Limits**

- ❌ Increased to 5M gas → Still failed
- ❌ Increased to 10M gas → Still failed
- ❌ Confirmed sufficient ETH balance

### **Solution 3: Use Local Anvil Fork**

- ✅ Successfully started anvil fork on port 8545
- ❌ Haven't tested deployment to local fork yet

## 🎯 **Next Steps**

### **Option A: Deploy to Local Anvil Fork (Recommended)**

```bash
forge script script/DeployEscrowFactory.s.sol --rpc-url http://localhost:8545 --broadcast
```

### **Option B: Create Base Sepolia Specific Script**

- Modify deployment script for Base Sepolia
- Use different CREATE3 salt/address
- Test on Base Sepolia testnet

### **Option C: Use Existing Deployment**

- Check if there's already a Base Sepolia deployment
- Update config to use existing addresses

## 📊 **Current Status**

- ✅ **Environment**: Fully configured
- ✅ **Dependencies**: Installed
- ✅ **Configuration**: Complete
- ❌ **Deployment**: Failed due to CREATE3 address conflict
- 🔄 **Next**: Need to choose deployment strategy

## 🔗 **Related Files**

- `eth/.env` - Environment configuration
- `eth/script/DeployEscrowFactory.s.sol` - Deployment script
- `scripts/mechanical-turk/evm-test-config.json` - Test configuration
- `docs/cross-turk/using-repo-as-evm-foundation-todo.md` - Original TODO

## 💡 **Lessons Learned**

1. **CREATE3 deployments are deterministic** - Same parameters = same address
2. **Base mainnet vs Base Sepolia** - Need different deployment strategies
3. **Environment variables** - Must match script requirements exactly
4. **Gas estimation** - Complex contracts need careful gas calculation
5. **Local testing first** - Anvil fork provides safe testing environment

## 🎯 **Recommendation**

Start with **Option A** (local anvil fork) to verify the deployment process works, then create a Base Sepolia specific deployment strategy.
