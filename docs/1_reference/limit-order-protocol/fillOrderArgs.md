# fillOrderArgs Function

## Overview

`fillOrderArgs` is a key function in the 1inch Limit Order Protocol (LOP) that allows takers to fill orders with additional arguments for custom behavior.

## Function Signature

```solidity
function fillOrderArgs(
    IOrderMixin.Order calldata order,
    bytes32 r,
    bytes32 vs,
    uint256 amount,
    TakerTraits takerTraits,
    bytes calldata args
) external payable returns(uint256 makingAmount, uint256 takingAmount, bytes32 orderHash);
```

## Parameters

### `order` (IOrderMixin.Order)

**Type Explanation:**

- `IOrderMixin.Order` is a custom struct type defined in the Limit Order Protocol
- `calldata` is a Solidity data location keyword that specifies the data is stored in the transaction's calldata (read-only, gas-efficient)
- This means the order struct is passed as a read-only reference from the transaction data: `calldata` is cheaper than `memory` for large structs

**Struct Definition:**
The `IOrderMixin.Order` struct contains the following fields:

```solidity
struct Order {
    uint256 salt;
    Address maker;
    Address receiver;
    Address makerAsset;
    Address takerAsset;
    uint256 makingAmount;
    uint256 takingAmount;
    MakerTraits makerTraits;
}
```

**Field Types:**

- `salt` - `uint256` - Unique identifier for the order
- `maker` - `Address` - Address of the order creator (custom type: `type Address is uint256`)
- `receiver` - `Address` - Address that receives the filled order (can be different from maker)
- `makerAsset` - `Address` - Token address the maker is selling
- `takerAsset` - `Address` - Token address the maker is buying
- `makingAmount` - `uint256` - Amount the maker is selling
- `takingAmount` - `uint256` - Amount the maker is buying
- `makerTraits` - `MakerTraits` - Configuration flags for the maker (custom type: `type MakerTraits is uint256`)

**Custom Types:**

**`Address`** (`type Address is uint256`):

- Custom type from `@1inch/solidity-utils`
- Encodes an address as `uint256` with potential flags in high bits
- Uses only the lower 160 bits for the actual address
- High bits can store additional flags or data

**`MakerTraits`** (`type MakerTraits is uint256`):

- Custom type that packs multiple maker preferences into a single `uint256`
- **High bits (247-255)**: Boolean flags for order behavior
  - `NO_PARTIAL_FILLS_FLAG` (255) - Order doesn't allow partial fills
  - `ALLOW_MULTIPLE_FILLS_FLAG` (254) - Order permits multiple fills
  - `PRE_INTERACTION_CALL_FLAG` (252) - Requires pre-interaction call
  - `POST_INTERACTION_CALL_FLAG` (251) - Requires post-interaction call
  - `NEED_CHECK_EPOCH_MANAGER_FLAG` (250) - Requires epoch manager check
  - `HAS_EXTENSION_FLAG` (249) - Order has extension data
  - `USE_PERMIT2_FLAG` (248) - Uses permit2 for approvals
  - `UNWRAP_WETH_FLAG` (247) - Requires WETH unwrapping
- **Low bits (0-199)**: Packed data
  - `uint80` (0-79): Last 10 bytes of allowed sender address
  - `uint40` (80-119): Expiration timestamp
  - `uint40` (120-159): Nonce or epoch
  - `uint40` (160-199): Series

**Important Distinction:**
The `Order` struct only contains **maker-related data**. The following fields are **NOT** part of the Order struct but are passed as separate function parameters to `fillOrderArgs`:

- `taker` - Determined by who calls the function
- `takerTraits` - Passed as a separate parameter
- `extension` - Part of the `args` parameter
- `interaction` - Part of the `args` parameter

### `r` (bytes32)

R component of the ECDSA signature. Part of the signature proving the maker authorized this order.

### `vs` (bytes32)

VS component of the ECDSA signature (combines `v` and `s`). Part of the signature proving the maker authorized this order.

### `amount` (uint256)

