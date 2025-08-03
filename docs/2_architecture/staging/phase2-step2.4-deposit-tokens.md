# Phase 2 Step 2.4: Deposit Tokens

_Tokens deposited in both escrows for atomic swap_

---

## Overview

**Who funds what:**

1. **User funds Ethereum escrow** - After the Fusion+ order is filled, user deposits their tokens into the Ethereum escrow contract by calling the `deposit()` function on the deployed `EscrowSrc` instance (requires user to sign transaction via MetaMask or similar wallet)
2. **Resolver funds ICP escrow** - Resolver deposits their tokens into our ICP escrow canister by calling our `deposit_tokens()` method via `dfx canister call` (requires resolver to have ICP tokens and cycles)

**How the funding happens:**

- **Ethereum side:** User manually calls the existing 1inch escrow smart contract to deposit their tokens (no UI involved)
- **ICP side:** Resolver manually calls our canister to deposit their tokens
- **Atomic requirement:** Both escrows must be funded before the swap can proceed
- **Cross-chain coordination:** Manual verification that both escrows are funded with the same hashlock and timelock

---

## Required Inputs

### **From Step 2.3:**

- **ICP escrow canister** (our implementation)
- **Escrow parameters** (hashlock, timelock)
- **Resolver tokens locked** in ICP escrow

---

## Deposit Approaches

### **✅ Option 1: Manual Deposits** _(MVP)_

- **Process:** Manual token transfers to escrows
- **Ethereum:** User deposits via existing 1inch contracts
- **ICP:** Resolver deposits via our canisters
- **Advantage:** Simple, direct control
- **Disadvantage:** Manual process

### **✅ Option 2: Automated Deposits** _(Stretch Goal)_

- **Process:** Automated token transfers
- **Ethereum:** Automated via 1inch contracts
- **ICP:** Automated via our canisters
- **Advantage:** Automated, efficient
- **Disadvantage:** Complex automation

---

## Deposit Flow

### **Ethereum Side:**

- **User deposits** tokens into Ethereum escrow
- **Via existing** 1inch contracts
- **We don't control** this process

### **ICP Side:**

- **Resolver deposits** tokens into ICP escrow
- **Via our canisters** (our implementation)
- **We control** this process

---

## Key Points

### **Atomic Requirement:**

- **Both escrows** must be funded
- **Same hashlock** used in both
- **Same timelock** duration
- **Either both succeed** or both fail

### **Safety Deposits:**

- **Resolver provides** safety deposits
- **Incentivizes** proper execution
- **Penalties** for misbehavior

---

## Outputs

### **For Phase 3:**

- **Deposit confirmation** on both chains
- **Both escrows funded** and ready for execution
- **Atomic swap** ready to proceed

---

## MVP Recommendation

### **Manual Deposits:**

- **User manually deposits** in Ethereum
- **Resolver manually deposits** in ICP
- **Simple and working** for MVP demo

---

## Implementation

### **✅ MVP Implementation (Manual Process):**

**What We Actually Do:**

#### **Ethereum Side (No Code - Use Existing):**

- **User calls** existing 1inch `EscrowSrc` contract via MetaMask
- **Method:** `deposit()` function in deployed escrow
- **Tool:** MetaMask + Ethers.js for transaction
- **Verification:** Etherscan API to confirm transaction

**Actual Process:**

**User deposits to Ethereum escrow:**

- **ICP NOT integrated in 1inch frontend yet** - No UI option available
- User needs to sign transactions - requires wallet interaction
- **Option 1:** Basic frontend with MetaMask integration
- **Option 2:** Command-line script with private key (less secure)
- **Option 3:** Manual MetaMask interaction with contract address

**User signing options:**

**Option 1 - Basic Frontend (Recommended):**

- Simple web page with MetaMask integration
- User connects wallet and approves transaction
- Frontend calls escrow contract deposit function
- MetaMask prompts user to sign transaction

**Option 2 - Script with Private Key:**

- User provides private key to script
- Script signs and sends transaction
- Less secure - private key exposure

**Option 3 - Manual MetaMask:**

- User manually enters contract address in MetaMask
- User manually calls deposit function
- User provides all parameters manually

#### **ICP Side (Our Rust Canister):**

