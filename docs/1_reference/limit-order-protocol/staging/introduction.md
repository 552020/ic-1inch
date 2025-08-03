# 1inch Limit Order Protocol

## Overview

The Limit Order Protocol is a key component of the 1inch ecosystem, offering extreme flexibility and high gas efficiency through its advanced features.

## Key Features

The protocol achieves its goals through the following features:

### Basic Features

- **Asset Receiver Selection**: Choose who receives the assets from an order
- **Fill Control**: Allow or disallow partial and multiple fills
- **Conditional Execution**: Define conditions that must be met before execution (e.g., stop-loss, take-profit orders)
- **Custom Interactions**: Specify arbitrary maker's code to execute before and after order filling
- **Flexible Approval**: Choose approval scheme for token spend (approve, permit, permit2)
- **WETH Unwrapping**: Request WETH to be unwrapped to ETH either before (to sell ETH) or after the swap (to receive ETH)
- **Private Orders**: Make an order private by specifying the only allowed taker's address
- **Expiration Control**: Set the order's expiration date
- **Order Management**: Assign a nonce or epoch to the order for easy cancellation later

### Advanced Features

- **Proxy Support**: Define a proxy to handle transfers of assets that are not compliant with IERC20, allowing the swapping of non-ERC20 tokens, such as ERC721 or ERC1155
- **Dynamic Exchange Rates**: Define functions to calculate, on-chain, the exchange rate for maker and taker assets. These functions can be used to implement:
  - Dutch auctions (where the rate decreases over time)
  - Range orders (where the rate depends on the volume already filled)
  - And other advanced order types

## RFQ Orders

**Note**: Separate RFQ orders are deprecated in v4. To create the most gas efficient order, use a basic order without extensions.

## Supported Tokens

- **ERC-20**: Standard fungible tokens
- **ERC-721**: Non-fungible tokens
- **ERC-1155**: Semi-fungible tokens
- **Other Standards**: Other token standards could be supported via external extension

## Contract Functions

### Constructor

```solidity
constructor(contract IWETH _weth) public
```

### Core Functions

#### `DOMAIN_SEPARATOR()`

```solidity
function DOMAIN_SEPARATOR() external view returns (bytes32)
```

Returns the domain separator for the current chain (EIP-712)

#### `pause()`

```solidity
function pause() external
```

Pauses all the trading functionality in the contract.

#### `unpause()`

```solidity
function unpause() external
```

Unpauses all the trading functionality in the contract.

---

**Previous**: [IUniswapV3SwapCallback](../aggregation-protocol/interfaces/IUniswapV3SwapCallback.md)  
**Next**: [OrderLib](./orderlib.md)

---

_© 2025 1inch Limited_  
[Privacy Policy](https://1inch.io/privacy-policy/) | [Terms of Service](https://1inch.io/terms-of-service/) | [Commercial API Terms of Use](https://1inch.io/commercial-api-terms/)
