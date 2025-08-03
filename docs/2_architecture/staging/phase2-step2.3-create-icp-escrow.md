# Phase 2 Step 2.3: Create ICP Escrow

_Resolver creates escrow using our ICP canisters_

---

## Overview

Resolver creates an ICP escrow using our canisters to hold resolver's tokens for the atomic swap.

---

## Required Inputs

### **From Step 2.2:**

- **Ethereum escrow address** (existing 1inch contract)
- **User tokens locked** in Ethereum escrow
- **Escrow parameters** (hashlock, timelock)

---

## Implementation

### **✅ MVP Implementation (Our Development Work):**

**We DO build ICP escrow canisters:**

- **ICP escrow canisters** - Our main development work
- **Hashlock and timelock logic** - Implemented in Rust
- **Token handling** - ICRC-1/ICRC-2 token integration
- **Cross-chain communication** - HTTP outcalls for verification

#### **Reusable Components from First Attempt:**

- **Complete HTLC logic** - Direct reuse of Rust implementation
- **HTTP outcall module** - Ready-to-use Ethereum verification
- **Token integration patterns** - ICRC-1 error handling and validation
- **Performance optimizations** - Caching and batch operations
- **Security patterns** - Hashlock verification and timelock management

#### **Key Patterns from Reference Implementations:**

- **Deterministic addressing** - Use order hash for escrow identification
- **Safety deposit mechanism** - Required for escrow deployment
- **Dutch auction integration** - Rate bump calculations for pricing
- **Fee management** - Protocol, integrator, and surplus fee handling
- **Resolver access control** - Whitelist-based authorization
- **Timelock coordination** - Cross-chain deployment timing

#### **Phased Implementation Strategy:**

**Phase 1 (MVP):** First Attempt ICP Implementation

- **Direct code reuse** - HTLC logic, HTTP outcalls, token handling

**Phase 2 (Stretch):** Solana Fusion Protocol

- **Order hash computation** - Escrow identification
- **Deterministic addressing** - Factory patterns
- **Resolver access control** - Authorization mechanisms

**Phase 3 (Stretch):** Advanced Features

- **Dutch auction logic** - Rate bump calculations
- **Cancellation premium** - Time-based calculations
- **Arbitrary call execution** - Complex resolver operations

#### **Canister Interface (Example):**

```rust
// Core escrow creation method
#[update]
pub async fn create_escrow(params: CreateEscrowParams) -> Result<String, EscrowError>

// Token deposit method
#[update]
pub async fn deposit_tokens(escrow_id: String, token: Principal, amount: u64) -> Result<(), EscrowError>

// Withdrawal with secret
#[update]
pub async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError>

// Cancellation after timeout
#[update]
pub async fn refund_escrow(escrow_id: String) -> Result<(), EscrowError>
```

#### **Timelock Granularity:**

- **Unit:** Nanoseconds since epoch (ICP standard)
- **Conversion:** Ethereum uses seconds, ICP uses nanoseconds
- **Example:** 300 seconds = 300,000,000,000 nanoseconds

**We DO need testing tools for the flow:**

- **Canister deployment scripts** (dfx commands)
- **Escrow creation utilities** (for testing our canisters)
- **Token transfer testing** (ICRC token interactions)
- **Integration testing scripts** (coordinate with Ethereum)
- **Monitoring tools** (minimal log for canister state during testing)

### **✅ Official Process (from 1inch Fusion+ Whitepaper):**

