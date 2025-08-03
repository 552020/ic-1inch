# Getting Testnet ETH on Base Sepolia

## 🚰 Base Sepolia Faucet

You can get free testnet ETH from the official Base Sepolia faucet:

**🔗 Official Faucet:** https://www.coinbase.com/faucets/base-ethereum-sepolia-faucet

### Steps:

1. Connect your wallet (MetaMask, etc.)
2. Select "Base Sepolia" network
3. Enter your address: `0x086153956EF36381bca361575EF7eF67886052A5`
4. Request testnet ETH
5. Wait for confirmation (usually takes a few minutes)

## 🎯 Alternative Faucets

If the official faucet is slow or has issues:

1. **Chainlink Faucet:** https://faucets.chain.link/base-sepolia
2. **Alchemy Faucet:** https://www.alchemy.com/faucets/base-sepolia-faucet

## 💰 How Much to Get

For the giveaway order, you need:

- **0.01 ETH** for the order value
- **~0.001 ETH** for gas fees
- **Total:** ~0.011 ETH

## ⚡ Quick Check

After getting ETH, you can check your balance:

```bash
npx hardhat run test-interaction.js --network base-sepolia
```

This will show your current ETH balance.

## 🎁 Once You Have ETH

Run the giveaway script again:

```bash
npx hardhat run fill-giveaway.js --network base-sepolia
```

The script should then successfully execute the giveaway order!
