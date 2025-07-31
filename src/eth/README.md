# Fusion+ Mechanical Turk - Ethereum Contracts

This directory contains the Ethereum smart contracts for the Fusion+ Mechanical Turk cross-chain atomic swap protocol.

## Overview

The Ethereum side of the Fusion+ Mechanical Turk implementation includes:

- **FusionEscrow.sol**: Main escrow contract for cross-chain atomic swaps
- **TestTokens.sol**: Test ERC-20 tokens for development and testing
- **Deployment Scripts**: Automated deployment and setup scripts
- **Tests**: Comprehensive test suite for all contract functionality

## Quick Start

### 1. Install Dependencies

```bash
npm install
```

### 2. Configure Environment

Copy the example environment file and configure your settings:

```bash
cp env.example .env
```

Edit `.env` with your configuration:
- `SEPOLIA_RPC_URL`: Your Sepolia RPC endpoint
- `PRIVATE_KEY`: Your deployment private key
- `ETHERSCAN_API_KEY`: Your Etherscan API key (optional)

### 3. Compile Contracts

```bash
npx hardhat compile
```

### 4. Run Tests

```bash
npx hardhat test
```

### 5. Deploy to Sepolia

```bash
npx hardhat run scripts/deploy.ts --network sepolia
```

### 6. Setup Test Environment

```bash
npx hardhat run scripts/setup-test-env.ts --network sepolia
```

## Contract Architecture

### FusionEscrow

The main escrow contract that handles cross-chain atomic swaps:

- **lockETHForSwap()**: Lock ETH for a swap with hashlock and timelock
- **claimLockedETH()**: Claim locked ETH with secret preimage and ICP receipt
- **refundLockedETH()**: Refund ETH after timelock expires
- **Resolver Management**: Authorize/deauthorize resolvers

### Test Tokens

- **TestICP**: Mock ICP token for testing
- **TestETH**: Mock ETH token for testing

## Development Workflow

1. **Local Development**: Use `npx hardhat test` for local testing
2. **Sepolia Testing**: Deploy to Sepolia testnet for integration testing
3. **Contract Verification**: Verify contracts on Etherscan after deployment

## Integration with ICP

This Ethereum side works in conjunction with:

- **ICP Orderbook Canister**: Manages cross-chain orders
- **ICP Escrow Canister**: Handles ICP-side escrow logic
- **Manual Relayer**: Coordinates cross-chain operations

## Security Features

- **Reentrancy Protection**: OpenZeppelin ReentrancyGuard
- **Access Control**: Owner-based resolver authorization
- **Timelock Protection**: Configurable timelock periods
- **Hashlock Verification**: Cryptographic secret verification
- **Emergency Functions**: Owner emergency withdrawal capability

## Testing

The test suite covers:

- ✅ Contract deployment and initialization
- ✅ ETH locking and claiming flows
- ✅ Timelock and refund mechanisms
- ✅ Resolver authorization
- ✅ Error conditions and edge cases

Run tests with:
```bash
npx hardhat test
```

## Deployment

### Sepolia Testnet

1. Configure `.env` with your Sepolia RPC URL and private key
2. Run deployment: `npx hardhat run scripts/deploy.ts --network sepolia`
3. Save contract addresses for frontend integration
4. Verify contracts on Etherscan (optional)

### Contract Addresses

After deployment, you'll get:
- `FusionEscrow`: Main escrow contract
- `TestICP`: Test ICP token
- `TestETH`: Test ETH token

## Next Steps

1. **Frontend Integration**: Connect React frontend to deployed contracts
2. **ICP Integration**: Coordinate with ICP canister deployment
3. **Cross-Chain Testing**: Test complete swap flows
4. **Production Deployment**: Deploy to mainnet when ready

## Troubleshooting

### Common Issues

1. **Insufficient Sepolia ETH**: Get test ETH from faucet
2. **RPC Connection Issues**: Check your RPC URL configuration
3. **Gas Estimation Errors**: Ensure sufficient ETH for deployment
4. **Contract Verification**: Use Etherscan API key for verification

### Getting Help

- Check the test files for usage examples
- Review the contract code for detailed documentation
- Run `npx hardhat help` for available commands