The amount the taker wants to fill from this order.

### `takerTraits` (TakerTraits)

Configuration for the taker including:

- Threshold values for minimum/maximum amounts
- Flags for permit handling
- Extension and interaction data

### `args` (bytes)

**Purpose**: Encoded arguments for taker-specific behavior, parsed based on `TakerTraits` flags.

**Structure**: `args` is a packed bytes array containing up to 3 components (in order):

1. **Target Address** (20 bytes, optional)

   - **When**: Only if `TakerTraits.argsHasTarget()` is true (bit 251)
   - **Purpose**: Address where maker's funds should be transferred (instead of default `msg.sender`)
   - **Usage**: For cross-chain swaps, this is typically the escrow address

2. **Extension Data** (variable length, optional)

   - **When**: Only if `TakerTraits.argsExtensionLength()` > 0 (bits 224-247)
   - **Purpose**: Additional data for order extension logic
   - **Usage**: Custom order behavior, permit data, etc.

3. **Interaction Data** (variable length, optional)
   - **When**: Only if `TakerTraits.argsInteractionLength()` > 0 (bits 200-223)
   - **Purpose**: Data for taker interaction callbacks
   - **Usage**: Custom taker-specific logic

**Code Verification**:

- **Parsing**: `_parseArgs()` in `OrderMixin.sol` lines 450-475
- **Structure**: `abi.encodePacked(targetBytes, extension, interaction)` in `CrossChainTestLib.sol` line 193
- **Flags**: `TakerTraitsLib.sol` defines the parsing flags (bits 251, 224-247, 200-223)

**Cross-Chain Relevance**: ✅ **CRITICAL** - The target address is used to specify where maker's funds go (typically the escrow address for cross-chain swaps).

## Returns

- `makingAmount` - Actual amount transferred from maker to taker
- `takingAmount` - Actual amount transferred from taker to maker
- `orderHash` - Hash of the filled order

## Signature Creation Process

The signature (`r`, `vs`) is created using EIP-712:

1. **Build the order** using `buildOrder(orderData, {...})`
2. **Sign the order** using `signOrder(order, chainId, lopv4Address, orderSigner)`
3. **Extract signature components** using `ethers.Signature.from()` to get `r` and `vs`

### Example from test code:

```javascript
const { r, yParityAndS: vs } = ethers.Signature.from(
  await signOrder(order, chainId, await lopv4.getAddress(), orderSigner)
);
```

## File Locations

### Interface Definition

- **Path**: `cross-chain-swap-fork/lib/limit-order-protocol/contracts/interfaces/IOrderMixin.sol`
- **Lines**: 148-170

### Implementation

- **Path**: `/limit-order-protocol/contracts/OrderMixin.sol`
- **Lines**: 133-145

### Test Examples

- **Path**: `/limit-order-settlement/test/helpers/fusionUtils.js`
- **Lines**: 70-99 (signature creation)
- **Lines**: 100-105 (function call encoding)

## Usage in Cross-Chain Context

In the cross-chain swap context, `fillOrderArgs` is called by resolvers to:

1. Execute the order on the source chain
2. Trigger the `_postInteraction()` callback
3. Create the source escrow via `EscrowFactory._postInteraction()`
4. Emit the `SrcEscrowCreated` event

The resolver then listens for the `SrcEscrowCreated` event to extract the `deployedAt` timestamp needed for creating the destination escrow.

---

## 🔍 **Field Analysis: Resolver-Settable vs Requires Maker Action**

When testing `fillOrderArgs` for cross-chain swaps, it's crucial to understand which fields can be set by the resolver vs which require specific actions from the maker.

### **✅ Resolver-Settable Fields**

These fields can be set by the resolver without requiring special actions from others:

#### **`makingAmount` & `takingAmount`** (uint256)

- **Type**: Simple integers
- **Resolver-Settable**: ✅ **YES** - Resolver can set any valid amounts
- **Testing**: Use reasonable test values (e.g., `1000000000000000000` for 1 token)
- **Production**: Must match maker's intended amounts

