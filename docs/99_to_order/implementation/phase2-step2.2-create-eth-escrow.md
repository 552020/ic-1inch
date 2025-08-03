# Phase 2 Step 2.2: Create Ethereum Escrow

_Resolver creates escrow using existing 1inch contracts_

---

## Overview

Resolver creates an Ethereum escrow using existing 1inch infrastructure to hold user's tokens.

---

## Required Inputs

### **From Step 2.1:**

- **Order details** (amounts, tokens, timelock)
- **Secret hash** (for escrow creation)
- **Order parameters** (for escrow setup)

---

## Implementation

### **✅ MVP Implementation (No Custom Code Required):**

**We do NOT build Ethereum escrow contracts ourselves:**

- **Use existing 1inch infrastructure** - contracts already deployed
- **Follow standard 1inch process** - documented above
- **No Solidity development** - this is NOT our implementation work

**We DO need testing tools for the flow:**

- **Scripts to call 1inch contracts** (using existing functions)
- **Order monitoring tools** (to track our test orders)
- **Secret management utilities** (for hashlock testing)
- **Integration testing scripts** (to coordinate ETH + ICP)

### **✅ Stretch Goals Implementation:**

**If we implement stretch goals, we would need:**

#### **For UI (Stretch Goal 1):**

- **Frontend integration** with 1inch contracts
- **Wallet connection** for Ethereum transactions (MetaMask integration)
- **Order management interface** for resolver role
- **Ethers.js-based forms** for escrow creation and order filling
- **Automated demo interface** for streamlined testing

#### **For Partial Fills (Stretch Goal 2):**

- **Enhanced testing tools** for partial fill scenarios
- **Merkle tree utilities** for secret management
- **Multi-resolver coordination** scripts

#### **For Relayer/Resolver (Stretch Goal 3):**

- **Automated resolver bot** that calls 1inch contracts
- **Order monitoring service** for Ethereum events
- **Cross-chain coordination** automation

---

## Creation Process

### **✅ Official Process (from 1inch Fusion+ Whitepaper):**

#### **Phase 2: Deposit Phase (Official Documentation)**

