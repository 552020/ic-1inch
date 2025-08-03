# 1inch Intent Swaps (Fusion) API - Get Gasless Swap Active Orders

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/swagger)_

---

## Overview

This endpoint retrieves a list of active gasless swap orders with pagination support. It's part of the Orders category in the 1inch Intent Swaps (Fusion) API.

## API Endpoint

#### **GET** `/fusion/orders/v2.0/{chain}/order/active`

**Get gasless swap active orders**

Retrieves a list of active gasless swap orders with pagination support.

**Base URL:** `https://api.1inch.dev`

**Endpoint:** `GET /fusion/orders/v2.0/{chain}/order/active`

---

## Parameters

| Parameter | Type   | Required | Description                                                 | Example |
| --------- | ------ | -------- | ----------------------------------------------------------- | ------- |
| `chain`   | number | Yes      | Chain ID (path parameter)                                   | `1`     |
| `page`    | number | No       | Pagination step, default: 1 (page = offset / limit)         | `1`     |
| `limit`   | number | No       | Number of active orders to receive (default: 100, max: 500) | `100`   |
| `version` | number | No       | Settlement extension version: 2.0 or 2.1. By default: all   | `2.0`   |

---

## Response Codes

| Code      | Description                    |
| --------- | ------------------------------ |
| `200`     | Array of queried active orders |
| `400`     | Input data is invalid          |
| `default` | Other error responses          |

---

## Response Schema

#### `GetActiveOrdersOutput`

```json
{
  "meta": {
    "totalItems": "number",
    "itemsPerPage": "number",
    "totalPages": "number",
    "currentPage": "number"
  },
  "items": [
    {
      "orderHash": "string",
      "signature": "string",
      "deadline": "string",
      "auctionStartDate": "string",
      "auctionEndDate": "string",
      "quoteId": "string",
      "remainingMakerAmount": "string",
      "extension": "string",
      "order": {
        "salt": "string",
        "maker": "string",
        "receiver": "string",
        "makerAsset": "string",
        "takerAsset": "string",
        "makingAmount": "string",
        "takingAmount": "string",
        "makerTraits": "string",
        "version": "string (Enum: 2.0, 2.1, 2.2)"
      }
    }
  ]
}
```

---

## Authentication

**Authorization - API KEY**

Sign in for automatic API key authentication.

---

## Code Examples

#### Node.js

```javascript
const axios = require("axios");

async function httpCall() {
  const url = "https://api.1inch.dev/fusion/orders/v2.0/1/order/active";

  const config = {
    headers: undefined,
    params: {
      page: 1,
      limit: 100,
      version: "2.0",
    },
    paramsSerializer: {
      indexes: null,
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

#### Python

```python
import requests

def http_call():
    url = "https://api.1inch.dev/fusion/orders/v2.0/1/order/active"

    params = {
        "page": 1,
        "limit": 100,
        "version": "2.0"
    }

    try:
        response = requests.get(url, params=params)
        print(response.json())
    except Exception as error:
        print(error)
```

#### cURL

```bash
curl -X GET "https://api.1inch.dev/fusion/orders/v2.0/1/order/active?page=1&limit=100&version=2.0" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

#### Go

```go
package main

import (
    "fmt"
    "net/http"
    "io/ioutil"
)

func httpCall() {
    url := "https://api.1inch.dev/fusion/orders/v2.0/1/order/active?page=1&limit=100&version=2.0"

    req, err := http.NewRequest("GET", url, nil)
    if err != nil {
        fmt.Println(err)
        return
    }

    req.Header.Set("Authorization", "Bearer YOUR_API_KEY")

    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        fmt.Println(err)
        return
    }
    defer resp.Body.Close()

    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        fmt.Println(err)
        return
    }

    fmt.Println(string(body))
}
```

---

## Data Models

### `ActiveOrdersOutput`

| Field                  | Type          | Description                                   |
| ---------------------- | ------------- | --------------------------------------------- |
| `orderHash`            | string        | Unique identifier for the order               |
| `signature`            | string        | Cryptographic signature of the order          |
| `deadline`             | string        | Timestamp when the order expires              |
| `auctionStartDate`     | string        | Start time of the Dutch auction               |
| `auctionEndDate`       | string        | End time of the Dutch auction                 |
| `quoteId`              | string        | Reference to the quote used for this order    |
| `remainingMakerAmount` | string        | Amount of maker tokens remaining to be filled |
| `extension`            | string        | Additional order parameters                   |
| `order`                | FusionOrderV4 | Complete order details                        |

### `FusionOrderV4`

| Field          | Type   | Description                                  |
| -------------- | ------ | -------------------------------------------- |
| `salt`         | string | Random salt for order uniqueness             |
| `maker`        | string | Address of the order maker                   |
| `receiver`     | string | Address to receive the swapped tokens        |
| `makerAsset`   | string | Token address being offered                  |
| `takerAsset`   | string | Token address being requested                |
| `makingAmount` | string | Amount of maker tokens to swap               |
| `takingAmount` | string | Amount of taker tokens to receive            |
| `makerTraits`  | string | Additional maker-specific parameters         |
| `version`      | string | Settlement extension version (2.0, 2.1, 2.2) |

---

## Usage Notes

### **Pagination**

- Use `page` parameter for pagination (default: 1)
- Use `limit` parameter to control results per page (max: 500)
- Response includes metadata with pagination information

### **Version Filtering**

- Use `version` parameter to filter by settlement extension version
- Supported versions: 2.0, 2.1
- Default behavior returns all versions

### **Chain Support**

- Specify chain ID in the URL path
- Example: `/fusion/orders/v2.0/1/order/active` for Ethereum mainnet

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
