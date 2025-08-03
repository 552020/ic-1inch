# EIP-712 Order Execution & Asset Locking

> **Key Question**: How does the resolver lock the maker's assets in the source escrow using the EIP-712 order intent?

## Overview

The Fusion protocol uses **EIP-712 signed orders** (intents) that allow resolvers to execute swaps on behalf of makers. When a resolver wins an auction, they use the maker's signed intent to lock the maker's assets in the source escrow through the **Limit Order Protocol**.

## EIP-712 Order Structure

### **The Signed Order Intent**

> **Important**: The maker signs a "blind" intent that **any whitelisted resolver** (or any resolver if public) can execute. The hashlock is **NOT in the EIP-712 order** - it's in the extension data that resolvers provide during execution.

```solidity
// EIP-712 Order structure (from Limit Order Protocol)
struct Order {
    uint256 salt;                    // Unique identifier
    Address maker;                   // User who signed the order
    Address receiver;                // Who receives the tokens (usually maker)
    Address makerAsset;              // Token the maker is selling
    Address takerAsset;              // Token the maker wants to receive
    uint256 makingAmount;            // Amount of makerAsset to sell
    uint256 takingAmount;            // Amount of takerAsset to receive
    MakerTraits makerTraits;         // Order configuration flags
    // NOTE: No hashlock here! It's in the extension data
}
```

### **What the Maker Signs**

```javascript
// Maker creates and signs this order off-chain
const order = {
  salt: "123456789",
  maker: "0x1234...maker", // Maker's address
  receiver: "0x1234...maker", // Usually same as maker
  makerAsset: "0xA0b8...USDC", // USDC token on Ethereum
  takerAsset: "0x2791...USDT", // USDT token on Polygon
  makingAmount: "1000000000", // 1000 USDC
  takingAmount: "999000000", // 999 USDT expected
  makerTraits: "0x...", // Configuration flags
};

// Maker signs with EIP-712
const signature = await maker.signTypedData(domain, types, order);
// signature = { r, s, v } or { r, vs } format
```

## How Asset Locking Works

### **Step 1: Resolver Calls Limit Order Protocol**

> **Important Note**: When the maker signs the intent, they **don't know the escrow address yet**. The escrow address is computed deterministically by the resolver using the order hash, resolver address, and other parameters. The maker only needs to approve tokens to the Limit Order Protocol.

```solidity
// Resolver executes the signed order through Limit Order Protocol
function fillOrder(
    IOrderMixin.Order calldata order,    // The signed order
    bytes32 r,                           // Signature component
    bytes32 vs,                          // Signature component
    uint256 amount,                      // Amount to fill
    TakerTraits takerTraits,            // Resolver's preferences
    bytes calldata args                  // Extension data
) external {
    // 1. Verify maker's signature
    address maker = _recoverOrderSigner(order, r, vs);
    require(maker == order.maker, "Invalid signature");

    // 2. Compute escrow address deterministically
    bytes32 orderHash = _hashOrder(order);
    address escrowAddress = _computeEscrowAddress(orderHash, msg.sender, args);

    // 3. Transfer maker's tokens to escrow
    IERC20(order.makerAsset).transferFrom(
        order.maker,           // From maker
        escrowAddress,         // To computed escrow address
        order.makingAmount     // Amount specified in order
    );

    // 4. Call postInteraction to create escrow
    _postInteraction(order, extension, orderHash, taker, ...);
}
```

### **Step 2: Post-Interaction Creates Escrow**

```solidity
// In BaseEscrowFactory._postInteraction
function _postInteraction(
    IOrderMixin.Order calldata order,
    bytes calldata extension,
    bytes32 orderHash,
    address taker,                    // The resolver
    uint256 makingAmount,
    uint256 takingAmount,
    uint256 remainingMakingAmount,
    bytes calldata extraData
) internal override {
    // Parse escrow parameters from extraData
    ExtraDataArgs memory args = abi.decode(extraData, (ExtraDataArgs));

    // Create source escrow with maker's tokens already inside
    address escrowSrc = ESCROW_SRC_IMPLEMENTATION.cloneDeterministic(
        _computeSalt(orderHash, args.srcChainId),
        makingAmount + args.srcSafetyDeposit  // Total value needed
    );

    // Maker's tokens are now locked in the escrow!
}
```

