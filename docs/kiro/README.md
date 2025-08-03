# KIRO Project Specifications

This folder references the detailed project specifications located in the root directory.

## 📁 Location: `.kiro/specs/`

The `.kiro/specs/` folder contains 5 project specifications for different components of the ICP Fusion+ implementation:

### **Core Implementation Projects**

1. **`icp-limit-order-protocol-mvp/`**

   - Core Limit Order Protocol implementation on ICP
   - Main architectural decisions and patterns
   - Integration with 1inch LOP standards

2. **`escrow_manager_fusion+/`**

   - HTLC escrow logic and management
   - Cross-chain coordination patterns
   - Chain Fusion integration strategy

3. **`relayer_canister_fusion+/`**

   - Order coordination and submission
   - Relayer responsibilities and patterns
   - Off-chain/on-chain coordination

4. **`fusion-plus-icp-mvp/`**
   - Overall ICP integration strategy
   - Complete system architecture
   - Cross-chain swap orchestration

### **Experimental Projects**

5. **`fusion-plus-mechanical-turk/`**
   - Simple implementation attempt (abandoned)
   - Alternative coordination approaches
   - Research and experimentation notes

## 🎯 **How to Use These Specs**

### **For Architecture Understanding**

```
1. Start with: fusion-plus-icp-mvp/ (overall strategy)
2. Deep dive: icp-limit-order-protocol-mvp/ (core patterns)
3. Integration: escrow_manager_fusion+/ (HTLC logic)
4. Coordination: relayer_canister_fusion+/ (order flow)
```

### **For Implementation Work**

```
Component → Relevant Spec
├── src/limit-order/ → icp-limit-order-protocol-mvp/
├── src/escrow_manager/ → escrow_manager_fusion+/
├── src/relayer/ → relayer_canister_fusion+/
└── Overall system → fusion-plus-icp-mvp/
```

### **For Problem Solving**

```
Issue Type → Check Spec
├── Order management → icp-limit-order-protocol-mvp/
├── Escrow coordination → escrow_manager_fusion+/
├── Cross-chain sync → fusion-plus-icp-mvp/
└── Secret management → relayer_canister_fusion+/
```

## 📋 **Spec Structure**

Each project typically contains:

- `requirements.md` - What needs to be built
- `design.md` - How it will be implemented
- `tasks.md` - Implementation roadmap

## 🔗 **Related Documentation**

- **Context Overview**: `docs/3_context/context-overview.md`
- **Implementation Status**: `internal/implementation-status-assessment.md`
- **Masterplan**: `docs/masterplan/`
- **1inch Analysis**: `docs/1_1Inch/`

---

**Note**: The `.kiro/` folder represents detailed technical specifications for each component. This README provides navigation and context for understanding how the specs relate to the actual implementation.
