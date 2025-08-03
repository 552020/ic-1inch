# ETHGlobal Unite DeFi Hackathon Requirements

## 🎯 **Track: Extend Fusion+ to ICP - $20,000**

Build a novel extension for 1inch Cross-chain Swap (Fusion+) that enables swaps between Ethereum and ICP.

---

## ✅ **Qualification Requirements (MUST IMPLEMENT)**

### **1. Preserve Hashlock and Timelock Functionality**

- **Requirement**: Maintain HTLC (Hash Time Lock Contract) functionality for non-EVM implementation
- **Implementation**:
  - ✅ `src/escrow_manager/` - HTLC escrow logic
  - ✅ `cross-chain-swap/contracts/BaseEscrow.sol` - Ethereum escrow contracts
  - ✅ Hashlock/timelock coordination between ICP and Ethereum

### **2. Bidirectional Swap Functionality**

- **Requirement**: Swaps possible both to and from Ethereum
- **Implementation**:
  - ⚠️ **ICP → ETH**: Partially implemented (escrow creation)
  - ⚠️ **ETH → ICP**: Basic structure exists
  - ❌ **Complete Flow**: End-to-end orchestration missing

### **3. On-chain Execution Demo**

- **Requirement**: Mainnet/L2 or testnet execution of token transfers for final demo
- **Implementation**:
  - ✅ **EVM Contracts**: Deployed on Base Sepolia
    - Limit Order Protocol contracts
    - Cross-chain swap escrow contracts
  - ✅ **ICP Canisters**: Ready for testnet deployment
  - ❌ **End-to-End Demo**: Integration flow incomplete

---

## 🎯 **Stretch Goals (NOT Required for Qualification)**

### **1. UI (User Interface)**

- **Status**: ⚠️ Basic frontend exists in `src/frontend/`
- **Priority**: Low - Not required for hackathon success

### **2. Enable Partial Fills**

- **Status**: ❌ Not implemented (requires Merkle tree secrets)
- **Priority**: Low - Advanced feature from 1inch whitepaper
- **Reference**: Whitepaper Section 2.5

---

## 🚨 **Current Implementation Gap**

### **THE CORE CHALLENGE: Integration Layer**

**Problem**: How to ensure atomic escrow creation across ICP ↔ Ethereum?

**Required for Demo**:

1. **Complete 4-Phase Flow**: Announcement → Deposit → Withdrawal → Recovery
2. **Secret Distribution System**: Conditional revelation after escrow verification
3. **Bidirectional Coordination**: Both ICP→ETH and ETH→ICP working flows

**Current Status**:

- ✅ Individual components implemented
- ❌ End-to-end orchestration missing
- ❌ Integration layer incomplete

---

## ✅ **Success Criteria for Hackathon**

### **Minimum Viable Demo**

1. **User submits swap request** (ETH ↔ ICP)
2. **Escrows created on both chains** with hashlock/timelock
3. **Secret revealed and swap completed** atomically
4. **Tokens transferred** to final recipients
5. **On-chain verification** of completed swap

### **Technical Requirements**

- ✅ HTLC functionality preserved
- ✅ Bidirectional swap capability
- ✅ On-chain execution (testnet acceptable)
- ✅ Real token transfers (not just logging)

### **Demo Requirements**

- **Live execution** on testnets during presentation
- **Working integration** between ICP and Ethereum
- **Atomic guarantee** demonstrated (either both succeed or both fail)

---

## 📋 **Implementation Priority**

### **Priority 1: Core Requirements (Must Have)**

1. Complete bidirectional escrow coordination
2. Implement secret revelation system
3. Add end-to-end atomic execution
4. Test complete flows on testnets

### **Priority 2: Demo Readiness (Should Have)**

1. Error handling and edge cases
2. Performance optimization
3. User experience improvements
4. Documentation and presentation materials

### **Priority 3: Stretch Goals (Nice to Have)**

1. UI improvements
2. Partial fills implementation
3. Advanced features from whitepaper

---

## 🔗 **Reference Materials**

- **Official Track**: ETHGlobal Unite DeFi - Extend Fusion+ to ICP
- **Technical Spec**: `docs/1_1Inch/1inch-fusion-plus-whitepaper.md`
- **Architecture**: `.kiro/specs/fusion-plus-icp-mvp/`
- **Integration Strategy**: `docs/masterplan/docs/icp_cross_chain_coordination.md`

---

**Bottom Line**: Focus on getting the basic atomic swap working end-to-end. Advanced features like Dutch auctions and partial fills are NOT required for hackathon success.
