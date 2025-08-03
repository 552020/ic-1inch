# GET - Get Auction Status for Resolver

_Check if a resolver won a specific auction for an order_

---

## Overview

This endpoint allows resolvers to check the current status of a Dutch auction for a specific order. Since Dutch auctions work by price decreasing over time, resolvers need to monitor when the price becomes profitable and race to execute on-chain.

---

## Endpoint

```
GET /fusion-plus/orders/v1.0/order/{orderHash}/auction/status
```

---

## Parameters

### **Path Parameters:**

| Parameter   | Type     | Required | Description                               | Example     |
| ----------- | -------- | -------- | ----------------------------------------- | ----------- |
| `orderHash` | `string` | ✅ Yes   | Hash of the order to check auction status | `0x1234...` |

### **Query Parameters:**

| Parameter         | Type     | Required | Description                             | Example     |
| ----------------- | -------- | -------- | --------------------------------------- | ----------- |
| `resolverAddress` | `string` | ✅ Yes   | Address of the resolver checking status | `0x5678...` |

---

## Request Example

### **cURL:**

```bash
curl -X GET "https://api.1inch.dev/fusion-plus/orders/v1.0/order/0x1234567890123456789012345678901234567890/auction/status?resolverAddress=0x5678901234567890123456789012345678901234" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

### **Node.js:**

```javascript
const axios = require("axios");

async function getAuctionStatus(orderHash, resolverAddress) {
  const url = `https://api.1inch.dev/fusion-plus/orders/v1.0/order/${orderHash}/auction/status`;

  const config = {
    headers: {
      Authorization: "Bearer YOUR_API_KEY",
    },
    params: {
      resolverAddress: resolverAddress,
    },
  };

  try {
    const response = await axios.get(url, config);
    console.log(response.data);
  } catch (error) {
    console.error(error);
  }
}
```

### **Python:**

```python
import requests

def get_auction_status(order_hash, resolver_address):
    url = f"https://api.1inch.dev/fusion-plus/orders/v1.0/order/{order_hash}/auction/status"

    headers = {
        "Authorization": "Bearer YOUR_API_KEY"
    }

    params = {
        "resolverAddress": resolver_address
    }

    try:
        response = requests.get(url, headers=headers, params=params)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")
        return None
```

---

## Response

### **Success Response (200 OK):**

```json
{
  "orderHash": "0x1234567890123456789012345678901234567890",
  "resolverAddress": "0x5678901234567890123456789012345678901234",
  "auctionStatus": "WON",
  "winningBid": "1000000000000000000",
  "auctionEndTime": "1640995200",
  "orderDetails": {
    "maker": "0x1234567890123456789012345678901234567890",
    "makingAmount": "1000000000000000000",
    "takingAmount": "2000000000000000000",
    "makerAsset": "0xA0b86a33E6441b8c4C8C0C8C0C8C0C8C0C8C0C8C",
    "takerAsset": "0xB0b86a33E6441b8c4C8C0C8C0C8C0C8C0C8C0C8C",
    "secretHashes": ["0x1234567890123456789012345678901234567890"]
  },
  "executionDeadline": "1640998800"
}
```

### **Response Schema:**

| Field               | Type     | Description                                 |
| ------------------- | -------- | ------------------------------------------- |
| `orderHash`         | `string` | Hash of the order                           |
| `resolverAddress`   | `string` | Address of the resolver                     |
| `auctionStatus`     | `string` | Status: `WON`, `LOST`, `PENDING`, `EXPIRED` |
| `winningBid`        | `string` | Amount of the winning bid (if won)          |
| `auctionEndTime`    | `number` | Timestamp when auction ended                |
| `orderDetails`      | `object` | Order details for execution                 |
| `executionDeadline` | `number` | Deadline for executing the order            |

---

## Error Responses

### **400 Bad Request:**

```json
{
  "error": "Invalid order hash or resolver address"
}
```

### **401 Unauthorized:**

```json
{
  "error": "Invalid API key"
}
```

### **404 Not Found:**

```json
{
  "error": "Order not found or auction not started"
}
```

### **409 Conflict:**

```json
{
  "error": "Auction still in progress"
}
```

---

## Auction Status Values

| Status       | Description                                                 |
| ------------ | ----------------------------------------------------------- |
| `EXECUTED`   | Order was executed by a resolver (not necessarily this one) |
| `AVAILABLE`  | Order is available for execution at current price           |
| `EXPIRED`    | Auction expired without any resolver executing              |
| `PROFITABLE` | Current price is profitable for this resolver               |

---

## Usage in Implementation

### **When Resolver Checks Auction Status:**

1. **While monitoring orders** - Resolver checks if price is profitable
2. **Before executing order** - Verify order is still available
3. **Periodic checking** - During auction to see if price became profitable

### **Integration with Phase 2:**

```javascript
// Step 2.1: Monitor Orders
const activeOrders = await getActiveOrders();

// Step 2.1.5: Check Auction Status
for (const order of activeOrders) {
  const auctionStatus = await getAuctionStatus(order.orderHash, resolverAddress);

  if (auctionStatus.auctionStatus === "PROFITABLE") {
    // Race to execute the order on-chain
    const executionResult = await executeOrder(order.orderHash);

    if (executionResult.success) {
      // Step 2.2: Get Escrow Factory Address
      const escrowFactoryAddress = await getEscrowFactoryAddress(chainId);

      // Step 2.3: Create Ethereum Escrow
      const escrowContract = await createEscrow(escrowFactoryAddress, orderDetails);
    }
  }
}
```

---

## Notes

- **Real-time price updates** - Price decreases automatically over time
- **Race to execute** - First resolver to confirm on-chain wins
- **Deadline enforcement** - Must execute before auction expires
- **Price curve** - Set by maker (start price, end price, duration)
- **No traditional bidding** - It's a race to execute at profitable price
