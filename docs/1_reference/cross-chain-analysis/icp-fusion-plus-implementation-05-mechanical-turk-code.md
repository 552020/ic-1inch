# ICP Fusion+ Implementation - Mechanical Turk Code Examples

> **Purpose**: Complete code implementations for the Mechanical Turk demo approach
> **Reference**: [Mechanical Turk Overview](icp-fusion-plus-implementation-04-mechanical-turk.md)

## 🔧 **Frontend Implementation**

### **Web2-Style Swap Interface**

```javascript
// Minimal swap interface - no blockchain complexity exposed
class SwapInterface {
  constructor() {
    this.wallet = null;
    this.supportedPairs = [
      { from: "ICP", to: "ETH", rate: 0.001 }, // Fixed rate for demo
    ];
  }

  async connectWallet() {
    // Standard MetaMask connection
    if (typeof window.ethereum !== "undefined") {
      this.wallet = await window.ethereum.request({
        method: "eth_requestAccounts",
      });
      return this.wallet[0];
    }
    throw new Error("MetaMask not found");
  }

  async initiateSwap(fromToken, toToken, amount) {
    // Create order structure
    const order = {
      id: generateOrderId(),
      maker: this.wallet[0],
      fromToken,
      toToken,
      fromAmount: amount,
      toAmount: this.calculateToAmount(fromToken, toToken, amount),
      timestamp: Date.now(),
    };

    // Sign order with MetaMask
    const signature = await this.signOrder(order);

    // Submit to resolver API
    const response = await fetch("/api/swap", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ order, signature }),
    });

    return await response.json();
  }

  async signOrder(order) {
    // EIP-712 signature (simplified)
    const domain = {
      name: "ICP-ETH Swap Demo",
      version: "1",
      chainId: 1,
    };

    const types = {
      SwapOrder: [
        { name: "id", type: "string" },
        { name: "maker", type: "address" },
        { name: "fromToken", type: "string" },
        { name: "toToken", type: "string" },
        { name: "fromAmount", type: "uint256" },
        { name: "toAmount", type: "uint256" },
        { name: "timestamp", type: "uint256" },
      ],
    };

    return await window.ethereum.request({
      method: "eth_signTypedData_v4",
      params: [this.wallet[0], JSON.stringify({ domain, types, message: order })],
    });
  }
}
```

## 🤖 **Centralized Resolver Implementation**

### **Node.js/Express Automated Market Maker**

