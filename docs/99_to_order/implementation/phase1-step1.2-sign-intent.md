# Phase 1 Step 1.2: Sign Intent

_All possible approaches for EIP-712 intent signing_

---

## Overview

Step 1.2 involves creating a cryptographically signed intent using EIP-712 standard. This step requires wallet interaction and cannot be done via `curl` alone.

**⚠️ Important:** This step is particularly affected by the fact that ICP is NOT integrated in 1inch frontend yet, requiring manual testing approaches.

---

## **🔑 Critical EIP-712 Delegation Mechanism**

### **The Real 1inch Fusion+ Architecture**

**Key Insight:** The signed EIP-712 intent authorizes the resolver to lock MAKER's tokens in the source escrow, while the resolver locks THEIR OWN tokens in the destination escrow.

#### **Step 1: Maker Signs Intent (NO Blockchain Transaction)**

```javascript
// Maker ONLY signs EIP-712 intent (off-chain)
// NO blockchain transaction from maker
const signedIntent = await maker.signFusionPlusOrder({
  makingAmount: "1000000000000000000", // 1 ETH
  takingAmount: "5000000000000000000", // 5000 ICP
  // ... other parameters
});

// Maker sends signed intent to relayer
await relayerService.submitIntent(signedIntent);
```

#### **Step 2: Resolver Creates Both Escrows Using Factory**

```solidity
// Resolver uses smart contract factory to create both escrows
// Source escrow: Locks MAKER's tokens using signed intent
await escrowFactory.createSourceEscrow(signedIntent); // Locks maker's ETH

// Destination escrow: Resolver locks THEIR OWN tokens
await escrowFactory.createDestinationEscrow(
  resolverTokens, // Resolver's own tokens
  hashlock,       // Same hashlock as source escrow
  timelock        // Same timelock as source escrow
); // Locks resolver's ICP
```

### **🎯 The Correct Delegation Mechanism**

The **signed EIP-712 intent authorizes the resolver to:**

1. **Lock maker's tokens** in the source escrow (using signed intent)
2. **Lock resolver's own tokens** in the destination escrow (resolver's tokens)
3. **Execute the swap** when secret is revealed

### **🔄 For Our ICP Implementation**

#### **The Problem:**

- **Maker signs EIP-712 on Ethereum** (off-chain)
- **Source escrow is on ICP** (our implementation)
- **How does ICP escrow verify maker's authorization to lock their tokens?**

#### **The Solution: Cross-Chain Signature Verification**

```rust
// ICP escrow accepts Ethereum EIP-712 signatures
#[update]
pub async fn create_source_escrow_with_ethereum_signature(
    signed_intent: EthereumSignedIntent,
    resolver_principal: Principal
) -> Result<String, EscrowError> {
    // Verify EIP-712 signature on ICP
    verify_ethereum_signature(signed_intent)?;

    // Lock maker's tokens using the verified authorization
    lock_maker_tokens(signed_intent).await
}

// Resolver creates destination escrow with their own tokens
#[update]
pub async fn create_destination_escrow(
    resolver_tokens: u64,
    hashlock: Vec<u8>,
    timelock: u64
) -> Result<String, EscrowError> {
    let caller = ic_cdk::caller(); // Resolver's principal

    // Lock resolver's own tokens
    lock_resolver_tokens(caller, resolver_tokens, hashlock, timelock).await
}
```

### **🔧 Implementation Approaches**

#### **Option 1: HTTP Outcall to Verify Ethereum Signature**

```rust
// ICP canister verifies Ethereum signature via HTTP outcall
async fn verify_ethereum_signature(
    signed_intent: EthereumSignedIntent
) -> Result<bool, EscrowError> {
    // Call Ethereum RPC to verify signature
    let verification_result = http_outcall::verify_signature(
        signed_intent.signature,
        signed_intent.order,
        signed_intent.maker_address
    ).await?;

    Ok(verification_result.verified)
}
```

#### **Option 2: Pre-verified Intent from Relayer**

```rust
// Relayer pre-verifies signature and provides proof
#[update]
pub async fn create_source_escrow_with_verified_intent(
    verified_intent: VerifiedIntent,
    resolver_principal: Principal
) -> Result<String, EscrowError> {
    // Relayer provides cryptographic proof of verification
    verify_intent_proof(verified_intent)?;

    // Lock maker's tokens
    lock_maker_tokens(verified_intent).await
}
```

### **🔄 Complete Flow for ICP**

