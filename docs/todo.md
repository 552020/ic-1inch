# Documentation Reorganization TODO

## 🎯 **Current Status**

✅ **Completed:**

- Moved chaotic docs to `docs/99_to_order/` staging area
- Created clean structure: `0_requirements/`, `1_reference/`, `2_architecture/`, `3_context/`
- Organized reference materials into subfolders:
  - `1_reference/1inch-analysis/` - 1inch specific docs
  - `1_reference/limit-order-protocol/` - LOP mechanics
  - `1_reference/resolver-patterns/` - Resolver entity docs
  - `1_reference/general-protocols/` - ECDSA, EIP712, SIWE, ICRC1
  - `1_reference/cross-chain-analysis/` - Cross-chain implementation
  - `1_reference/similar-projects/` - Moleswap, other protocols
- Moved official hackathon requirements to `0_requirements/`
- Identified and separated MVP design vs future reference design

## 📋 **TODO: Final Cleanup**

### **Priority 1: Review Staging Area**

- [ ] Review remaining files in `docs/99_to_order/`
- [ ] Decide on `mixbytes-*.md` files (audit-related?)
- [ ] Review `cross-turk/` folder (abandoned project)
- [ ] Review `issues/`, `0_varia/`, `2_impementations/` folders
- [ ] **Goal**: Eliminate 50%+ of remaining files through consolidation

### **Priority 2: Update Master README**

- [ ] Update `docs/README.md` to reflect new organized structure
- [ ] Remove references to old chaotic folders
- [ ] Add navigation to new subfolder organization
- [ ] Update workflow examples

### **Priority 3: Final Organization**

- [ ] Move any remaining useful docs from staging to appropriate sections
- [ ] Delete/archive truly redundant or outdated materials
- [ ] Create README files for each subfolder explaining its purpose
- [ ] **Goal**: End with 5-6 main folders max

### **Priority 4: Cross-Reference Cleanup**

- [ ] Update any internal links that point to old file locations
- [ ] Ensure `.kiro/specs/` references are accurate
- [ ] Update `docs/3_context/context-overview.md` with new structure
- [ ] **Goal**: All navigation works correctly

## 🎯 **Success Criteria**

### **Clean Structure Target:**

```
docs/
├── README.md                    # Updated master navigation
├── 0_requirements/              # Hackathon requirements
├── 1_reference/                 # Organized reference materials
│   ├── 1inch-analysis/
│   ├── limit-order-protocol/
│   ├── resolver-patterns/
│   ├── general-protocols/
│   ├── cross-chain-analysis/
│   └── similar-projects/
├── 2_architecture/              # Design and implementation
├── 3_context/                   # Quick developer reference
├── kiro/                        # Reference to .kiro specs
└── 99_to_order/                 # Minimal staging (or removed)
```

### **Reduction Goals:**

- **Before**: 15+ scattered folders and files
- **After**: 5-6 organized main sections
- **Elimination**: 50%+ reduction in total files
- **Navigation**: Clear path from requirements → implementation

## 🔄 **Next Steps**

1. **Review staging area** and make final decisions
2. **Update master README** with new structure
3. **Test navigation** to ensure everything works
4. **Archive staging** once cleanup is complete

---

**Note**: Focus on hackathon requirements and eliminate complexity. The goal is clarity, not comprehensive documentation.