## The Complete Flow

### **Phase 1: Maker Preparation**

```javascript
// 1. Maker approves tokens to Limit Order Protocol (NOT to escrow!)
// The maker doesn't know the escrow address yet - it will be computed later
await usdcToken.approve(limitOrderProtocol.address, "1000000000");

// 2. Maker signs EIP-712 order intent
// At this point, the maker has NO IDEA where their tokens will be sent
// They're trusting the protocol to compute the escrow address correctly
const order = {
  maker: makerAddress,
  makerAsset: usdcToken.address,
  makingAmount: "1000000000", // 1000 USDC
  // ... other parameters
  // NOTE: No escrow address in the order! It's computed deterministically
};

const signature = await signer.signTypedData(domain, types, order);

// 3. Order is distributed to resolvers
// Resolvers will compute the escrow address when they execute
await orderbook.publishOrder(order, signature);
```

### **Phase 2: Resolver Execution**

```javascript
// Resolver wins auction and executes
async function executeOrder(order, signature) {
  // 1. Prepare escrow parameters
  const escrowParams = buildEscrowParams(order);

  // 2. Call Limit Order Protocol with special extension
  const tx = await limitOrderProtocol.fillOrder(
    order, // Maker's signed order
    signature.r, // Signature
    signature.vs, // Signature
    order.makingAmount, // Full amount
    takerTraits, // Resolver preferences
    escrowParams // Escrow creation data
  );

  // Result: Maker's 1000 USDC is now locked in source escrow!
}
```

### **Phase 3: What Happens Inside the Transaction**

```
1. Limit Order Protocol verifies maker's signature ✅
2. Protocol transfers 1000 USDC from maker to escrow address ✅
3. Protocol calls postInteraction on EscrowFactory ✅
4. EscrowFactory creates source escrow with tokens inside ✅
5. Resolver is recorded as the "taker" of the order ✅
6. Escrow is locked with resolver's chosen hashlock ✅
```

### **Phase 4: The Hashlock Revelation Process**

```javascript
// After both escrows are created:

// 1. Resolver has locked assets but needs secret to withdraw
const srcEscrow = "0x123..."; // Has maker's 1000 USDC
const dstEscrow = "0x456..."; // Has resolver's 999 USDT

// 2. Maker reveals secret to resolver (off-chain)
// This is when the "intent becomes complete"
const secret = "0xabc123...";
await maker.revealSecretToResolver(resolver, secret);

// 3. Resolver uses secret to withdraw from both escrows
await srcEscrow.withdraw(secret); // Gets 1000 USDC
await dstEscrow.withdraw(secret); // Maker gets 999 USDT

// 4. Cross-chain swap complete!
```

## Deterministic Escrow Address Computation

### **How Escrow Addresses are Computed**

```javascript
// The resolver computes the escrow address using deterministic parameters
function computeEscrowAddress(order, resolver, extraData) {
  // 1. Hash the order to get unique identifier
  const orderHash = hashOrder(order);

  // 2. Extract chain-specific parameters
  const { srcChainId, hashlock, timelocks } = extraData;

  // 3. Compute salt for CREATE2
  const salt = keccak256(
    abi.encodePacked(
      orderHash, // Unique per order
      srcChainId, // Chain identifier
      resolver // Resolver address
    )
  );

  // 4. Compute CREATE2 address
  const escrowAddress = computeAddress(
    ESCROW_SRC_IMPLEMENTATION, // Implementation contract
    salt, // Computed salt
    ESCROW_FACTORY_ADDRESS // Factory address
  );

  return escrowAddress;
}
```

### **Why This Works**

1. **Deterministic**: Same inputs always produce same address
2. **Unique**: Each order + resolver combination gets unique escrow
3. **Predictable**: Both maker and resolver can compute the same address
4. **Secure**: Only the winning resolver can actually create the escrow

### **Maker's Trust Model: "Blind Intent" Signing**

```javascript
// The maker signs a "BLIND INTENT" that means:
"Any whitelisted resolver (or any resolver if public) can execute this swap
 using ANY hashlock they choose, as long as they provide the secret later"

// When maker signs, they trust:
1. The Limit Order Protocol to verify signatures correctly
2. The EscrowFactory to compute addresses deterministically
3. The protocol to only transfer to computed escrow addresses
4. The escrow implementation to handle funds securely
5. The timelock mechanism to allow cancellation if no secret is revealed

// Maker does NOT need to trust:
- Any specific resolver (they compete in auction)
- Any specific escrow address (computed deterministically)
- Any specific hashlock (resolver chooses during execution)
- The timing of execution (protected by timelocks)
- The resolver to reveal the secret (they lose money if they don't)
```

