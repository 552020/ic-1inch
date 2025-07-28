# TODO List - HTLC Implementation

## 🔐 Authorization & Security

### **High Priority**

- [ ] **Resolver Authorization Model**

  - [x] Define what constitutes an "authorized resolver" ✅
  - [x] Implement resolver-only model (makers cannot create escrows) ✅
  - [x] Add `is_authorized_resolver(caller: Principal)` function ✅
  - [ ] Decide: MVP open vs production restricted

- [ ] **Deposit Authorization**

  - [ ] Add caller verification in `deposit_tokens()`
  - [ ] Decide: Only maker vs maker + resolver
  - [ ] Implement `caller() != escrow.maker` check
  - [x] Simplified API: removed `recipient` field, using only `maker` ✅

- [ ] **Refund Authorization**
  - [ ] Clarify refund philosophy: anyone vs restricted
  - [ ] Traditional HTLC allows anyone to refund after timelock
  - [ ] Decide if we want stricter rules for production

### **Medium Priority**

- [ ] **Concurrent Access Protection**

  - [ ] Test with `dfx canister call` flood on same `escrow_id`
  - [ ] Add mutex or atomic operations if needed
  - [ ] Ensure thread-safe operations

- [ ] **Escrow ID Generation**
  - [ ] Replace `format!("escrow_{}", time())` with UUID
  - [ ] Add `uuid` dependency to Cargo.toml
  - [ ] Use `uuid::Uuid::new_v4().to_string()` for collision resistance

## 💰 Token Transfers (Critical)

### **High Priority**

- [ ] **ICRC-1 Integration**

  - [ ] Implement actual token transfers in `claim_escrow()`
  - [ ] Implement actual token transfers in `refund_escrow()`
  - [ ] Add cross-canister calls to `icrc1_transfer`
  - [ ] Handle transfer fees properly
  - [ ] Add error handling for failed transfers

- [ ] **Transfer Error Handling**
  - [ ] What happens if ICRC-1 transfer fails?
  - [ ] Rollback escrow state on transfer failure
  - [ ] Atomic operations: state change + transfer

## 🔍 Audit & Debugging

### **Medium Priority**

- [ ] **Preimage Storage**

  - [ ] Add `preimage: Option<Vec<u8>>` to `Escrow` struct
  - [ ] Store revealed preimage for audit purposes
  - [ ] Consider privacy implications

- [ ] **Enhanced Logging**
  - [ ] Add detailed logs for all state transitions
  - [ ] Log caller principals for security auditing
  - [ ] Log transfer amounts and recipients

## 🧪 Testing & Validation

### **High Priority**

- [ ] **Concurrent Access Testing**

  - [ ] Test multiple simultaneous calls to same escrow
  - [ ] Test race conditions in state transitions
  - [ ] Stress test with high-frequency calls

- [ ] **Authorization Testing**
  - [ ] Test unauthorized caller attempts
  - [ ] Test resolver authorization scenarios
  - [ ] Test depositor-only operations

### **Medium Priority**

- [ ] **Integration Testing**
  - [ ] Test with real ICRC-1 tokens
  - [ ] Test cross-canister communication
  - [ ] Test error scenarios with actual transfers

## 🚀 Production Readiness

### **High Priority**

- [ ] **Security Audit**

  - [ ] Formal security review before mainnet
  - [ ] Review authorization model
  - [ ] Review concurrent access patterns

- [ ] **Error Recovery**
  - [ ] Handle partial failures (state updated but transfer failed)
  - [ ] Add recovery mechanisms
  - [ ] Test edge cases

### **Medium Priority**

- [ ] **Performance Optimization**

  - [ ] Profile canister performance
  - [ ] Optimize memory usage
  - [ ] Consider batch operations

- [ ] **Monitoring & Metrics**
  - [ ] Add metrics collection
  - [ ] Monitor escrow lifecycle
  - [ ] Track success/failure rates

## 📚 Documentation

### **Medium Priority**

- [ ] **API Documentation**

  - [ ] Complete Candid interface documentation
  - [ ] Add usage examples
  - [ ] Document authorization requirements

- [ ] **Security Documentation**
  - [ ] Document authorization model
  - [ ] Document security assumptions
  - [ ] Add security best practices guide

## 🎯 MVP vs Production Decisions

### **To Decide:**

1. **Resolver Authorization**

   - [ ] MVP: Open (anyone can be resolver)
   - [ ] Production: Whitelisted resolvers

2. **Deposit Authorization**

   - [ ] MVP: Only maker
   - [ ] Production: Maker + authorized resolvers

3. **Refund Authorization**

   - [ ] MVP: Anyone after timelock (traditional HTLC)
   - [ ] Production: Restricted to depositor + resolvers

4. **Escrow ID Strategy**
   - [ ] MVP: Simple timestamp-based
   - [ ] Production: Collision-resistant with caller info

## 📋 Next Steps Priority

1. **Week 1**: Authorization model + ICRC-1 transfers
2. **Week 2**: Testing + security audit
3. **Week 3**: Production hardening + documentation
4. **Week 4**: Deployment + monitoring

---

**Last Updated**: [Current Date]
**Status**: MVP Implementation Phase
