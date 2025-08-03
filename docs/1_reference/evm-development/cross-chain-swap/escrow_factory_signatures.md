# EscrowFactory Function Signatures

## Overview

This document details the **function signatures** for the EscrowFactory API, with special focus on the **2 functions that resolvers actually use**.

## TL;DR

**Resolver Functions:**

- `addressOfEscrowSrc(Immutables)` → `address`
- `createDstEscrow(Immutables, uint256)` → `void` (payable)

**Both need the same complex `Immutables` struct!**

---

## Function Signatures

### **1. `addressOfEscrowSrc()` - View Function**

```solidity
function addressOfEscrowSrc(IBaseEscrow.Immutables calldata immutables) external view returns (address)
```

**Purpose:** Compute deterministic address for source escrow
**Gas Cost:** Free (view function)
**Returns:** `address` - the computed escrow address

### **2. `createDstEscrow()` - State-Changing Function**

```solidity
function createDstEscrow(IBaseEscrow.Immutables calldata dstImmutables, uint256 srcCancellationTimestamp) external payable
```

**Purpose:** Deploy destination escrow contract
**Gas Cost:** High (deploys contract)
**ETH Required:** Yes (for safety deposit)
**Returns:** `void` (no return value)

---

## The `IBaseEscrow.Immutables` Struct

Both functions require this complex struct:

```solidity
struct Immutables {
    bytes32 orderHash;        // Order hash from Limit Order Protocol
    bytes32 hashlock;         // Hash of the secret (keccak256(secret))
    Address maker;            // Address who created the order
    Address taker;            // Address who fills the order
    Address token;            // Token contract address
    uint256 amount;           // Token amount to swap
    uint256 safetyDeposit;    // Safety deposit amount (in native token)
    Timelocks timelocks;      // Time constraints struct
}
```

### **Struct Field Details:**

| Field           | Type        | Description         | Example                       |
| --------------- | ----------- | ------------------- | ----------------------------- |
| `orderHash`     | `bytes32`   | Hash from LOP order | `0x1234...`                   |
| `hashlock`      | `bytes32`   | Hash of secret      | `keccak256("secret123")`      |
| `maker`         | `Address`   | Order creator       | `0x7099...`                   |
| `taker`         | `Address`   | Order filler        | `0xf39F...`                   |
| `token`         | `Address`   | Token contract      | `0xA0b8...` (USDC)            |
| `amount`        | `uint256`   | Token amount        | `100000000` (100 USDC)        |
| `safetyDeposit` | `uint256`   | ETH deposit         | `1000000000000000000` (1 ETH) |
| `timelocks`     | `Timelocks` | Time struct         | See below                     |

---

## The `Timelocks` Struct

```solidity
struct Timelocks {
    uint256 withdrawalSrcTimelock;           // When source withdrawal starts
    uint256 publicWithdrawalSrcTimelock;     // When public withdrawal starts
    uint256 cancellationSrcTimelock;         // When source cancellation starts
    uint256 publicCancellationSrcTimelock;   // When public cancellation starts
    uint256 withdrawalDstTimelock;           // When destination withdrawal starts
    uint256 publicWithdrawalDstTimelock;     // When public withdrawal starts
    uint256 cancellationDstTimelock;         // When destination cancellation starts
}
```

### **Timelock Values (in seconds):**

- `withdrawalSrcTimelock`: 300 (5 minutes)
- `publicWithdrawalSrcTimelock`: 600 (10 minutes)
- `cancellationSrcTimelock`: 900 (15 minutes)
- `publicCancellationSrcTimelock`: 1200 (20 minutes)
- `withdrawalDstTimelock`: 300 (5 minutes)
- `publicWithdrawalDstTimelock`: 600 (10 minutes)
- `cancellationDstTimelock`: 900 (15 minutes)

---

## How Arguments Are Passed

### **Example: `addressOfEscrowSrc()` Call**

```solidity
// Create the Immutables struct
IBaseEscrow.Immutables memory immutables = IBaseEscrow.Immutables({
    orderHash: 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef,
    hashlock: keccak256(abi.encodePacked("mysecret123")),
    maker: Address.wrap(0x70997970C51812dc3A010C7d01b50e0d17dc79C8),
    taker: Address.wrap(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266),
    token: Address.wrap(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48), // USDC
    amount: 100000000, // 100 USDC (6 decimals)
    safetyDeposit: 1000000000000000000, // 1 ETH (18 decimals)
    timelocks: Timelocks({
        withdrawalSrcTimelock: 300,
        publicWithdrawalSrcTimelock: 600,
        cancellationSrcTimelock: 900,
        publicCancellationSrcTimelock: 1200,
        withdrawalDstTimelock: 300,
        publicWithdrawalDstTimelock: 600,
        cancellationDstTimelock: 900
    })
});

// Call the function
address escrowAddress = escrowFactory.addressOfEscrowSrc(immutables);
```

### **Example: `createDstEscrow()` Call**

```solidity
// Same immutables struct as above
IBaseEscrow.Immutables memory dstImmutables = immutables;

// Current timestamp for cancellation
uint256 srcCancellationTimestamp = block.timestamp;

// Call with ETH for safety deposit
escrowFactory.createDstEscrow{value: 1000000000000000000}(dstImmutables, srcCancellationTimestamp);
```

---

## Using `cast` to Call Functions

### **View Function (Free):**

```bash
# This would be complex with cast due to struct encoding
# Better to use a script or web3 library
```

### **State-Changing Function:**

```bash
# This requires complex struct encoding + ETH value
# Not practical with cast alone
```

---

## Key Insights

1. **Same Struct for Both:** Both functions use identical `Immutables` struct
2. **Complex Input:** The struct contains all swap parameters
3. **Deterministic:** `addressOfEscrowSrc()` always returns same address for same inputs
4. **Expensive:** `createDstEscrow()` costs gas + requires ETH deposit
5. **Sequential:** Usually call `addressOfEscrowSrc()` first, then `createDstEscrow()`

---

## Integration with Example Scripts

The `CreateOrder.s.sol` script shows how these functions are called:

1. **Compute Address:** `addressOfEscrowSrc(immutables)`
2. **Send Safety Deposit:** Transfer ETH to computed address
3. **Fill Order:** Creates source escrow via LOP
4. **Deploy Destination:** `createDstEscrow(immutables, timestamp)`

This creates the complete cross-chain swap setup! 🎯
