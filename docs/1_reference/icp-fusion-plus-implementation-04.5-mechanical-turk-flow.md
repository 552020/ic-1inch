# ICP Fusion+ Mechanical Turk - Detailed Flow

> **Purpose**: Define the exact technical flow for the minimal working demo
> **Scope**: Single resolver, favorable conditions, basic functionality
> **Focus**: Cross-chain coordination mechanics

## 🎯 **Core Architecture**

### **Actors**

- **Maker (User)**: Lives on the frontend, wants to swap ICP ↔ ETH

  - Has gasless Web3 experience (no rich Web3 interactions)
  - Signs transactions without paying gas fees
  - Feels like Web2 application usage

- **Taker/Resolver (MVP)**: Also lives on the frontend, provides liquidity

  - Currently: human user account on same frontend interface
  - Future: sophisticated bot/automated entity
  - Accepts orders and provides cross-chain liquidity

- **Relayer**: Infrastructure owner and coordinator
  - Owns the orderbook canister
  - Owns both escrow contracts on both chains (ICP & ETH)
  - Pays gas fees and cycle costs
  - Takes small cut for infrastructure services

### **Infrastructure**

- **Orderbook Canister**: Stores swap orders (relayer-owned)
- **ICP Src/Dst Escrow**: Locks ICP tokens with conditions
- **ETH Src/Dst Escrow**: Locks ETH tokens with conditions
- **Relayer Service**: Monitors both chains, coordinates swaps

---

## 🔄 **ICP → ETH Swap Flow**

> **Assumption**: Best case scenario - all parties behave correctly, no network issues, no timeouts

### **Phase 1: Order Placement & Maker Lock**

```
1.1 Maker on the frontend (served by ICP asset canister)
    ├── Maker signs in with MetaMask (SIWE)
    ├── Creates ICP principal + ETH address pair
    └── Accesses swap interface

1.2 Order Creation (frontend + orderbook canister)
    ├── Maker: "Swap 10 ICP for 0.01 ETH"
    ├── Order stored in Orderbook Canister (relayer-owned), gasless for the maker
    └── Order includes: amount, rate, maker addresses, timeout

1.3 Maker Locks Funds (ICP Src Escrow)
    ├── Maker transfers 10 ICP to Src Escrow canister
    ├── Condition: "Release only if ETH Dst Escrow has 0.01 ETH"
    ├── Relayer pays ICP cycles for this transaction
    └── Maker funds are now locked with cross-chain condition
```

### **Phase 2: Taker/Resolver Acceptance & Lock**

```
2.1 Taker/Resolver Authentication
    ├── Taker/Resolver signs in with MetaMask (SIWE)
    ├── Same frontend, different role from maker
    └── Accesses liquidity provider interface

2.2 Order Acceptance
    ├── Taker/Resolver scans Orderbook Canister continuously
    ├── Finds: "10 ICP → 0.01 ETH" order
    └── Taker/Resolver clicks "Accept"

2.3 Taker/Resolver Locks Funds (ETH Dst Escrow)
    ├── Taker/Resolver transfers 0.01 ETH + safety deposit to ETH contract
    ├── ETH contract: "EscrowDst" (do we need hashlock condition?)
    ├── Contract emits event: "FundsLocked(orderID, amount, resolver)" OR
    ├── Taker/Resolver provides transaction hash for verification
    └── Taker/Resolver funds are now locked, waiting for confirmation
```

### **Phase 3: Cross-Chain Verification**

```
3.1 Cross-Chain Verification (Key Technical Challenge)

    Option A: Smart Contract Events (Common in Solidity)
    ├── ETH contract emits "FundsLocked(orderID, amount, resolver)" event
    ├── Off-chain relayer monitors these events via Web3 providers
    ├── Pro: Standard pattern, reliable, automatic
    └── Con: Requires off-chain monitoring service

    Option B: Transaction Hash Verification
    ├── Taker/Resolver provides ETH transaction hash to system
    ├── ICP canister uses HTTP outcalls to verify transaction on ETH
    ├── Pro: Direct verification, no events needed
    └── Con: Requires HTTP outcalls, more complex verification

    Option C: Manual Relayer Verification (MVP Approach)
    ├── Relayer is a frontend user with admin privileges
    ├── Relayer manually checks: ICP Src Escrow has funds ✅
    ├── Relayer manually checks: ETH Dst Escrow has funds ✅
    ├── Relayer clicks "Approve Swap" button in frontend
    └── System sends "OK" signal to maker to proceed

    **MVP Analysis**: Option C is simplest for testing the flow
    - No complex event monitoring needed
    - No HTTP outcalls complexity
    - Human verification ensures correctness
    - Easy to debug and iterate
```

