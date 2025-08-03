# 1inch Fusion+ Flow Overview

_Simple technical overview of the 4-phase process_

---

## 4-Phase Process

### **Phase 1: Announcement** _(Manual Testing Only - ICP Not Integrated Yet)_

- **Step 1.1: Get Quote** - Query pricing and auction parameters
- **Step 1.2: Sign Intent** - EIP-712 signature with quote data
- **Step 1.3: Submit Order** - Send signed order to 1inch Fusion+ API
- **Dutch Auction** - 1inch servers run Dutch auction among resolvers
- **Resolver Selection** - Winning resolver gets exclusive rights to fill order
- **Resolver monitors** API for new orders

**⚠️ Important:** ICP is NOT integrated in 1inch frontend yet - manual testing required. See [Phase 1 Step 1.2](phase1-step1.2-sign-intent.md#phase-1-announcement---current-state--limitations) for detailed explanation.

### **Phase 2: Deposit**

- **Step 2.1: Monitor Orders** - Resolver monitors 1inch API for new orders
- **Step 2.1.5: Check Auction Status** - Resolver checks if they won the Dutch auction
- **Step 2.2: Get Escrow Factory Address** - Resolver fetches current factory address
- **Step 2.3: Create Ethereum Escrow** - Resolver creates escrow via existing 1inch contracts
- **Step 2.4: Create ICP Escrow** - Resolver creates escrow via our canisters
- **Step 2.5: Deposit Tokens** - Tokens deposited in both escrows

### **Phase 3: Execution**

- **Resolver reveals secret** to both escrows
- **Tokens unlocked** on both chains
- **Atomic swap completed**

### **Phase 4: Recovery** _(Implicit HTLC Requirement)_

- **Timeout scenarios** handled
- **Public withdrawal/cancellation** available
- **Note:** Required for complete HTLC functionality, not explicitly mentioned in subject requirements

---

## Detailed Implementation Documents

### **Phase 1: Announcement**

- [Phase 1 Step 1.2: Sign Intent](phase1-step1.2-sign-intent.md) - All possible approaches for EIP-712 intent signing
- [Phase 1 Step 1.3: Submit Order](phase1-step1.3-submit-order.md) - All possible approaches for submitting signed orders

### **Phase 2: Deposit**

- [Phase 2 Step 2.1: Monitor Orders](phase2-step2.1-monitor-orders.md) - Resolver monitors 1inch API for new orders
- [Phase 2 Step 2.1.5: Check Auction Status](internal/apis/swap/fusion-plus/order_get-auction-status.md) - Resolver checks if they won the Dutch auction
- [Phase 2 Step 2.2: Get Escrow Factory Address](internal/apis/swap/intent_swaps_fusion/endpoints/order_get-escrow-factory.md) - Resolver fetches current factory address
- [Phase 2 Step 2.3: Create Ethereum Escrow](phase2-step2.2-create-eth-escrow.md) - Resolver creates escrow using existing 1inch contracts
- [Phase 2 Step 2.4: Create ICP Escrow](phase2-step2.3-create-icp-escrow.md) - Resolver creates escrow using our ICP canisters
- [Phase 2 Step 2.5: Deposit Tokens](phase2-step2.4-deposit-tokens.md) - Tokens deposited in both escrows

### **Phase 3: Execution**

- [Phase 3: Execution](phase3-execution.md) - Atomic swap execution - secret revelation and token transfer

### **Phase 4: Recovery**

- [Phase 4: Recovery](phase4-recovery.md) - Timeout handling and public withdrawal/cancellation

---

## Communication Flow

### **End User → Frontend → 1inch API**

- User interacts with frontend (dApp)
- Frontend calls 1inch Fusion+ APIs
- APIs handle order submission and **Dutch auction execution**
- **1inch servers** run Dutch auction among resolvers
- **Winning resolver** gets exclusive rights to fill the order

### **Resolver → 1inch API → Smart Contracts**

- Resolver (professional taker) monitors orders via API
- When resolver accepts order, they interact with smart contracts
- Resolver deposits funds into escrows

### **Direct Escrow Interactions**

#### **Normal Flow (via 1inch API):**

- **Resolver** → **1inch API** → **Ethereum Escrow Contract** (deposits user's ETH)
- **Resolver** → **1inch API** → **ICP Escrow Contract** (deposits their ICP)
- **Resolver** → **1inch API** → **Both Escrows** (reveals secret to unlock funds)

#### **Timeout Scenarios (direct contract interaction):**

- **User** → **Ethereum Escrow** (withdraws ICP if timeout)
- **User** → **ICP Escrow** (withdraws ETH if timeout)

_Note: In normal operation, users interact via 1inch API. Direct escrow interaction only happens during recovery/timeout scenarios._

---

## Dutch Auction Process

### **✅ How 1inch Dutch Auction Works:**

#### **1. Order Submission:**

- **Maker** submits signed order to 1inch Fusion+ API
- **Order** is broadcast to all authorized resolvers
- **Dutch auction** begins automatically on 1inch servers

#### **2. Auction Competition:**

- **Price decreases automatically** over time according to maker's curve
- **Resolvers watch** for profitable price points
- **First resolver to execute** on-chain wins the order
- **No traditional bidding** - it's a race to execute at the right price

#### **3. Resolver Assignment:**

- **First resolver to execute** on-chain wins the order
- **Other resolvers** are excluded once order is filled
- **Auction expires** if no resolver executes before end price

**⚠️ Important:** The Dutch auction is handled entirely by 1inch infrastructure. Our resolver needs to monitor orders and race to execute when the price becomes profitable.

---

## Resolver Interaction Flow

### **✅ How Resolver Creates Escrows:**

#### **1. Resolver Monitors Orders:**

- **Technical:** `curl` calls to 1inch Fusion+ API
- **Process:** Resolver checks for new orders to fill
- **API Endpoint:** `GET /fusion/orders/v2.0/{chain}/order/active`

#### **1.5. Resolver Checks Auction Status:**

- **Technical:** `GET /fusion-plus/orders/v1.0/order/{orderHash}/auction/status`
- **Process:** Resolver checks if they won the Dutch auction for specific orders
- **Result:** `WON`, `LOST`, `PENDING`, or `EXPIRED` status

#### **2. Resolver Gets Escrow Factory Address:**

- **Technical:** `GET /fusion-plus/orders/v1.0/order/escrow?chainId={chainId}`
- **Process:** Resolver fetches current escrow factory contract address
- **Result:** Dynamic factory address for destination chain

#### **3. Resolver Creates Ethereum Escrow:**

- **Technical:** Via 1inch Limit Order Protocol contracts
- **Process:** Resolver fills order using existing 1inch contracts
- **Result:** Ethereum escrow created with user's tokens

#### **3. Resolver Creates ICP Escrow:**

- **Technical:** Direct canister calls via `dfx`
- **Process:** Resolver calls our ICP canister to create escrow
- **Result:** ICP escrow created with resolver's tokens

#### **4. Resolver Executes Swap:**

- **Technical:** Reveals secret to both escrows
- **Process:** Same secret unlocks tokens on both chains
- **Result:** Atomic swap completed

### **✅ Key Insight:**

**The resolver is the first to interact with our ICP canisters** - they create the escrows and manage the entire swap execution process.

### **Smart Contracts Communication**

- **Ethereum escrow** ↔ **ICP escrow** (via secret hash)
- **No direct communication** - linked by shared secret
- **Atomic execution** - both succeed or both fail