#### **Step 1: Maker Signs Intent (Off-Chain)**

```javascript
// Maker signs EIP-712 intent (NO blockchain transaction)
const signedIntent = await maker.signFusionPlusOrder({
  makingAmount: "1000000000000000000", // 1 ETH
  takingAmount: "5000000000000000000", // 5000 ICP
  // ... other parameters
});

// Send to relayer
await relayerService.submitIntent(signedIntent);
```

#### **Step 2: Resolver Creates Source Escrow (Maker's Tokens)**

```rust
// Resolver calls ICP canister with maker's signed intent
#[update]
pub async fn create_source_escrow_with_ethereum_signature(
    signed_intent: EthereumSignedIntent,
    resolver_principal: Principal
) -> Result<String, EscrowError> {
    // Verify Ethereum signature on ICP
    verify_ethereum_signature(signed_intent)?;

    // Lock maker's ICP tokens (not resolver's tokens)
    lock_maker_tokens(signed_intent).await
}
```

#### **Step 3: Resolver Creates Destination Escrow (Resolver's Tokens)**

```rust
// Resolver locks their own tokens in destination escrow
#[update]
pub async fn create_destination_escrow(
    resolver_tokens: u64,
    hashlock: Vec<u8>,
    timelock: u64
) -> Result<String, EscrowError> {
    let caller = ic_cdk::caller(); // Resolver's principal

    // Lock resolver's own tokens
    lock_resolver_tokens(caller, resolver_tokens, hashlock, timelock).await
}
```

#### **Step 4: Atomic Execution**

```rust
// When secret is revealed, both escrows complete
// Maker's tokens go to resolver, resolver's tokens go to maker
claim_source_escrow(source_escrow_id, preimage).await;
claim_destination_escrow(destination_escrow_id, preimage).await;
```

### **✅ Key Architectural Corrections**

1. **❌ Wrong:** Resolver locks their own tokens in both escrows
   **✅ Correct:** Resolver locks MAKER's tokens in source escrow, THEIR OWN tokens in destination escrow

2. **❌ Wrong:** Maker performs blockchain transactions
   **✅ Correct:** Maker only signs off-chain intent

3. **❌ Wrong:** Both escrows use same token source
   **✅ Correct:** Source escrow uses maker's tokens, destination escrow uses resolver's tokens

4. **❌ Wrong:** ICP needs separate delegation mechanism
   **✅ Correct:** ICP needs to verify Ethereum EIP-712 signatures for source escrow only

### ** The Real Challenge**

**How do we verify Ethereum EIP-712 signatures on ICP for the source escrow?**

This is the core technical challenge - we need to implement cross-chain signature verification so that our ICP source escrow can trust that the maker authorized the resolver to lock their tokens.

The destination escrow is simpler - resolver just locks their own tokens directly.

---

## Required Inputs

### **From Step 1.1 (Quote):**

- **Quote data** (pricing, auction parameters)
- **QuoteId** (required for order submission)
- **Presets** (fast/medium/slow auction options)

### **Additional Requirements:**

- **Wallet/Private key** (for EIP-712 signing)
- **Order data preparation** (structured data for signing)

---

## Signing Approaches

### **✅ Option 1: Command-Line Scripts** _(Recommended for MVP)_

#### **Node.js with ethers.js:**

- **Library:** `ethers.js` for EIP-712 signing

#### **Python with web3.py:**

- **Library:** `web3.py` for EIP-712 signing

#### **Shell Scripts with Wallet Tools:**

- **Tools:** Hardhat, Foundry, or custom wallet CLI

### **✅ Option 2: Backend Service**

#### **Server-Side Signing:**

...

#### **Microservice Approach:**

...

### **✅ Option 3: Web Application/Frontend** _(Most Common Approach)_

#### **React/Vue/Angular Web App:**

- **Architecture:** Standard web application

#### **MetaMask Integration:**

- **Architecture:** Web page with MetaMask connection

#### **WalletConnect Integration:**

- **Architecture:** Web app with WalletConnect

### **✅ Option 4: Browser Extensions**

#### **Web Extension:**

- **Architecture:** Chrome/Firefox extension

### **✅ Option 5: Mobile Solutions**

#### **Mobile App:**

- **Architecture:** Native mobile application

#### **Wallet App Integration:**

- **Architecture:** Integration with existing wallet apps

## EIP-712 Structure

### **Domain Data:**

