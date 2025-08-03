# Escrows in 1inch Fusion+ Context

_Comprehensive definitions and explanations of escrows in cross-chain atomic swaps_

---

## What Are Escrows?

### **Core Definition:**

**Escrows are smart contracts that hold funds conditionally** in cross-chain atomic swap protocols.

### **HTLC (Hashed Timelock Contract) Nature:**

- **Secured by secret hash** - Funds locked with cryptographic hash
- **Secured by timelock** - Funds have time-based expiration
- **Atomic execution** - Either both chains succeed or both fail

### **Fusion+ Escrow Structure:**

#### **Two Escrows Required:**

1. **Source Escrow** - Holds user's assets on source chain (Ethereum in our case)
2. **Destination Escrow** - Holds resolver's assets on destination chain (ICP in our case)

#### **Linked by Secret Hash:**

- **Same secret hash** used in both escrows
- **Atomic guarantee** - Both escrows use identical hashlock
- **Cross-chain coordination** - Secret revelation unlocks both escrows

### **Deployment Locations:**

- **Source escrow**: On source chain (Ethereum)
- **Destination escrow**: On destination chain (ICP)
- **Network specific**: Testnet vs mainnet deployment

---

## Escrow Functionality

### **Core Functions:**

1. **Deposit** - Lock funds with hashlock and timelock
2. **Claim** - Release funds with correct secret (preimage)
3. **Refund** - Return funds after timelock expiration
4. **State Management** - Track escrow status and balances

### **Security Mechanisms:**

- **Hashlock** - Cryptographic hash of secret preimage
- **Timelock** - Time-based expiration for safety
- **Atomic execution** - All-or-nothing swap completion
- **Public verification** - Anyone can verify escrow state

---

## Integration with 1inch Fusion+

### **Order Flow Integration:**

- **Signed orders** specify escrow parameters
- **Resolver network** creates and manages escrows
- **Cross-chain coordination** ensures atomic execution

### **API Integration:**

- **Fusion+ APIs** for order submission and monitoring
- **Escrow creation** triggered by resolver actions
- **State tracking** through API endpoints

---

## Implementation Considerations

### **Ethereum Side:**

- **Use existing 1inch contracts** - No custom development needed
- **EscrowSrc contracts** - Standard 1inch escrow implementation
- **Limit Order Protocol** - Integration with existing 1inch infrastructure

### **ICP Side:**

- **Custom canister implementation** - Our development work
- **Rust-based HTLC** - Implement escrow logic in Rust
- **ICRC-1 integration** - Token standard compatibility
- **HTTP outcalls** - Cross-chain verification capabilities

---

## Testing Requirements

### **Success Scenarios:**

- **Happy path** - Successful atomic swap execution
- **Secret revelation** - Correct preimage unlocks both escrows
- **Token transfers** - Assets moved to final recipients

### **Failure Scenarios:**

- **Timeout path** - Timelock expiration triggers refunds
- **Partial failures** - One chain succeeds, other fails
- **Network issues** - Cross-chain coordination failures

### **Success Criteria:**

- **Atomic guarantee** - Both escrows succeed or both fail
- **No funds lost** - All assets accounted for
- **State consistency** - Both chains show same final state
