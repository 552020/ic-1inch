# The Mysterious AccessToken: A Deep Dive

## Overview

The **AccessToken** is a critical component of the 1inch Cross-Chain Swap protocol that serves as a **gatekeeper mechanism** for public functions. It's a dedicated ERC20 token that controls access to certain protocol functions, preventing spam and abuse while creating economic barriers for participation.

## Token Details

### **Token Information:**

- **Name:** "ACCESS"
- **Symbol:** "ACCESS"
- **Type:** ERC20 Token
- **Purpose:** Access control for public functions

### **Deployed Addresses:**

**Main Chains (Ethereum, Base, Polygon, etc.):**

```
0xACCe550000159e70908C0499a1119D04e7039C28
```

**ZkSync:**

```
0xC2c4fE863EC835D7DdbFE91Fe33cf1C7Df45Fa7C
```

## How It Works

### **Core Mechanism:**

The AccessToken uses a simple but effective pattern:

```solidity
modifier onlyAccessTokenHolder() {
    if (_ACCESS_TOKEN.balanceOf(msg.sender) == 0) revert InvalidCaller();
    _;
}
```

**Any balance > 0** grants access to public functions.

### **Functions That Require AccessToken:**

#### **1. Public Withdrawal Functions:**

```solidity
function publicWithdraw(bytes32 secret, Immutables calldata immutables)
    external
    onlyAccessTokenHolder()  // ← Requires AccessToken
    onlyAfter(immutables.timelocks.get(TimelocksLib.Stage.SrcPublicWithdrawal))
    onlyBefore(immutables.timelocks.get(TimelocksLib.Stage.SrcCancellation))
{
    _withdrawTo(secret, immutables.taker.get(), immutables);
}
```

#### **2. Public Cancellation Functions:**

```solidity
function publicCancel(Immutables calldata immutables)
    external
    onlyAccessTokenHolder()  // ← Requires AccessToken
    onlyAfter(immutables.timelocks.get(TimelocksLib.Stage.SrcPublicCancellation))
{
    _cancel(immutables);
}
```

#### **3. Limit Order Protocol Integration:**

```solidity
// In ResolverValidationExtension.sol
if (!_isWhitelisted(allowedTime, extraData[:whitelistSize], resolversCount, taker)) {
    if (allowedTime > block.timestamp || _ACCESS_TOKEN.balanceOf(taker) == 0)
        revert ResolverCanNotFillOrder();
    // ... charge fee if needed
}
```

## Use Cases

### **1. Cross-Chain Swap Recovery**

**Scenario:** A cross-chain swap fails because the taker disappears

- **Normal flow:** Taker calls `withdraw()` (no token needed)
- **Recovery:** Anyone with AccessToken can call `publicWithdraw()` to complete the swap

### **2. Stuck Swap Resolution**

**Scenario:** A swap gets stuck due to network issues

- **Normal flow:** Taker calls `cancel()` (no token needed)
- **Recovery:** Anyone with AccessToken can call `publicCancel()` to resolve

### **3. Limit Order Protocol Access**

**Scenario:** Non-whitelisted resolvers want to participate

- **Whitelisted:** Can participate without AccessToken
- **Non-whitelisted:** Need AccessToken to participate

### **4. Public Recovery Participation**

**Scenario:** Anyone can help complete failed swaps

- **"Mechanical Turk" Model:** Bots can monitor for unclaimed escrows
- **Economic Incentive:** Safety deposits reward public participants
- **Decentralized Recovery:** Anyone with token can help resolve issues

## Why This Design?

### **Problems It Solves:**

1. **Spam Prevention:** Without AccessToken, anyone could spam public functions
2. **Economic Barrier:** Creates cost to participate in public recovery functions
3. **Value Creation:** AccessToken becomes valuable as protocol usage grows
4. **Access Control:** Simple but effective gatekeeping mechanism
5. **Decentralized Recovery:** Enables community participation in swap resolution

### **Benefits:**

1. **Simple:** Just check if balance > 0
2. **Flexible:** Can be distributed independently
3. **Cross-chain:** Same token works across all chains
4. **Economic:** Creates value for token holders
5. **Dual Purpose:** Serves both resolvers and public participants

## Implementation Details

### **Token Distribution:**

- **Test Environment:** Mock tokens with 1 token = access
- **Production:** Real token with economic value
- **Distribution:** Protocol-specific (not 1inch governance token)

### **Chain-Specific Deployments:**

- **Main chains:** Same address across all chains
- **ZkSync:** Different address due to unique architecture
- **Future chains:** May need different addresses

### **Integration Points:**

1. **EscrowFactory:** Deploys escrows with AccessToken
2. **BaseEscrow:** Implements access control
3. **EscrowSrc/EscrowDst:** Use access control for public functions
4. **ResolverValidationExtension:** Integrates with Limit Order Protocol

### **Dual-Use Nature:**

**Primary Users (Resolvers):**

- **Whitelisted resolvers:** No token needed
- **Non-whitelisted resolvers:** Must hold token to participate in LOP
- **Lightweight KYC alternative:** Token = permission for non-whitelisted

**Secondary Users (Public Participants):**

- **Anyone with token:** Can call public recovery functions
- **Bots and monitors:** Can help complete failed swaps
- **Economic incentives:** Safety deposits reward participation

## Security Considerations

### **Access Control:**

- **Private functions:** Only specific users (no token needed)
- **Public functions:** Anyone with token can call
- **Timelock protection:** Still applies even with token

### **Economic Security:**

- **Token value:** Creates economic disincentive for abuse
- **Distribution:** Controlled to prevent manipulation
- **Cross-chain:** Same token across all chains prevents arbitrage

## Testing

### **Mock Implementation:**

```solidity
// In test setup
accessToken = new TokenMock("ACCESS", "ACCESS");
accessToken.mint(bob.addr, 1); // Grant access
```

### **Test Scenarios:**

1. **With token:** Can call public functions
2. **Without token:** Cannot call public functions
3. **Token burning:** Loses access after burning
4. **Cross-chain:** Same token works on different chains

## Conclusion

The **AccessToken** is a **simple but powerful** access control mechanism that:

1. **Prevents spam** through economic barriers
2. **Enables recovery** of failed swaps
3. **Creates value** for token holders
4. **Maintains security** through timelock protection
5. **Works cross-chain** for global protocol access
6. **Enables decentralized recovery** through community participation

It's essentially a **"membership card"** for the cross-chain swap protocol that serves **dual purposes**:

- **Resolvers:** Access control for non-whitelisted participants
- **Public:** Community-driven recovery of failed swaps

The token creates a **decentralized "mechanical turk" model** where anyone with the token can help resolve issues and earn rewards! 🎫