**Source:** [1inch Fusion+ Whitepaper](docs/1inch/1inch-fusion-plus-whitepaper.md#phase-2-deposit-phase)

> "3. The resolver deposits the maker's tokens into the source chain escrow contract. The escrow incorporates the secret hash, token type and amount, target address, and timelock specifications for both chains."

#### **Step 1: Get Future Escrow Address**

- **Function:** `EscrowFactory.addressOfEscrowSrc(immutables)`
- **Purpose:** Get deterministic address of future escrow
- **Action:** Send safety deposit to this address
- **Source:** [1inch cross-chain-swap README.md](docs/cross-chain-swap/README.md#functions-for-resolver-to-use)
- **Contract Address:** Sepolia testnet addresses available in 1inch documentation

#### **Step 2: Fill Order via Limit Order Protocol**

- **Function:** `OrderMixin.fillOrderArgs()` or `fillContractOrderArgs()`
- **Purpose:** Fill the Fusion order and deploy `EscrowSrc` clone
- **Result:** Ethereum escrow created with user's tokens
- **Source:** [1inch cross-chain-swap README.md](docs/cross-chain-swap/README.md#functions-for-resolver-to-use)
- **Contract Address:** OrderMixin contract address on Sepolia testnet

#### **Safety Deposit Handling:**

- **Must be sent before order filling** - if not sent, the fill transaction will fail
- **Amount:** Determined by escrow parameters (typically small amount of native token)
- **Purpose:** Incentivizes proper execution and covers transaction costs
- **Recovery:** Safety deposit goes to the executor of withdrawal/cancellation

### **✅ Key Points (from Official Documentation):**

- **Proxy Pattern:** Each swap gets a unique proxy contract
- **Deterministic Address:** Escrow address computed from parameters
- **Safety Deposit:** Must be sent before order filling
- **Limit Order Protocol:** Standard 1inch order filling mechanism
- **Source:** [1inch cross-chain-swap README.md](docs/cross-chain-swap/README.md#one-time-contracts-for-each-swap)

---

## Key Points

### **We Don't Build This (MVP Context):**

- **No custom Solidity contracts** - use existing 1inch contracts
- **No custom deployment** - 1inch handles infrastructure
- **No custom logic** - standard 1inch escrow behavior
- **No Ethereum development** - this is NOT our implementation work

### **Resolver's Role (Manual for MVP):**

- **Calls existing contracts** with order parameters
- **Provides liquidity** (resolver's tokens)
- **Handles gas costs** for Ethereum transactions
- **Sends safety deposit** to computed escrow address
- **Fills order** via Limit Order Protocol

### **✅ MVP Implementation:**

**For the MVP demo, we will act as BOTH user AND resolver:**

#### **As User:**

- **Submit orders** via 1inch API (manual)
- **Sign intents** with our wallet (manual)
- **Monitor order status** (manual)

#### **As Resolver:**

- **Monitor for our own orders** (manual)
- **Act as resolver** using our own tokens
- **Call existing 1inch contracts** on Ethereum
- **Call our ICP canisters** for ICP escrows
- **Manage secret revelation** and escrow unlocking

#### **Why We Must Act as Resolver:**

- **No existing 1inch resolver** handles ICP ↔ ETH swaps yet
- **We're testing end-to-end** cross-chain functionality
- **We need to demonstrate** the complete flow
- **We provide liquidity** for our own test swaps

---

## Outputs

### **For Step 2.3:**

- **Ethereum escrow address** (existing 1inch contract)
- **User tokens locked** in Ethereum escrow
- **Escrow parameters** (hashlock, timelock)

---

## Reference Implementations

### **✅ Secretus Examples:**

#### **1. Solana Fusion Protocol**

- **Source:** [secretus/solana-fusion-protocol](secretus/solana-fusion-protocol/)
- **Description:** 1inch Fusion implementation for Solana chain
- **Key Features:** Dutch auction mechanism, resolver competition, MEV-resistant swapping

#### **2. goulHash (EVM to Cardano)**

- **Source:** [secretus/goulHash](secretus/goulHash/)
- **Description:** Proof of concept for EVM to Cardano cross-chain swap using Fusion+
- **Key Features:** HTLC implementation, same hash function as Fusion+, deterministic addressing

#### **3. SwappaTEE (Cross-chain Resolver)**

- **Source:** [secretus/SwappaTEE](secretus/SwappaTEE/)
- **Description:** Cross-chain resolver example with TEE (Trusted Execution Environment)
- **Key Features:** Multi-chain support, resolver automation, security through TEE

### **✅ Key Insights from Reference Implementations:**

- **HTLC Pattern:** All implementations use the same hashlock/timelock pattern
- **Deterministic Addressing:** Escrow addresses computed from parameters
- **Resolver Role:** Professional market makers execute the swaps
- **Cross-Chain Coordination:** Secret sharing enables atomic execution

---

## Validation

### **✅ Official Confirmation: Direct Contract Interaction**

**Multiple official sources confirm that resolvers interact directly with Ethereum escrow contracts during Step 2.2:**

#### **1. 1inch Help Center Guide**

> "The winning resolver **deposits the maker's assets into an escrow contract on the source chain**, and then deposits the corresponding assets into an escrow on the destination chain."
> — meaning resolvers interact with native chain specs (e.g. ETH contracts) directly

#### **2. Fusion+ Whitepaper**

> "The resolver deposits the maker's tokens into the source chain escrow contract… then deposits the taker amount into the escrow contract on the destination chain…"
> This underscores that the resolver **uses standard 1inch escrow contracts on both chains**

#### **3. Cross-Chain Swap Repository**

The official `1inch/cross-chain-swap` repository clearly outlines the resolver's on-chain interactions:

- Use `EscrowFactory.addressOfEscrowSrc()` to compute the Ethereum escrow address
- Call `fillOrderArgs()` on the Limit Order Protocol to deploy and fund the **EscrowSrc** on Ethereum
- Then call `EscrowFactory.createDstEscrow()` on the **destination chain** to deploy **EscrowDst**

### **✅ Architecture Confirmation**

**This validates our understanding:**

- **Step 2.2:** Resolver → Ethereum escrow contracts (direct interaction)
- **Step 2.3:** Resolver → ICP canisters (direct interaction)
- **No cross-chain mediation** during escrow creation
- **Independent escrow creation** on each chain
- **Linked only by shared secret hash** for atomic execution

### **✅ Documentation Accuracy**

Our documentation correctly states:

- Resolver calls existing 1inch contracts directly
- No custom Solidity development needed
- Standard 1inch escrow creation process
- ICP canisters are NOT involved in Step 2.2

---

## MVP Recommendation

### **Use Existing Infrastructure (No Coding Required):**

- **Resolver calls** existing 1inch escrow contracts
- **Standard process** - no custom development
- **Proven and tested** infrastructure
- **Manual execution** for MVP demo

### **✅ What We Actually Do for MVP:**

#### **Dual Role Testing:**

1. **As User:** Submit order via 1inch API (manual)
2. **As Resolver:** Monitor for our own orders (manual)
3. **As Resolver:** Call existing 1inch contracts on Ethereum
4. **As Resolver:** Call our ICP canisters for ICP escrows
5. **As Resolver:** Manage secret revelation and unlocking

#### **Tools We Need:**

- **Wallet for ETH Sepolia** (MetaMask + 1inch order signer)
- **Our ICP canister** deployed to testnet
- **Scripts or manual process** acting as resolver
- **Test tokens** on both chains for liquidity
- **Contract addresses** for Sepolia testnet (EscrowFactory, OrderMixin)
- **Ethers.js/web3.js** for contract interactions

### **✅ Development Scope:**

- **Ethereum side:** Use existing 1inch contracts (no development)
- **ICP side:** Build our escrow canisters (our development work)
- **Integration:** Manual coordination for MVP demo
- **Testing:** Act as both user and resolver for end-to-end validation
