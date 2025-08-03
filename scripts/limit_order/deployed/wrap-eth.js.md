# Wrap ETH to WETH Script

## 🎯 Overview

This script converts **native ETH** to **WETH (Wrapped Ethereum)** on Base Sepolia testnet. WETH is required for most DeFi protocols, including the Limit Order Protocol.

## 🪙 ETH vs WETH Explained

### **ETH (Native Ethereum)**
- The **native currency** of Ethereum blockchain
- Used for **gas fees** and direct transfers
- **Cannot be used** in most DeFi protocols
- Think of it as **cash**

### **WETH (Wrapped Ethereum)**
- **ETH wrapped** in ERC-20 token standard
- **Required** for most DeFi protocols
- **1 WETH = 1 ETH** (same value, different format)
- Think of it as a **gift card** with the same value

## 🔧 How the Script Works

### **1. Setup & Configuration**
```javascript
// Load environment variables
const MAKER_PRIVATE_KEY = process.env.MAKER_PRIVATE_KEY;
const MAKER_ADDRESS = process.env.MAKER_ADDRESS;

// WETH contract address on Base Sepolia
const WETH_ADDRESS = "0x4200000000000000000000000000000000000006";
```

### **2. Connect to Blockchain**
```javascript
// Create provider (connection to Base Sepolia)
const provider = new ethers.JsonRpcProvider("https://sepolia.base.org");

// Create wallet with private key
const makerWallet = new ethers.Wallet(MAKER_PRIVATE_KEY, provider);
```

### **3. Get WETH Contract**
```javascript
// WETH contract interface
const wethContract = new ethers.Contract(WETH_ADDRESS, [
    "function deposit() external payable",        // Wrap ETH → WETH
    "function balanceOf(address owner) view returns (uint256)", // Check balance
    "function symbol() view returns (string)"    // Get token symbol
], makerWallet);
```

### **4. Check Current Balances**
```javascript
// Get ETH balance
const ethBalance = await provider.getBalance(MAKER_ADDRESS);

// Get WETH balance
const wethBalance = await wethContract.balanceOf(MAKER_ADDRESS);
```

### **5. Wrap ETH to WETH**
```javascript
// Amount to wrap (0.02 ETH)
const wrapAmount = ethers.parseEther("0.02");

// Call WETH deposit function
const tx = await wethContract.deposit({
    value: wrapAmount,    // Send ETH with transaction
    gasLimit: 200000      // Gas limit for transaction
});
```

## 🚀 Running the Script

### **Prerequisites**
1. **Environment Variables** in `.env` file:
   ```env
   MAKER_PRIVATE_KEY=0x... (your private key)
   MAKER_ADDRESS=0x... (your wallet address)
   ```

2. **Sufficient ETH** in the maker address for:
   - Amount to wrap (0.02 ETH)
   - Gas fees (~0.001 ETH)

### **Command**
```bash
npx hardhat run scripts/wrap-eth.js --network base-sepolia
```

## 📊 Expected Output

```
🔄 Wrapping ETH to WETH for Giveaway...

🎯 Maker Address: 0x8CB80b37cc7193D0f055b1189F25eB903D888D3A
🔑 Maker Private Key: 36128084b9...

💰 Current Balances:
   ETH: 0.226923721771910977 ETH
   WETH: 0.0 WETH

🔄 Wrapping 0.02 ETH to WETH...
📝 Transaction hash: 0x...
⏳ Waiting for confirmation...

✅ ETH wrapped successfully!
📦 Block number: 29156087
💰 Gas used: 29440

💰 New Balances:
   ETH: 0.206923721771910977 ETH
   WETH: 0.02 WETH

🎁 Now the maker has enough WETH for the giveaway!
🔗 View transaction: https://sepolia.basescan.org/tx/0x...
```

## 🔍 What Happens Under the Hood

### **1. Transaction Creation**
- Script creates a **transaction** to call WETH's `deposit()` function
- Sends **0.02 ETH** along with the transaction
- Includes **gas fees** for transaction processing

### **2. Blockchain Execution**
- **Base Sepolia network** processes the transaction
- **WETH contract** receives the ETH
- **Mints** equivalent amount of WETH tokens
- **Sends** WETH tokens to your address

### **3. Balance Update**
- Your **ETH balance** decreases by 0.02 ETH
- Your **WETH balance** increases by 0.02 WETH
- **Total value** remains the same

## 🎯 Why This is Needed

### **For Limit Order Protocol:**
- LOP only works with **ERC-20 tokens** (like WETH)
- **Native ETH** cannot be used directly
- **WETH** is the ERC-20 version of ETH

### **For Giveaway Orders:**
- Maker needs **WETH** to give away
- Taker receives **WETH** (can unwrap back to ETH later)
- Enables **token-to-token** swaps

## 🔧 Technical Details

### **WETH Contract Functions Used:**
- `deposit()` - Wraps ETH to WETH
- `balanceOf(address)` - Checks WETH balance
- `withdraw(uint256)` - Unwraps WETH to ETH (not used in this script)

### **Gas Estimation:**
- **Typical gas used:** ~29,440 gas
- **Gas price:** ~1 gwei
- **Total cost:** ~0.00003 ETH

### **Error Handling:**
- **Insufficient funds** - Not enough ETH
- **Gas estimation** - Transaction too expensive
- **Network issues** - RPC connection problems

## 🎁 Next Steps

After wrapping ETH to WETH:
1. **Create giveaway orders** with WETH
2. **Fill orders** as taker
3. **Unwrap WETH** back to ETH if needed

## 🔗 Related Scripts

- `scripts/giveaway-order.js` - Create giveaway orders
- `scripts/fill-giveaway.js` - Claim giveaway orders
- `scripts/check-order-status.js` - Check order status

---

**💡 Pro Tip:** WETH is just ETH in a format that DeFi protocols can use. Think of it as converting cash to a gift card - same value, different format! 🎁 