### **Phase 4: Atomic Completion**

```
4.1 Key Release to Taker/Resolver
    ├── ICP Src Escrow condition satisfied: "ETH locked ✅"
    ├── Secret/key generated and given to taker/resolver
    ├── Taker/Resolver can now initiate final transfers
    └── Timer starts: taker/resolver must complete within timeout

4.2 ETH Transfer First (Creates Receipt)
    ├── Taker/Resolver transfers 0.01 ETH to maker's ETH address
    ├── ETH Dst Escrow contract generates transfer receipt
    ├── Receipt contains: transaction hash, amount, recipient, timestamp
    └── Taker/Resolver now has proof of ETH transfer completion

4.3 ICP Claim (Requires Receipt + Key)
    ├── Taker/Resolver submits: key + ETH transfer receipt to ICP Src Escrow
    ├── ICP canister verifies: receipt is valid, amount matches, recent timestamp
    ├── ICP Src Escrow releases 10 ICP to taker/resolver (minus relayer cut)
    └── Swap completed atomically: both transfers confirmed ✅

**Atomicity Guarantee**:
- Taker/Resolver must complete ETH transfer to get receipt
- Taker/Resolver must have receipt to claim ICP tokens
- If ETH transfer fails, no ICP can be claimed
- If receipt is invalid, ICP remains locked
```

---

## 🔄 **ETH → ICP Swap Flow**

> **Assumption**: Best case scenario - all parties behave correctly, no network issues, no timeouts

### **Same Receipt-Based Pattern, Reversed Chains**

### **Phase 1: Order Placement & Maker Lock**

```
1.1 Maker on the frontend (served by ICP asset canister)
    ├── Maker signs in with MetaMask (SIWE)
    ├── Creates ICP principal + ETH address pair
    └── Accesses swap interface

1.2 Order Creation (frontend + orderbook canister)
    ├── Maker: "Swap 0.01 ETH for 10 ICP"
    ├── Order stored in Orderbook Canister (relayer-owned), gasless for the maker
    └── Order includes: amount, rate, maker addresses, timeout

1.3 Maker Locks Funds (ETH Src Escrow)
    ├── Maker transfers 0.01 ETH to ETH Src Escrow contract
    ├── Condition: "Release only if ICP Dst Escrow has 10 ICP"
    ├── Maker pays ETH gas fees for this transaction
    └── Maker funds are now locked with cross-chain condition
```

### **Phase 2: Taker/Resolver Acceptance & Lock**

```
2.1 Taker/Resolver Authentication
    ├── Taker/Resolver signs in with MetaMask (SIWE)
    ├── Same frontend, different role from maker
    └── Accesses liquidity provider interface

2.2 Order Acceptance
    ├── Taker/Resolver scans Orderbook Canister continuously
    ├── Finds: "0.01 ETH → 10 ICP" order
    └── Taker/Resolver clicks "Accept"

2.3 Taker/Resolver Locks Funds (ICP Dst Escrow)
    ├── Taker/Resolver transfers 10 ICP + safety deposit to ICP canister
    ├── ICP canister: "EscrowDst" with verification condition
    ├── Relayer pays ICP cycles for this transaction
    └── Taker/Resolver funds are now locked, waiting for confirmation
```

### **Phase 3: Cross-Chain Verification**

```
3.1 Cross-Chain Verification (Same options as ICP → ETH flow)
    ├── Manual relayer verification (MVP approach)
    ├── Relayer checks: ETH Src Escrow has 0.01 ETH ✅
    ├── Relayer checks: ICP Dst Escrow has 10 ICP ✅
    └── Relayer clicks "Approve Swap" → sends "OK" to maker
```