```javascript
// Required Dependencies
const express = require("express");
const { ethers } = require("ethers"); // For Ethereum interactions and signature verification
const { Agent, HttpAgent } = require("@dfinity/agent"); // For ICP canister calls
const { Principal } = require("@dfinity/principal"); // For ICP identity management

// Automated resolver that acts as the "market maker"
class MechanicalTurkResolver {
  constructor() {
    // ICP Integration: Using DFINITY agent for canister communication
    this.icpAgent = new HttpAgent({ host: "https://ic0.app" });
    this.icpCanisterId = process.env.ICP_SWAP_CANISTER_ID;

    // Ethereum Integration: Using ethers.js for blockchain interaction
    this.ethProvider = new ethers.providers.JsonRpcProvider(process.env.ETH_RPC_URL);
    this.ethWallet = new ethers.Wallet(process.env.ETH_PRIVATE_KEY, this.ethProvider);
    this.ethContract = new ethers.Contract(process.env.ETH_SWAP_CONTRACT_ADDRESS, SwapContractABI, this.ethWallet);

    // Demo Liquidity Pool: In-memory reserves (production would use database)
    this.reserveICP = 1000; // 1000 ICP available for swaps
    this.reserveETH = 1.0; // 1 ETH available for swaps

    // Rate Management: Fixed rates for demo (production would use price feeds)
    this.exchangeRates = {
      ICP_TO_ETH: 0.001, // 1 ICP = 0.001 ETH
      ETH_TO_ICP: 1000, // 1 ETH = 1000 ICP
    };
  }

  // Main Entry Point: Process incoming swap requests from frontend
  async processSwapRequest(order, signature) {
    try {
      console.log(`Processing swap: ${order.fromAmount} ${order.fromToken} → ${order.toAmount} ${order.toToken}`);

      // STEP 1: Cryptographic Verification
      // Using ethers.js to verify EIP-712 signature from MetaMask
      const isValid = await this.validateEIP712Signature(order, signature);
      if (!isValid) throw new Error("Invalid signature - user didn't actually sign this order");

      // STEP 2: Business Logic Validation
      // Check if we have enough liquidity to fulfill the swap
      if (!this.hasLiquidity(order)) {
        throw new Error(`Insufficient liquidity: need ${order.toAmount} ${order.toToken}, have ${this.getReserve(order.toToken)}`);
      }

      // STEP 3: Cross-Chain Atomic Swap Execution
      // This is the core operation - coordinate between ICP and Ethereum
      const result = await this.executeAtomicSwap(order);

      // STEP 4: State Management
      // Update our internal liquidity tracking
      this.updateReserves(order);

      // STEP 5: Transaction Logging
      await this.logTransaction(order, result);

      return { success: true, txHash: result.txHash, message: "Swap completed successfully" };
    } catch (error) {
      console.error("Swap failed:", error);
      return { success: false, error: error.message };
    }
  }

  // Core Operation: Execute the actual cross-chain swap
  async executeAtomicSwap(order) {
    console.log("Executing atomic swap...");

    // Handle ICP → ETH swap
    if (order.fromToken === "ICP" && order.toToken === "ETH") {
      // OPERATION 1: Lock User's ICP on ICP Network
      // Using DFINITY agent to call our ICP canister
      console.log(`Locking ${order.fromAmount} ICP from user ${order.maker}`);
      const icpLockResult = await this.icpAgent.call(this.icpCanisterId, {
        methodName: "lock_icp_for_swap",
        arg: {
          order_id: order.id,
          user: Principal.fromText(order.maker), // Convert ETH address to ICP Principal
          amount: BigInt(order.fromAmount * 100000000), // Convert to e8s (ICP's smallest unit)
        },
      });

      if (!icpLockResult.success) {
        throw new Error(`Failed to lock ICP: ${icpLockResult.error}`);
      }

      // OPERATION 2: Send ETH to User on Ethereum
      // Using ethers.js to send ETH transaction (we pay gas fees)
      console.log(`Sending ${order.toAmount} ETH to user ${order.maker}`);
      const ethTransaction = await this.ethWallet.sendTransaction({
        to: order.maker,
        value: ethers.utils.parseEther(order.toAmount.toString()),
        gasLimit: 21000, // Standard ETH transfer
        gasPrice: await this.ethProvider.getGasPrice(),
      });

      // Wait for Ethereum confirmation
      const ethReceipt = await ethTransaction.wait();
      console.log(`ETH sent in transaction: ${ethReceipt.transactionHash}`);

      // OPERATION 3: Claim User's ICP for Ourselves
      // Complete the swap by claiming the locked ICP
      console.log("Claiming locked ICP...");
      const icpClaimResult = await this.icpAgent.call(this.icpCanisterId, {
        methodName: "claim_locked_icp",
        arg: {
          order_id: order.id,
          resolver: this.icpAgent.getPrincipal(), // Our resolver identity
        },
      });

      if (!icpClaimResult.success) {
        // This is bad - user got ETH but we didn't get ICP
        console.error("CRITICAL: Failed to claim ICP after sending ETH!");
        throw new Error("Swap partially failed - manual intervention required");
      }

      return {
        txHash: ethReceipt.transactionHash,
        icpLockId: icpLockResult.lock_id,
        ethAmount: order.toAmount,
        icpAmount: order.fromAmount,
      };
    }

    // Handle ETH → ICP swap (reverse direction)
    else if (order.fromToken === "ETH" && order.toToken === "ICP") {
      // Similar logic but reversed...
      throw new Error("ETH → ICP swaps not implemented in this demo");
    }

    throw new Error(`Unsupported swap pair: ${order.fromToken} → ${order.toToken}`);
  }

  // Signature Verification: Ensure user actually signed the order
  async validateEIP712Signature(order, signature) {
    try {
      // Reconstruct the EIP-712 domain and types (must match frontend)
      const domain = {
        name: "ICP-ETH Swap Demo",
        version: "1",
        chainId: 1, // Ethereum mainnet
      };

      const types = {
        SwapOrder: [
          { name: "id", type: "string" },
          { name: "maker", type: "address" },
          { name: "fromToken", type: "string" },
          { name: "toToken", type: "string" },
          { name: "fromAmount", type: "uint256" },
          { name: "toAmount", type: "uint256" },
          { name: "timestamp", type: "uint256" },
        ],
      };

      // Use ethers.js to verify the signature
      const recoveredAddress = ethers.utils.verifyTypedData(domain, types, order, signature);

      // Check if recovered address matches the claimed maker
      const isValid = recoveredAddress.toLowerCase() === order.maker.toLowerCase();
      console.log(`Signature verification: ${isValid ? "VALID" : "INVALID"} (recovered: ${recoveredAddress})`);

      return isValid;
    } catch (error) {
      console.error("Signature verification failed:", error);
      return false;
    }
  }

  // Liquidity Management: Check if we can fulfill the swap
  hasLiquidity(order) {
    const requiredAmount = order.toAmount;
    const availableAmount = this.getReserve(order.toToken);

    console.log(`Liquidity check: need ${requiredAmount} ${order.toToken}, have ${availableAmount}`);
    return availableAmount >= requiredAmount;
  }

  getReserve(token) {
    return token === "ETH" ? this.reserveETH : this.reserveICP;
  }

  // State Updates: Maintain our liquidity pool
  updateReserves(order) {
    if (order.fromToken === "ICP" && order.toToken === "ETH") {
      this.reserveICP += order.fromAmount; // We gained ICP
      this.reserveETH -= order.toAmount; // We spent ETH
    } else if (order.fromToken === "ETH" && order.toToken === "ICP") {
      this.reserveETH += order.fromAmount; // We gained ETH
      this.reserveICP -= order.toAmount; // We spent ICP
    }

    console.log(`Updated reserves: ${this.reserveICP} ICP, ${this.reserveETH} ETH`);
  }

  // Transaction Logging: Keep records for monitoring and debugging
  async logTransaction(order, result) {
    const logEntry = {
      timestamp: new Date().toISOString(),
      orderId: order.id,
      user: order.maker,
      fromToken: order.fromToken,
      toToken: order.toToken,
      fromAmount: order.fromAmount,
      toAmount: order.toAmount,
      txHash: result.txHash,
      status: "completed",
    };

    // In production: save to database
    // For demo: just log to console
    console.log("Transaction completed:", JSON.stringify(logEntry, null, 2));
  }
}

// Express.js API Integration
const app = express();
app.use(express.json());

const resolver = new MechanicalTurkResolver();

// API Endpoint: Handle swap requests from frontend
app.post("/api/swap", async (req, res) => {
  try {
    const { order, signature } = req.body;
    const result = await resolver.processSwapRequest(order, signature);
    res.json(result);
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

// Health Check: Monitor resolver status
app.get("/api/status", (req, res) => {
  res.json({
    status: "operational",
    reserves: {
      ICP: resolver.reserveICP,
      ETH: resolver.reserveETH,
    },
    rates: resolver.exchangeRates,
  });
});

module.exports = { MechanicalTurkResolver, app };
```

