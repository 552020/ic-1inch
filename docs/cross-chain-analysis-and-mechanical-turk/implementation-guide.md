# Implementation Guide

## For EVM to EVM Chains

### Prerequisites

1. **Two EVM-compatible chains** (e.g., Ethereum ↔ Polygon)
2. **Foundry development environment**
3. **Private keys** for deployment
4. **Native tokens** for gas fees on both chains

### Step 1: Deploy Contracts

#### Deploy on Chain A (Source)

```bash
# Configure environment
export CHAIN_A_RPC="https://eth-mainnet.alchemyapi.io/v2/YOUR_KEY"
export PRIVATE_KEY="your_private_key"

# Deploy contracts
forge script script/DeployEscrowFactory.s.sol \
  --rpc-url $CHAIN_A_RPC \
  --private-key $PRIVATE_KEY \
  --broadcast \
  --verify
```

#### Deploy on Chain B (Destination)

```bash
# Configure environment
export CHAIN_B_RPC="https://polygon-mainnet.alchemyapi.io/v2/YOUR_KEY"

# Deploy contracts (same script)
forge script script/DeployEscrowFactory.s.sol \
  --rpc-url $CHAIN_B_RPC \
  --private-key $PRIVATE_KEY \
  --broadcast \
  --verify
```

### Step 2: Verify Deployments

Check the `deployments/` directory for contract addresses:

```
deployments/
├── chainA/
│   ├── EscrowFactory.json
│   └── deployment-info.json
└── chainB/
    ├── EscrowFactory.json
    └── deployment-info.json
```

### Step 3: Build Off-Chain Resolver

You need to build the off-chain resolver system that coordinates between chains:

```javascript
class CrossChainResolver {
  constructor(chainAConfig, chainBConfig) {
    this.chainA = new ethers.JsonRpcProvider(chainAConfig.rpc);
    this.chainB = new ethers.JsonRpcProvider(chainBConfig.rpc);
    this.factoryA = new ethers.Contract(chainAConfig.factory, ABI, this.chainA);
    this.factoryB = new ethers.Contract(chainBConfig.factory, ABI, this.chainB);
  }

  async executeSwap(order) {
    // 1. Deploy EscrowSrc on Chain A
    const escrowSrcTx = await this.factoryA.createEscrowSrc(order);

    // 2. Deploy EscrowDst on Chain B
    const escrowDstTx = await this.factoryB.createEscrowDst(order);

    // 3. Wait for secret from user
    const secret = await this.waitForSecret(order);

    // 4. Withdraw from both chains
    await this.withdrawFromBoth(secret, order);
  }
}
```

### Step 4: Test the Integration

Create test swaps to verify the system works:

```javascript
// Test swap: ETH → MATIC
const testOrder = {
  maker: "0x...",
  sourceChain: 1, // Ethereum
  destChain: 137, // Polygon
  sourceToken: "0x...", // ETH
  destToken: "0x...", // MATIC
  amount: ethers.parseEther("1.0"),
  secret: "0x...",
};

await resolver.executeSwap(testOrder);
```

## For EVM to Non-EVM (e.g., ICP)

### What You Can Reuse

#### EVM Side (Complete)

- ✅ All contracts from this repository
- ✅ Factory and proxy patterns
- ✅ Timelock mechanisms
- ✅ Withdrawal logic

#### Design Patterns (Adaptable)

- ✅ Factory pattern for ICP canisters
- ✅ Timelock concepts
- ✅ Escrow logic flow
- ✅ Security mechanisms

### What You Need to Build

#### 1. ICP Smart Contracts (Canisters)

```rust
// ICP Escrow Canister (Rust)
#[derive(CandidType, Deserialize)]
pub struct EscrowSrc {
    pub maker: Principal,
    pub taker: Principal,
    pub token: Principal,
    pub amount: u64,
    pub hashlock: [u8; 32],
    pub timelocks: Timelocks,
    pub safety_deposit: u64,
}

#[update]
async fn withdraw(secret: Vec<u8>, immutables: EscrowSrc) -> Result<(), String> {
    // Verify secret hash
    let hash = sha256(&secret);
    if hash != immutables.hashlock {
        return Err("Invalid secret".to_string());
    }

    // Check timelocks
    if ic_cdk::api::time() < immutables.timelocks.withdrawal_start {
        return Err("Too early".to_string());
    }

    // Transfer tokens
    transfer_tokens(immutables.token, immutables.taker, immutables.amount).await
}
```

