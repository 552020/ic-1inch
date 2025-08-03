# OrderMixin

The OrderMixin provides the core functionality for processing and executing limit orders in the 1inch Limit Order Protocol.

## Functions

### Constructor

```solidity
constructor(contract IWETH weth) internal
```

### Core Functions

#### `bitInvalidatorForOrder(maker, slot)`

```solidity
function bitInvalidatorForOrder(address maker, uint256 slot) external view returns (uint256)
```

See {IOrderMixin-bitInvalidatorForOrder}.

#### `remainingInvalidatorForOrder(maker, orderHash)`

```solidity
function remainingInvalidatorForOrder(address maker, bytes32 orderHash) external view returns (uint256)
```

See {IOrderMixin-remainingInvalidatorForOrder}.

#### `rawRemainingInvalidatorForOrder(maker, orderHash)`

```solidity
function rawRemainingInvalidatorForOrder(address maker, bytes32 orderHash) external view returns (uint256)
```

See {IOrderMixin-rawRemainingInvalidatorForOrder}.

#### `simulate(target, data)`

```solidity
function simulate(address target, bytes data) external
```

See {IOrderMixin-simulate}.

#### `cancelOrder(makerTraits, orderHash)`

```solidity
function cancelOrder(MakerTraits makerTraits, bytes32 orderHash) public
```

See {IOrderMixin-cancelOrder}.

#### `cancelOrders(makerTraits, orderHashes)`

```solidity
function cancelOrders(MakerTraits[] makerTraits, bytes32[] orderHashes) external
```

See {IOrderMixin-cancelOrders}.

#### `bitsInvalidateForOrder(makerTraits, additionalMask)`

```solidity
function bitsInvalidateForOrder(MakerTraits makerTraits, uint256 additionalMask) external
```

See {IOrderMixin-bitsInvalidateForOrder}.

#### `hashOrder(order)`

```solidity
function hashOrder(struct IOrderMixin.Order order) external view returns (bytes32)
```

See {IOrderMixin-hashOrder}.

#### `checkPredicate(predicate)`

```solidity
function checkPredicate(bytes predicate) public view returns (bool)
```

See {IOrderMixin-checkPredicate}.

#### `fillOrder(order, r, vs, amount, takerTraits)`

```solidity
function fillOrder(struct IOrderMixin.Order order, bytes32 r, bytes32 vs, uint256 amount, TakerTraits takerTraits) external payable returns (uint256, uint256, bytes32)
```

See {IOrderMixin-fillOrder}.

#### `fillOrderArgs(order, r, vs, amount, takerTraits, args)`

```solidity
function fillOrderArgs(struct IOrderMixin.Order order, bytes32 r, bytes32 vs, uint256 amount, TakerTraits takerTraits, bytes args) external payable returns (uint256, uint256, bytes32)
```

See {IOrderMixin-fillOrderArgs}.

#### `fillContractOrder(order, signature, amount, takerTraits)`

```solidity
function fillContractOrder(struct IOrderMixin.Order order, bytes signature, uint256 amount, TakerTraits takerTraits) external returns (uint256, uint256, bytes32)
```

See {IOrderMixin-fillContractOrder}.

#### `fillContractOrderArgs(order, signature, amount, takerTraits, args)`

```solidity
function fillContractOrderArgs(struct IOrderMixin.Order order, bytes signature, uint256 amount, TakerTraits takerTraits, bytes args) external returns (uint256, uint256, bytes32)
```

See {IOrderMixin-fillContractOrderArgs}.

---

**Previous**: [OrderLib](./orderlib.md)  
**Next**: [ApprovalPreInteraction](./extensions/approvalpreinteraction.md)

---

_© 2025 1inch Limited_  
[Privacy Policy](https://1inch.io/privacy-policy/) | [Terms of Service](https://1inch.io/terms-of-service/) | [Commercial API Terms of Use](https://1inch.io/commercial-api-terms/)