## Key Components

### **1. EIP-712 Domain**

```javascript
const domain = {
  name: "1inch Limit Order Protocol",
  version: "4",
  chainId: 1, // Ethereum mainnet
  verifyingContract: limitOrderProtocol.address,
};
```

### **2. Order Types**

```javascript
const types = {
  Order: [
    { name: "salt", type: "uint256" },
    { name: "maker", type: "address" },
    { name: "receiver", type: "address" },
    { name: "makerAsset", type: "address" },
    { name: "takerAsset", type: "address" },
    { name: "makingAmount", type: "uint256" },
    { name: "takingAmount", type: "uint256" },
    { name: "makerTraits", type: "uint256" },
  ],
};
```

### **3. Extension Data (Escrow Parameters)**

> **Key Point**: The hashlock (hash of secret) is **NOT signed by the maker**. It's provided by the resolver during execution. This means the intent is valid **immediately** when signed, but the swap only completes when the secret is revealed.

```solidity
// Extra data passed to postInteraction (NOT signed by maker)
struct ExtraDataArgs {
    bytes32 hashlock;           // Hash of secret - PROVIDED BY RESOLVER
    uint32 srcChainId;          // Source chain ID
    Address dstToken;           // Destination token
    uint256 srcSafetyDeposit;   // Source safety deposit
    uint256 dstSafetyDeposit;   // Destination safety deposit
    Timelocks timelocks;        // Time constraints
}
```

### **Intent Validity Timeline**

```
1. Maker signs EIP-712 order → Intent is VALID immediately ✅
2. Resolver executes with hashlock → Assets locked in escrows ✅
3. Maker reveals secret → Resolver can complete swap ✅
4. Without secret revelation → Escrows can be cancelled after timelock ⏰
```

## What Prevents Resolver from Stealing?

> **Critical Question**: What stops the resolver from taking the maker's tokens and running away?

### **Multiple Protection Mechanisms**

#### **1. Safety Deposits (Economic Security)**

> **Your Bank Analogy is Perfect!** The escrow contracts are like "bank vaults" that hold the money, and the secret is like a "key" that unlocks both vaults simultaneously.

```solidity
// Resolver must deposit their own money BEFORE getting maker's tokens
struct Immutables {
    uint256 safetyDeposit;  // Resolver loses this if they don't complete swap
    // ...
}

// When resolver creates escrow, they must send:
// - The destination tokens (999 USDT)
// - PLUS safety deposit (e.g., 0.1 ETH worth ~$200)
// Total: ~$1200 locked by resolver vs $1000 from maker
```

### **How "Locking" Works in Code**

#### **Step 1: Creating the "Bank Vaults" (Escrow Contracts)**

```solidity
// Source Escrow = "Vault A" (holds maker's money)
contract EscrowSrc {
    uint256 public amount;        // 1000 USDC
    bytes32 public hashlock;      // "Lock that needs maker's key"
    address public maker;         // Maker's address
    address public taker;         // Resolver's address

    // Money is "locked" - can only be withdrawn with secret
    function withdraw(bytes32 secret) external {
        require(keccak256(abi.encode(secret)) == hashlock, "Wrong key!");
        require(msg.sender == taker, "Only resolver can withdraw");

        // Transfer 1000 USDC to resolver
        IERC20(token).transfer(taker, amount);
    }
}

// Destination Escrow = "Vault B" (holds resolver's money)
contract EscrowDst {
    uint256 public amount;        // 999 USDT
    bytes32 public hashlock;      // "Same lock - needs same key!"
    address public maker;         // Maker's address
    address public taker;         // Resolver's address

    // Money is "locked" - can only be withdrawn with secret
    function withdraw(bytes32 secret) external {
        require(keccak256(abi.encode(secret)) == hashlock, "Wrong key!");
        require(msg.sender == maker, "Only maker can withdraw");

        // Transfer 999 USDT to maker
        IERC20(token).transfer(maker, amount);
    }
}
```