- **Resolver calls** our canister via `dfx canister call`
- **Method:** `deposit_tokens(escrow_id: String, amount: u64)`
- **Tool:** `dfx` CLI + our Rust canister
- **Verification:** Canister query to check balance

**Actual Process:**

**Resolver deposits to ICP escrow:**

- Resolver runs dfx command with escrow ID and amount
- dfx sends call to our deployed canister
- Canister receives deposit_tokens method call
- Canister transfers tokens from resolver to escrow
- Canister updates internal state
- Resolver verifies deposit via balance query

**Manual verification:**

- Resolver checks escrow balance via dfx query
- Resolver confirms tokens are locked in escrow
- Resolver verifies hashlock and timelock match

#### **Cross-Chain Coordination (Manual Scripts):**

- **JavaScript/TypeScript script** to monitor both chains
- **Ethers.js** for Ethereum transaction monitoring
- **dfx commands** for ICP canister interaction
- **Manual verification** of both escrow balances

### **Testing Tools Needed:**

#### **Ethereum Tools:**

- **Ethers.js** - Monitor transactions and balances
- **Etherscan API** - Verify transaction confirmations
- **MetaMask** - User wallet for deposits

#### **ICP Tools:**

- **dfx CLI** - Canister deployment and interaction
- **Rust canister** - Our escrow implementation
- **ICRC-1 interface** - Token transfer methods

#### **Coordination Tools:**

- **Node.js script** - Monitor both chains
- **Balance verification** - Confirm both escrows funded
- **Timing coordination** - Ensure atomic requirement

### **Stretch Goals Implementation:**

#### **Automated Coordination:**

- **Rust backend service** - Automated cross-chain monitoring
- **TypeScript automation** - Scripts for both chains
- **Database tracking** - Escrow state management

#### **Atomic Guarantees:**

- **Cross-chain transaction coordination** - Synchronized deposits
- **Rollback mechanisms** - Handle partial failures with manual claim/refund process
- **State management** - Track deposit status
- **Timeout watcher script** - Monitor timelock expiration (stretch goal)

#### **Failure Recovery:**

- **Partial deposit handling** - Refund mechanisms
- **Timeout management** - Handle network delays
- **Error recovery** - Automatic retry logic

---

## Reference Implementations

### **From First Attempt ICP Implementation:**

- **Token transfer patterns** - ICRC-1 deposit mechanisms
- **Balance verification** - Escrow funding confirmation
- **Error handling** - Deposit failure scenarios

### **From SwappaTEE:**

- **Safety deposit handling** - Incentive mechanisms
- **Cross-chain coordination** - Timing and verification
- **Failure scenarios** - Partial deposit handling

### **From Solana Fusion Protocol:**

- **Escrow funding patterns** - Token deposit workflows
- **Balance management** - Escrow account handling
- **Fee handling** - Protocol fee deposits

---

## Technical Details

### **Ethereum Deposit Process:**

1. **User calls** existing 1inch escrow contract
2. **Tokens transferred** to Ethereum escrow
3. **Escrow funded** with user's tokens
4. **Transaction confirmed** on Ethereum

### **ICP Deposit Process:**

1. **Resolver calls** our canister deposit method
2. **Tokens transferred** to ICP escrow canister
3. **Escrow funded** with resolver's tokens
4. **Canister state updated** on ICP

### **Verification Requirements:**

- **Both escrows funded** before proceeding
- **Same hashlock** in both escrows
- **Same timelock** duration
- **Safety deposits** provided by resolver
- **ICP escrow funded before timelock expires** on Ethereum side to prevent user refund

### **Failure Scenarios:**

- **Partial funding** - One escrow funded, other not
- **Timing mismatch** - Deposits too far apart
- **Insufficient funds** - Not enough tokens for deposit
- **Network issues** - Transaction failures

---

## Integration with Phase 3

### **Prerequisites for Phase 3:**

- **Both escrows confirmed funded**
- **Hashlock verified** in both chains
- **Timelock synchronized** between chains
- **Safety deposits** in place

### **Handoff to Phase 3:**

- **Atomic swap ready** to execute
- **Secret revelation** can begin
- **Cross-chain verification** complete
