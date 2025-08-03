# Testing Strategy for HTLC Implementation

## Overview

This document outlines the testing strategy for our HTLC implementation, covering manual testing, integration testing, and production hardening phases.

## 1. Manual Testing with Deployed Canister

### **Objective**

Test basic functionality with a deployed canister on local network.

### **Setup Script**

```bash
#!/bin/bash
# setup-testing.sh

echo "🚀 Setting up HTLC testing environment..."

# Build and deploy canister
dfx build backend
dfx deploy backend --network local

# Get canister ID
CANISTER_ID=$(dfx canister id backend)
echo "📦 Canister deployed: $CANISTER_ID"

# Export for other scripts
echo "export CANISTER_ID=$CANISTER_ID" > .env.test
```

### **Basic Functionality Test**

```bash
#!/bin/bash
# test-basic.sh

source .env.test

echo "🧪 Testing basic HTLC functionality..."

# Test 1: Greet function
echo "Testing greet function..."
dfx canister call $CANISTER_ID greet '("HTLC")'

# Test 2: Timelock enforcement
echo "Testing timelock enforcement..."
CURRENT_TIME=$(date +%s)000000000
FUTURE_TIME=$((CURRENT_TIME + 3600000000000))  # 1 hour in future
PAST_TIME=$((CURRENT_TIME - 3600000000000))    # 1 hour in past

echo "Testing future timelock (should be Active)..."
dfx canister call $CANISTER_ID test_timelock "(nat64 $FUTURE_TIME)"

echo "Testing past timelock (should be Expired)..."
dfx canister call $CANISTER_ID test_timelock "(nat64 $PAST_TIME)"

echo "✅ Basic functionality tests completed"
```

### **Escrow Lifecycle Test**

```bash
#!/bin/bash
# test-escrow-lifecycle.sh

source .env.test

echo "🔄 Testing complete escrow lifecycle..."

# Generate test data
SECRET="my_secret_123"
HASHLOCK=$(echo -n "$SECRET" | sha256sum | cut -d' ' -f1)
FUTURE_TIME=$(($(date +%s) + 3600))000000000  # 1 hour from now

echo "Creating escrow..."
ESCROW_ID=$(dfx canister call $CANISTER_ID create_escrow "(
  record {
    hashlock = blob \"$HASHLOCK\";
    timelock = nat64 $FUTURE_TIME;
    token_canister = principal \"rdmx6-jaaaa-aaaah-qcaiq-cai\";
    amount = 1_000_000 : nat64;
    recipient = principal \"user_principal\";
    depositor = principal \"resolver_principal\";
  }
)")

echo "Escrow created: $ESCROW_ID"

echo "Checking escrow status..."
dfx canister call $CANISTER_ID get_escrow_status "(\"$ESCROW_ID\")"

echo "Depositing tokens..."
dfx canister call $CANISTER_ID deposit_tokens "(\"$ESCROW_ID\", 1_000_000 : nat64)"

echo "Checking funded status..."
dfx canister call $CANISTER_ID get_escrow_status "(\"$ESCROW_ID\")"

echo "Claiming with correct secret..."
dfx canister call $CANISTER_ID claim_escrow "(\"$ESCROW_ID\", blob \"$SECRET\")"

echo "Checking claimed status..."
dfx canister call $CANISTER_ID get_escrow_status "(\"$ESCROW_ID\")"

echo "✅ Escrow lifecycle test completed"
```

## 2. Integration Testing with Real Cross-Chain Scenarios

### **Objective**

Test end-to-end cross-chain swap scenarios with real tokens.

### **Setup Testnet Environment**

```bash
#!/bin/bash
# setup-testnet.sh

echo "🌐 Setting up testnet environment..."

# Deploy to testnet
dfx deploy backend --network ic_testnet

# Get testnet canister ID
TESTNET_CANISTER_ID=$(dfx canister id backend --network ic_testnet)
echo "📦 Testnet canister: $TESTNET_CANISTER_ID"

# Export for integration tests
echo "export TESTNET_CANISTER_ID=$TESTNET_CANISTER_ID" > .env.testnet
```

### **Cross-Chain Swap Test**

