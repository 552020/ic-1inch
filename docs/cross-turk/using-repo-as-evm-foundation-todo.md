# Using Cross-Chain-Swap Repository as EVM Foundation - TODO List

> **Purpose**: Step-by-step to-do list for integrating the cross-chain-swap repository as EVM foundation
> **Target**: ICP<>EVM Fusion+ implementation
> **Status**: 🚀 **Ready to implement**

## 📋 **Phase 1: Repository Setup**

### **1.1 Fork and Clone Repository**

- [x] Fork `https://github.com/1inch/cross-chain-swap` on GitHub
- [x] Clone your fork: `git clone https://github.com/YOUR_USERNAME/cross-chain-swap.git evm`
- [x] Add upstream remote: `git remote add upstream https://github.com/1inch/cross-chain-swap.git`
- [x] Verify remotes: `git remote -v`
- [x] Replace your current `evm/` folder with the cloned repository

### **1.2 Environment Setup**

- [x] Install Foundry: `curl -L https://foundry.paradigm.xyz | bash`
- [x] Install dependencies: `forge install`
- [x] Create `.env` file with required variables:
  ```bash
  DEPLOYER_PRIVATE_KEY=your_private_key
  MAKER_PRIVATE_KEY=your_private_key
  CHAIN_ID=84532  # Base Sepolia
  RPC_URL=https://base-sepolia.g.alchemy.com/v2/YOUR_KEY
  DEPLOYER_ADDRESS=0x8CB80b37cc7193D0f055b1189F25eB903D888D3A
  ```

## 🚀 **Phase 2: EVM Foundation Deployment**

### **2.1 Deploy Contracts on Base Sepolia**

**Step 1: Test on Local Fork First** ✅

```bash
# Start local fork with mainnet (creates a copy of mainnet on your computer)
anvil --fork-url $MAINNET_RPC --chain-id 31337 --port 8545

# Deploy using fork (uses real 1inch contracts)
forge script script/DeployEscrowFactoryLocal.s.sol --rpc-url http://localhost:8545 --broadcast --sender YOUR_DEPLOYER_ADDRESS --private-key YOUR_PRIVATE_KEY
```

**Step 2: Test the Script on Local Fork**

```bash
# Test the deployment on local fork
./scripts/mechanical-turk/create_order.sh
```

**Step 3: Deploy to Base Sepolia**

```bash
# Deploy to Base Sepolia with mock tokens (no 1inch dependency)
forge script script/DeployEscrowFactoryBaseSepolia.s.sol --rpc-url $BASE_SEPOLIA_RPC --broadcast --sender YOUR_DEPLOYER_ADDRESS --private-key YOUR_PRIVATE_KEY
```

**Step 4: Verify and Setup Testing**

- [ ] Verify all contracts on Etherscan
- [x] Copy testing script to your project: `cp evm/examples/scripts/create_order.sh scripts/mechanical-turk/`
- [x] Copy test config to your project: `cp evm/examples/config/config.json scripts/mechanical-turk/evm-test-config.json`
- [x] Edit `scripts/mechanical-turk/evm-test-config.json` with your deployed addresses:
  ```json
  {
    "escrowFactory": "0x...", // From deployment output
    "limitOrderProtocol": "0x111111125421cA6dc452d289314280a0f8842A65",
    "srcToken": "0x4200000000000000000000000000000000000006", // WETH on Base Sepolia
    "dstToken": "0x0000000000000000000000000000000000000000", // ETH
    "srcAmount": 1000000000000000000, // 1 WETH
    "dstAmount": 1000000000000000000, // 1 ETH
    "stages": ["deployMocks", "deployEscrowSrc", "deployEscrowDst"]
  }
  ```
- [x] Make script executable: `chmod +x scripts/mechanical-turk/create_order.sh`
- [ ] Test the deployment with the testing scripts

### **2.1.4 Configuration Setup**

