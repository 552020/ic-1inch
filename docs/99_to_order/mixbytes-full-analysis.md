# MixBytes Full Analysis: 1inch Protocol Deep Dive

_Complete analysis of "Modern DEXes, how they're made: 1inch Limit Order Protocol, Fusion & Fusion+"_

---

## Article Overview

**Source:** [MixBytes Blog - Modern DEXes, how they're made: 1inch Limit Order Protocol, Fusion & Fusion+](https://mixbytes.io/blog/modern-dex-es-how-they-re-made-1inch-limit-order-protocols)

**Author:** Sergey Boogerwooger (Security researcher at MixBytes)

**Focus:** Technical deep dive into 1inch's three-layer protocol architecture: Limit Order Protocol → Fusion → Fusion+

---

## Key Technical Links Extracted

### **✅ Core Repositories:**

#### **1. Limit Order Protocol (LOP):**

- **Main Repository:** [1inch/limit-order-protocol](https://github.com/1inch/limit-order-protocol) - "The main repository for this protocol is here"
- **Core Contract:** [OrderMixin.sol](https://github.com/1inch/limit-order-protocol/blob/cbcbae635487376279c87ca9e9697b521ab7481a/contracts/OrderMixin.sol#L28) - Main order settlement logic
- **MakerTraits:** [MakerTraitsLib.sol](https://github.com/1inch/limit-order-protocol/blob/cbcbae635487376279c87ca9e9697b521ab7481a/contracts/libraries/MakerTraitsLib.sol#L13-L27) - Order condition encoding
- **Order Struct:** [IOrderMixin.sol](https://github.com/1inch/limit-order-protocol/blob/cbcbae635487376279c87ca9e9697b521ab7481a/contracts/interfaces/IOrderMixin.sol#L10-L19) - Order structure definition
- **Order Hashing:** [OrderLib.sol](https://github.com/1inch/limit-order-protocol/blob/cbcbae635487376279c87ca9e9697b521ab7481a/contracts/OrderLib.sol#L50-L66) - Deterministic order hash calculation
- **Extension System:** [ExtensionLib.sol](https://github.com/1inch/limit-order-protocol/blob/cbcbae635487376279c87ca9e9697b521ab7481a/contracts/libraries/ExtensionLib.sol#L17-L25) - Extension data handling
- **Extensions Directory:** [Extensions](https://github.com/1inch/limit-order-protocol/tree/cbcbae635487376279c87ca9e9697b521ab7481a/contracts/extensions) - All extension contracts

#### **2. Fusion Protocol:**

- **Repository:** [1inch/fusion-sdk](https://github.com/1inch/fusion-sdk) - "The functionality of creating Fusion limit orders can be found in this repository"
- **FusionOrder Class:** [fusion-order.ts](https://github.com/1inch/fusion-sdk/blob/505c9df23dfbe9782eec27419c80ce927e9bbbeb/src/fusion-order/fusion-order.ts#L25) - Contains `auctionDetails` field
- **Auction Details:** [auctionDetails](https://github.com/1inch/fusion-sdk/blob/035520eaed26852269c0cc7b1d5fceac0997c83c/src/fusion-order/fusion-order.ts#L47) - Dutch auction parameters
- **FusionExtension:** [Constructor](https://github.com/1inch/fusion-sdk/blob/505c9df23dfbe9782eec27419c80ce927e9bbbeb/src/fusion-order/fusion-order.ts#L76-L83) - Constructor for Dutch auction parameters
- **Auction Details Documentation:** [auction-details](https://github.com/1inch/fusion-sdk/tree/505c9df23dfbe9782eec27419c80ce927e9bbbeb/src/fusion-order/auction-details#auctiondetails) - Encoding of Fusion auction parameters
- **DutchAuctionCalculator:** [Extension](https://github.com/1inch/limit-order-protocol/blob/cbcbae635487376279c87ca9e9697b521ab7481a/contracts/extensions/DutchAuctionCalculator.sol#L15) - Extension for auction calculations
- **Video Tutorial:** [YouTube Video](https://www.youtube.com/watch?v=7kgt5Nn_y3s&t=957s) - How Fusion orders functionality can be integrated into any DeFi frontend

#### **3. Fusion+ Protocol (Cross-Chain):**

- **Repository:** [1inch/cross-chain-swap](https://github.com/1inch/cross-chain-swap) - "The repository used for cross-chain swaps is here"
- **Escrow Factory:** [BaseEscrowFactory.sol](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L121) - `createDstEscrow()` function
- **Destination Escrow Creation:** [createDstEscrow](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L122-L127) - Safety deposit check
- **Timelock Validation:** [Timelock Check](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L129-L132) - Correctness of timelock value
- **Escrow Deployment:** [Deployment](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L134-L135) - Escrow deployment process
- **Token Transfer:** [Transfer](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L136-L138) - Transfer of needed tokens
- **CREATE2 Documentation:** [cloneDeterministic](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/documentation/src/contracts/libraries/Clones.sol/library.Clones.md#clonedeterministic) - Deterministic deployment
- **Address Calculation:** [addressOfEscrowSrc](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L146-L148) and [addressOfEscrowDst](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L153-L155) - Escrow address calculation
- **Post-Interaction:** [\_postInteraction](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L44-L64) - Source chain escrow deployment
- **Hashlock Setting:** [Hashlock](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/BaseEscrowFactory.sol#L86-L88) - Setting hashlock in arguments
- **Merkle Tree:** [MerkleStorageInvalidator.sol](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/MerkleStorageInvalidator.sol#L62-L68) - Merkle tree values and invalidation logic
- **Escrow Source:** [EscrowSrc.sol](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/EscrowSrc.sol#L24) - Core escrow functions
- **Address Restrictions:** [Address Modifiers](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/EscrowSrc.sol#L55-L57) - Address restriction modifiers
- **Time Restrictions:** [Time Modifiers](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/EscrowSrc.sol#L56-L57) - Time restriction modifiers
- **Hashlock Verification:** [Hashlock](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/EscrowSrc.sol#L114) - Hashlock verification
- **Bitcoin Script Example:** [Hashlock Example](https://gist.github.com/xhliu/79c416b6c1357bde22532e9038a25a22#file-hltc-js) - "Look at an example Bitcoin Script for hashlock here – easy, isn't it?"

### **✅ Technical Documentation:**

#### **Order Settlement Functions:**

- **fillOrder()** - Basic order filling with signature verification
- **fillOrderArgs()** - Order filling with additional arguments
- **fillContractOrder()** - Contract-based order filling (EIP-1271)
- **fillContractOrderArgs()** - Contract filling with arguments

#### **Extension System:**

- **ApprovalPreInteraction.sol** - Token approval handling
- **DutchAuctionCalculator** - Auction price calculations
- **ERC721Proxy** - NFT token support
- **FeeTaker** - Fee collection mechanisms

#### **Gas Optimization:**

- **"clones-with-immutable-args"** - [Repository](https://github.com/wighawag/clones-with-immutable-args?tab=readme-ov-file#cloneswithimmutableargs) - "look at this repo"
- **EIP-1167** - [Minimal proxy standard](https://eips.ethereum.org/EIPS/eip-1167) - Minimalistic transparent proxy
- **CREATE2** - [Deterministic address generation](https://eips.ethereum.org/EIPS/eip-1014) - Deterministic address generation
- **Immutables Example:** [EscrowSrc.sol](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/EscrowSrc.sol#L38) - "all functions contain Immutables calldata immutables parameters"
- **Immutables Validation:** [Escrow.sol](https://github.com/1inch/cross-chain-swap/blob/60228e33b46ac0bfbf888aa9f0a8d5fd46243c6e/contracts/Escrow.sol#L30) - "onlyValidImmutables(immutables) check"

---

## Technical Implementation Details

### **✅ Limit Order Protocol (LOP) Architecture:**

#### **Order Structure:**

```solidity
struct Order {
    address maker;
    address taker;
    address makerAsset;
    address takerAsset;
    uint256 makingAmount;
    uint256 takingAmount;
    bytes32 salt;
    bytes makerTraits;
}
```

#### **Key Features:**

- **Programmable conditions** via predicates
- **Extension system** for additional data
- **Partial fills** support
- **Multiple approval methods** (approve, permit, permit2)
- **ETH/WETH wrapping** before/after swaps
- **Custom proxies** for non-standard tokens

#### **Order Validation Flow:**

1. **Extension validation** - `isValidExtension()`
2. **Sender/expiration checks**
3. **Predicate evaluation** - External static call
4. **Amount calculations** - Exact making/taking amounts
5. **Invalidation checks** - BitInvalidator
6. **Pre-interaction** - Maker setup
7. **Asset transfers** - Maker → Taker, Taker → Maker
8. **Post-interaction** - Cleanup

### **✅ Fusion Protocol (Dutch Auctions):**

#### **Auction Parameters:**

```solidity
struct AuctionDetails {
    uint256 startTime;
    uint256 duration;
    uint256 bumpRate;
    uint256[] points;  // Piecewise linear function
    uint256 gasCost;
    uint256 gasEstimation;
}
```

#### **Key Features:**

- **Dutch auction mechanism** - Price decreases over time
- **Piecewise linear functions** - Multiple price points
- **Gas cost compensation** - For order fillers
- **Fast execution** - First bid wins

### **✅ Fusion+ Protocol (Cross-Chain Atomic Swaps):**

#### **4-Phase Swap Process:**

**Phase 1: Announcement**

1. Maker creates Fusion limit order with `hash(secret)`
2. Relayer shares order with resolvers
3. Dutch auction begins

**Phase 2: Deposit**

1. Winning resolver creates Ethereum escrow
2. Resolver creates destination chain escrow
3. Safety deposit provided

**Phase 3: Execution**

1. Relayer verifies both escrows
2. Maker reveals secret
3. Resolver unlocks tokens on both chains

**Phase 4: Recovery**

1. Timeout handling
2. Public recovery mechanisms
3. Safety deposit distribution

#### **Escrow Contract Architecture:**

**Gas-Optimized Deployment:**

```solidity
// Immutables pattern for gas optimization
function withdraw(bytes32 secret, Immutables calldata immutables)
    external
    onlyTaker(immutables)
    onlyAfter(immutables.timelocks.get(TimelocksLib.Stage.SrcWithdrawal))
    onlyBefore(immutables.timelocks.get(TimelocksLib.Stage.SrcCancellation))
{
    _withdrawTo(secret, msg.sender, immutables);
}

// Parameter validation
modifier onlyValidImmutables(Immutables calldata immutables) {
    require(
        address(this) == addressOfEscrow(immutables.hash()),
        "Invalid immutables"
    );
    _;
}
```

#### **Partial Fills with Merkle Trees:**

- **Multiple secrets** for progressive fills
- **Merkle tree structure** for efficient verification
- **N+1 secrets** (4 parts = 4 secrets + 1 completion)
- **Progressive unlocking** based on fill percentage

---

## Critical Implementation Insights

### **✅ Gas Optimization Strategy:**

#### **"Clones-with-immutable-args" Approach:**

- **Minimalistic transparent proxy** (EIP-1167)
- **Parameters in bytecode** instead of storage
- **200 gas per byte** cost analysis
- **Tens of thousands of gas saved** per deployment

#### **Immutable Parameter Pattern:**

- **Parameters passed with every call**
- **CREATE2 address validation**
- **No storage operations**
- **Compact bytecode**

### **✅ Security Mechanisms:**

#### **Hashlock Implementation:**

- **Secret generation** and hashing
- **Cross-chain secret sharing**
- **Atomic execution guarantee**
- **Timeout protection**

#### **Timelock System:**

- **Multiple stages** with specific time windows
- **Public recovery periods**
- **Safety deposit incentives**
- **Automatic timeout handling**

### **✅ Recovery Mechanisms:**

#### **Public Recovery:**

- **Anyone can complete failed swaps**
- **Safety deposit rewards**
- **Timeout-based recovery**
- **Interrupted process handling**

---

## Implications for ICP Implementation

### **✅ Architecture Adaptations:**

#### **1. Gas Optimization for ICP:**

- **Minimal canister deployment** (similar to proxy pattern)
- **Parameter passing** instead of storage
- **Efficient bytecode** for cross-chain operations
- **Cycle cost optimization**

#### **2. Immutables Pattern:**

- **Parameter validation** in our Motoko canisters
- **Deterministic addressing** for escrow canisters
- **Integrity checks** for swap parameters
- **Efficient parameter passing**

#### **3. Partial Fills:**

- **Merkle tree implementation** in Motoko
- **Multiple secret management**
- **Progressive fill logic**
- **Secret verification system**

#### **4. Recovery Mechanisms:**

- **Safety deposit system**
- **Timeout handling**
- **Public recovery functions**
- **Interrupted swap recovery**

### **✅ Technical Requirements:**

#### **1. Merkle Tree Implementation:**

```motoko
// Merkle tree for partial fills
type MerkleTree = {
    secrets: [Blob];
    root: Blob;
};

// Get secret for specific fill percentage
func getSecretForFill(fillPercentage: Nat) : Blob {
    let secretIndex = getSecretIndex(fillPercentage);
    merkleTree.secrets[secretIndex]
};
```

#### **2. Immutables Validation:**

```motoko
// Validate parameters using canister address
func validateImmutables(immutables: EscrowParams) : Bool {
    let expectedAddress = computeEscrowAddress(immutables);
    Principal.fromActor(this) == expectedAddress
};
```

#### **3. Gas Optimization:**

- **Minimal canister state**
- **Efficient parameter passing**
- **Compact bytecode**
- **Cycle cost optimization**

---

## Key Insights for Our Implementation

### **✅ Architecture Decisions:**

#### **1. Follow 1inch Patterns:**

- **Use immutables pattern** for parameter validation
- **Implement Merkle trees** for partial fills
- **Optimize for gas/cycles** in deployment
- **Include recovery mechanisms**

#### **2. ICP-Specific Adaptations:**

- **Canister-based immutables** instead of CREATE2
- **Motoko Merkle tree** implementation
- **HTTP outcalls** for cross-chain verification
- **Cycle optimization** instead of gas optimization

#### **3. Production Readiness:**

- **Safety deposit system**
- **Timeout handling**
- **Public recovery functions**
- **Interrupted swap recovery**

### **✅ Implementation Priority:**

#### **Phase 1: Core Escrows (MVP)**

- **Basic escrow functionality**
- **Hashlock + timelock**
- **Parameter validation**
- **Atomic execution**

#### **Phase 2: Advanced Features (Stretch Goals)**

- **Merkle tree partial fills**
- **Recovery mechanisms**
- **Safety deposits**
- **Gas/cycle optimization**

---

## Useful Links for Implementation

### **✅ Core Repositories:**

- **[1inch/limit-order-protocol](https://github.com/1inch/limit-order-protocol)** - Base LOP implementation
- **[1inch/fusion-sdk](https://github.com/1inch/fusion-sdk)** - Fusion protocol SDK
- **[1inch/cross-chain-swap](https://github.com/1inch/cross-chain-swap)** - Fusion+ escrow contracts
- **[0age/clones-with-immutable-args](https://github.com/0age/clones-with-immutable-args)** - Gas optimization pattern

### **✅ Technical Standards:**

- **[EIP-1167](https://eips.ethereum.org/EIPS/eip-1167)** - Minimal proxy standard
- **[EIP-1271](https://eips.ethereum.org/EIPS/eip-1271)** - Contract signature standard
- **[CREATE2](https://eips.ethereum.org/EIPS/eip-1014)** - Deterministic address generation

### **✅ Documentation:**

- **1inch Fusion+ Whitepaper** - Atomic swap implementation details
- **MixBytes Blog** - Technical analysis and insights
- **1inch API Documentation** - Integration guides

---

## Conclusion

This MixBytes article provides **comprehensive technical details** that significantly enhance our understanding of the 1inch Fusion+ implementation:

### **✅ New Technical Knowledge:**

1. **Gas optimization techniques** for escrow deployment
2. **Immutables parameter pattern** for validation
3. **Merkle tree implementation** for partial fills
4. **Recovery mechanism details**
5. **Production-ready architecture patterns**

### **✅ Implementation Impact:**

- **Better gas/cycle optimization** in our ICP canisters
- **More robust parameter validation**
- **Advanced partial fills capability**
- **Production-ready recovery systems**

### **✅ Key Links for Reference:**

- **Core repositories** for implementation patterns
- **Technical standards** for best practices
- **Documentation** for integration guidance

**This article significantly improves our technical foundation for the ICP implementation and provides all the necessary links for deep technical research!** 🎯

---

_Reference: [MixBytes - "Modern DEXes, how they're made: 1inch Limit Order Protocol, Fusion & Fusion+"](https://mixbytes.io/blog/modern-dex-es-how-they-re-made-1inch-limit-order-protocols)_
