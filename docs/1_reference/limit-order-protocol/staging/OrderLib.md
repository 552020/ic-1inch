# OrderLib

The library provides common functionality for processing and manipulating limit orders. It provides functionality to calculate and verify order hashes, calculate trade amounts, and validate extension data associated with orders. The library also contains helper methods to get the receiver of an order and call getter functions.

## Functions

### `hash(order, domainSeparator)`

```solidity
function hash(struct IOrderMixin.Order order, bytes32 domainSeparator) internal pure returns (bytes32 result)
```

Calculates the hash of an order.

**Parameters:**

| Name              | Type                       | Description                                              |
| ----------------- | -------------------------- | -------------------------------------------------------- |
| `order`           | `struct IOrderMixin.Order` | The order to be hashed.                                  |
| `domainSeparator` | `bytes32`                  | The domain separator to be used for the EIP-712 hashing. |

**Return Values:**

| Name     | Type      | Description                           |
| -------- | --------- | ------------------------------------- |
| `result` | `bytes32` | The keccak256 hash of the order data. |

### `getReceiver(order)`

```solidity
function getReceiver(struct IOrderMixin.Order order) internal pure returns (address)
```

Returns the receiver address for an order.

**Parameters:**

| Name    | Type                       | Description |
| ------- | -------------------------- | ----------- |
| `order` | `struct IOrderMixin.Order` | The order.  |

**Return Values:**

| Name  | Type      | Description                                                                                                  |
| ----- | --------- | ------------------------------------------------------------------------------------------------------------ |
| `[0]` | `address` | The address of the receiver, either explicitly defined in the order or the maker's address if not specified. |

### `calculateMakingAmount(order, extension, requestedTakingAmount, remainingMakingAmount, orderHash)`

```solidity
function calculateMakingAmount(struct IOrderMixin.Order order, bytes extension, uint256 requestedTakingAmount, uint256 remainingMakingAmount, bytes32 orderHash) internal view returns (uint256)
```

Calculates the making amount based on the requested taking amount.

If getter is specified in the extension data, the getter is called to calculate the making amount, otherwise the making amount is calculated linearly.

**Parameters:**

| Name                    | Type                       | Description                                     |
| ----------------------- | -------------------------- | ----------------------------------------------- |
| `order`                 | `struct IOrderMixin.Order` | The order.                                      |
| `extension`             | `bytes`                    | The extension data associated with the order.   |
| `requestedTakingAmount` | `uint256`                  | The amount the taker wants to take.             |
| `remainingMakingAmount` | `uint256`                  | The remaining amount of the asset left to fill. |
| `orderHash`             | `bytes32`                  | The hash of the order.                          |

**Return Values:**

| Name  | Type      | Description                                 |
| ----- | --------- | ------------------------------------------- |
| `[0]` | `uint256` | The amount of the asset the maker receives. |

### `calculateTakingAmount(order, extension, requestedMakingAmount, remainingMakingAmount, orderHash)`

```solidity
function calculateTakingAmount(struct IOrderMixin.Order order, bytes extension, uint256 requestedMakingAmount, uint256 remainingMakingAmount, bytes32 orderHash) internal view returns (uint256)
```

Calculates the taking amount based on the requested making amount.

If getter is specified in the extension data, the getter is called to calculate the taking amount, otherwise the taking amount is calculated linearly.

**Parameters:**

| Name                    | Type                       | Description                                          |
| ----------------------- | -------------------------- | ---------------------------------------------------- |
| `order`                 | `struct IOrderMixin.Order` | The order.                                           |
| `extension`             | `bytes`                    | The extension data associated with the order.        |
| `requestedMakingAmount` | `uint256`                  | The amount the maker wants to receive.               |
| `remainingMakingAmount` | `uint256`                  | The remaining amount of the asset left to be filled. |
| `orderHash`             | `bytes32`                  | The hash of the order.                               |

**Return Values:**

| Name  | Type      | Description                              |
| ----- | --------- | ---------------------------------------- |
| `[0]` | `uint256` | The amount of the asset the taker takes. |

### `isValidExtension(order, extension)`

```solidity
function isValidExtension(struct IOrderMixin.Order order, bytes extension) internal pure returns (bool, bytes4)
```

Validates the extension associated with an order.

**Parameters:**

| Name        | Type                       | Description                              |
| ----------- | -------------------------- | ---------------------------------------- |
| `order`     | `struct IOrderMixin.Order` | The order to validate against.           |
| `extension` | `bytes`                    | The extension associated with the order. |

**Return Values:**

| Name  | Type     | Description                                                           |
| ----- | -------- | --------------------------------------------------------------------- |
| `[0]` | `bool`   | True if the extension is valid, false otherwise.                      |
| `[1]` | `bytes4` | The error selector if the extension is invalid, 0x00000000 otherwise. |

## Errors

### `MissingOrderExtension()`

```solidity
error MissingOrderExtension()
```

Error to be thrown when the extension data of an order is missing.

### `UnexpectedOrderExtension()`

```solidity
error UnexpectedOrderExtension()
```

Error to be thrown when the order has an unexpected extension.

### `InvalidExtensionHash()`

```solidity
error InvalidExtensionHash()
```

Error to be thrown when the order extension hash is invalid.

---

**Previous**: [Introduction](./introduction.md)  
**Next**: [OrderMixin](./ordermixin.md)

---

_© 2025 1inch Limited_  
[Privacy Policy](https://1inch.io/privacy-policy/) | [Terms of Service](https://1inch.io/terms-of-service/) | [Commercial API Terms of Use](https://1inch.io/commercial-api-terms/)
