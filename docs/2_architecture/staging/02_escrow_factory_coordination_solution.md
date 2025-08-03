# Coordination Solution: Single Transaction Escrow Creation

## Executive Summary

The core problem remains: **EVM escrow creation requires two separate transactions** (source + destination), which creates coordination challenges. This document proposes a solution that **eliminates the need for separate source escrows** in EVM chains.

## 1. The Core Problem

### 1.1 Current EVM Escrow Creation Flow

```
EVM → ICP Order:
1. User creates destination escrow on EVM (for ICP tokens)
2. User creates source escrow on EVM (for EVM tokens) ❌ PROBLEM
3. Resolver fills order
4. Both escrows must be coordinated

ICP → EVM Order:
1. User locks assets in ICP escrow ✅ (reverse gas model)
2. Resolver creates destination escrow on EVM ✅
3. Resolver creates source escrow on EVM ❌ PROBLEM
4. Both escrows must be coordinated
```

### 1.2 The Coordination Problem

**Why this is problematic:**

- **Two separate transactions** = potential for one to fail
- **Timing coordination** between source and destination escrows
- **Gas costs** for two escrow deployments
- **State synchronization** across chains
- **Rollback complexity** if one escrow fails

## 2. Proposed Solution: Single Escrow Architecture

### 2.1 Core Concept

**Instead of creating two escrows, create only ONE escrow that handles both sides of the swap:**

```
New Architecture:
1. User creates order → Virtual escrows in ICP
2. Resolver fills order → Create ONE escrow on target chain
3. Single escrow handles both source and destination assets
4. No coordination needed between multiple escrows
```

### 2.2 Single Escrow Design

```solidity
// Single escrow that handles both sides of the swap
contract CrossChainEscrow {
    struct SwapData {
        address sourceToken;
        address destinationToken;
        uint256 sourceAmount;
        uint256 destinationAmount;
        address maker;
        address taker;
        bytes32 hashlock;
        uint256 timelocks;
    }

    SwapData public swapData;
    bool public sourceLocked;
    bool public destinationLocked;
    bool public completed;

    constructor(SwapData memory _swapData) {
        swapData = _swapData;
    }

    // Lock source assets (EVM tokens)
    function lockSourceAssets() external payable {
        require(!sourceLocked, "Source already locked");
        require(msg.sender == swapData.maker, "Only maker can lock");

        // Transfer tokens from maker to escrow
        IERC20(swapData.sourceToken).transferFrom(
            msg.sender,
            address(this),
            swapData.sourceAmount
        );

        sourceLocked = true;
        emit SourceLocked(swapData.sourceAmount);
    }

    // Lock destination assets (EVM tokens)
    function lockDestinationAssets() external payable {
        require(!destinationLocked, "Destination already locked");
        require(msg.sender == swapData.taker, "Only taker can lock");

        // Transfer tokens from taker to escrow
        IERC20(swapData.destinationToken).transferFrom(
            msg.sender,
            address(this),
            swapData.destinationAmount
        );

        destinationLocked = true;
        emit DestinationLocked(swapData.destinationAmount);
    }

    // Complete swap with secret
    function completeSwap(bytes32 secret) external {
        require(sourceLocked && destinationLocked, "Both sides must be locked");
        require(!completed, "Already completed");
        require(keccak256(abi.encodePacked(secret)) == swapData.hashlock, "Invalid secret");

        // Transfer source assets to taker
        IERC20(swapData.sourceToken).transfer(
            swapData.taker,
            swapData.sourceAmount
        );

        // Transfer destination assets to maker
        IERC20(swapData.destinationToken).transfer(
            swapData.maker,
            swapData.destinationAmount
        );

        completed = true;
        emit SwapCompleted(secret);
    }

    // Cancel swap (timelock-based)
    function cancelSwap() external {
        require(block.timestamp > swapData.timelocks, "Timelock not expired");
        require(!completed, "Already completed");

        // Return assets to original owners
        if (sourceLocked) {
            IERC20(swapData.sourceToken).transfer(
                swapData.maker,
                swapData.sourceAmount
            );
        }

        if (destinationLocked) {
            IERC20(swapData.destinationToken).transfer(
                swapData.taker,
                swapData.destinationAmount
            );
        }

        emit SwapCancelled();
    }
}
```

## 3. Updated Flow Architecture

### 3.1 EVM → ICP Orders

```
1. User creates order → Virtual escrows in ICP
2. User locks EVM tokens in single escrow on EVM
3. Resolver fills order → Creates ICP escrow
4. Resolver locks ICP tokens in ICP escrow
5. Resolver reveals secret → Both escrows unlock
```

### 3.2 ICP → EVM Orders

```
1. User creates order → Virtual escrows in ICP
2. User locks ICP tokens in virtual escrow (reverse gas model)
3. Resolver fills order → Creates single escrow on EVM
4. Resolver locks EVM tokens in single escrow
5. Resolver reveals secret → Both escrows unlock
```

## 4. Implementation Strategy

### 4.1 Single Escrow Factory