### **Phase 4: Atomic Completion with Receipt**

```
4.1 Key Release to Taker/Resolver
    ├── ETH Src Escrow condition satisfied: "ICP locked ✅"
    ├── Secret/key generated and given to taker/resolver
    ├── Taker/Resolver can now initiate final transfers
    └── Timer starts: taker/resolver must complete within timeout

4.2 ICP Transfer First (Creates Receipt)
    ├── Taker/Resolver transfers 10 ICP to maker's ICP principal
    ├── ICP Dst Escrow canister generates transfer receipt
    ├── Receipt contains: transaction ID, amount, recipient, timestamp
    └── Taker/Resolver now has proof of ICP transfer completion

4.3 ETH Claim (Requires Receipt + Key)
    ├── Taker/Resolver submits: key + ICP transfer receipt to ETH Src Escrow
    ├── ETH contract verifies: receipt is valid, amount matches, recent timestamp
    ├── ETH Src Escrow releases 0.01 ETH to taker/resolver
    └── Swap completed atomically: both transfers confirmed ✅

**Atomicity Guarantee**:
- Taker/Resolver must complete ICP transfer to get receipt
- Taker/Resolver must have receipt to claim ETH tokens
- If ICP transfer fails, no ETH can be claimed
- If receipt is invalid, ETH remains locked
```

---

## 🤔 **Technical Challenges & Solutions**

### **1. Cross-Chain Event Reading**

**Challenge**: How does ICP canister know ETH funds are locked?

**Solution Options**:

- **A. Off-Chain Relayer**: Monitors both chains, updates canisters
- **B. HTTP Outcalls**: ICP canister directly queries ETH node
- **C. Transaction Proofs**: Resolver provides proof, canister verifies

**Recommendation**: Start with **Option A** (off-chain relayer) for simplicity

### **2. Event Reliability**

**Challenge**: What if events are missed or delayed?

**Solutions**:

- **Polling**: Regular checks instead of event listening
- **Timeouts**: Orders expire if not completed
- **Manual Override**: Admin can resolve stuck orders

### **3. Atomic Guarantees**

**Challenge**: Ensuring both sides complete or both fail

**Solutions**:

- **Hashlock/Timelock**: Standard HTLC pattern
- **Key-Based Release**: Secret unlocks both escrows
- **Timeout Recovery**: Funds return to original owners

---

## 🛠️ **Implementation Strategy**

### **Phase 1: Basic Flow**

- ✅ Single resolver (no competition)
- ✅ Off-chain relayer monitoring
- ✅ Simple timeout handling
- ✅ Manual fallback for errors

### **Phase 2: Reliability**

- ⏳ Multiple verification methods
- ⏳ Automatic retry mechanisms
- ⏳ Better error handling
- ⏳ Monitoring and alerts

### **Phase 3: Scaling**

- ⏳ Multiple resolvers
- ⏳ Auction mechanisms
- ⏳ Economic incentives
- ⏳ Full Fusion+ features

---

## 🎯 **Success Criteria**

### **Technical**

- User can swap ICP ↔ ETH successfully
- Atomic completion (all or nothing)
- Cross-chain verification works reliably
- No funds lost due to technical issues

### **User Experience**

- Single frontend for both roles (user/resolver)
- Clear status updates during swap
- Reasonable completion times (< 5 minutes)
- Error states are handled gracefully

---

## 📋 **Next Steps**

1. **Choose Cross-Chain Verification Method**: Off-chain relayer vs HTTP outcalls
2. **Define Escrow Contract Interfaces**: ICP and ETH contract specifications
3. **Build Event Monitoring**: Reliable cross-chain state synchronization
4. **Test Atomic Scenarios**: Success, failure, and timeout cases

**Key Decision Needed**: Should the relayer be off-chain (JavaScript service) or on-chain (ICP canister)?

> **Recommendation**: Off-chain relayer for flexibility and easier cross-chain monitoring