#### **Step 2: The "Bank Teller" Process (Resolver Execution)**

```solidity
// Resolver acts as "bank teller" setting up both vaults
contract Resolver {
    function executeSwap(Order calldata order) external {
        // 1. Create "Vault A" for maker's money
        address srcEscrow = factory.createEscrowSrc{value: 0}(
            orderHash,
            hashlock,     // "Lock chosen by resolver"
            order.maker,  // "Vault belongs to maker"
            address(this) // "But resolver can withdraw with key"
        );

        // 2. Create "Vault B" for resolver's money + deposit
        address dstEscrow = factory.createEscrowDst{
            value: 999e6 + 0.1 ether  // 999 USDT + 0.1 ETH safety deposit
        }(
            orderHash,
            hashlock,     // "Same lock - same key needed!"
            address(this), // "Vault belongs to resolver"
            order.maker   // "But maker can withdraw with key"
        );

        // 3. Both vaults are now "locked" - waiting for maker's key
    }
}
```

#### **Step 3: The "Key Exchange" (Secret Revelation)**

```javascript
// Off-chain process (like giving key to bank teller)
async function secretRevelation() {
  // 1. Maker checks both vaults are properly funded
  const srcBalance = await srcEscrow.getBalance(); // 1000 USDC ✅
  const dstBalance = await dstEscrow.getBalance(); // 999 USDT + 0.1 ETH ✅

  if (srcBalance === "1000000000" && dstBalance >= "999000000") {
    // 2. Maker gives "key" to resolver
    const secret = "0xabc123..."; // The secret key
    await maker.revealSecret(resolver, secret);

    // 3. Resolver uses key to unlock both vaults
    await srcEscrow.withdraw(secret); // Gets 1000 USDC
    await dstEscrow.withdraw(secret); // Maker gets 999 USDT
  }
}
```

#### **Step 4: The "Vault Unlocking" (Withdrawals)**

```solidity
// When resolver calls withdraw with the secret:
function withdraw(bytes32 secret) external {
    // 1. Check if "key" matches the "lock"
    require(keccak256(abi.encode(secret)) == hashlock, "Wrong key!");

    // 2. Check who can open this specific vault
    require(msg.sender == authorizedWithdrawer, "Not authorized");

    // 3. "Unlock the vault" and transfer money
    locked = false;
    IERC20(token).transfer(msg.sender, amount);

    emit VaultUnlocked(secret, amount);
}
```

#### **2. Atomic Locking (Both Escrows Required)**

```javascript
// Resolver can't withdraw from source escrow until BOTH escrows exist
// and they have the secret from the maker

// Source escrow (has maker's 1000 USDC)
await srcEscrow.withdraw(secret); // ❌ FAILS without secret

// Destination escrow (has resolver's 999 USDT)
await dstEscrow.withdraw(secret); // ❌ FAILS without secret

// Resolver gets secret ONLY after they've locked their own tokens!
```

#### **3. Secret Revelation Control**

```javascript
// The maker controls the final step:
// 1. Resolver creates both escrows ✅
// 2. Resolver locks their own tokens ✅
// 3. Maker sees both escrows are funded ✅
// 4. ONLY THEN does maker reveal secret to resolver ✅

// If resolver tries to cheat:
// - Maker doesn't reveal secret
// - Resolver loses their safety deposit
// - Maker gets their tokens back after timelock
```

#### **4. Timelock Protection**

```solidity
// If resolver disappears or tries to cheat:
// - Maker can cancel and get tokens back after timelock
// - Resolver loses their safety deposit
// - No one can steal funds - they just get returned

function publicCancel() external {
    require(block.timestamp > timelocks.publicCancellation);
    // Maker gets their tokens back
    // Resolver loses safety deposit
}
```

### **Economic Incentive Analysis**

```javascript
// For resolver to profit by stealing:
// They would need to steal: $1000 (maker's USDC)
// But they would lose: $999 (their USDT) + $200 (safety deposit) = $1199
// Net result: LOSE $199 ❌

// For resolver to profit legitimately:
// They get: $1000 (maker's USDC) + resolver fee (e.g., $5)
// They pay: $999 (to maker via destination escrow)
// Net result: PROFIT $6 ✅
```

### **The Complete Protection Flow**