#### **`salt`** (uint256)

- **Type**: Unique identifier
- **Resolver-Settable**: ✅ **YES** - Resolver can generate any unique value
- **Testing**: Use timestamp or random value
- **Production**: Must be unique per order

#### **`makerTraits`** (MakerTraits)

- **Type**: Packed uint256 with flags
- **Resolver-Settable**: ✅ **YES** - Resolver can set appropriate flags
- **Testing**: Use minimal flags (e.g., `POST_INTERACTION_CALL_FLAG` for escrow creation)
- **Production**: Must match maker's preferences

### **✅ Resolver-Settable Fields (Continued)**

#### **`receiver`** (Address)

- **Type**: Custom uint256 wrapper
- **Resolver-Settable**: ✅ **YES** - Resolver can set to EscrowFactory address
- **Reason**: At order submission time, the escrow doesn't exist yet, so `receiver` must be the EscrowFactory address
- **Testing**: Use the deployed EscrowFactory address on Base Sepolia
- **Production**: Always the EscrowFactory address for cross-chain swaps
- **Code Verification**:
  - In `OrderLib.sol` line 72-75: `getReceiver()` returns `order.receiver.get()` if not zero, otherwise `order.maker.get()`
  - In example scripts: `receiver: address(0)` is used, which means it defaults to the maker address
  - **SDK Verification**: In `cross-chain-sdk/src/sdk/types.ts` line 39: `receiver?: string // by default: walletAddress (makerAddress)`
  - **SDK Implementation**: In `cross-chain-sdk/src/sdk/sdk.ts` lines 138-139: `receiver: params.receiver ? new Address(params.receiver) : undefined`
  - **Cross-Chain Order**: In `cross-chain-sdk/src/cross-chain-order/cross-chain-order.ts` line 124: `customReceiver: orderInfo.receiver`
  - **For cross-chain swaps**: The receiver should be the EscrowFactory address (not the maker address)

#### **`makerAsset`** (Address)

- **Type**: Custom uint256 wrapper
- **Resolver-Settable**: ✅ **YES** - Resolver can use any valid ERC-20 address
- **Testing**: Use testnet tokens (DAI, WETH on Base Sepolia)
- **Production**: Must be the actual token the maker is selling

#### **`takerAsset`** (Address)

- **Type**: Custom uint256 wrapper
- **Resolver-Settable**: ✅ **YES** - Resolver can use symbolic addresses
- **Testing**: Use symbolic address for ICP (see `fillOrderArgs_makerAsset.md`)
- **Production**: Must represent the destination asset

### **✅ All Fields Are Resolver-Settable**

All Order fields can be set by the resolver without requiring special actions from others:

#### **`maker`** (Address)

- **Type**: Custom uint256 wrapper
- **Resolver-Settable**: ✅ **YES** - Just a public address, resolver can set to any valid address
- **Note**: The signature must match the `maker` address, but resolver can choose which address to use
- **Testing**: Use any valid address, ensure signature matches

---

## 🧪 **Testing Strategy**

### **For Development/Testing:**

1. **Use a known maker address** that has signed the order
2. **Set reasonable amounts** for `makingAmount` and `takingAmount`
3. **Use testnet tokens** for `makerAsset` (DAI, WETH on Base Sepolia)
4. **Use symbolic addresses** for `takerAsset` (ICP representation)
5. **Set appropriate flags** in `makerTraits` for escrow creation
6. **Generate unique salt** for each test order

### **Example Test Order:**

```javascript
const testOrder = {
  salt: ethers.getBigInt(Date.now()), // Unique identifier
  maker: "0xMAKER_ADDRESS", // Must be signed by this address
  receiver: "0xESCROW_FACTORY_ADDRESS", // TBD: factory or escrow?
  makerAsset: "0xDAI_ADDRESS_ON_BASE_SEPOLIA", // Test token
  takerAsset: "0x00000000000000000000000000000000ICP1C4N", // Symbolic ICP
  makingAmount: ethers.parseEther("1"), // 1 DAI
  takingAmount: ethers.parseUnits("1", 8), // 1 ICP (8 decimals)
  makerTraits: POST_INTERACTION_CALL_FLAG, // Enable escrow creation
};
```