### **Package Dependencies**

```json
{
  "dependencies": {
    "express": "^4.18.0", // Web server framework
    "ethers": "^5.7.0", // Ethereum blockchain interaction
    "@dfinity/agent": "^0.19.0", // ICP canister communication
    "@dfinity/principal": "^0.19.0", // ICP identity management
    "cors": "^2.8.5", // Cross-origin requests
    "helmet": "^6.0.0", // Security middleware
    "winston": "^3.8.0" // Logging framework
  }
}
```

## 🔗 **ICP Canister Implementation**

### **Rust Canister for Token Locking**

```rust
// Simplified ICP canister for token locking
use ic_cdk::*;
use ic_cdk_macros::*;
use std::collections::HashMap;

#[derive(Default)]
struct SwapCanister {
    locked_tokens: HashMap<String, u64>, // order_id -> amount
    authorized_resolvers: Vec<Principal>,
}

thread_local! {
    static STATE: std::cell::RefCell<SwapCanister> = std::cell::RefCell::new(SwapCanister::default());
}

#[update]
async fn lock_icp_for_swap(order_id: String, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    // Verify caller has sufficient ICP
    let balance = get_icp_balance(caller).await?;
    if balance < amount {
        return Err("Insufficient ICP balance".to_string());
    }

    // Lock tokens
    STATE.with(|state| {
        state.borrow_mut().locked_tokens.insert(order_id.clone(), amount);
    });

    // Transfer ICP to canister
    transfer_icp_to_canister(caller, amount).await?;

    Ok(())
}

#[update]
async fn claim_locked_icp(order_id: String, resolver: Principal) -> Result<(), String> {
    // Verify resolver is authorized
    let is_authorized = STATE.with(|state| {
        state.borrow().authorized_resolvers.contains(&resolver)
    });

    if !is_authorized {
        return Err("Unauthorized resolver".to_string());
    }

    // Transfer locked ICP to resolver
    let amount = STATE.with(|state| {
        state.borrow_mut().locked_tokens.remove(&order_id)
    }).ok_or("Order not found")?;

    transfer_icp_from_canister(resolver, amount).await?;

    Ok(())
}

#[update]
fn add_authorized_resolver(resolver: Principal) -> Result<(), String> {
    // Only admin can add resolvers
    if ic_cdk::caller() != get_admin_principal() {
        return Err("Only admin can add resolvers".to_string());
    }

    STATE.with(|state| {
        state.borrow_mut().authorized_resolvers.push(resolver);
    });

    Ok(())
}

#[query]
fn get_locked_amount(order_id: String) -> Option<u64> {
    STATE.with(|state| {
        state.borrow().locked_tokens.get(&order_id).copied()
    })
}

// Helper functions (would be implemented based on ICP standards)
async fn get_icp_balance(principal: Principal) -> Result<u64, String> {
    // Implementation depends on ICP ledger integration
    Ok(1000_000_000) // Placeholder
}

async fn transfer_icp_to_canister(from: Principal, amount: u64) -> Result<(), String> {
    // Implementation depends on ICP ledger integration
    Ok(())
}

async fn transfer_icp_from_canister(to: Principal, amount: u64) -> Result<(), String> {
    // Implementation depends on ICP ledger integration
    Ok(())
}

fn get_admin_principal() -> Principal {
    // Return the admin principal
    Principal::from_text("rdmx6-jaaaa-aaaah-qcaiq-cai").unwrap()
}
```