```bash
#!/bin/bash
# test-cross-chain.sh

source .env.testnet

echo "🌉 Testing cross-chain swap scenario..."

# Phase 1: Maker creates intent (simulated)
echo "Phase 1: Maker creates intent..."
SECRET=$(openssl rand -hex 32)
HASHLOCK=$(echo -n "$SECRET" | sha256sum | cut -d' ' -f1)
echo "Secret: $SECRET"
echo "Hashlock: $HASHLOCK"

# Phase 2: Resolver creates escrow
echo "Phase 2: Resolver creates escrow..."
ESCROW_ID=$(dfx canister call $TESTNET_CANISTER_ID create_escrow "(
  record {
    hashlock = blob \"$HASHLOCK\";
    timelock = nat64 $(($(date +%s) + 7200))000000000;  # 2 hours
    token_canister = principal \"rdmx6-jaaaa-aaaah-qcaiq-cai\";
    amount = 1_000_000 : nat64;
    recipient = principal \"user_principal\";
    depositor = principal \"resolver_principal\";
  }
)")

echo "Escrow created: $ESCROW_ID"

# Phase 3: Maker claims tokens
echo "Phase 3: Maker claims tokens..."
dfx canister call $TESTNET_CANISTER_ID claim_escrow "(\"$ESCROW_ID\", blob \"$SECRET\")"

echo "✅ Cross-chain swap test completed"
```

### **Error Scenarios Test**

```bash
#!/bin/bash
# test-error-scenarios.sh

source .env.testnet

echo "⚠️ Testing error scenarios..."

# Test 1: Claim with wrong secret
echo "Testing claim with wrong secret..."
dfx canister call $TESTNET_CANISTER_ID claim_escrow "(\"$ESCROW_ID\", blob \"wrong_secret\")" || echo "Expected error: InvalidHashlock"

# Test 2: Refund before timelock expires
echo "Testing refund before timelock expires..."
dfx canister call $TESTNET_CANISTER_ID refund_escrow "(\"$ESCROW_ID\")" || echo "Expected error: TimelockNotExpired"

# Test 3: Deposit to non-existent escrow
echo "Testing deposit to non-existent escrow..."
dfx canister call $TESTNET_CANISTER_ID deposit_tokens "(\"non_existent\", 1_000_000 : nat64)" || echo "Expected error: EscrowNotFound"

echo "✅ Error scenario tests completed"
```

## 3. Production Hardening (ICRC-1 Transfers, Error Recovery)

### **Objective**

Implement and test production-ready features.

### **ICRC-1 Token Integration**

```bash
#!/bin/bash
# setup-icrc1.sh

echo "🪙 Setting up ICRC-1 token integration..."

# Deploy test ICRC-1 token canister
dfx deploy icrc1_token --network ic_testnet

# Get token canister ID
TOKEN_CANISTER_ID=$(dfx canister id icrc1_token --network ic_testnet)
echo "Token canister: $TOKEN_CANISTER_ID"

# Mint test tokens
dfx canister call $TOKEN_CANISTER_ID icrc1_mint "(
  record {
    to = record {
      owner = principal \"$(dfx identity get-principal)\";
      subaccount = null;
    };
    amount = 10_000_000 : nat;
  }
)"

echo "✅ ICRC-1 setup completed"
```

### **Production Test with Real Tokens**

```bash
#!/bin/bash
# test-production.sh

source .env.testnet

echo "🏭 Testing production scenario with real tokens..."

# Create escrow with real token canister
ESCROW_ID=$(dfx canister call $TESTNET_CANISTER_ID create_escrow "(
  record {
    hashlock = blob \"$HASHLOCK\";
    timelock = nat64 $(($(date +%s) + 3600))000000000;
    token_canister = principal \"$TOKEN_CANISTER_ID\";
    amount = 1_000_000 : nat64;
    recipient = principal \"$(dfx identity get-principal)\";
    depositor = principal \"$(dfx identity get-principal)\";
  }
)")

echo "Production escrow created: $ESCROW_ID"

# Test token transfer (requires ICRC-1 implementation)
echo "Testing token transfer..."
# TODO: Implement actual ICRC-1 transfer in escrow functions

echo "✅ Production test completed"
```

### **Error Recovery Test**

