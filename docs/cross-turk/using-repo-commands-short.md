# Short Commands for Local Deployment

## 1. Fund the Deployer Address

```bash
# Fund the deployer address with ETH (required for gas fees)
source .env && cast send --rpc-url http://localhost:8545 --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 $DEPLOYER_ADDRESS --value 10000000000000000000
```

### ❌ Common Errors and Solutions

#### **Error 1: Insufficient funds**

```bash
# WRONG - Trying to send from address with no ETH
cast send --rpc-url http://localhost:8545 --private-key $DEPLOYER_PRIVATE_KEY $DEPLOYER_ADDRESS --value 10ether
# Error: Insufficient funds for gas * price + value
```

**Why it fails**: You're trying to send ETH from an address that doesn't have ETH yet.

#### **Error 2: Missing --unlocked flag**

```bash
# WRONG - Missing --unlocked flag
cast send --rpc-url http://localhost:8545 --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 $DEPLOYER_ADDRESS --value 10ether
# Error: Error accessing local wallet. Did you set a private key [...] or use the --unlocked flag?
```

**Why it fails**: Cast doesn't know how to sign for anvil's unlocked accounts without the `--unlocked` flag.

### ✅ Check Balance After Funding

```bash
cast balance $DEPLOYER_ADDRESS --rpc-url http://localhost:8545
```

## 2. Deploy EscrowFactory Contract

```bash
# For local testing (mainnet fork)
source .env && forge script script/DeployEscrowFactory.s.sol --rpc-url http://localhost:8545 --broadcast --private-key $DEPLOYER_PRIVATE_KEY --sender $DEPLOYER_ADDRESS

# For Base Sepolia testnet
source .env && forge script script/DeployEscrowFactory.s.sol --rpc-url https://base-sepolia.g.alchemy.com/v2/QpubPhWfpIvpujMTF84v5 --broadcast --private-key $DEPLOYER_PRIVATE_KEY --sender $DEPLOYER_ADDRESS
```

## 3. Fund the Maker Address

```bash
# Fund the maker address with ETH (required for gas fees)
cast send --rpc-url http://localhost:8545 --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 $MAKER_ADDRESS --value 1000000000000000000
```

## 4. Mint Tokens to Maker

```bash
# Mint tokens to the maker address (using the deployed ERC20True contract)
cast send 0x5aAdFB43eF8dAF45DD80F4676345b7676f1D70e3 \
  "mint(address,uint256)" \
  $MAKER_ADDRESS \
  1000000 \
  --unlocked \
  --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
  --rpc-url http://localhost:8545
```

## 5. Approve Limit Order Protocol

```bash
# Allow the Limit Order Protocol to spend tokens on behalf of the maker
cast send 0x5aAdFB43eF8dAF45DD80F4676345b7676f1D70e3 \
  "approve(address,uint256)" \
  0x111111125421cA6dc452d289314280a0f8842A65 \
  1000000 \
  --private-key $MAKER_PRIVATE_KEY \
  --rpc-url http://localhost:8545
```

## 6. Update Config Files

After deployment, copy the new deployment address and update:

- `scripts/mechanical-turk/evm-test-config.json`
- `eth/examples/config/eth-escrow-config.json`

## 7. Test the Deployment

```bash
cd examples && cp ../.env . && ./scripts/create_order.sh
```

## Notes

- Replace `YOUR_DEPLOYER_ADDRESS` with your actual deployer address
- The funding command uses anvil's default account (has unlimited ETH)
- The deployment uses environment variables from `.env`
- Make sure anvil is running before executing these commands
- Use `--unlocked` flag when using anvil's pre-funded accounts
- **Important**: Add `0x` prefix to private keys in `.env` file:
  ```bash
  DEPLOYER_PRIVATE_KEY=0x0859f53bbd86ceec2936b5a47e99cbcefd1fa6c865c57344bc31054a5851f074
  MAKER_PRIVATE_KEY=0x36128084b96c8990f0d9633a998b4b6c902e6ce6f1a388189e9558a339ee3f6f
  ```
