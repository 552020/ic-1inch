# Issue: Base Sepolia Deployment Failing

## Problem Description

The deployment script `DeployEscrowFactory.s.sol` is failing when trying to deploy to Base Sepolia testnet. The deployment reverts with empty revert data, preventing successful contract deployment.

## Root Cause Analysis

### 1. Missing Fee Token Configuration

- **Issue**: The deployment script originally didn't have a fee token configured for Base Sepolia (chain ID 84532)
- **Impact**: Deployment would fail because `FEE_TOKEN[block.chainid]` would return `address(0)`
- **Solution**: Added Base Sepolia fee token configuration

### 2. Incorrect Token Address

- **Issue**: Initially tried to use Base mainnet DAI address (`0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb`) on Base Sepolia
- **Impact**: Token doesn't exist on Base Sepolia, causing deployment to revert
- **Solution**: Switched to USDC address for Base Sepolia

### 3. Checksum Address Issues

- **Issue**: USDC address had invalid checksum format
- **Impact**: Compilation errors preventing deployment
- **Solution**: Fixed checksum format to `0x036CBD53842c5426634e7929541ec2318F3DCF7C`

## Current Status

After implementing the fixes above, the deployment still reverts with empty revert data. This suggests additional issues:

### Potential Remaining Issues

1. **Token Existence**: The USDC address `0x036CBD53842c5426634e7929541ec2318F3DCF7C` might not exist on Base Sepolia
2. **CREATE3 Deployer**: The CREATE3 deployer contract might not be deployed on Base Sepolia
3. **Gas Issues**: Deployment might require more gas than the default limit
4. **Contract Dependencies**: The EscrowFactory might have dependencies that don't exist on Base Sepolia

## Error Details

```
Error: script failed: <empty revert data>
```

## Steps to Reproduce

1. Set up environment with Base Sepolia RPC URL
2. Create keystore with private key
3. Run: `./scripts/deploy.sh base base-swap-maker`
4. Deployment reverts with empty data

## Proposed Solutions

### Option 1: Verify Token Address

- Verify if USDC address `0x036CBD53842c5426634e7929541ec2318F3DCF7C` exists on Base Sepolia
- Find correct USDC address for Base Sepolia if different

### Option 2: Use Different Testnet

- Deploy to Linea testnet instead (already configured)
- Deploy to Sonic testnet instead (already configured)

### Option 3: Investigate CREATE3 Deployer

- Check if CREATE3 deployer contract exists on Base Sepolia
- Deploy CREATE3 deployer if missing

### Option 4: Increase Gas Limit

- Try deployment with higher gas limit
- Check if deployment requires more gas than default

## Environment Details

- **Network**: Base Sepolia (Chain ID: 84532)
- **RPC URL**: `https://base-sepolia.g.alchemy.com/v2/QpubPhWfpIvpujMTF84v5`
- **Deployer Address**: `0x8CB80b37cc7193D0f055b1189F25eB903D888D3A`
- **Balance**: 0.22999 ETH (sufficient for gas)

## Files Modified

- `eth/script/DeployEscrowFactory.s.sol` - Added Base Sepolia fee token configuration

## Priority

**Medium** - This blocks deployment to Base Sepolia testnet, but alternative testnets are available.

## Next Steps

1. Verify USDC token address on Base Sepolia
2. Check CREATE3 deployer contract existence
3. Try deployment with increased gas limit
4. Consider deploying to alternative testnet as temporary solution