#### 2. Address Format Handling

```rust
// Handle EVM ↔ ICP address conversion
pub struct AddressMapper {
    evm_to_icp: HashMap<String, Principal>,
    icp_to_evm: HashMap<Principal, String>,
}

impl AddressMapper {
    pub fn map_evm_to_icp(&self, evm_address: &str) -> Option<Principal> {
        self.evm_to_icp.get(evm_address).copied()
    }

    pub fn map_icp_to_evm(&self, icp_principal: &Principal) -> Option<String> {
        self.icp_to_evm.get(icp_principal).cloned()
    }
}
```

#### 3. Cross-Chain Bridge/Resolver

```javascript
class EVMICPBridge {
  constructor(evmConfig, icpConfig) {
    this.evm = new ethers.JsonRpcProvider(evmConfig.rpc);
    this.icp = new HttpAgent({ host: icpConfig.host });
    this.addressMapper = new AddressMapper();
  }

  async executeSwap(order) {
    // 1. Map addresses
    const icpAddress = this.addressMapper.mapEVMToICP(order.maker);

    // 2. Deploy EVM escrow
    await this.deployEVMEscrow(order);

    // 3. Deploy ICP escrow
    await this.deployICPEscrow({
      ...order,
      maker: icpAddress,
    });

    // 4. Coordinate withdrawal
    await this.coordinateWithdrawal(order);
  }
}
```

### Development Steps

1. **Set up ICP development environment**

   ```bash
   # Install dfx (ICP SDK)
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

   # Create new ICP project
   dfx new fusion_icp_bridge
   cd fusion_icp_bridge
   ```

2. **Implement ICP escrow canisters**

   - Port escrow logic to Rust
   - Implement timelock mechanisms
   - Add token transfer functionality

3. **Build address mapping system**

   - Create bidirectional mapping
   - Handle different address formats
   - Implement validation logic

4. **Develop cross-chain bridge**

   - Monitor both chains for events
   - Coordinate escrow deployments
   - Manage secret distribution

5. **Test the integration**
   - Start with testnet deployments
   - Test various swap scenarios
   - Verify security mechanisms

## Configuration Examples

### EVM Chain Configuration

```json
{
  "ethereum": {
    "chainId": 1,
    "rpc": "https://eth-mainnet.alchemyapi.io/v2/YOUR_KEY",
    "factory": "0x...",
    "gasPrice": "20000000000"
  },
  "polygon": {
    "chainId": 137,
    "rpc": "https://polygon-mainnet.alchemyapi.io/v2/YOUR_KEY",
    "factory": "0x...",
    "gasPrice": "30000000000"
  }
}
```

### ICP Configuration

```json
{
  "icp": {
    "host": "https://ic0.app",
    "canisterId": "rdmx6-jaaaa-aaaah-qdrqq-cai",
    "identity": "./identity.pem"
  }
}
```

## Security Considerations

### For EVM Deployments

1. **Verify contracts** on block explorers
2. **Test on testnets** first
3. **Use multisig wallets** for ownership
4. **Monitor gas prices** for efficiency
5. **Implement circuit breakers** for emergencies

### For Cross-Chain Bridges

1. **Secure key management** for resolver
2. **Implement failsafes** for stuck transactions
3. **Monitor both chains** continuously
4. **Have recovery procedures** for failures
5. **Test edge cases** thoroughly

## Monitoring and Maintenance

### Key Metrics to Track

1. **Swap success rate**
2. **Average completion time**
3. **Gas costs on both chains**
4. **Resolver performance**
5. **Failed transaction reasons**

### Operational Procedures

1. **Regular health checks** of all components
2. **Update gas price strategies** based on network conditions
3. **Monitor resolver balances** and top up as needed
4. **Handle stuck transactions** with appropriate recovery
5. **Update contracts** when necessary (through governance)
