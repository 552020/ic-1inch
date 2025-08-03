# 1inch Intent Swaps (Fusion) API - Submit Limit Order

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/swagger)_

---

## Overview

This endpoint submits a signed gasless limit order that resolvers will be able to fill. It's the primary endpoint for creating new intent-based swap orders in the 1inch Intent Swaps (Fusion) API.

## API Endpoint

#### **POST** `/fusion/relayer/v2.0/{chain}/order/submit`

**Submit a limit order that resolvers will be able to fill**

Creates a new gasless swap order by submitting a signed intent. The order is broadcast to the resolver network and enters a Dutch auction for competitive filling.

**Base URL:** `https://api.1inch.dev`

**Endpoint:** `POST /fusion/relayer/v2.0/{chain}/order/submit`

---

## Parameters

| Parameter | Type   | Required | Description               | Example |
| --------- | ------ | -------- | ------------------------- | ------- |
| `chain`   | number | Yes      | Chain ID (path parameter) | `1`     |

---

## Request Body

**Content Type:** `application/json`

### Request Body Schema

#### `SignedOrderInput`

```json
{
  "order": {
    "salt": "string",
    "makerAsset": "string",
    "takerAsset": "string",
    "maker": "string",
    "receiver": "0x0000000000000000000000000000000000000000",
    "makingAmount": "string",
    "takingAmount": "string",
    "makerTraits": "0"
  },
  "signature": "string",
  "extension": "0x",
  "quoteId": "string"
}
```

### Request Body Fields

| Field       | Type       | Required | Description                                 |
| ----------- | ---------- | -------- | ------------------------------------------- |
| `order`     | OrderInput | Yes      | Complete order details                      |
| `signature` | string     | Yes      | EIP-712 signature of the order              |
| `extension` | string     | Yes      | Additional order parameters (default: "0x") |
| `quoteId`   | string     | Yes      | Reference to the quote used for this order  |

### `OrderInput` Object

| Field          | Type   | Required | Description                                                   |
| -------------- | ------ | -------- | ------------------------------------------------------------- |
| `salt`         | string | Yes      | Random salt for order uniqueness                              |
| `makerAsset`   | string | Yes      | Token address being offered                                   |
| `takerAsset`   | string | Yes      | Token address being requested                                 |
| `maker`        | string | Yes      | Address of the order maker                                    |
| `receiver`     | string | No       | Address to receive the swapped tokens (default: zero address) |
| `makingAmount` | string | Yes      | Amount of maker tokens to swap                                |
| `takingAmount` | string | Yes      | Amount of taker tokens to receive                             |
| `makerTraits`  | string | No       | Additional maker-specific parameters (default: "0")           |

---

## Response Codes

| Code  | Description                                   |
| ----- | --------------------------------------------- |
| `201` | The gasless order has been successfully saved |
| `400` | Input data is invalid                         |

---

## Response Schema

### Success Response (201)

```json
{
  "statusCode": 201,
  "message": "Order submitted successfully"
}
```

### Error Response (400)

```json
{
  "statusCode": 400,
  "message": "string",
  "error": "Bad Request"
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
  const url = "https://api.1inch.dev/fusion/relayer/v2.0/1/order/submit";

  const config = {
    headers: undefined,
    params: {},
    paramsSerializer: {
      indexes: null,
    },
  };

  const body = {
    order: {
      salt: "string",
      makerAsset: "string",
      takerAsset: "string",
      maker: "string",
      receiver: "0x0000000000000000000000000000000000000000",
      makingAmount: "string",
      takingAmount: "string",
      makerTraits: "0",
    },
    signature: "string",
    extension: "0x",
    quoteId: "string",
  };

  try {
    const response = await axios.post(url, body, config);
    console.log(response.data);
  } catch (error) {
    console.error(error);
  }
}
```

#### Python