- [ ] Copy example config: `cp evm/examples/config/config.json evm/examples/config/my-icp-evm-config.json`
- [ ] Edit `my-icp-evm-config.json` for your deployment:
  ```json
  {
    "escrowFactory": "0x...", // From deployment output
    "limitOrderProtocol": "0x111111125421cA6dc452d289314280a0f8842A65",
    "srcToken": "0x4200000000000000000000000000000000000006", // WETH on Base Sepolia
    "dstToken": "0x0000000000000000000000000000000000000000", // ETH
    "srcAmount": 1000000000000000000, // 1 WETH
    "dstAmount": 1000000000000000000, // 1 ETH
    "stages": ["deployMocks", "deployEscrowSrc", "deployEscrowDst"]
  }
  ```

### **2.2 Test EVM Foundation**

- [ ] Run test script: `./scripts/mechanical-turk/create_order.sh`
- [ ] Verify escrow creation works
- [ ] Test withdrawal functionality
- [ ] Test cancellation functionality
- [ ] Verify event monitoring works

### **2.3 Update Your Project Scripts**

- [ ] Update `scripts/mechanical-turk/deploy-mechanical-turk.sh` to use new contract addresses
- [ ] Update `scripts/mechanical-turk/mechanical-turk-manual-test.sh` to use new structure
- [ ] Update any other scripts that reference the old `evm/` folder
- [ ] Test all your existing scripts with the new EVM foundation

## 🔧 **Phase 3: Integration with Your ICP Components**

### **3.1 Update Your Relayer Configuration**

- [ ] Create `scripts/relayer/evm-config.js`:
  ```javascript
  const EVM_CONFIG = {
    chainId: 84532,
    factoryAddress: "0x...", // From deployment
    rpcUrl: "https://base-sepolia.g.alchemy.com/v2/YOUR_KEY",
    eventSignatures: {
      srcEscrowCreated:
        "0x0e534c62f0afd2fa0f0fa71198e8aa2d549f24daf2bb47de0d5486c7ce9288ca",
      dstEscrowCreated:
        "0xc30e111dcc74fddc2c3a4d98ffb97adec4485c0a687946bf5b22c2a99c7ff96d",
    },
  };
  ```

### **3.2 Update Your Frontend**

- [ ] Update `src/frontend/src/` to use new contract addresses
- [ ] Update any contract ABIs to use the new EVM contracts
- [ ] Test frontend integration with new EVM foundation
- [ ] Verify SIWE authentication works with new contracts

### **3.3 Update Your ICP Canisters**

- [ ] Update `src/escrow/src/` to reference new EVM contract addresses
- [ ] Update any cross-chain coordination logic
- [ ] Test ICP canister integration with new EVM foundation
- [ ] Verify threshold ECDSA integration works

## 🧪 **Phase 4: Testing & Validation**

### **4.1 End-to-End Testing**

- [ ] Test complete ICP<>EVM swap flow
- [ ] Test escrow creation on both chains
- [ ] Test asset locking verification
- [ ] Test withdrawal on both chains
- [ ] Test cancellation scenarios
- [ ] Test error handling and edge cases

### **4.2 Performance Testing**

- [ ] Test gas costs for escrow creation
- [ ] Test transaction finality times
- [ ] Test event monitoring performance
- [ ] Test cross-chain coordination timing

### **4.3 Security Testing**

- [ ] Test timelock mechanisms
- [ ] Test safety deposit functionality
- [ ] Test access control
- [ ] Test signature verification
- [ ] Test replay protection

## 📚 **Phase 5: Documentation & Maintenance**

### **5.1 Update Documentation**

- [ ] Update `README.md` to reflect new EVM foundation
- [ ] Update deployment scripts documentation
- [ ] Update testing procedures
- [ ] Document any customizations made to the EVM contracts

### **5.2 Maintenance Setup**

- [ ] Set up monitoring for EVM contracts
- [ ] Set up alerts for contract events
- [ ] Create backup procedures
- [ ] Document update procedures for upstream changes

## 📝 **Notes & Reminders**

### **Important Reminders:**

- ✅ The EVM escrows work independently - no cross-chain awareness needed
- ✅ Use the examples folder for testing: `evm/examples/scripts/create_order.sh`
- ✅ Base Sepolia is recommended for PoC due to low gas costs and fast finality
- ✅ Keep your fork updated with upstream changes: `git fetch upstream && git merge upstream/main`
- ✅ Document any customizations you make to the EVM contracts

### **Troubleshooting:**
