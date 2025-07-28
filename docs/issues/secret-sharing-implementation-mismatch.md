# HTLC Implementation Analysis: On-Chain vs Off-Chain Responsibilities

## Issue Summary

Our current HTLC escrow implementation is **functionally correct** but needs clarification on the **threat model** and **off-chain coordination requirements**. This document analyzes the proper scope of on-chain vs off-chain responsibilities in atomic swaps.

## Current Implementation Analysis

### Our Current Approach

```rust
// Current claim_escrow function - ANYONE can call with secret
pub async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    // Cryptographic unlock via preimage - this is CORRECT
    // No identity-based authorization - this is CORRECT for HTLCs
}
```

**This is actually CORRECT for HTLCs:**

1. ✅ **Cryptographic unlock** - Anyone with preimage can claim (that's how HTLCs work)
2. ✅ **No identity checks** - The preimage IS the authorization
3. ✅ **Atomic unlock** - Preserves the trustless nature of atomic swaps
4. ✅ **On-chain simplicity** - Contract only validates cryptographic conditions

## Protocol Requirements (From Documentation)

### 1inch Fusion+ Cross-Chain Swap Protocol

**From `internal/cross-chain-swap/README.md`:**

- **Resolvers play major role** in execution
- **Off-chain mechanism** to verify created escrow and distribute user-defined secret
- **Security considerations**: "The security of protocol transactions is affected by the off-chain distribution of the user's secret"
- **Recommended**: "Resolvers are recommended to watch for the event emitted in `EscrowDst.publicWithdraw` function"

**Key Flow:**

1. User signs order (off-chain)
2. Resolver executes on-chain via Limit Order Protocol
3. Resolver deploys escrow clones on both chains
4. **Secret distribution happens off-chain** ← This is the key insight
5. Resolver withdraws using received secret

### 1inch Fusion SDK

**From `internal/fusion-sdk/README.md`:**

- **Resolvers** are authorized market makers
- **Order settlement** involves resolver executing fill order transaction
- **Competitive Dutch auction** model
- **Partial fills** supported with multiple secrets

### Solana Fusion Protocol

**From `secretus/solana-fusion-protocol/docs/whitepaper.md`:**

- **Resolvers** are "professional market participants"
- **Authorization required**: "resolver's address must be authorized by going through KYC/KYB procedure"
- **Competitive model**: "Resolvers compete with one another in Fusion's Dutch auctions"

### goulHash Implementation

**From `secretus/goulHash/README.md`:**

- **Multi-chain HTLC** implementation (Ethereum ↔ Cardano)
- **Same hash** used across both chains
- **Same secret** unlocks both escrows
- **Direct secret usage** - no coordination layer

#### **Detailed Analysis:**

**Creation Phase:**

- **Ethereum**: `payable` constructor combines creation + funding
- **Cardano**: Separate validator creation + UTXO funding transaction
- **Key Insight**: Creation/funding can be combined OR separate (implementation choice)

**Funding Phase:**

- **Ethereum**: `locked_amount = msg.value` during constructor
- **Cardano**: `txOut(scriptAddr, amount)` locks UTXO to validator
- **Key Insight**: Both lock funds immediately with hashlock/timelock

**Claiming Function Analysis:**

**Ethereum (`EscrowSrc.sol`):**

```solidity
function withdraw(bytes32 secret) external notExpired {
    require(_keccakBytes32(secret) == hash, "Hash invalid");
    payable(recipient).transfer(address(this).balance);
}
```

**Cardano (`escrow.ak`):**

```aiken
validator escrow(hash: ByteArray, lock_until: Int) {
    spend(..., redeemer: ByteArray, ...) {
        and {
            True,
            check_hash(redeemer, hash),  // Preimage validation
        }
    }
}
```

**Key Insights:**

1. ✅ **Cryptographic unlock only** - No identity checks
2. ✅ **Same hash function** - `keccak256` used on both chains
3. ✅ **Timelock enforcement** - Ethereum: `notExpired`, Cardano: validator logic
4. ✅ **Direct transfer** - No complex state management

**Considerations:**

- **Simpler than our implementation** - No complex state tracking
- **Proven pattern** - Direct HTLC without extra layers
- **Cross-chain consistency** - Same secret unlocks both escrows
- **No resolver authorization** - Pure cryptographic unlock

### SwappaTEE Implementation

**From `secretus/SwappaTEE/README.md`:**

- **Cross-chain resolver example**
- **Docker-based testing** environment
- **Multiple chain support** (Ethereum, BSC, XRPL)

#### **Detailed Analysis:**

**Creation Phase:**

- **Ethereum**: `deploySrc()` - Deploys source escrow via Limit Order Protocol
- **XRPL**: `createDestinationEscrow()` - Creates destination escrow via TEE
- **Key Insight**: Uses 1inch's official cross-chain-swap contracts

**Funding Phase:**

- **Ethereum**: Safety deposit sent during `deploySrc()` via `payable`
- **XRPL**: `fundEscrow()` - Separate funding transaction to TEE
- **Key Insight**: Different funding mechanisms per chain

**Claiming Function Analysis:**

**Ethereum (`Resolver.sol`):**

```solidity
function withdraw(IEscrow escrow, bytes32 secret, IBaseEscrow.Immutables calldata immutables) external {
    escrow.withdraw(secret, immutables);
}
```

**XRPL (via TEE):**

```typescript
const xrplWithdrawal = await xrpClient.withdraw(xrpEscrow.escrowId, secret, xrpTaker.address, false);
```

**Key Insights:**

1. ✅ **Uses official 1inch contracts** - Full Fusion+ compatibility
2. ✅ **TEE for XRPL** - Trusted execution environment for non-EVM chain
3. ✅ **Coordinated withdrawal** - Both chains use same secret
4. ✅ **Safety deposits** - Required on both chains
5. ✅ **Event monitoring** - Watches for escrow deployment events

**Considerations:**

- **Production-ready** - Uses official 1inch cross-chain-swap contracts
- **TEE integration** - Handles non-EVM chains via trusted execution
- **Complex setup** - Requires Docker, multiple chains, TEE
- **Full Fusion+ flow** - Complete implementation of the protocol

## Correct Understanding: On-Chain vs Off-Chain

### What Belongs On-Chain (HTLC Contract)

```rust
// ✅ CORRECT - These belong in the canister
pub async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    // 1. Verify preimage matches hashlock
    // 2. Check timelock hasn't expired
    // 3. Transfer tokens to caller
    // 4. Emit event for monitoring
}
```

### What Belongs Off-Chain (Protocol Coordination)

```rust
// ❌ WRONG - These don't belong in the canister
pub async fn signal_safe_to_share_secret(...) // Off-chain coordination
pub async fn share_secret_with_relayer(...)    // Off-chain messaging
pub async fn receive_secret_from_relayer(...)  // Off-chain messaging
```

## Our Flow Diagram vs Implementation

### Diagram Requirements (Off-Chain Coordination):

```
Phase 3: Withdrawal Phase
1. Relayer signals to Maker that it's safe to share secrets (off-chain)
2. Maker shares encrypted secrets with Relayer (off-chain)
3. Relayer shares secrets with Taker (off-chain)
4. Taker withdraws using secret (on-chain) ← This is our responsibility
```

### Our Current Implementation (On-Chain HTLC):

```
Phase 3: Withdrawal Phase
1. Anyone can call claim_escrow() with secret (on-chain) ← This is CORRECT
2. Cryptographic validation of preimage
3. Atomic unlock via hashlock verification
```

## Corrected Recommendations

### ✅ Keep Current Implementation

```rust
pub async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    // This is CORRECT - preserve HTLC unlockability
    // The preimage IS the authorization
}
```

### ✅ Add Event Emission

```rust
// Add event emission for off-chain monitoring
pub async fn claim_escrow(escrow_id: String, preimage: Vec<u8>) -> Result<(), EscrowError> {
    // ... existing logic ...

    // Emit event for off-chain coordination
    emit_escrow_claimed_event(escrow_id, caller());

    Ok(())
}
```

### ✅ Implement Off-Chain Coordination (Separate Service)

```typescript
// This belongs in a separate resolver service, NOT in the canister
class FusionResolver {
  async monitorEscrows() {
    // Watch for escrow events
    // Coordinate secret sharing
    // Ensure atomic execution
  }

  async shareSecretWithTaker(escrowId: string, secret: string) {
    // Off-chain secret distribution
    // Only after confirming both legs are ready
  }
}
```

## Security Model Clarification

### Current Risks (Actually Valid):

1. **Secret leakage** - If secret is leaked, anyone can claim
2. **MEV vulnerability** - Front-running possible if secret is public
3. **Timing attacks** - Race conditions in secret sharing

### Protocol Security Model:

1. **Off-chain secret protection** - Secrets shared only through trusted channels
2. **Relayer coordination** - Resolvers wait for confirmation before sharing secrets
3. **Event monitoring** - Resolvers watch for public withdrawal events
4. **Cryptographic unlock** - On-chain validation via preimage

## Implementation Priority

### High Priority:

1. ✅ **Keep current `claim_escrow()`** - It's correct for HTLCs
2. ✅ **Add event emission** - For off-chain monitoring
3. ✅ **Implement off-chain resolver service** - For coordination

### Medium Priority:

1. ✅ **Add monitoring endpoints** - For escrow status
2. ✅ **Implement secret management** - Off-chain coordination
3. ✅ **Add partial fill support** - Multiple secrets per order

### Low Priority:

1. ✅ **Dutch auction integration** - Price discovery mechanism
2. ✅ **Advanced resolver competition** - Multiple resolver support
3. ✅ **MEV protection** - Advanced timing mechanisms

## Conclusion

Our current implementation is **architecturally correct** for HTLCs. The "problem" isn't with our on-chain logic, but with our understanding of the **separation of concerns**:

- **On-chain**: Cryptographic validation and atomic unlock
- **Off-chain**: Secret coordination and protocol orchestration

**Immediate Action:** Add event emission to `claim_escrow()` for off-chain monitoring.

**Future Enhancement:** Build separate off-chain resolver service for coordination, keeping the canister focused on HTLC mechanics.

**Key Insight:** HTLCs are trustless by design - the preimage IS the authorization. Don't try to enforce off-chain protocol logic inside the contract.