## ⚡ **Ethereum Smart Contract**

### **Solidity Contract for ETH Management**

```solidity
// Simplified ETH contract for receiving/sending ETH
pragma solidity ^0.8.0;

contract SwapDemo {
    address public owner;
    mapping(address => bool) public authorizedResolvers;
    mapping(string => uint256) public lockedETH;

    event ETHLocked(string indexed orderId, uint256 amount, address resolver);
    event ETHSent(address indexed user, uint256 amount, string orderId);
    event ResolverAdded(address indexed resolver);

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner");
        _;
    }

    modifier onlyResolver() {
        require(authorizedResolvers[msg.sender], "Unauthorized resolver");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function lockETHForSwap(string memory orderId) external payable onlyResolver {
        require(msg.value > 0, "Must send ETH");
        lockedETH[orderId] = msg.value;
        emit ETHLocked(orderId, msg.value, msg.sender);
    }

    function sendETHToUser(address payable user, uint256 amount, string memory orderId) external onlyResolver {
        require(address(this).balance >= amount, "Insufficient contract balance");
        require(lockedETH[orderId] >= amount, "Insufficient locked amount");

        lockedETH[orderId] -= amount;
        user.transfer(amount);

        emit ETHSent(user, amount, orderId);
    }

    function addResolver(address resolver) external onlyOwner {
        authorizedResolvers[resolver] = true;
        emit ResolverAdded(resolver);
    }

    function removeResolver(address resolver) external onlyOwner {
        authorizedResolvers[resolver] = false;
    }

    function getContractBalance() external view returns (uint256) {
        return address(this).balance;
    }

    function getLockedAmount(string memory orderId) external view returns (uint256) {
        return lockedETH[orderId];
    }

    // Emergency functions
    function withdrawEmergency() external onlyOwner {
        payable(owner).transfer(address(this).balance);
    }

    receive() external payable {
        // Allow contract to receive ETH
    }
}
```

