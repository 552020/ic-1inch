# 1inch Fusion+ API - Get Cross Chain Swap Active Orders

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/fusion-plus/swagger)_

---

## Overview

This endpoint retrieves a list of active cross-chain swap orders with pagination support. It's part of the Orders category in the 1inch Fusion+ API.

## API Endpoint

#### **GET** `/fusion-plus/orders/v1.0/order/active`

**Get cross chain swap active orders**

Retrieves a list of active cross-chain swap orders with pagination support.

**Base URL:** `https://api.1inch.dev`

**Endpoint:** `GET /fusion-plus/orders/v1.0/order/active`

---

## Parameters

| Parameter  | Type   | Required | Description                                                 | Example |
| ---------- | ------ | -------- | ----------------------------------------------------------- | ------- |
| `page`     | number | No       | Pagination step, default: 1 (page = offset / limit)         | `1`     |
| `limit`    | number | No       | Number of active orders to receive (default: 100, max: 500) | `100`   |
| `srcChain` | number | No       | Source chain of cross chain                                 | `1`     |
| `dstChain` | number | No       | Destination chain of cross chain                            | `137`   |

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
      "deadline": "number",
      "auctionStartDate": "number",
      "auctionEndDate": "number",
      "quoteId": "string",
      "remainingMakerAmount": "string",
      "makerBalance": "string",
      "makerAllowance": "string",
      "isMakerContract": "boolean",
      "extension": "string",
      "srcChainId": "number",
      "dstChainId": "number",
      "order": {
        "salt": "string",
        "maker": "string",
        "receiver": "string",
        "makerAsset": "string",
        "takerAsset": "string",
        "makingAmount": "string",
        "takingAmount": "string",
        "makerTraits": "string",
        "secretHashes": ["string"],
        "fills": ["string"]
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
  const url = "https://api.1inch.dev/fusion-plus/orders/v1.0/order/active";

  const config = {
    headers: undefined,
    params: {},
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
    url = "https://api.1inch.dev/fusion-plus/orders/v1.0/order/active"

    config = {
        "headers": None,
        "params": {},
        "paramsSerializer": {
            "indexes": None,
        },
    }

    try:
        response = requests.get(url, **config)
        print(response.json())
    except Exception as error:
        print(error)
```

#### cURL

```bash
curl -X GET "https://api.1inch.dev/fusion-plus/orders/v1.0/order/active" \
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
    url := "https://api.1inch.dev/fusion-plus/orders/v1.0/order/active"

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

| Field                  | Type               | Description                                   |
| ---------------------- | ------------------ | --------------------------------------------- |
| `orderHash`            | string             | Unique identifier for the order               |
| `signature`            | string             | Cryptographic signature of the order          |
| `deadline`             | number             | Timestamp when the order expires              |
| `auctionStartDate`     | number             | Start time of the Dutch auction               |
| `auctionEndDate`       | number             | End time of the Dutch auction                 |
| `quoteId`              | string             | Reference to the quote used for this order    |
| `remainingMakerAmount` | string             | Amount of maker tokens remaining to be filled |
| `makerBalance`         | string             | Current balance of maker tokens               |
| `makerAllowance`       | string             | Approved allowance for the order              |
| `isMakerContract`      | boolean            | Whether the maker is a smart contract         |
| `extension`            | string             | Additional order parameters                   |
| `srcChainId`           | number             | Source blockchain network ID                  |
| `dstChainId`           | number             | Destination blockchain network ID             |
| `order`                | CrossChainOrderDto | Complete order details                        |

### `CrossChainOrderDto`

| Field          | Type     | Description                              |
| -------------- | -------- | ---------------------------------------- |
| `salt`         | string   | Random salt for order uniqueness         |
| `maker`        | string   | Address of the order maker               |
| `receiver`     | string   | Address to receive the swapped tokens    |
| `makerAsset`   | string   | Token address being offered              |
| `takerAsset`   | string   | Token address being requested            |
| `makingAmount` | string   | Amount of maker tokens to swap           |
| `takingAmount` | string   | Amount of taker tokens to receive        |
| `makerTraits`  | string   | Additional maker-specific parameters     |
| `secretHashes` | string[] | Array of secret hashes for partial fills |
| `fills`        | string[] | Array of fill amounts                    |

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
