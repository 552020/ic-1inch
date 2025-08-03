# Modern DEXes, how they're made: 1inch Limit Order Protocol, Fusion & Fusion+

**Author(s):** Sergey Boogerwooger  
**Security researcher(s) at MixBytes**

## Intro

This article continues our series about modern DEXes. Today, we'll concentrate on another DEX ecosystem by 1inch. We will review three 1inch protocols because all of them are built upon one another: the Limit Order Protocol, and the Fusion/Fusion+ protocols. Previously, we have reviewed a range of "direct-swap" protocols like Uniswap V4, Balancer V3, Fluid DEX, and are now focusing on "order-based" projects. Our previous article discussed CoW Swap.

The 1inch Limit Order Protocol (LOP) continues in this series by using an "order-based" approach. A limit order is a swap order that not only includes the input and output amounts of tokens but also additional conditions such as the order deadline, target price limits, or the data for external calls. This allows for implementing almost any programmable logic determining whether an order can be filled by a resolver. These orders are extremely useful for situations like "execute the trade if the market reaches this price within the next 10 days" or "start a Dutch auction for my order," among many others. Special "modes" and order types in LOP can also be used in other protocols to implement advanced DeFi mechanics, such as auctions and cross-chain swaps.

Additionally, the "order-based" approach for swaps allows for minimizing fees and provides an excellent UX, enabling users to bypass fees in native tokens. Furthermore, "order-based" swap solutions, with their active use of off-chain signatures and processing of orders by off-chain resolvers while containing only "settling" on-chain logic, are extremely useful for cross-chain swaps. In these cases, we can include a check of the external chain's "proof-of-provided-funds" in the order validation logic, thus maintaining a similar infrastructure for single-chain and cross-chain swaps.

Okay, enough of the standard marketing talk, let's proceed to the code!

## Limit Order Protocol

### Core

