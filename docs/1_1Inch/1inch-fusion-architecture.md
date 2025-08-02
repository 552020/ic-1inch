# Fusion Architecture Deep Dive

## Contract Architecture

### Core Smart Contracts

```
EscrowFactory
├── Creates EscrowSrc implementations (once per chain)
├── Creates EscrowDst implementations (once per chain)
├── Deploys proxy contracts (for each swap)
└── Manages deterministic addresses

EscrowSrc (Source Chain)
├── Holds user's tokens
├── Implements withdrawal logic
├── Handles cancellation
└── Manages timelocks

EscrowDst (Destination Chain)
├── Holds resolver's tokens
├── Implements withdrawal logic
├── Handles cancellation
└── Manages timelocks

BaseEscrow (Shared Logic)
├── Common escrow functionality
├── Timelock management
├── Rescue fund mechanisms
└── Access control
```

### Factory + Proxy Pattern

#### Why This Pattern?

1. **Gas Efficiency**: Deploy implementations once, create lightweight proxies
2. **Deterministic Addresses**: Compute addresses before deployment
3. **Standardization**: All escrows use same tested logic
4. **Scalability**: Can create thousands of escrows cheaply

#### Implementation Details

```solidity
// Factory Constructor (Once per chain)
constructor(...) {
    // Deploy implementations
    ESCROW_SRC_IMPLEMENTATION = address(new EscrowSrc(...));
    ESCROW_DST_IMPLEMENTATION = address(new EscrowDst(...));

    // Compute proxy bytecode hashes
    _PROXY_SRC_BYTECODE_HASH = ProxyHashLib.computeProxyBytecodeHash(ESCROW_SRC_IMPLEMENTATION);
    _PROXY_DST_BYTECODE_HASH = ProxyHashLib.computeProxyBytecodeHash(ESCROW_DST_IMPLEMENTATION);
}

// Proxy Creation (For each swap)
function _deployEscrow(bytes32 salt, uint256 value, address implementation) internal returns (address) {
    return implementation.cloneDeterministic(salt, value);
}
```

## Cross-Chain Flow

### Step-by-Step Process

1. **User Creates Order**

   ```
   User signs order intent:
   - Source chain & token
   - Destination chain & token
   - Amounts
   - Timelocks
   - Secret hash
   ```

2. **Resolver Competition**

   ```
   Multiple resolvers compete:
   - Resolver A: 0.1% fee
   - Resolver B: 0.08% fee
   - Resolver C: 0.05% fee (wins)
   ```

3. **Escrow Deployment**

   ```
   Winning resolver:
   - Deploys EscrowSrc on source chain
   - Deploys EscrowDst on destination chain
   - Uses same order parameters
   ```

4. **Token Locking**

   ```
   - User's tokens locked in EscrowSrc
   - Resolver's tokens locked in EscrowDst
   - Safety deposits locked on both chains
   ```

5. **Secret Revelation & Withdrawal**
   ```
   - User reveals secret to resolver
   - Resolver withdraws from both escrows
   - Tokens transferred to final recipients
   ```

### Timelock Stages

```
Timeline: [Deploy] → [Finality] → [Private Withdrawal] → [Public Withdrawal] → [Cancellation] → [Public Cancellation]

Source Chain (EscrowSrc):
- Private Withdrawal: Only taker can withdraw
- Public Withdrawal: Anyone with secret can withdraw
- Private Cancellation: Only taker can cancel
- Public Cancellation: Anyone can cancel

Destination Chain (EscrowDst):
- Private Withdrawal: Only maker can withdraw
- Public Withdrawal: Anyone with secret can withdraw
- Cancellation: Only maker can cancel
```

## Security Mechanisms

### 1. Hashlock Security

- Secret is hashed and stored in escrow
- Only correct secret can unlock funds
- Same secret works on both chains

### 2. Timelock Security

- Multiple time periods for different operations
- Fallback mechanisms if resolver fails
- Public periods allow anyone to complete swap

### 3. Safety Deposits

- Resolvers must deposit native tokens
- Incentivizes proper behavior
- Penalties for malicious actions

### 4. Economic Incentives

- Resolvers compete for execution fees
- Safety deposits motivate completion
- Public withdrawal periods provide fallbacks

## Deployment Architecture

### Multi-Chain Deployment

```
Chain A (e.g., Ethereum)
├── EscrowFactory
├── EscrowSrc Implementation
├── EscrowDst Implementation
└── Libraries & Dependencies

Chain B (e.g., Polygon)
├── EscrowFactory
├── EscrowSrc Implementation
├── EscrowDst Implementation
└── Libraries & Dependencies

Chain C (e.g., Arbitrum)
├── EscrowFactory
├── EscrowSrc Implementation
├── EscrowDst Implementation
└── Libraries & Dependencies
```

### Address Consistency

For EVM chains, addresses are consistent:

- Same private key → Same address on all chains
- No address mapping needed
- Simplified cross-chain operations

## Resolver Network

### Off-Chain Infrastructure

```
Resolver Network
├── Order Monitoring
│   ├── Listen for new orders
│   ├── Calculate profitability
│   └── Submit competitive bids
├── Cross-Chain Coordination
│   ├── Deploy escrows on both chains
│   ├── Monitor timelock stages
│   └── Execute withdrawals
├── Risk Management
│   ├── Safety deposit management
│   ├── Gas price optimization
│   └── Failure recovery
└── Secret Management
    ├── Secure secret storage
    ├── Timely revelation
    └── Withdrawal coordination
```

### Resolver Responsibilities

1. **Deployment**: Create escrows on both chains
2. **Funding**: Provide tokens for destination chain
3. **Coordination**: Manage cross-chain timing
4. **Execution**: Complete withdrawals with secrets
5. **Recovery**: Handle failures and cancellations
