# Issue: Orderbook WASM Build Location Inconsistency

**Issue ID:** `orderbook-wasm-build-location`  
**Priority:** Medium  
**Status:** Open  
**Created:** 2025-02-08  
**Component:** Orderbook Canister Build System

## Problem Description

The orderbook canister's WASM build output location is inconsistent with test script expectations, causing test failures and confusion during development.

### Current Behavior

- **Expected Location (by test script):** `src/orderbook/target/wasm32-unknown-unknown/release/orderbook.wasm`
- **Actual Location:** `./target/wasm32-unknown-unknown/release/orderbook.wasm` (project root)

### Impact

1. **Test Script Failures:** The `test_data_types.sh` script initially failed because it looked for the WASM file in the wrong location
2. **Developer Confusion:** Developers expect build artifacts to be in the component's local directory
3. **CI/CD Issues:** Automated build processes may fail if they expect consistent artifact locations

## Root Cause Analysis

The issue stems from Cargo's workspace configuration. When building from within `src/orderbook/`, Cargo uses the workspace root's `target/` directory instead of creating a local one.

### Workspace Structure Impact

```
ic-1inch/                           # Workspace root
├── Cargo.toml                      # Workspace manifest
├── target/                         # ← WASM files built here
│   └── wasm32-unknown-unknown/
│       └── release/
│           └── orderbook.wasm
└── src/
    └── orderbook/
        ├── Cargo.toml              # Member manifest
        └── src/
            └── lib.rs
```

## Affected Components

- ✅ **Test Scripts:** Fixed in `scripts/orderbook/test_data_types.sh`
- ⚠️ **Build Documentation:** Needs updating
- ⚠️ **CI/CD Pipelines:** May need adjustment
- ⚠️ **Developer Onboarding:** Documentation should clarify build locations

## Proposed Solutions

### Option 1: Update All References (Current Approach)

- ✅ **Pros:** Follows Cargo workspace conventions
- ❌ **Cons:** Requires updating all scripts and documentation

### Option 2: Configure Local Target Directory

```toml
# In src/orderbook/Cargo.toml
[build]
target-dir = "target"
```

- ✅ **Pros:** Intuitive for developers
- ❌ **Cons:** Goes against Cargo workspace best practices

### Option 3: Symbolic Links

Create symbolic links from expected locations to actual locations

- ✅ **Pros:** Maintains both conventions
- ❌ **Cons:** Platform-specific, adds complexity

## Recommended Solution

**Option 1** is recommended as it follows Cargo workspace conventions and is the most maintainable long-term approach.

## Action Items

### Immediate (Completed)

- [x] Fix `test_data_types.sh` to look in correct location
- [x] Verify WASM build works correctly

### Short-term

- [ ] Update build documentation to clarify WASM output locations
- [ ] Review other scripts that might have similar issues
- [ ] Add build location documentation to README

### Long-term

- [ ] Standardize all build scripts across the project
- [ ] Consider adding a build wrapper script that handles path differences
- [ ] Update CI/CD pipelines if needed

## Testing

### Verification Steps

1. Run `cargo build --target wasm32-unknown-unknown --release` from `src/orderbook/`
2. Verify WASM file exists at `./target/wasm32-unknown-unknown/release/orderbook.wasm`
3. Run test script: `./scripts/orderbook/test_data_types.sh`
4. Confirm all tests pass

### Test Results

```bash
# From project root
$ ./scripts/orderbook/test_data_types.sh
Tests Run: 10
Tests Passed: 10
Tests Failed: 0
🎉 All tests passed!
```

## Related Issues

- None currently identified

## Notes

This issue was discovered during Task 2 implementation when enhancing data types for 1inch LOP compatibility. The fix was applied immediately to unblock development, but proper documentation and standardization should follow.

## Resolution

**Status:** Partially Resolved  
**Resolution Date:** 2025-02-08  
**Resolution:** Test script updated to use correct path. Documentation updates pending.

### Changes Made

- Updated `scripts/orderbook/test_data_types.sh` to look for WASM at `../../target/wasm32-unknown-unknown/release/orderbook.wasm`
- Verified build process works correctly with workspace configuration

### Remaining Work

- Documentation updates
- Standardization across other scripts
- Developer onboarding documentation