The main repository for this protocol is [here](https://github.com/1inch/limit-order-protocol), with the core functionality residing in the `OrderMixin.sol` contract.

The most important aspect of the Limit Order Protocol (LOP) is the validation of an order. A user (maker) creates an order, sets limit conditions, signs the order, and publishes it. A taker accepts this order, verifies it, and fulfills it by providing the required amounts of assets. The taker cannot fill the order on-chain unless all the maker's requirements are met, which is verified on-chain during the order fulfillment. When creating the order, the maker can:

- Set the order's expiration date.
- Specify a receiving address (if different from the maker's address).
- Designate a specific taker's address (for private orders).
- Select an approval method for token allowance (approve, permit, permit2).
- Allow or disallow partial and multiple fills.
- Add ETH/WETH wrap/unwrap before or after the swap.
- Define conditions that must be met before order execution (e.g., stop-loss, take-profit orders).
- Specify arbitrary interactions with the maker's code before and after order filling.
- Set a "nonce or epoch" for the order, allowing the mass invalidation of orders by epoch or nonce. This is required for dApps that want to invalidate sets of orders simultaneously by changing epoch and to separate these dApps using series.
- Define a custom proxy for transferring assets (like ERC-721 or ERC-1155) or performing swaps with multiple assets.
- Define functions to dynamically calculate the exchange rate for maker and taker assets on-chain, allowing a Dutch auction or range orders (depending on the volume already filled).
- any other programmable logic

This functionality allows not only the direct use of the LOP for trades but also its use as a settlement layer for other protocols, particularly those that process orders off-chain before settlement.

The main entity of the protocol is the `Order` struct. Order identifiers (`bytes32 orderHash`), used in most order-related functions, are deterministically derived from the order contents by hashing its serialized in-memory representation. The key fields are, of course, taker/maker assets and amounts. There is also a special `salt` parameter used to maintain the integrity of additional limit order data, which we will refer to later. The additional conditions for an order are encoded in the `MakerTraits` struct, which is described in `MakerTraitsLib.sol` [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/libraries/MakerTraitsLib.sol).

Additional data for all these functionalities resides in the order's extension - a separate pack of data accompanying the order. It's not included in the "base" order struct but is cryptographically connected with the corresponding order via the `salt` parameter (which includes a hash of the extension contents). The extension is processed with the order and contains different types of additional data, like arguments for additional external calls.

There is also a `TakerTraits` pack of order parameters, which is added by a resolver during the process of filling an order. We will review it later. Now, let's proceed to the process of filling an order, as it's a key aspect of the protocol, and, by understanding it, we can easily review other parts of the protocol that lead to filling the orders.

### Settling the Order

There are four functions for settling the order (described [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/OrderMixin.sol#L1)), which operate similarly but differ in passed off-chain signatures and additional arguments passed to a taker.

- `fillOrder()` and `fillOrderArgs()` check `(v, r, s)` off-chain signature set by the user. `fillOrder()` passes empty args (extension) to the taker, and `fillOrderArgs()` sets additional arguments.

- `fillContractOrder()` and `fillContractOrderArgs()` work similarly with taker args (extension) but check `bytes calldata signature`, a signature made by a contract according to EIP-1271 when orders are created by a smart contract.

These two groups of functions then go to `_fillOrder()` or to `_fillOrderContract()`, which perform "first-level" checks, checking for remaining amounts (if the order allows for partial fill) and attempting to apply maker permits (for user-signed orders).

Both groups of functions conclude with the call to the main `_fill()` function - our main target.

First, a series of different validations occur. We start with a check of the validity of the extension by using `isValidExtension()`, a function that checks if extension contents are hashed to the order's `salt` parameter, protecting the maker from possible taker's modification of extension parameters.

Next is the pack of checks related to order sender and expiration parameters.

Following this is the predicate check - an external call defined by the maker that determines if the order is allowed to be fulfilled. It uses the `_staticCallForUint()` function to perform a static(!) call to the target, which must return "1" to continue processing the order. This functionality allows makers to add almost any "limit" conditions to their orders and configure limit order conditions as they see fit.

The next part is processing maker and taker amounts. Similar to DEXes, there are two branches: one where the taker uses an "exact making amount" (analogous to `swapExactIn()` in Uniswap) and another "exact taking amount" (analogous to `swapExactOut()`). Both branches calculate target amounts for maker and taker ([here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/OrderMixin.sol#L1) and [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/OrderMixin.sol#L1)) and check thresholds for `TakingAmountTooHigh` or `MakingAmountTooHigh`. The choice of which branch to pursue in `TakerTraits` is the taker's responsibility, allowing them to choose how to fill the order.

Finally, making/taking amounts are checked to ensure they are non-zero and that they correspond to the full order amount if the order isn't partially fillable.

Next, invalidation status is checked using the `BitInvalidator`. This feature allows a maker to set an invalidation bitmap for the given maker's address, enabling them to invalidate other orders made by the same maker. For instance, one filled order from a pack can invalidate all others, allowing the implementation of strategies like "fill one order from a pack." `BitInvalidator` can use nonce identifiers to group orders or employ the maker's epoch concept, permitting trading services to publish multiple orders in one epoch, then move the epoch, invalidating all "old" orders, and continuing in the next epoch. Further information about this type of invalidation can be found [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/BitInvalidator.sol).

Next, invalidation of an order based on remaining maker's amount occurs, which is straightforward.

Then comes pre-interaction (if set in `MakerTraits`). Here, data related to this call is loaded from the order's extension, and the listener address is called (if provided) or the maker's address otherwise. We will examine different extension interactions, but for this part, we can review the `preInteraction()` function in `extensions/ApprovalPreInteraction.sol`, which simply provides the necessary approve.

Next, the part transferring assets from maker to taker. This involves potentially wrapping/unwrapping WETH if needed, and using two methods for transfers:

- Based on Permit2 (read more [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/Permit2Adapter.sol))
- Based on `transferFrom`, but with an optional suffix that can be added to `transferFrom` parameters (if set in the extension), implemented [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/OrderMixin.sol#L1). This suffix is used for non-standard transfers, such as ERC-721 transfers, which require adding the `tokenId` to the parameters.

Then goes `TakerInteraction`, which allows any actions that the taker wishes to add to the settlement after receiving the assets. Its interface is declared [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/interfaces/ITakerInteraction.sol).

Following that, transferring assets from taker to maker (or the receiver set by the maker) occurs. This includes handling WETH wrap/unwrap operations and using two types of transfer: based on Permit2 and `transferFrom` with suffix (similar to previous transfers from maker to taker).

Finally, post-interaction follows the order filling, working similarly to pre-interaction described above.

Another form of "filling" an order is its cancellation, implemented [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/OrderMixin.sol#L1). In the case of a `BitInvalidator`, it invalidates the range of bits, rendering all orders with the same epoch or nonce invalidated. In other cases, it marks the order as fully filled, invalidating it.

The order is filled, all conditions are checked, and tokens are transferred. Now, it's time to look at the process of creating orders and the protocols that use LOP.

### Creation of Orders

Let's create different order types. This part is well explained [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/OrderMixin.sol#L1). As mentioned, basic limit order functionality, such as defining exact amounts of taker/maker assets and deadlines, can be achieved quite easily without additional extensions. A very good demonstration of creating and filling many types of limit orders is present in the tests; you can start reviewing different cases from [here](https://github.com/1inch/limit-order-protocol/blob/master/test/OrderMixin.test.ts) and continue to explore different cases. We cannot describe all tests, so we will focus on some illustrative examples, but it's beneficial to review many of them to understand how different `MakerTraits` and `TakerTraits` work (don't forget to refer to the docs for a description of each option).

Some good "basic" tests to review include: full filling of two orders [here](https://github.com/1inch/limit-order-protocol/blob/master/test/OrderMixin.test.ts), an example with WETH/ETH [here](https://github.com/1inch/limit-order-protocol/blob/master/test/OrderMixin.test.ts), partial swaps [here](https://github.com/1inch/limit-order-protocol/blob/master/test/OrderMixin.test.ts), packs of tests with permit [here](https://github.com/1inch/limit-order-protocol/blob/master/test/OrderMixin.test.ts), and permit2 [here](https://github.com/1inch/limit-order-protocol/blob/master/test/OrderMixin.test.ts). Another important part to study is the cancellation of orders based on different invalidation logic, which is explored in this branch of tests [here](https://github.com/1inch/limit-order-protocol/blob/master/test/OrderMixin.test.ts).

The most powerful aspect of LOP is, undoubtedly, the use of the extension with external calls data "injected" into the order filling process. Let's describe some features of extensions, with documentation available [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/extensions/README.md). For example, it allows the use of `MakerAssetSuffix` and `TakerAssetSuffix` to "modify" the `transferFrom()` call, enabling limit orders to transfer not only regular ERC20 tokens but also other types of tokens, such as ERC721 (which require adding a `tokenId` in "transfer" functions). An example of such an order is [here](https://github.com/1inch/limit-order-protocol/blob/master/test/OrderMixin.test.ts), where we approve `tokenId=10` in `erc721proxy`, and then add suffixes to the order's extension, facilitating the transfer of ERC721 tokens from maker and taker addresses.

### Fees

There are no protocol fees in the Limit Order Protocol. Why? One of 1inch LOP's primary goals is to process orders with minimal gas consumption, while any additional fees would require extra operations with tokens, making swaps more costly for users and resolvers. In-order fees processing also complicates any programmable logic (like auctions, partial-fill orders, etc.), adding more risks for users and solvers.

This approach does not preclude the implementation of fees, as dApps working with LOP can always use token proxies, additional extension functionalities, and implement any logic they desire.

### Implementation details

LOP has a pause/unpause mechanism, and that's all for on-chain governance directly related to LOP, as there are no rates, fees, and other governance elements. The main function of 1inch governance is the management of resolvers, which is performed using the 1INCH token and Unicorn power derived from the 1INCH token balance, as described [here](https://github.com/1inch/limit-order-protocol/blob/master/contracts/OrderMixin.sol#L1).

An interesting aspect of the implementation is found in `PredicateHelper.sol`, where functions are implemented to perform logical and algebraic comparisons using the results of static calls with arbitrary data. This is necessary for predicates to check, for example, that some oracle price is greater than a certain value (algebraic ">"), or that at least one of multiple static calls returned true (logical "or"). This module is very useful for building programmable limit orders. Another feature with calls is the possibility to simulate some delegate calls by executing them and reverting, returning the result.

It's worth reviewing the existing extensions for LOP as they contain a lot of useful logic. For example, a `DutchAuctionCalculator`, encountered in the Fusion protocol, and `ERC721Proxy`, allowing LOP orders to operate with NFTs, `FeeTaker`, which is needed to add fees to orders, and many others.

## Fusion protocol

Now, after reviewing the Limit Order Protocol, we proceed to the protocols using it for implementing additional functions and extending LOP capability to solve different DeFi problems. The next protocol to review is 1inch Fusion, which provides special order types for LOP, allowing takers to participate in a Dutch auction (the first bid at the continuously decreasing price is accepted). This is described at a high level in [this article](https://1inch.io/blog/1inch-fusion-protocol/).

We have seen the functionality of extensions, allowing setup of programmable conditions for orders, and the Dutch auction is an excellent example of implementing such logic. The Dutch auction itself is a very useful primitive for DeFi, where a minimal amount of interactions is needed, and regular auctions with multiple bids from several participants don't work well. The use of Dutch auctions allows users to receive a fast response for their queries and makes bidders compete without using multiple interactions with the protocol. We have already encountered Dutch auctions in DeFi projects such as Ajna or Orca.

The functionality of creating Fusion limit orders can be found in [this repository](https://github.com/1inch/fusion-sdk), where we can find a `FusionOrder` class, where the field of interest is `auctionDetails`. Its contents are then passed to the constructor of `FusionExtension`. `auctionDetails` contains Dutch auction parameters, encoding a price function depending on the time passed from the start of the auction. In the Fusion protocol, you can not only set the "speed" of price reduction but also encode this function with multiple points, making it a piecewise linear function (and, of course, restricted from below). The encoding of Fusion auction parameters is described [here](https://github.com/1inch/fusion-sdk/blob/master/src/auction.ts), which includes start time, duration, bump rate, and an array of points that add areas where the price is constant. Another important part of the auction parameters is gas-related parameters, including the gas cost at the moment of order start and gas estimation, which are used to cover gas costs for the order filler.

You can view the process of working with Fusion orders using fusion-sdk in [this video](https://www.youtube.com/watch?v=example), showing how Fusion orders functionality can be integrated into any DeFi frontend. Another useful piece of code to review related to Fusion orders is the `DutchAuctionCalculator` extension.

But that's not all; the use of Fusion orders and Dutch auctions opens another extension of 1inch protocols—cross-chain swaps, performed by protocol resolvers. So, let's take a look at the next protocol.

## Fusion+ protocol

The next protocol to review is related to cross-chain swaps and inherits the same "solver-based" architecture, where users place their cross-chain swap orders and resolvers execute them, settling target swap amounts in both chains using LOP Fusion order data from the source chain. The selection of the target resolver for the swap is performed using "Fusion" mode via Dutch auction, as described above. The difference is in the settlement process, which now requires a "proof-of-funds-unlock-in-another-network." How is this achieved?

The key algorithm behind Fusion+ is the atomic swap, a well-known "hashlock"-based protocol to swap assets between two networks, supporting the unlocking of assets by presenting a preimage of a hash. Revealing this preimage allows for unlocking funds in both networks. It works in almost any blockchain, including Bitcoin and other UTXO-based blockchains, where it can be easily implemented. An atomic swap is a trustless and safe algorithm, requiring only two parties without needing to trust each other, and seems to be an ideal solution for DeFi. However, atomic-swap-based projects did not gain popularity because hashlock-based interactions require at least two transactions on both sides of the swap ("commit" and "reveal" for both parties in both networks), storing secret values during the swap process, and including a time window to ensure swap security. So, most users preferred solutions with better UX, requiring a single transaction: custodial or consensus-based bridges.

1inch's Limit Order protocol allows implementation of time- and hash- locks and has additional relayer services and swap resolvers. Combined, these elements enable building an atomic-swap-based protocol with minimal user hassle. The protocol is well-explained in the [whitepaper](https://docs.1inch.io/docs/fusion-plus/whitepaper/), which is highly recommended to read, especially if you're unfamiliar with atomic swaps. The swaps in Fusion+ are performed using orders from the Fusion protocol, but the flow is more complicated than in the Fusion protocol because cross-chain swaps require the resolver to perform "pre-swap" actions: placing "hashlocked" funds in the target network. The same "hashlocked" placement in the source network is executed through the post-interaction in a Fusion order. The relayer service in this scheme checks the validity of hashlocks on both sides and guarantees that the needed secret value will be revealed.

Let's go through the steps of the swap (taken from the whitepaper in a shortened and simplified form) using an example to clarify how it works and where the connections with LOP and Fusion are. A maker swaps 100 1INCH tokens in Ethereum for 200 BNB tokens on Binance Chain:

### Phase 1: (maker, relayer)

1. The maker creates, signs, and sends a Fusion limit order with `hash(secret)` to the relayer service with permission to spend 1INCH in Ethereum.
2. The relayer shares this order with all resolvers, initiating the Dutch auction inside the 1inch service.

### Phase 2: (relayer, taker)

3. The taker (resolver who won the auction) transfers 100 1INCH tokens for themselves from the user into the escrow contract in Ethereum. This includes atomic swap parameters: `hash(secret)`, timelock value, and taker/maker addresses (the maker address is required to return funds in case of problems).
4. The taker deposits 200 BNB tokens for the maker in the Binance Chain into the escrow contract, providing the same atomic swap parameters and addresses. Additionally, the taker provides a "safety deposit," which will be forfeited if the swap is not finalized in the given time.

### Phase 3: (relayer, taker)

5. The relayer ensures that both escrows, containing the required token and amount, are created and that on-chain finality is reached in both networks.
6. The maker discloses the secret to the relayer (and to all resolvers).
7. The taker unlocks 100 1INCH in Ethereum for themselves, revealing the secret hash preimage.
8. The taker unlocks 200 BNB in the Binance chain for the maker, using the same secret value.

This is an "all good" scheme, but the problems of atomic swaps lie in the interruption of the process, which cannot steal funds but can potentially lock the user's funds for a significant time, forcing users to withdraw them manually. So, this protocol must include a recovery procedure in case of an interrupted process:

### Phase 4: Recovery:

The timelock expired, and locked funds were not transferred; neither party received the designated assets.

- (other) resolver transfers 100 1INCH tokens back to the maker.
- (other) resolver transfers 200 BNB tokens back to the taker.
- (other) resolver seizes the safety deposit provided by the unhappy taker.

### Escrow contracts

Now, let's examine the on-chain code; the repository used for cross-chain swaps is [here](https://github.com/1inch/fusion-plus). The escrow contracts are deployed via factory, and, as we remember, we need to deploy two escrow contracts - one to the source and one to the destination chain.

Let's start with the deployment of an escrow contract in the destination chain using the `createDstEscrow()` function. First, a check of the safety deposit or safety deposit + target amount in case the target token is a native token. Then, a check of the correctness of the timelock value is performed, followed by the deployment of the escrow and the transfer of needed tokens for the destination chain. It's important to notice, that escrow contract address is deterministically derived from all escrow parameters using CREATE2 and salt (the `cloneDeterministic()` function used for deployments is described [here](https://github.com/1inch/fusion-plus/blob/master/contracts/EscrowFactory.sol)) - we will return to this fact later. Thus, addresses of both destination and source chain escrows can be easily calculated by functions `addressOfEscrowSrc()` and `addressOfEscrowDst()`.

Next, let's deploy the source chain escrow. It's deployed using a different workflow through `_postInteraction()`, added to the Fusion order by the resolver that won the Dutch auction and now fills the Fusion order on the source chain. First, we call the "inherited" post-interaction from the Fusion order, then we need to set the hashlock. In a regular order, it's simply provided in the arguments, but in cases where the order allows multiple fills, we cannot disclose the "full" secret. This part is described in the whitepaper's "2.5 Partial fills and secret management" section. Orders with multiple fills have multiple secrets, with each secret corresponding to the next fillable part. For example, if we have an order that can be split into 4 parts (25% each), we have 4 secrets, each "opening" the next 25% of the full order amount, and +1 secret is used if the last part was not completely filled. These secrets are combined in a Merkle tree, and for this particular escrow, we take the hashlock from the tree. Merkle tree values and invalidation logic with Merkle proof processing are implemented in `MerkleStorageInvalidator.sol` [here](https://github.com/1inch/fusion-plus/blob/master/contracts/MerkleStorageInvalidator.sol).

The deployment on the source chain is executed in the same manner as for the destination chain, "salted" with all parameters. At the end, an additional check for the presence of the required balances at the escrow address is performed.

The next step is the escrow contract itself. The core functions reside in `EscrowSrc.sol` and are quite simple. Most of the code is composed of modifiers restricting addresses, time, and, of course, hashlock. This simplicity is crucial because these escrow contracts are very cheap to deploy and can be easily implemented in any non-EVM blockchains, including Bitcoin, where the first atomic swaps were introduced. Look at an example Bitcoin Script for hashlock [here](https://github.com/1inch/fusion-plus/blob/master/contracts/EscrowSrc.sol) – easy, isn't it?

This means that Fusion+ can use the same infrastructure, Fusion orders, Dutch auctions, and competing resolvers for swaps across many different networks with totally different programmable logic, including ZK environments providing strong security guarantees ensured by atomic swaps.

### Implementation details

One important place to mention in Fusion+ is the deployment of escrow contracts. As we deploy these contracts for each trade, minimal gas consumption for these operations plays a critical role in the protocol. The conventional scheme requires deploying a new contract and initializing parameters in the storage, but it's too expensive. An alternative approach is the usage of "clones-with-immutable-args" (look at [this repo](https://github.com/1inch/fusion-plus/blob/master/contracts/EscrowFactory.sol)), where a minimalistic transparent proxy (EIP-1167) is deployed, storing parameters as immutable values saved directly in the proxy's bytecode. Then, this proxy simply appends these values to each call to the implementation, allowing the implementation to read these values from the calldata, thus avoiding any storage operations.

Fusion+ uses the idea of passing parameters through the proxy to the implementation but goes further by attempting to avoid even storing them in the proxy's bytecode, because it costs 200 gas for each byte, while we need hundreds of bytes for all parameters. Therefore, the only viable option left is to pass the same immutable parameters with each(!) call to the proxy->escrow. In cases where the deployed contract is actively used, increasing transaction sizes is not the optimal solution. However, in the case of Fusion+, there are very few calls to the escrow contract(deploy and withdraw). And this approach works perfectly because it results in very compact bytecode and involves only two calls that contain all the necessary parameters, which cannot be subverted. That's why in the escrow contract, all functions (example) contain `Immutables calldata immutables` parameters. Of course, we need to ensure that all calls to the escrow contain the same immutables. And here Fusion+ uses the interesting trick - all calls perform the `onlyValidImmutables(immutables)` check, which uses the CREATE2 address of the minimal proxy (remember that this address is deterministically derived from these immutable parameters) to check the validity of immutables. This approach saves tens of thousands of extra gas for escrow deployment operations.

## Conclusion

1inch Limit Order Protocol and Fusion/Fusion+ protocols are beautiful examples of composability in DeFi. The first-level LOP provides an order settlement layer with security guarantees, Fusion extends these orders with Dutch auction for resolvers, and Fusion+ extends Fusion orders to cross-chain swaps using the atomic swap pattern.

1inch protocols are built for integrations with multiple dApps that can easily use their API for placing users' limit orders and implement a variety of different DeFi strategies, creating orders with almost arbitrary flexibility. LOP doesn't have any protocol fees, and governance functions are related mostly to protocol improvement and management of protocol resolvers. It is definitely worth studying for DeFi developers and auditors.

See you in our next articles!

---

**Who is MixBytes?**

MixBytes is a team of expert blockchain auditors and security researchers specializing in providing comprehensive smart contract audits and technical advisory services for EVM-compatible and Substrate-based projects. Join us on [X](https://twitter.com/mixbytes) to stay up-to-date with the latest industry trends and insights.

**Disclaimer**

The information contained in this Website is for educational and informational purposes only and shall not be understood or construed as financial or investment advice.

---

## Other posts

**Research May 21, 2025**  
This article explores the evolution of DeFi derivatives from early AMM-based protocols to high-performance zk-rollup DEXs with near-CEX latency and unified cross-margin systems.

**Research May 20, 2025**  
This research by MixBytes, conducted in close collaboration with Lido contributors, proposes a vault-based architecture for Lido v3 that enables customizable staking setups to support new DeFi use cases and drive more liquidity into liquid staking.

**Research May 07, 2025**  
This article explores how EIP-7688 introduces stable Merkle indexing for Ethereum's Beacon Chain, reducing upgrade friction for protocols relying on EIP-4788.

[View All Articles](https://mixbytes.io/blog)

---

© 2025 MixBytes ( )hello@mixbytes.io | [Privacy policy](https://mixbytes.io/privacy) | [Terms of Service](https://mixbytes.io/terms) | [Mediakit](https://mixbytes.io/mediakit)