```
1. Resolver wants to execute order
   ↓
2. Resolver must create BOTH escrows and fund destination escrow
   Cost: $999 USDT + $200 safety deposit = $1199 locked
   ↓
3. Maker sees both escrows are properly funded
   ↓
4. Maker reveals secret to resolver
   ↓
5. Resolver withdraws from both escrows
   Gets: $1000 USDC + $200 safety deposit back + $5 fee
   Net: $6 profit ✅

// If resolver tries to cheat at any step:
// - Maker doesn't reveal secret
// - Resolver loses $1199, gets $0
// - Maker gets refund after timelock
```

## Security Mechanisms

### **1. Signature Verification**

```solidity
function _recoverOrderSigner(
    Order calldata order,
    bytes32 r,
    bytes32 vs
) internal view returns (address) {
    bytes32 orderHash = _hashTypedDataV4(
        keccak256(abi.encode(ORDER_TYPEHASH, order))
    );

    return ECDSA.recover(orderHash, r, vs);
}
```

### **2. Approval Mechanism**

```javascript
// Maker must pre-approve tokens
// This is what allows the resolver to move maker's funds
await token.approve(limitOrderProtocol, amount);
```

### **3. Order Uniqueness**

```solidity
// Each order has unique salt to prevent replay
mapping(bytes32 => uint256) public remainingOrderAmounts;

function fillOrder(...) {
    bytes32 orderHash = _hashOrder(order);
    require(remainingOrderAmounts[orderHash] > 0, "Order filled");

    remainingOrderAmounts[orderHash] -= amount;
}
```

## Example: Complete Asset Locking Flow

### **Maker Side (Off-chain)**

```javascript
// 1. Approve USDC to Limit Order Protocol
await usdc.approve(limitOrderProtocol.address, "1000000000");

// 2. Create order intent
const order = {
  salt: Date.now(),
  maker: "0x1234...maker",
  receiver: "0x1234...maker",
  makerAsset: "0xA0b8...USDC", // USDC on Ethereum
  takerAsset: "0x2791...USDT", // USDT on Polygon
  makingAmount: "1000000000", // 1000 USDC
  takingAmount: "999000000", // 999 USDT
  makerTraits: "0x0",
};

// 3. Sign with EIP-712
const signature = await signer.signTypedData(domain, types, order);

// 4. Publish to resolvers
await publishOrder(order, signature);
```

### **Resolver Side (On-chain)**

```solidity
// Resolver executes when profitable
contract Resolver {
    function executeSwap(
        Order calldata order,
        bytes32 r,
        bytes32 vs,
        bytes calldata escrowData
    ) external {
        // This single call:
        // 1. Verifies maker's signature
        // 2. Transfers maker's USDC to escrow
        // 3. Creates source escrow
        // 4. Records resolver as taker

        limitOrderProtocol.fillOrder(
            order,                    // Maker's signed intent
            r, vs,                    // Maker's signature
            order.makingAmount,       // Full amount
            buildTakerTraits(),       // Resolver config
            escrowData               // Escrow parameters
        );

        // Result: 1000 USDC locked in source escrow!
    }
}
```

## Why This Design Works

### **1. Trustless Execution**

- Maker pre-signs intent, doesn't need to be online
- Resolver can execute anytime using the signature
- No additional maker interaction required

### **2. Atomic Asset Transfer**

- Signature verification and asset transfer happen atomically
- Either the whole transaction succeeds or fails
- No partial states possible

### **3. Flexible Timing**

- Order can be executed whenever resolver finds it profitable
- Maker doesn't need to coordinate timing
- Auction mechanism handles optimal execution timing

### **4. Cross-Chain Coordination**

- Same signature can be used to compute escrow addresses on both chains
- Deterministic escrow creation enables cross-chain coordination
- Secret revelation mechanism ensures atomic completion

## Conclusion

The resolver locks the maker's assets by **executing the maker's pre-signed EIP-712 order intent** through the Limit Order Protocol. The signature acts as permission for the resolver to:

1. **Transfer** the maker's tokens to the escrow
2. **Create** the source escrow contract
3. **Lock** the assets until the secret is revealed
4. **Coordinate** with the destination chain escrow

This design enables trustless, atomic cross-chain swaps where makers only need to sign once, and resolvers handle all the complex cross-chain coordination using that signature.
