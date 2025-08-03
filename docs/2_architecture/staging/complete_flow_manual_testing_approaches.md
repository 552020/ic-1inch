# Manual Testing Approaches for Fusion+ Flow

## Overview

This document outlines the different manual testing approaches we've created to test the complete Fusion+ flow, from order creation to swap completion.

## Testing Philosophy

Instead of relying solely on automated test files, we've created a **hybrid approach** that combines:

- **Backend scripts** for automation
- **Manual interventions** for human oversight
- **Step-by-step workflows** for complete control
- **Real contract interactions** on Base Sepolia

## Available Testing Approaches

### 1. **Full Manual Flow** (Recommended)

**Script**: `./scripts/run-full-manual-test.sh`

**What it does**:

- Orchestrates the complete Fusion+ flow
- Combines automated and manual steps
- Provides human oversight at critical points
- Creates all necessary test data

**Flow**:

1. **Order Creation** (Automated)
2. **Relayer Simulation** (Automated)
3. **Escrow Creation** (Manual intervention)
4. **Secret Revelation** (Automated)
5. **Verification** (Manual + Automated)

### 2. **Individual Component Testing**

#### Order Creation

```bash
./scripts/manual-order-creation.sh
```

- Generates secret and hashlock
- Creates order data
- Saves files for next steps

#### Relayer Simulation

```bash
./scripts/manual-relayer.sh
```

- Simulates order reception
- Broadcasts to resolvers
- Simulates Dutch auction
- Creates resolver action data

#### Secret Revelation

```bash
./scripts/manual-secret-revelation.sh
```

- Reads secret from file
- Simulates secret sharing
- Creates revelation data
- Simulates escrow unlocking

### 3. **Backend Scripts** (Node.js/Python)

**For advanced users** who want to create custom backend scripts:

```javascript
// Example: Custom order creation
const ethers = require("ethers");

async function createFusionOrder() {
  // Custom order creation logic
  const order = {
    maker: "0x8CB80b37cc7193D0f055b1189F25eB903D888D3A",
    taker: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
    // ... more order data
  };

  const signature = await signOrder(order);
  await sendToRelayer(order, signature);
}
```

## Manual Testing Workflows

### Workflow 1: Complete Flow Test

```bash
# Run the complete manual test
./scripts/run-full-manual-test.sh
```

**Output**:

- `order.json` - Order data
- `secret.txt` - Secret for revelation
- `resolver-action.json` - Resolver action data
- `secret-revelation.json` - Secret revelation data

### Workflow 2: Step-by-Step Testing

```bash
# Step 1: Create order
./scripts/manual-order-creation.sh

# Step 2: Simulate relayer
./scripts/manual-relayer.sh

# Step 3: Manual escrow creation (using Foundry)
forge script test/deployed/TestETHOnly.s.sol --broadcast

# Step 4: Reveal secret
./scripts/manual-secret-revelation.sh
```

### Workflow 3: Custom Testing

```bash
# Create custom order with specific parameters
./scripts/manual-order-creation.sh

# Modify order.json with custom values
# Run custom relayer simulation
# Test specific escrow scenarios
```

## Manual Interventions

### Critical Manual Steps

1. **Escrow Creation Verification**

   ```bash
   # Check escrow creation on Base Sepolia
   forge script test/deployed/TestETHOnly.s.sol --fork-url https://sepolia.base.org
   ```

2. **Secret Management**

   ```bash
   # Manually verify secret is correct
   cat secret.txt
   echo -n "$(cat secret.txt)" | sha256sum
   ```

3. **Contract Verification**
   ```bash
   # Verify escrow addresses
   forge script test/deployed/TestAddressComputationDeployed.s.sol
   ```

### Human-in-the-Loop Points

- **Order validation** - Verify order parameters
- **Escrow creation** - Confirm on-chain deployment
- **Secret revelation** - Manual secret sharing
- **Swap completion** - Verify token transfers

## Testing Data Management

### Generated Files

| File                     | Purpose                | Created By                    |
| ------------------------ | ---------------------- | ----------------------------- |
| `order.json`             | Order data             | `manual-order-creation.sh`    |
| `secret.txt`             | Secret for revelation  | `manual-order-creation.sh`    |
| `resolver-action.json`   | Resolver action data   | `manual-relayer.sh`           |
| `secret-revelation.json` | Secret revelation data | `manual-secret-revelation.sh` |

### Environment Variables

```bash
export MAKER_PRIVATE_KEY="your_maker_private_key"
export RESOLVER_PRIVATE_KEY="your_resolver_private_key"
export ESCROW_FACTORY="0xd6Bb18429854140eAC0fA53fd756Db2be05aaaf3"
export BASE_RPC_URL="https://sepolia.base.org"
```

## Testing Scenarios

### Scenario 1: Basic ETH Swap

- **Maker**: 1 ETH → **Taker**: 1500 USDC
- **Testing**: Basic escrow creation and secret revelation

### Scenario 2: Large Order

- **Maker**: 10 ETH → **Taker**: 15000 USDC
- **Testing**: Large amount handling and gas optimization

### Scenario 3: Partial Fill

- **Maker**: 5 ETH → **Taker**: 7500 USDC (partial)
- **Testing**: Partial fill mechanism (if implemented)

### Scenario 4: Failed Swap

- **Maker**: 1 ETH → **Taker**: 1500 USDC (timeout)
- **Testing**: Cancellation and refund mechanisms

## Advantages of Manual Testing

### ✅ **Benefits**

- **Full control** over the testing process
- **Human oversight** at critical points
- **Real contract interactions** on Base Sepolia
- **Flexible** - can test any scenario
- **Educational** - understand each step
- **Debugging** - easy to identify issues

### ⚠️ **Limitations**

- **Time-consuming** - requires manual intervention
- **Error-prone** - human mistakes possible
- **Not repeatable** - each test is unique
- **Limited automation** - some steps manual

## Integration with Automated Tests

### Hybrid Approach

1. **Use manual testing** for:

   - Initial setup and validation
   - Complex scenarios
   - Human verification
   - Educational purposes

2. **Use automated tests** for:
   - Regression testing
   - Continuous integration
   - Quick validation
   - Repetitive tasks

### Example Integration

```bash
# Manual: Create order and simulate relayer
./scripts/manual-order-creation.sh
./scripts/manual-relayer.sh

# Automated: Test escrow creation
forge script test/deployed/TestETHOnly.s.sol --broadcast

# Manual: Verify and complete
./scripts/manual-secret-revelation.sh
```

## Next Steps

### Immediate Actions

1. **Run the full manual test** to validate the approach
2. **Create custom scenarios** for specific testing needs
3. **Integrate with real contracts** on Base Sepolia
4. **Document any issues** and refine the process

### Future Enhancements

1. **Add more automation** to manual steps
2. **Create Node.js backend scripts** for advanced testing
3. **Integrate with real relayer service**
4. **Add professional resolver simulation**
5. **Implement Dutch auction testing**

## Conclusion

Manual testing provides a **comprehensive approach** to testing the Fusion+ flow with full human control and oversight. This approach complements automated testing and provides valuable insights into the complete system behavior.

The scripts we've created enable you to test the **entire Fusion+ flow** manually, from order creation to swap completion, while maintaining full control over each step of the process.