## 🌐 **Frontend HTML Interface**

### **Simple Web Interface**

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>ICP ↔ ETH Swap Demo</title>
    <style>
      body {
        font-family: Arial, sans-serif;
        max-width: 600px;
        margin: 0 auto;
        padding: 20px;
      }
      .container {
        background: #f5f5f5;
        padding: 20px;
        border-radius: 10px;
      }
      button {
        background: #007bff;
        color: white;
        border: none;
        padding: 10px 20px;
        border-radius: 5px;
        cursor: pointer;
      }
      button:hover {
        background: #0056b3;
      }
      input {
        width: 100%;
        padding: 10px;
        margin: 10px 0;
        border: 1px solid #ddd;
        border-radius: 5px;
      }
      .status {
        margin: 10px 0;
        padding: 10px;
        border-radius: 5px;
      }
      .success {
        background: #d4edda;
        color: #155724;
      }
      .error {
        background: #f8d7da;
        color: #721c24;
      }
    </style>
  </head>
  <body>
    <div class="container">
      <h1>ICP ↔ ETH Swap Demo</h1>
      <p>Swap your ICP for ETH with zero gas fees!</p>

      <div id="wallet-section">
        <button onclick="connectWallet()">Connect MetaMask</button>
        <div id="wallet-status"></div>
      </div>

      <div id="swap-section" style="display: none;">
        <h3>Swap Details</h3>
        <input type="number" id="icp-amount" placeholder="ICP Amount" step="0.1" />
        <input type="number" id="eth-amount" placeholder="ETH Amount (calculated)" readonly />
        <button onclick="executeSwap()">Execute Swap</button>
      </div>

      <div id="status"></div>
    </div>

    <script src="https://cdn.ethers.io/lib/ethers-5.7.0.umd.min.js"></script>
    <script>
      let wallet = null;
      const swapInterface = new SwapInterface();

      async function connectWallet() {
        try {
          const address = await swapInterface.connectWallet();
          document.getElementById("wallet-status").innerHTML = `Connected: ${address}`;
          document.getElementById("swap-section").style.display = "block";
        } catch (error) {
          showStatus(`Error: ${error.message}`, "error");
        }
      }

      async function executeSwap() {
        const icpAmount = document.getElementById("icp-amount").value;
        if (!icpAmount) {
          showStatus("Please enter ICP amount", "error");
          return;
        }

        try {
          showStatus("Processing swap...", "info");
          const result = await swapInterface.initiateSwap("ICP", "ETH", parseFloat(icpAmount));

          if (result.success) {
            showStatus(`Swap completed! Transaction: ${result.txHash}`, "success");
          } else {
            showStatus(`Swap failed: ${result.error}`, "error");
          }
        } catch (error) {
          showStatus(`Error: ${error.message}`, "error");
        }
      }

      function showStatus(message, type) {
        const statusDiv = document.getElementById("status");
        statusDiv.innerHTML = `<div class="status ${type}">${message}</div>`;
      }

      // Auto-calculate ETH amount
      document.getElementById("icp-amount").addEventListener("input", function () {
        const icpAmount = this.value;
        const ethAmount = icpAmount * 0.001; // Fixed rate
        document.getElementById("eth-amount").value = ethAmount.toFixed(6);
      });
    </script>
  </body>
</html>
```

This code file contains all the implementation details while the main document focuses on the conceptual overview and architecture decisions.