```json
{
  "name": "1inch Fusion",
  "version": "2.0",
  "chainId": 1,
  "verifyingContract": "settlement_address"
}
```

### **Types:**

```json
{
  "Order": [
    { "name": "salt", "type": "bytes32" },
    { "name": "makerAsset", "type": "address" },
    { "name": "takerAsset", "type": "address" },
    { "name": "maker", "type": "address" },
    { "name": "receiver", "type": "address" },
    { "name": "makingAmount", "type": "uint256" },
    { "name": "takingAmount", "type": "uint256" },
    { "name": "makerTraits", "type": "uint256" }
  ]
}
```

### **Order Data:**

```json
{
  "salt": "random_32_byte_salt",
  "makerAsset": "from_token_address",
  "takerAsset": "to_token_address",
  "maker": "user_wallet_address",
  "receiver": "user_wallet_address",
  "makingAmount": "amount_from_quote",
  "takingAmount": "amount_from_quote",
  "makerTraits": "0"
}
```

---

## Output Format

### **Signed Order Structure:**

```json
{
  "order": {
    "salt": "string",
    "makerAsset": "string",
    "takerAsset": "string",
    "maker": "string",
    "receiver": "string",
    "makingAmount": "string",
    "takingAmount": "string",
    "makerTraits": "string"
  },
  "signature": "string",
  "extension": "0x",
  "quoteId": "string"
}
```

---

## Security Considerations

### **Private Key Management:**

- **Environment variables** for development
- **Hardware wallets** for production
- **Secure key storage** (HSM, encrypted files)
- **Never hardcode** private keys

### **Signature Validation:**

- **Verify EIP-712 structure** before signing
- **Validate quote data** against original request
- **Check timestamps** for quote expiration
- **Verify chain ID** matches expected network

### **Error Handling:**

- **Invalid quote data** → Reject signing
- **Expired quotes** → Request new quote
- **Network errors** → Retry with backoff
- **Signature failures** → Log and retry

---

## Integration with Step 1.3

### **Input for Step 1.3:**

- **Signed order JSON** (complete structure)
- **API endpoint** for submission
- **Authentication** (API key)

### **Validation Before Submission:**

- **Signature verification** (optional double-check)
- **Quote ID validation** (matches original quote)
- **Order data integrity** (no tampering)

---

## MVP Recommendation

### **Phase 1: Command-Line Scripts**

- **Node.js + ethers.js** for signing
- **Environment variables** for private keys
- **JSON file output** for Step 1.3
- **Simple error handling** and validation

### **Phase 2: Backend Service** _(Stretch Goal)_

- **API endpoint** for signing
- **Secure key management** (HSM)
- **Production-ready** error handling
- **Scalable architecture**

### **Phase 3: Web Interface** _(Stretch Goal)_

- **MetaMask integration** for user wallets
- **No private key management** required
- **User-friendly** experience
- **Complete workflow** integration

---

## **Phase 1: Announcement - Current State & Limitations**

### **✅ Current Reality - ICP Not Integrated:**

#### **1inch Frontend Status:**

- **ICP is NOT available** in 1inch's network selection dropdown
- **Users CANNOT select ICP** as a source or destination chain
- **ICP ↔ ETH swaps are NOT possible** through the existing UI
- **This is exactly why we're building this extension**

#### **What This Means for Our MVP:**

##### **Phase 1: Announcement - Manual Only:**

- **No UI option:** Users cannot use 1inch frontend for ICP swaps
- **Manual testing required:** Wallet interaction + `curl` commands for order submission
- **Technical limitation:** EIP-712 signing requires wallet (MetaMask, etc.) - cannot be done via `curl` alone
- **Our role:** Provide testing scripts and documentation for manual order creation
- **Stretch goal:** Build the first ICP-enabled UI for 1inch Fusion+

##### **Why This Matters:**

- **We're not extending existing functionality** - we're creating the first ICP integration
- **Our implementation will be the foundation** for future ICP support in 1inch
- **Manual testing requires wallet interaction** - EIP-712 signing cannot be done via `curl` alone
- **Our stretch goal UI** would be the first user-friendly interface for ICP swaps

### **✅ Project Significance:**

**This project is more significant than initially understood:**

- **First ICP integration** for 1inch Fusion+
- **Foundation for future ICP support** in 1inch ecosystem
- **Pioneering cross-chain swaps** between ICP and Ethereum
- **Manual testing is the current reality** - not just a development convenience
