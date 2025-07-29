# Token Canister Dependency Issue

_Issue: MVP deployment fails due to non-existent token canisters_

---

## 🚨 **Problem**

During MVP testing, `fill_order` calls fail with:

```
TokenCallFailed = "Balance check failed: (DestinationInvalid, \"No route to canister kuze4-556gg-fsyvb-b5gma-zaakk-anqoq-ulwxj-yy7sd-wf5iz-hb74c-lae\")"
```

## 🔍 **Root Cause**

The limit order protocol requires **real ICRC-1 token canisters** to:

1. Check maker balances before order creation
2. Check taker balances before order filling
3. Execute token transfers during order settlement

**Current Issue:** Test scripts use **non-existent canister IDs** that were generated but never deployed.

### **CRC32 Principal Validation Issue**

Additionally, the Internet Computer enforces **CRC32 checksum validation** on principal strings. Invalid principals like `"rdmx6-jaaaa-aaaah-qcaiq-cai"` will fail even if the canister doesn't exist.

**Solution:** Use valid principal strings that pass CRC32 validation.

## 🎯 **MVP Requirements**

### **Functional Requirements:**

- ✅ Order creation with balance validation
- ✅ Order filling with balance validation
- ✅ Token transfers during settlement
- ✅ Asset pair validation (prevent same-token swaps)

### **Non-Functional Requirements:**

- 🎯 **Minimal complexity** - Avoid over-engineering for MVP
- 🎯 **Realistic testing** - Use production-like tokens
- 🎯 **Simple deployment** - No additional canister deployments
- 🎯 **Valid principals** - Use CRC32-compliant principal strings

## 💡 **Proposed Solutions**

### **Option 1: Use Built-in Tokens (RECOMMENDED)**

```bash
# Use tokens that exist on local network with valid principals
MAKER_TOKEN="aaaaa-aa"  # ICP token (management canister - always valid)
TAKER_TOKEN="rdmx6-jaaaa-aaaaa-aaadq-cai"  # Valid ckBTC principal

dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"$MAKER_TOKEN\",
  principal \"$TAKER_TOKEN\",
  1000000000:nat64,
  100000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

**Pros:**

- ✅ No deployment overhead
- ✅ Realistic testing with actual ICRC-1 tokens
- ✅ Simple parameters, no mock complexity
- ✅ Production-like behavior
- ✅ Valid CRC32-compliant principals

**Cons:**

- ❌ Limited to available tokens on local network

### **Option 2: Deploy Mock Token**

```bash
# Deploy a simple mock ICRC-1 token
dfx deploy mock_icrc1_token

# Use the deployed canister ID (automatically CRC32-valid)
MOCK_TOKEN_ID=$(dfx canister id mock_icrc1_token)
```

**Pros:**

- ✅ Full control over token behavior
- ✅ Can test edge cases
- ✅ Automatically valid principal (deployed canister ID)

**Cons:**

- ❌ Additional deployment complexity
- ❌ Mock parameters to manage
- ❌ Not production-like

### **Option 3: Use Management Canister as Placeholder**

```bash
# Use management canister for both tokens (simplest)
dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"aaaaa-aa\",  # Management canister
  principal \"aaaaa-aa\",  # Management canister (same - will fail validation)
  1000000000:nat64,
  100000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

**Pros:**

- ✅ Simplest implementation
- ✅ Always valid principal
- ✅ Tests asset pair validation

**Cons:**

- ❌ Same-token swap will be rejected (as intended)
- ❌ Not realistic for actual trading

## 🏆 **Recommendation**

**Use Option 1 (Built-in Tokens)** for MVP because:

1. **Minimal Complexity** - No additional deployments
2. **Realistic Testing** - Actual ICRC-1 tokens with real behavior
3. **Simple Parameters** - No mock configurations
4. **Production-Ready** - Same approach as production deployment
5. **Valid Principals** - CRC32-compliant canister IDs

## 📋 **Implementation Tasks**

- [ ] Update test scripts to use built-in tokens with valid principals
- [ ] Document valid token canister IDs in README
- [ ] Add validation to ensure token canisters exist
- [ ] Update manual testing guide with correct token examples
- [ ] Add CRC32 principal validation to error handling

## 🔗 **Related**

- [Manual Testing Guide](../manual-testing-guide.md)
- [Production Deployment Guide](../production-deployment-guide.md)
- [ICRC-1 Implementation](../icrc1-implementation.md)
- [Internet Computer Principal Specification](https://internetcomputer.org/docs/references/ic-interface-spec#textual-ids)

---

_Status: Open_  
_Priority: High_  
_Type: MVP Blocking_
