# Fusion+ Relayer Service - JavaScript/Next.js Implementation

> **Purpose**: Implements the enhanced relayer from Fusion+ whitepaper as a Next.js application
> **Reference**: [Fusion+ Whitepaper](1inch-fusion-plus-whitepaper.md), [Fusion to Fusion+ Upgrade Guide](fusion-to-fusion-plus-upgrade.md)

> **⚠️ IMPORTANT**: This relayer implementation is **for testing purposes only**. In production, we will integrate with 1inch's existing off-chain relayer service. 1inch will not change their infrastructure for our ICP implementation - we need to adapt to their existing Fusion+ architecture.

> **📋 Architecture Note**: The whitepaper describes the relayer as an off-chain service (1inch's centralized infrastructure). Our Next.js implementation is for development/testing only. In production, we will interface with 1inch's existing off-chain relayer to properly replicate their Fusion+ architecture.

## 🏗️ **Project Structure**

```
fusion-plus-relayer/
├── src/
│   ├── pages/
│   │   ├── api/
│   │   │   ├── announce-order.js
│   │   │   ├── notify-escrow.js
│   │   │   ├── reveal-secret.js
│   │   │   └── verify-resolver.js
│   │   └── index.js
│   ├── lib/
│   │   ├── relayer.js
│   │   ├── eip712.js
│   │   ├── icp-client.js
│   │   └── evm-client.js
│   └── utils/
│       ├── crypto.js
│       └── validation.js
├── package.json
└── README.md
```

## 🔧 **Core Implementation**

### **1. Main Relayer Service Class**

```javascript
// src/lib/relayer.js
import { ethers } from "ethers";
import { sha256 } from "../utils/crypto.js";
import { validateEIP712Signature } from "../lib/eip712.js";
import { ICPClient } from "../lib/icp-client.js";
import { EVMClient } from "../lib/evm-client.js";

export class FusionPlusRelayer {
  constructor() {
    this.orders = new Map(); // orderHash -> order state
    this.secrets = new Map(); // orderHash -> secret
    this.verifiedResolvers = new Map(); // KYC verified resolvers
    this.icpClient = new ICPClient();
    this.evmClient = new EVMClient();
  }

  // PHASE 1: Announcement Phase
  // Reference: docs/1inch-fusion-plus-whitepaper.md (lines 84-90)
  async announceOrder(order, signature, secret) {
    try {
      // 🔐 STEP 1: Verify maker's EIP-712 signature
      const isValid = await validateEIP712Signature(order, signature, order.maker);
      if (!isValid) {
        throw new Error("Invalid signature");
      }

      // 🔑 STEP 2: Verify secret matches hash in order (Fusion+ requirement)
      const secretHash = sha256(secret);
      if (secretHash !== order.secretHash) {
        throw new Error("Secret hash mismatch");
      }

      // 📝 STEP 3: Store order and secret
      const orderHash = this.computeOrderHash(order);
      this.orders.set(orderHash, {
        order,
        signature,
        status: "ANNOUNCED",
        createdAt: Date.now(),
        srcEscrow: null,
        dstEscrow: null,
        finalityTimer: null,
      });

      // Store secret securely (only relayer knows this)
      this.secrets.set(orderHash, secret);

      // 📢 STEP 4: Broadcast to verified resolvers
      await this.broadcastToResolvers(order);

      // 🕐 STEP 5: Start Dutch auction timer
      this.startAuctionTimer(orderHash);

      return { success: true, orderHash };
    } catch (error) {
      console.error("Error announcing order:", error);
      throw error;
    }
  }

  // PHASE 2: Monitor Escrow Creation
  // Reference: docs/fusion-to-fusion-plus-upgrade.md (lines 253-262)
  async notifyEscrowCreated(orderHash, escrowCanister, escrowType) {
    try {
      const orderState = this.orders.get(orderHash);
      if (!orderState) {
        throw new Error("Order not found");
      }

      // Update escrow information
      if (escrowType === "SOURCE") {
        orderState.srcEscrow = escrowCanister;
      } else if (escrowType === "DESTINATION") {
        orderState.dstEscrow = escrowCanister;
      }

      // Check if both escrows are created
      if (orderState.srcEscrow && orderState.dstEscrow) {
        orderState.status = "ESCROWS_CREATED";

        // Start finality lock timer (Fusion+ enhancement)
        const finalityDelay = 600000; // 10 minutes in milliseconds
        orderState.finalityTimer = Date.now() + finalityDelay;

        // Schedule finality check
        setTimeout(() => {
          this.checkFinalityAndRevealSecret(orderHash);
        }, finalityDelay);

        console.log(`Both escrows created for order ${orderHash}`);
      }

      return { success: true };
    } catch (error) {
      console.error("Error notifying escrow creation:", error);
      throw error;
    }
  }

  // PHASE 3: Secret Revelation (After Finality)
  // Reference: docs/1inch-fusion-plus-whitepaper.md (lines 100-106)
  async revealSecretIfReady(orderHash) {
    try {
      const orderState = this.orders.get(orderHash);
      if (!orderState) {
        throw new Error("Order not found");
      }

      // Check if finality period has passed
      const currentTime = Date.now();
      if (orderState.finalityTimer && currentTime >= orderState.finalityTimer) {
        orderState.status = "FINALITY_PASSED";

        // Get secret and resolver list
        const secret = this.secrets.get(orderHash);
        const resolvers =
          orderState.order.resolvers.length === 0
            ? Array.from(this.verifiedResolvers.keys()) // Public order
            : orderState.order.resolvers; // Private order

        if (secret) {
          // 🔓 Share secret with all eligible resolvers
          for (const resolver of resolvers) {
            await this.notifyResolverSecret(resolver, orderHash, secret);
          }

          // Update status
          orderState.status = "SECRET_REVEALED";
          console.log(`Secret revealed for order ${orderHash}`);
        }
      }

      return { success: true };
    } catch (error) {
      console.error("Error revealing secret:", error);
      throw error;
    }
  }

  // KYC/KYB Resolver Management (Fusion+ requirement)
  // Reference: docs/fusion-to-fusion-plus-upgrade.md (lines 302-331)
  async verifyResolver(resolver, kycHash, kycProof) {
    try {
      // Only admin can verify resolvers
      if (!this.isAdmin()) {
        throw new Error("Only admin can verify resolvers");
      }

      // Verify KYC documentation (simplified)
      if (!this.verifyKYCProof(kycHash, kycProof)) {
        throw new Error("Invalid KYC proof");
      }

      this.verifiedResolvers.set(resolver, {
        kycHash,
        verifiedAt: Date.now(),
        active: true,
      });

      return { success: true };
    } catch (error) {
      console.error("Error verifying resolver:", error);
      throw error;
    }
  }

  // Helper methods
  async broadcastToResolvers(order) {
    const resolvers =
      order.resolvers.length === 0
        ? Array.from(this.verifiedResolvers.keys()) // Public order
        : order.resolvers; // Private order

    // Notify each resolver
    for (const resolver of resolvers) {
      try {
        await this.notifyResolverOrder(resolver, order);
      } catch (error) {
        console.error(`Failed to notify resolver ${resolver}:`, error);
      }
    }
  }

  async notifyResolverOrder(resolver, order) {
    // Send order to resolver (could be via WebSocket, HTTP, etc.)
    console.log(`Notifying resolver ${resolver} about new order`);
    // Implementation depends on how resolvers are notified
  }

  async notifyResolverSecret(resolver, orderHash, secret) {
    // Send secret to resolver
    console.log(`Sending secret to resolver ${resolver} for order ${orderHash}`);
    // Implementation depends on how secrets are shared
  }

  computeOrderHash(order) {
    // Compute deterministic order hash
    const orderData = JSON.stringify(order);
    return ethers.utils.keccak256(ethers.utils.toUtf8Bytes(orderData));
  }

  startAuctionTimer(orderHash) {
    // Start Dutch auction timer
    console.log(`Starting Dutch auction for order ${orderHash}`);
    // Implementation for Dutch auction logic
  }

  checkFinalityAndRevealSecret(orderHash) {
    // Check finality and reveal secret if ready
    this.revealSecretIfReady(orderHash);
  }

  isAdmin() {
    // Check if caller is admin
    return true; // Simplified for demo
  }

  verifyKYCProof(kycHash, kycProof) {
    // Verify KYC documentation
    return true; // Simplified for demo
  }
}
```

### **2. API Endpoints (Next.js)**

```javascript
// src/pages/api/announce-order.js
import { FusionPlusRelayer } from "../../lib/relayer.js";

const relayer = new FusionPlusRelayer();

export default async function handler(req, res) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method not allowed" });
  }

  try {
    const { order, signature, secret } = req.body;

    const result = await relayer.announceOrder(order, signature, secret);

    res.status(200).json(result);
  } catch (error) {
    console.error("Error in announce-order API:", error);
    res.status(500).json({ error: error.message });
  }
}
```

```javascript
// src/pages/api/notify-escrow.js
import { FusionPlusRelayer } from "../../lib/relayer.js";

const relayer = new FusionPlusRelayer();

export default async function handler(req, res) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method not allowed" });
  }

  try {
    const { orderHash, escrowCanister, escrowType } = req.body;

    const result = await relayer.notifyEscrowCreated(orderHash, escrowCanister, escrowType);

    res.status(200).json(result);
  } catch (error) {
    console.error("Error in notify-escrow API:", error);
    res.status(500).json({ error: error.message });
  }
}
```

```javascript
// src/pages/api/reveal-secret.js
import { FusionPlusRelayer } from "../../lib/relayer.js";

const relayer = new FusionPlusRelayer();

export default async function handler(req, res) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method not allowed" });
  }

  try {
    const { orderHash } = req.body;

    const result = await relayer.revealSecretIfReady(orderHash);

    res.status(200).json(result);
  } catch (error) {
    console.error("Error in reveal-secret API:", error);
    res.status(500).json({ error: error.message });
  }
}
```

### **3. EIP-712 Signature Verification**

```javascript
// src/lib/eip712.js
import { ethers } from "ethers";

export async function validateEIP712Signature(order, signature, expectedSigner) {
  try {
    // 1. Create domain separator
    const domain = {
      name: "1inch Fusion+",
      version: "1",
      chainId: order.chainIdSrc,
      verifyingContract: "0x...", // Fusion+ contract address
    };

    // 2. Define types
    const types = {
      Order: [
        { name: "salt", type: "uint256" },
        { name: "maker", type: "address" },
        { name: "receiver", type: "address" },
        { name: "makerAsset", type: "address" },
        { name: "takerAsset", type: "address" },
        { name: "makingAmount", type: "uint256" },
        { name: "takingAmount", type: "uint256" },
        { name: "secretHash", type: "bytes32" },
        { name: "makerTraits", type: "uint256" },
      ],
    };

    // 3. Recreate the hash
    const orderHash = ethers.utils._TypedDataEncoder.hash(domain, types, order);

    // 4. Recover signer from signature
    const recoveredSigner = ethers.utils.verifyMessage(ethers.utils.arrayify(orderHash), signature);

    // 5. Verify it matches expected signer
    return recoveredSigner.toLowerCase() === expectedSigner.toLowerCase();
  } catch (error) {
    console.error("Error validating EIP-712 signature:", error);
    return false;
  }
}
```

### **4. ICP Client Integration**

```javascript
// src/lib/icp-client.js
export class ICPClient {
  constructor() {
    this.baseUrl = process.env.ICP_NETWORK_URL || "https://ic0.app";
  }

  async callCanister(canisterId, method, args) {
    try {
      const response = await fetch(`${this.baseUrl}/api/v2/canister/${canisterId}/call`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          method,
          args,
        }),
      });

      return await response.json();
    } catch (error) {
      console.error("Error calling ICP canister:", error);
      throw error;
    }
  }

  async queryCanister(canisterId, method, args) {
    try {
      const response = await fetch(`${this.baseUrl}/api/v2/canister/${canisterId}/query`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          method,
          args,
        }),
      });

      return await response.json();
    } catch (error) {
      console.error("Error querying ICP canister:", error);
      throw error;
    }
  }
}
```

### **5. EVM Client Integration**

```javascript
// src/lib/evm-client.js
import { ethers } from "ethers";

export class EVMClient {
  constructor() {
    this.providers = {
      ethereum: new ethers.providers.JsonRpcProvider(process.env.ETH_RPC_URL),
      polygon: new ethers.providers.JsonRpcProvider(process.env.POLYGON_RPC_URL),
      bsc: new ethers.providers.JsonRpcProvider(process.env.BSC_RPC_URL),
    };
  }

  async sendTransaction(chainId, to, data, value = "0x0") {
    try {
      const provider = this.getProvider(chainId);
      const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider);

      const tx = await wallet.sendTransaction({
        to,
        data,
        value,
        gasLimit: 500000,
      });

      return await tx.wait();
    } catch (error) {
      console.error("Error sending EVM transaction:", error);
      throw error;
    }
  }

  getProvider(chainId) {
    const chainMap = {
      1: "ethereum",
      137: "polygon",
      56: "bsc",
    };

    const chainName = chainMap[chainId];
    return this.providers[chainName];
  }
}
```

## 🚀 **Usage Example**

```javascript
// Example usage in a Next.js page
import { FusionPlusRelayer } from "../lib/relayer.js";

const relayer = new FusionPlusRelayer();

// Phase 1: Announce order
const order = {
  salt: 12345,
  maker: "0x1234...",
  receiver: "0x1234...",
  makerAsset: "0xICP...",
  takerAsset: "0xUSDC...",
  makingAmount: "1000000000",
  takingAmount: "100000000",
  secretHash: "0xabc...",
  makerTraits: 0,
  chainIdSrc: 1,
  chainIdDst: 137,
};

const signature = "0x..."; // EIP-712 signature
const secret = "my-secret-key";

const result = await relayer.announceOrder(order, signature, secret);
console.log("Order announced:", result);
```

## 📦 **Package.json Dependencies**

```json
{
  "name": "fusion-plus-relayer",
  "version": "1.0.0",
  "dependencies": {
    "next": "^13.0.0",
    "react": "^18.0.0",
    "ethers": "^5.7.0",
    "crypto": "^1.0.1"
  },
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start"
  }
}
```

This JavaScript/Next.js implementation provides a simple, practical approach for integrating with 1inch's existing infrastructure while maintaining the core Fusion+ functionality!