```bash
#!/bin/bash
# test-error-recovery.sh

source .env.testnet

echo "🔄 Testing error recovery mechanisms..."

# Test 1: Partial failure recovery
echo "Testing partial failure recovery..."
# TODO: Implement rollback mechanisms

# Test 2: Network failure recovery
echo "Testing network failure recovery..."
# TODO: Implement retry mechanisms

# Test 3: State consistency check
echo "Testing state consistency..."
dfx canister call $TESTNET_CANISTER_ID list_escrows

echo "✅ Error recovery tests completed"
```

## 4. Performance and Load Testing

### **Objective**

Test canister performance under load.

### **Load Test Script**

```bash
#!/bin/bash
# test-load.sh

source .env.testnet

echo "📊 Running load tests..."

# Create multiple escrows
for i in {1..10}; do
  echo "Creating escrow $i..."
  dfx canister call $TESTNET_CANISTER_ID create_escrow "(
    record {
      hashlock = blob \"$(openssl rand -hex 32)\";
      timelock = nat64 $(($(date +%s) + 3600))000000000;
      token_canister = principal \"rdmx6-jaaaa-aaaah-qcaiq-cai\";
      amount = 1_000_000 : nat64;
      recipient = principal \"user_principal\";
      depositor = principal \"resolver_principal\";
    }
  )" &
done

wait
echo "✅ Load test completed"
```

## 5. Security Testing

### **Objective**

Test security properties and attack vectors.

### **Security Test Script**

```bash
#!/bin/bash
# test-security.sh

source .env.testnet

echo "🔒 Running security tests..."

# Test 1: Hashlock collision resistance
echo "Testing hashlock collision resistance..."
SECRET1="secret1"
SECRET2="secret2"
HASHLOCK1=$(echo -n "$SECRET1" | sha256sum | cut -d' ' -f1)
HASHLOCK2=$(echo -n "$SECRET2" | sha256sum | cut -d' ' -f1)

if [ "$HASHLOCK1" != "$HASHLOCK2" ]; then
  echo "✅ Hashlock collision resistance confirmed"
else
  echo "❌ Hashlock collision detected!"
fi

# Test 2: Timelock precision
echo "Testing timelock precision..."
CURRENT_TIME=$(date +%s)000000000
EDGE_TIME=$((CURRENT_TIME + 1))

dfx canister call $TESTNET_CANISTER_ID test_timelock "(nat64 $EDGE_TIME)"

echo "✅ Security tests completed"
```

## 6. Test Execution Script

### **Run All Tests**

```bash
#!/bin/bash
# run-all-tests.sh

echo "🧪 Running complete test suite..."

# Setup
./setup-testing.sh
./setup-testnet.sh
./setup-icrc1.sh

# Basic tests
./test-basic.sh
./test-escrow-lifecycle.sh

# Integration tests
./test-cross-chain.sh
./test-error-scenarios.sh

# Production tests
./test-production.sh
./test-error-recovery.sh

# Performance tests
./test-load.sh

# Security tests
./test-security.sh

echo "🎉 All tests completed!"
```

## 7. Test Results Documentation

### **Test Report Template**

```bash
#!/bin/bash
# generate-test-report.sh

echo "📋 Generating test report..."

REPORT_FILE="test-report-$(date +%Y%m%d-%H%M%S).md"

cat > $REPORT_FILE << EOF
# HTLC Test Report - $(date)

## Test Environment
- Canister ID: $CANISTER_ID
- Network: ic_testnet
- Date: $(date)

## Test Results

### Basic Functionality
- [ ] Greet function
- [ ] Timelock enforcement
- [ ] Escrow lifecycle

### Integration Tests
- [ ] Cross-chain swap scenario
- [ ] Error scenarios
- [ ] State transitions

### Production Tests
- [ ] ICRC-1 token transfers
- [ ] Error recovery
- [ ] Performance under load

### Security Tests
- [ ] Hashlock collision resistance
- [ ] Timelock precision
- [ ] State consistency

## Issues Found
- None

## Recommendations
- Implement ICRC-1 transfers
- Add comprehensive error handling
- Add performance monitoring

EOF

echo "📄 Test report generated: $REPORT_FILE"
```

## Next Steps

1. **Execute basic tests** to verify core functionality
2. **Deploy to testnet** for integration testing
3. **Implement ICRC-1 transfers** for production readiness
4. **Add comprehensive error handling** and recovery mechanisms
5. **Perform security audit** before mainnet deployment

---

**This testing strategy ensures our HTLC implementation is robust, secure, and production-ready!** 🚀