**Source:** [1inch Fusion+ Whitepaper](docs/1inch/1inch-fusion-plus-whitepaper.md#phase-2-deposit-phase)

> "4. The resolver deposits the taker amount into the escrow contract on the destination chain, employing the same secret hash and providing relevant escrow details."

### **✅ Stretch Goals Implementation:**

**If we implement stretch goals, we would need:**

#### **For UI (Stretch Goal 1):**

- **Frontend integration** with our ICP canisters
- **Plug wallet connection** for ICP transactions
- **Escrow management interface** for resolver role

#### **For Partial Fills (Stretch Goal 2):**

- **Merkle tree logic** in our ICP canisters
- **Indexed secret management** for progressive fills
- **Multi-resolver coordination** in canister logic

#### **For Relayer/Resolver (Stretch Goal 3):**

- **Automated canister interaction** scripts
- **ICP event monitoring** service
- **Cross-chain coordination** automation

---

## Creation Approaches

### **✅ Option 1: Direct Canister Calls** _(MVP)_

- **Process:** Resolver calls our canister methods directly
- **Tool:** `dfx canister call` commands
- **Method:** Direct canister interaction
- **Advantage:** Simple, direct control
- **Disadvantage:** Manual process

### **✅ Option 2: API Wrapper** _(Stretch Goal)_

- **Process:** Resolver calls our API wrapper
- **Tool:** HTTP API endpoints
- **Method:** REST API calls to our service
- **Advantage:** Standard API interface
- **Disadvantage:** Additional infrastructure

---

## Our Implementation

### **What We Build:**

- **ICP escrow canisters** (our main development)
- **Canister methods** for escrow creation
- **Hashlock and timelock** logic
- **Token handling** for ICP tokens

### **Resolver's Role:**

- **Calls our canisters** with escrow parameters
- **Provides ICP tokens** for the swap
- **Handles cycle costs** for ICP transactions

### **✅ MVP Testing Approach:**

**For the MVP demo, we will act as resolver:**

- **Deploy our canisters** to ICP testnet
- **Call our canister methods** directly via dfx
- **Provide test ICP tokens** for liquidity
- **Coordinate with Ethereum escrows** manually

#### **Cycle Cost Awareness:**

- **Depositors (resolvers)** must have enough cycles for escrow creation
- **Not just tokens** - cycles are required for canister operations
- **Cycle management** is part of resolver responsibilities

---

## Key Functions

### **Canister Methods:**

- **Create escrow** with hashlock and timelock
- **Deposit tokens** into escrow
- **Set escrow parameters** (amounts, tokens)

### **✅ Reference Implementations:**

#### **Secretus Examples:**

- **goulHash (EVM to Cardano)** - [secretus/goulHash](secretus/goulHash/)
  - **HTLC implementation** with same hash function as Fusion+
  - **Deterministic addressing** pattern
  - **Cross-chain coordination** approach

#### **1inch Cross-Chain Swap:**

- **EscrowDst pattern** - [1inch cross-chain-swap](docs/cross-chain-swap/)
  - **Destination chain escrow** implementation
  - **Same secret hash** linking both chains
  - **Timelock mechanisms** for safety

#### **First Attempt Implementation (Reusable):**

- **Swap Canister** - [first_bash/icp/src/swap_canister](../first_bash/icp/src/swap_canister/)
  - **Complete HTLC implementation** in Rust (859 lines)
  - **ICRC-1 token integration** with proper error handling
  - **Hashlock verification** using SHA256
  - **Timelock management** in nanoseconds
  - **Ethereum verification** via HTTP outcalls
  - **Performance optimization** with caching
  - **Comprehensive error handling** and validation

#### **HTTP Outcall Module (Reusable):**

- **Cross-chain communication** - [first_bash/icp/src/http_outcall_module](../first_bash/icp/src/http_outcall_module/)
  - **Ethereum JSON-RPC integration**
  - **Transaction verification** capabilities
  - **Event monitoring** for Ethereum state
  - **Security headers** and response sanitization
  - **Comprehensive error handling**

#### **Solana Fusion Protocol (Reference):**

- **Fusion Protocol Implementation** - [secretus/solana-fusion-protocol](secretus/solana-fusion-protocol/)
  - **Complete Fusion protocol** in Rust (855 lines)
  - **Dutch auction mechanism** with rate bump calculations
  - **Deterministic escrow addressing** using PDA seeds
  - **Fee management** (protocol, integrator, surplus fees)
  - **Order hash computation** for escrow identification
  - **Cancellation premium** calculations
  - **Whitelist-based resolver access** control

#### **SwappaTEE Cross-Chain Resolver (Reference):**

- **Resolver Implementation** - [secretus/SwappaTEE](secretus/SwappaTEE/)
  - **Cross-chain resolver contract** in Solidity (99 lines)
  - **Safety deposit handling** for escrow deployment
  - **Deterministic escrow addressing** via factory
  - **Order filling integration** with Limit Order Protocol
  - **Arbitrary call execution** for complex operations
  - **Timelock management** for deployment coordination

---

## Outputs

### **For Step 2.4:**

- **ICP escrow canister** (our implementation)
- **Escrow parameters** (hashlock, timelock)
- **Resolver tokens locked** in ICP escrow

---

## MVP Recommendation

### **Direct Canister Calls:**

- **Resolver uses `dfx`** to call our canisters
- **Simple canister methods** for escrow creation
- **Direct interaction** - no API layer needed

### **✅ Development Scope:**

- **ICP side:** Build our escrow canisters (our main development work)
- **Ethereum side:** Use existing 1inch contracts (no development)
- **Integration:** Manual coordination for MVP demo
- **Testing:** Act as resolver to test our canisters

### **✅ Tools We Need:**

- **dfx CLI** for canister deployment and interaction
- **ICP testnet** for deployment and testing
- **Test ICP tokens** for escrow deposits
- **Rust development environment** for canister development
- **HTTP outcalls** for Ethereum verification
- **Cycle management tools** for testing and deployment
