# GET - Get Actual Escrow Factory Contract Address

_Get the current escrow factory contract address for a specific chain_

---

## Overview

This endpoint returns the actual escrow factory contract address for a given chain. Resolvers need this address to create escrows on the destination chain.

---

## Endpoint

```
GET /fusion-plus/orders/v1.0/order/escrow
```

---

## Parameters

### **Query Parameters:**

| Parameter | Type     | Required | Description                        | Example        |
| --------- | -------- | -------- | ---------------------------------- | -------------- |
| `chainId` | `number` | ✅ Yes   | Chain ID for the target blockchain | `1` (Ethereum) |

---

## Request Example

### **cURL:**

```bash
curl -X GET "https://api.1inch.dev/fusion-plus/orders/v1.0/order/escrow?chainId=1" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

### **Node.js:**

```javascript
const axios = require("axios");

async function getEscrowFactoryAddress() {
  const url = "https://api.1inch.dev/fusion-plus/orders/v1.0/order/escrow";

  const config = {
    headers: {
      Authorization: "Bearer YOUR_API_KEY",
    },
    params: {
      chainId: "1",
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

def get_escrow_factory_address():
    url = "https://api.1inch.dev/fusion-plus/orders/v1.0/order/escrow"

    headers = {
        "Authorization": "Bearer YOUR_API_KEY"
    }

    params = {
        "chainId": "1"
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
  "address": "0x1234567890123456789012345678901234567890"
}
```

### **Response Schema:**

| Field     | Type     | Description                                                 |
| --------- | -------- | ----------------------------------------------------------- |
| `address` | `string` | The escrow factory contract address for the specified chain |

---

## Error Responses

### **400 Bad Request:**

```json
{
  "error": "Invalid chain ID"
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
  "error": "Escrow factory not found for chain"
}
```

---

## Usage in Implementation

### **When Resolver Needs Escrow Factory:**

1. **Before creating destination escrow** - Resolver calls this endpoint
2. **Chain-specific addresses** - Different chains have different factory addresses
3. **Dynamic updates** - Factory addresses may change, so always fetch current address

### **Integration with Phase 2:**

```javascript
// Step 2.2: Create Ethereum Escrow
const escrowFactoryAddress = await getEscrowFactoryAddress(chainId);
const escrowContract = await createEscrow(escrowFactoryAddress, params);
```

---

## Supported Chains

| Chain ID | Chain Name       | Factory Address |
| -------- | ---------------- | --------------- |
| `1`      | Ethereum Mainnet | Dynamic         |
| `137`    | Polygon          | Dynamic         |
| `56`     | BSC              | Dynamic         |
| `42161`  | Arbitrum One     | Dynamic         |
| `10`     | Optimism         | Dynamic         |

---

## Notes

- **Dynamic addresses** - Factory addresses may change, always fetch current address
- **Chain-specific** - Each chain has its own escrow factory contract
- **Resolver requirement** - Resolvers need this address to create escrows
- **No caching** - Don't cache this address as it may change