```solidity
contract SingleEscrowFactory {
    mapping(bytes32 => address) public escrowAddresses;

    function createSingleEscrow(
        address sourceToken,
        address destinationToken,
        uint256 sourceAmount,
        uint256 destinationAmount,
        address maker,
        address taker,
        bytes32 hashlock,
        uint256 timelocks
    ) external returns (address escrow) {

        CrossChainEscrow.SwapData memory swapData = CrossChainEscrow.SwapData({
            sourceToken: sourceToken,
            destinationToken: destinationToken,
            sourceAmount: sourceAmount,
            destinationAmount: destinationAmount,
            maker: maker,
            taker: taker,
            hashlock: hashlock,
            timelocks: timelocks
        });

        escrow = address(new CrossChainEscrow(swapData));
        escrowAddresses[hashlock] = escrow;

        emit SingleEscrowCreated(escrow, hashlock, maker, taker);
    }

    function getEscrowAddress(bytes32 hashlock) external view returns (address) {
        return escrowAddresses[hashlock];
    }
}
```

### 4.2 Updated Migration Strategy

```rust
// Updated migration coordinator
impl MigrationCoordinator {
    // Migrate to single escrow instead of two separate escrows
    async fn migrate_to_single_escrow(&self, request: &MigrationRequest) -> Result<String, String> {
        let order = self.orderbook_canister.get_order_with_escrows(&request.order_id)?;

        match order.order_type {
            OrderType::EVMToICP => {
                // Create single escrow on EVM for EVM tokens
                let escrow_address = self.evm_client.create_single_escrow(
                    order.source_token,
                    order.destination_token,
                    order.source_amount,
                    order.destination_amount,
                    order.maker,
                    order.taker,
                    order.hashlock,
                    order.timelocks
                ).await?;

                // Create ICP escrow for ICP tokens
                let icp_escrow_address = self.icp_client.create_escrow(
                    order.destination_token,
                    order.destination_amount,
                    order.hashlock,
                    order.timelocks
                ).await?;

                Ok(escrow_address)
            }
            OrderType::ICPToEVM => {
                // Create single escrow on EVM for EVM tokens
                let escrow_address = self.evm_client.create_single_escrow(
                    order.destination_token,
                    order.source_token,
                    order.destination_amount,
                    order.source_amount,
                    order.taker,
                    order.maker,
                    order.hashlock,
                    order.timelocks
                ).await?;

                // ICP escrow already exists (created during order creation)

                Ok(escrow_address)
            }
        }
    }
}
```

## 5. Benefits of Single Escrow Architecture

### 5.1 **Eliminates Coordination Problems**

- **One transaction** instead of two
- **No timing coordination** needed
- **Atomic operations** within single escrow
- **Simplified rollback** if escrow creation fails

### 5.2 **Reduces Gas Costs**

- **Single deployment** instead of two
- **Fewer transactions** overall
- **Optimized gas usage** for resolvers

### 5.3 **Improves User Experience**

- **Simpler flow** for users
- **Fewer failure points**
- **Clearer state management**

### 5.4 **Enhances Security**

- **Single point of control** for assets
- **Reduced attack surface**
- **Simpler audit trail**

## 6. Updated Virtual Escrow Strategy

### 6.1 Modified Virtual Escrow Structure

```rust
#[derive(CandidType, Deserialize, Clone)]
pub struct VirtualEscrow {
    pub id: String,
    pub order_id: String,
    pub escrow_type: EscrowType,
    pub target_chain: Chain,
    pub target_address: Option<String>,
    pub status: EscrowStatus,

    // Single escrow parameters
    pub source_token: String,
    pub destination_token: String,
    pub source_amount: u128,
    pub destination_amount: u128,
    pub maker: Principal,
    pub taker: Principal,
    pub hashlock: String,
    pub timelocks: TimelockConfig,
    pub safety_deposit: u128,

    // Migration tracking
    pub migration_tx_hash: Option<String>,
    pub deployed_at: Option<u64>,
    pub error: Option<String>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum EscrowType {
    Single, // Single escrow for both sides
    ICP,    // ICP-specific escrow
}
```

### 6.2 Updated Order Structure

```rust
#[derive(CandidType, Deserialize, Clone)]
pub struct OrderWithVirtualEscrows {
    pub id: String,
    pub order_type: OrderType,
    pub source_chain: String,
    pub destination_chain: String,
    pub source_token: String,
    pub destination_token: String,
    pub source_amount: u128,
    pub destination_amount: u128,
    pub hashlock: String,
    pub timelocks: TimelockConfig,
    pub status: OrderStatus,

    // Single virtual escrow for EVM chains
    pub evm_escrow: Option<VirtualEscrow>,
    // ICP escrow for ICP-specific operations
    pub icp_escrow: Option<VirtualEscrow>,

    // Order metadata
    pub created_at: u64,
    pub evm_signature: Option<String>,
    pub resolver: Option<Principal>,
    pub fill_tx_hash: Option<String>,
}
```

## 7. Implementation Priority

### 7.1 **Phase 1: Single Escrow Contract**

1. **Develop SingleEscrowFactory** contract
2. **Implement CrossChainEscrow** contract
3. **Add comprehensive testing** for single escrow logic

### 7.2 **Phase 2: Updated Migration Logic**

1. **Modify migration coordinator** for single escrow
2. **Update virtual escrow** data structures
3. **Implement single escrow** creation flow

### 7.3 **Phase 3: Integration**

1. **Deploy single escrow** contracts to EVM chains
2. **Update backend services** for new flow
3. **Test complete integration** with ICP

## 8. Conclusion

The **single escrow architecture** solves the fundamental coordination problem by:

1. **Eliminating the need** for two separate escrow transactions
2. **Simplifying the coordination** between source and destination
3. **Reducing gas costs** and complexity
4. **Improving security** and user experience

This approach transforms the complex cross-chain coordination into a **single, atomic operation** within one escrow contract, making the entire system much more reliable and efficient.
