# Local Deployment Data

## Overview

This document contains the deployment data for the **local Anvil deployment** of the 1inch Cross-Chain Swap protocol. All contracts are deployed on `localhost:8545` (chain ID: 31337).

## Deployment Summary

**Deployment Date:** Latest successful deployment  
**Network:** Local Anvil (localhost:8545)  
**Chain ID:** 31337  
**Deployer:** `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (Anvil account #0)

## Contract Addresses

### **Core Contracts**

| Contract                     | Address                                      | Description                 |
| ---------------------------- | -------------------------------------------- | --------------------------- |
| **EscrowFactory**            | `0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0` | Main factory contract       |
| **FeeBank**                  | `0x75537828f2ce51be7289709686A69CbFDbB714F1` | Fee collection contract     |
| **EscrowSrc Implementation** | `0xE451980132E65465d0a498c53f0b5227326Dd73F` | Source escrow template      |
| **EscrowDst Implementation** | `0x5392A33F7F677f59e833FEBF4016cDDD88fF9E67` | Destination escrow template |

### **Mock Tokens**

| Token           | Address                                      | Name         | Symbol | Type       |
| --------------- | -------------------------------------------- | ------------ | ------ | ---------- |
| **FeeToken**    | `0x5FbDB2315678afecb367f032d93F642f64180aa3` | Fee Token    | FEE    | ERC20 Mock |
| **AccessToken** | `0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512` | Access Token | ACCESS | ERC20 Mock |

### **External Dependencies**

| Contract                 | Address                                      | Description      |
| ------------------------ | -------------------------------------------- | ---------------- |
| **Limit Order Protocol** | `0x111111125421cA6dc452d289314280a0f8842A65` | Mock LOP address |

## Deployment Configuration

### **EscrowFactory Parameters**

```solidity
EscrowFactory(
    limitOrderProtocol: 0x111111125421cA6dc452d289314280a0f8842A65,
    feeToken: 0x5FbDB2315678afecb367f032d93F642f64180aa3,
    accessToken: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512,
    feeBankOwner: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266,
    rescueDelaySrc: 691200, // 8 days
    rescueDelayDst: 691200  // 8 days
)
```

### **Mock Token Details**

**FeeToken:**

- **Name:** "Fee Token"
- **Symbol:** "FEE"
- **Decimals:** 18
- **Type:** TokenMock (ERC20 + Ownable)
- **Owner:** `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`

**AccessToken:**

- **Name:** "Access Token"
- **Symbol:** "ACCESS"
- **Decimals:** 18
- **Type:** TokenMock (ERC20 + Ownable)
- **Owner:** `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`

## Gas Usage

| Transaction                  | Gas Used  | Cost (ETH)           |
| ---------------------------- | --------- | -------------------- |
| **FeeToken Deployment**      | 946,043   | 0.000946043000946043 |
| **AccessToken Deployment**   | 946,115   | 0.000835309505202695 |
| **EscrowFactory Deployment** | 4,027,167 | 0.003777123880230731 |
| **Total**                    | 5,919,325 | 0.005558476386379469 |

## Usage Instructions

### **1. Interacting with Contracts**

**Using Foundry:**

```bash
# Check EscrowFactory
cast call 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 "FACTORY()" --rpc-url http://localhost:8545

# Check FeeToken
cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "name()" --rpc-url http://localhost:8545

# Check AccessToken
cast call 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "symbol()" --rpc-url http://localhost:8545
```

### **2. Minting Tokens**

**Mint AccessToken (for testing):**

```bash
# Mint 1 token to deployer
cast send 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "mint(address,uint256)" \
  0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 1000000000000000000 \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  --rpc-url http://localhost:8545
```

### **3. Testing with Example Scripts**

**Update config.json:**

```json
{
  "escrowFactory": "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0",
  "limitOrderProtocol": "0x111111125421cA6dc452d289314280a0f8842A65",
  "deployer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
  "maker": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
}
```

**Run example script:**

```bash
cd examples
./scripts/create_order.sh local
```

## Testing Scenarios

### **1. AccessToken Testing**

**Test public functions:**

- Deploy an escrow
- Try to call `publicWithdraw()` without AccessToken → Should fail
- Mint AccessToken to caller
- Try to call `publicWithdraw()` with AccessToken → Should succeed

### **2. FeeToken Testing**

**Test fee collection:**

- Create a swap that generates fees
- Check FeeBank balance
- Verify fee distribution

### **3. Full Workflow Testing**

**Complete cross-chain swap simulation:**

1. Deploy source escrow
2. Deploy destination escrow
3. Test withdrawal with secret
4. Test cancellation
5. Test public recovery functions

## Network Configuration

**Anvil Settings:**

- **RPC URL:** `http://localhost:8545`
- **Chain ID:** 31337
- **Block Time:** 1 second
- **Accounts:** 10 pre-funded accounts

**Default Account #0:**

- **Address:** `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`
- **Private Key:** `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80`
- **Balance:** 10000 ETH

## Troubleshooting

### **Common Issues:**

1. **"Anvil not running"**

   ```bash
   anvil
   ```

2. **"Contract not found"**

   - Verify contract addresses
   - Check if deployment was successful

3. **"Insufficient funds"**
   - Use Anvil's pre-funded accounts
   - Check account balances

### **Reset Deployment:**

```bash
# Stop Anvil
pkill anvil

# Start fresh
anvil

# Redeploy
./scripts/deploy-local.sh
```

## Notes

- **All addresses are deterministic** on Anvil
- **Mock tokens have no real value** - for testing only
- **AccessToken requires > 0 balance** for public functions
- **FeeToken is used for protocol fees**
- **Deployment is persistent** until Anvil is restarted

---

**Last Updated:** Latest deployment  
**Status:** ✅ Successfully deployed and functional
