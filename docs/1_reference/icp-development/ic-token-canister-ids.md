# Internet Computer Token Canister IDs

_Reference guide for valid token principals on the Internet Computer_

---

## 🎯 **MVP Decision: Use ckETH**

For our ChainFusion+ MVP, we will use **ckETH** as it aligns with our cross-chain atomic swap goals.

### **✅ Selected Tokens for MVP:**

```bash
# Valid token principals for testing
MAKER_TOKEN="aaaaa-aa"  # ICP token (management canister - always valid)
TAKER_TOKEN="ss2fx-dyaaa-aaaar-qacoq-cai"  # ckETH Ledger
```

---

## 📋 **Complete Token Reference**

### **ckBTC (Chain-key Bitcoin)**

- **Ledger:** `mxzaz-hqaaa-aaaar-qaada-cai`
- **Minter:** `mqygn-kiaaa-aaaar-qaadq-cai`
- **Index:** `n5wcd-faaaa-aaaar-qaaea-cai`

[Official ckBTC Documentation](https://internetcomputer.org/docs/defi/chain-key-tokens/ckbtc/overview)

### **ckETH (Chain-key Ethereum) - SELECTED FOR MVP**

- **Ledger:** `ss2fx-dyaaa-aaaar-qacoq-cai`
- **Minter:** `sv3dd-oaaaa-aaaar-qacoa-cai`
- **Index:** `s3zol-vqaaa-aaaar-qacpa-cai`
- **Archive:** `xob7s-iqaaa-aaaar-qacra-cai`

[Reference: ckETH canister IDs](https://github.com/dfinity/ic/blob/master/rs/ethereum/cketh/mainnet/canister_ids.json)

### **ckTestBTC (Testnet Bitcoin)**

- **Ledger:** `mc6ru-gyaaa-aaaar-qaaaq-cai`
- **Minter:** `ml52i-qqaaa-aaaar-qaaba-cai`
- **Index:** `mm444-5iaaa-aaaar-qaabq-cai`

[Reference: ckTestBTC canister IDs](https://forum.dfinity.org/t/ckbtc-a-canister-issued-bitcoin-twin-token-on-the-ic-1-1-backed-by-btc/17606/2?u=manu)

### **ckSepoliaETH (Testnet Ethereum)**

- **Ledger:** `apia6-jaaaa-aaaar-qabma-cai`
- **Minter:** `jzenf-aiaaa-aaaar-qaa7q-cai`
- **Index:** `sh5u2-cqaaa-aaaar-qacna-cai`
- **Archive:** `sa4so-piaaa-aaaar-qacnq-cai`

[Reference: ckSepoliaETH canister IDs](https://github.com/dfinity/ic/blob/master/rs/ethereum/cketh/testnet/canister_ids.json)

---

## 🚀 **MVP Implementation**

### **Working Test Command:**

```bash
dfx canister call backend create_order "(
  principal \"$MAKER_PRINCIPAL\",
  principal \"aaaaa-aa\",  # ICP token
  principal \"ss2fx-dyaaa-aaaar-qacoq-cai\",  # ckETH Ledger
  1000000000:nat64,
  100000:nat64,
  $(($(date +%s) + 3600))000000000:nat64
)"
```

### **Why ckETH for MVP:**

1. **ChainFusion+ Alignment** - Cross-chain atomic swaps with Ethereum
2. **Real Token** - Actual ICRC-1 token with real behavior
3. **Valid Principal** - CRC32-compliant canister ID
4. **Production-Ready** - Same token used in production
5. **Ethereum Bridge** - Enables ICP ↔ ETH atomic swaps

---

## 📝 **Usage Notes**

### **For Testing:**

- Use **ICP** (`aaaaa-aa`) as maker token
- Use **ckETH Ledger** (`ss2fx-dyaaa-aaaar-qacoq-cai`) as taker token
- Both are valid, CRC32-compliant principals

### **For Production:**

- Can use any combination of these valid tokens
- All principals are CRC32-validated
- Real ICRC-1 tokens with actual functionality

### **For Development:**

- These canister IDs are stable and documented
- No need to deploy mock tokens
- Realistic testing environment

---

## 🔗 **Related**

- [Token Canister Dependency Issue](issues/token-canister-dependency.md)
- [Manual Testing Guide](manual-testing-guide.md)
- [Production Deployment Guide](production-deployment-guide.md)

---

_Last Updated: Current_  
_Status: Active Reference_