---

## 🔍 **Open Questions**

1. **1inch API**: Check `https://api.1inch.dev/fusion-plus/orders/v1.0/order/escrow` for guidance
2. **MakerTraits flags**: Which specific flags are required for cross-chain escrow creation?

---

## 📋 **Next Steps**

1. **Test the 1inch API endpoint** to get the EscrowFactory address
2. **Analyze `makerTraits` flags** to understand which are required for cross-chain swaps

Great question — you're hitting the heart of the **cross-chain problem with non-EVM chains** like ICP.

Let’s break it down clearly.

---

## ❓ Your Context

- You're calling `fillOrderArgs()` on **Base Sepolia (EVM)**.
- This creates the **source escrow**.
- You want to link it to a **destination escrow on ICP**.
- But ICP **is not EVM** — it has no token _address_, no ERC-20, and no shared state.

---

## ✅ What You _Must_ Provide in the EVM Order

In the `fillOrderArgs()` call, the **Fusion+ resolver** needs to:

- Fill a signed order that includes:

  - `makerAsset` (EVM token you're giving away)
  - `takerAsset` (EVM representation of the token you're receiving **from ICP**)

> That `takerAsset` is typically a **placeholder** for a bridged token or synthetic representation.

---

## 🔧 Solutions for Testing with ICP

### Option 1: **Use a dummy ERC-20 on Base Sepolia**

Use a fake token address like `0xMockICP` in `takerAsset`, and manually interpret that in your ICP canister.

In this case:

- You **mint mock ICP** to the user on ICP
- Even though the EVM side says "I'm getting `token 0x123`", your relayer interprets that as "give the user X ICP on the other side"

> Think of `takerAsset` on EVM as a **symbol**, not an actual asset on ICP.

---

### Option 2: **Wrap real assets using a bridge (for prod)**

Later, you can:

- Use ICPSwap's `osUSDC`, `osICP`, etc.
- Issue a wrapped version on EVM (if trust assumptions allow)
- Or create synthetic tokens (e.g., `eICP`) with mint/burn control

---

## 🧪 For Hackathon / Testnet Flow

You can safely do:

1. Use DAI or WETH on Sepolia as `makerAsset`
2. Use a mock ERC20 address as `takerAsset` (e.g. `"0x00000000000000000000000000000000ICP1C4N"` — doesn’t have to exist)
3. Use `fillOrderArgs()` with this order
4. Let the relayer listen for `SrcEscrowCreated`
5. The relayer then:

   - Triggers `createDstEscrow()` on ICP
   - Mints real ICP on the Internet Computer

6. On `withdraw()`, release DAI to the resolver

---

## 📦 Example Order for Testing

```json
{
  "makerAsset": "0xDAI_ADDRESS_ON_BASE_SEPOLIA",
  "takerAsset": "0x00000000000000000000000000000000ICP1C4N", // dummy
  "makingAmount": "1000000000000000000",  // 1 DAI
  "takingAmount": "100000000",           // 1 ICP (as uint8 precision)
  ...
}
```

---

## ✅ **Summary**

**ALL 7 Order fields are Resolver-Settable** for cross-chain Fusion+ swaps between Ethereum and ICP:

### **✅ Resolver-Settable Fields (7/7):**

- `makingAmount` & `takingAmount` - Simple integers
- `salt` - Unique identifier
- `makerTraits` - Packed flags
- `receiver` - **EscrowFactory address** (confirmed via code analysis)
- `makerAsset` - Any valid ERC-20 address
- `takerAsset` - Symbolic addresses for ICP
- `maker` - **Just a public address** (resolver can set to any valid address)

### **🎯 Key Insight:**

The resolver can set all fields to any reasonable values. The only requirement is that the **signature matches the `maker` address** that the resolver chooses to use.

This makes testing extremely straightforward - you can construct complete test orders with any reasonable values!
i
