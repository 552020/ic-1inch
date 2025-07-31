# Ethereum Test Addresses

## Test Accounts (for Sepolia testnet)

### Maker Account
- **Address**: `0x742d35Cc6634C0532925a3b8D4C0532925a3b8D4`
- **Private Key**: `0x...` (Add your test private key)
- **Purpose**: Creates cross-chain orders (ICP → ETH or ETH → ICP)

### Resolver Account  
- **Address**: `0x8ba1f109551bD432803012645Hac189451b934`
- **Private Key**: `0x...` (Add your test private key)
- **Purpose**: Accepts and fulfills cross-chain orders

### Relayer Account
- **Address**: `0x...` (Add relayer address)
- **Private Key**: `0x...` (Add your test private key)
- **Purpose**: Coordinates cross-chain swaps, pays gas fees

## Contract Addresses (Sepolia)

### Fusion Escrow Contract
- **Address**: `0x...` (Will be populated after deployment)
- **Purpose**: Handles ETH-side escrow for atomic swaps

### Test Token Contracts
- **Mock ICP Token**: `0x...` (ERC-20 for testing)
- **Mock ETH Token**: Native ETH or WETH

## Setup Instructions

1. **Get Sepolia ETH**: Use faucet to fund test accounts
2. **Deploy contracts**: Run Hardhat deployment script
3. **Update addresses**: Fill in actual deployed contract addresses
4. **Test cross-chain**: Use mechanical turk test guide

## Security Notes

⚠️ **Never use real private keys or mainnet addresses for testing!**