```python
import requests
import json

def http_call():
    url = "https://api.1inch.dev/fusion/relayer/v2.0/1/order/submit"

    body = {
        "order": {
            "salt": "string",
            "makerAsset": "string",
            "takerAsset": "string",
            "maker": "string",
            "receiver": "0x0000000000000000000000000000000000000000",
            "makingAmount": "string",
            "takingAmount": "string",
            "makerTraits": "0"
        },
        "signature": "string",
        "extension": "0x",
        "quoteId": "string"
    }

    headers = {
        "Content-Type": "application/json",
        "Authorization": "Bearer YOUR_API_KEY"
    }

    try:
        response = requests.post(url, json=body, headers=headers)
        print(response.json())
    except Exception as error:
        print(error)
```

#### cURL

```bash
curl -X POST "https://api.1inch.dev/fusion/relayer/v2.0/1/order/submit" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "order": {
      "salt": "string",
      "makerAsset": "string",
      "takerAsset": "string",
      "maker": "string",
      "receiver": "0x0000000000000000000000000000000000000000",
      "makingAmount": "string",
      "takingAmount": "string",
      "makerTraits": "0"
    },
    "signature": "string",
    "extension": "0x",
    "quoteId": "string"
  }'
```

#### Go

```go
package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "net/http"
    "io/ioutil"
)

type OrderInput struct {
    Salt          string `json:"salt"`
    MakerAsset    string `json:"makerAsset"`
    TakerAsset    string `json:"takerAsset"`
    Maker         string `json:"maker"`
    Receiver      string `json:"receiver"`
    MakingAmount  string `json:"makingAmount"`
    TakingAmount  string `json:"takingAmount"`
    MakerTraits   string `json:"makerTraits"`
}

type SignedOrderInput struct {
    Order     OrderInput `json:"order"`
    Signature string     `json:"signature"`
    Extension string     `json:"extension"`
    QuoteID   string     `json:"quoteId"`
}

func httpCall() {
    url := "https://api.1inch.dev/fusion/relayer/v2.0/1/order/submit"

    body := SignedOrderInput{
        Order: OrderInput{
            Salt:          "string",
            MakerAsset:    "string",
            TakerAsset:    "string",
            Maker:         "string",
            Receiver:      "0x0000000000000000000000000000000000000000",
            MakingAmount:  "string",
            TakingAmount:  "string",
            MakerTraits:   "0",
        },
        Signature: "string",
        Extension: "0x",
        QuoteID:   "string",
    }

    jsonBody, err := json.Marshal(body)
    if err != nil {
        fmt.Println(err)
        return
    }

    req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonBody))
    if err != nil {
        fmt.Println(err)
        return
    }

    req.Header.Set("Content-Type", "application/json")
    req.Header.Set("Authorization", "Bearer YOUR_API_KEY")

    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        fmt.Println(err)
        return
    }
    defer resp.Body.Close()

    bodyBytes, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        fmt.Println(err)
        return
    }

    fmt.Println(string(bodyBytes))
}
```

---

## Workflow Integration

### **Complete Order Lifecycle:**

1. **Quote Generation** → Use Quoter endpoints to get pricing
2. **Intent Signing** → User signs order with wallet (EIP-712)
3. **Order Submission** → **This endpoint** submits signed order
4. **Order Broadcasting** → Order appears in active orders list
5. **Dutch Auction** → Resolvers compete to fill the order
6. **Order Execution** → Winning resolver executes the swap

### **Integration with Other Endpoints:**

- **Before**: Use Quoter endpoints to get `quoteId`
- **After**: Use Orders endpoints to monitor order status
- **Monitoring**: Use "Get active orders" to track order lifecycle

---

## Usage Notes

### **Order Creation Process:**

1. **Generate quote** using Quoter API
2. **Sign order** using EIP-712 standard
3. **Submit order** using this endpoint
4. **Monitor status** using Orders API

### **Signature Requirements:**

- Must be valid EIP-712 signature
- Must match the order data exactly
- Must be signed by the maker address

### **Chain Support:**

- Specify chain ID in the URL path
- Example: `/fusion/relayer/v2.0/1/order/submit` for Ethereum mainnet

### **Error Handling:**

- Validate all required fields before submission
- Ensure signature matches order data
- Check quoteId validity

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
