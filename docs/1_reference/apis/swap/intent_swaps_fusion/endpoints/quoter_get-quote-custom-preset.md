# 1inch Intent Swaps (Fusion) API - Get Quote with Custom Preset

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/swagger)_

---

## Overview

This endpoint retrieves quote details for a gasless swap with custom Dutch auction parameters. It allows users to specify their own auction duration, amounts, and curve points instead of using predefined presets (fast/medium/slow).

## API Endpoint

#### **POST** `/fusion/quoter/v2.0/{chain}/quote/receive`

**Get quote with custom preset details**

Retrieves comprehensive quote information with user-defined Dutch auction parameters, providing maximum flexibility for custom swap strategies.

**Base URL:** `https://api.1inch.dev`

**Endpoint:** `POST /fusion/quoter/v2.0/{chain}/quote/receive`

---

## Parameters

| Parameter                | Type    | Required | Description                                                                                       | Example                                      |
| ------------------------ | ------- | -------- | ------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| `chain`                  | number  | Yes      | Chain ID (path parameter)                                                                         | `1`                                          |
| `fromTokenAddress`       | string  | Yes      | Address of "FROM" token                                                                           | `0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2` |
| `toTokenAddress`         | string  | Yes      | Address of "TO" token                                                                             | `0x6b175474e89094c44da98b954eedeac495271d0f` |
| `amount`                 | number  | Yes      | Amount to take from "FROM" token to get "TO" token                                                | `100000`                                     |
| `walletAddress`          | string  | Yes      | An address of the wallet or contract who will create Fusion order                                 | `0x0000000000000000000000000000000000000000` |
| `enableEstimate`         | boolean | Yes      | If enabled then get estimation from 1inch swap builder and generates quoteId, by default is false | `false`                                      |
| `fee`                    | number  | No       | Fee in bps format, 1% is equal to 100bps                                                          | `100`                                        |
| `showDestAmountMinusFee` | object  | Yes      | -                                                                                                 | -                                            |
| `isPermit2`              | string  | No       | Permit2 allowance transfer encoded call                                                           | `0x`                                         |
| `surplus`                | boolean | Yes      | -                                                                                                 | -                                            |
| `permit`                 | string  | No       | Permit, user approval sign                                                                        | `0x`                                         |
| `source`                 | object  | No       | -                                                                                                 | -                                            |

---

## Request Body

**Content Type:** `application/json`

### Request Body Schema

#### `CustomPresetInput`

```json
{
  "auctionDuration": 0,
  "auctionStartAmount": 0,
  "auctionEndAmount": 0,
  "points": ["string"]
}
```

### Request Body Fields

| Field                | Type    | Required | Description                                        |
| -------------------- | ------- | -------- | -------------------------------------------------- |
| `auctionDuration`    | number  | Yes      | Duration of the Dutch auction in seconds           |
| `auctionStartAmount` | integer | Yes      | Starting amount for the auction                    |
| `auctionEndAmount`   | integer | Yes      | Final amount after the auction completes           |
| `points`             | array   | Yes      | Array of auction curve points defining price decay |

---

## Response Codes

| Code      | Description                                                              |
| --------- | ------------------------------------------------------------------------ |
| `200`     | Returns slippage, quoteId and presets with custom preset details as well |
| `400`     | Input data is invalid                                                    |
| `default` | Other error responses                                                    |

---

## Response Schema

#### `GetQuoteOutput`

```json
{
  "quoteId": "object",
  "fromTokenAmount": "string",
  "toTokenAmount": "string",
  "feeToken": "string",
  "fee": {
    "receiver": "string",
    "bps": "number",
    "whitelistDiscountPercent": "number",
    "integratorFee": "number"
  },
  "presets": {
    "fast": {
      /* Standard fast preset */
    },
    "medium": {
      /* Standard medium preset */
    },
    "slow": {
      /* Standard slow preset */
    },
    "custom": {
      "bankFee": "string",
      "auctionDuration": "number",
      "startAuctionIn": "number",
      "initialRateBump": "number",
      "auctionStartAmount": "string",
      "startAmount": "string",
      "auctionEndAmount": "string",
      "exclusiveResolver": "object",
      "tokenFee": "string",
      "estP": "number",
      "points": [
        {
          "delay": "number",
          "coefficient": "number"
        }
      ],
      "allowPartialFills": "boolean",
      "allowMultipleFills": "boolean",
      "gasCost": {
        "gasBumpEstimate": "number",
        "gasPriceEstimate": "string"
      }
    }
  },
  "settlementAddress": "string",
  "whitelist": ["string"],
  "recommended_preset": "string (Enum: fast, medium, slow, custom)",
  "suggested": "boolean",
  "prices": {
    "usd": {
      "fromToken": "string",
      "toToken": "string"
    }
  },
  "volume": {
    "usd": {
      "fromToken": "string",
      "toToken": "string"
    }
  },
  "surplusFee": "number"
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
  const url = "https://api.1inch.dev/fusion/quoter/v2.0/1/quote/receive";

  const config = {
    headers: undefined,
    params: {
      fromTokenAddress: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
      toTokenAddress: "0x6b175474e89094c44da98b954eedeac495271d0f",
      amount: "100000",
      walletAddress: "0x0000000000000000000000000000000000000000",
      enableEstimate: "false",
      fee: "100",
      isPermit2: "0x",
      permit: "0x",
    },
    paramsSerializer: {
      indexes: null,
    },
  };

  const body = {
    auctionDuration: 0,
    auctionStartAmount: 0,
    auctionEndAmount: 0,
    points: ["string"],
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
    url = "https://api.1inch.dev/fusion/quoter/v2.0/1/quote/receive"

    params = {
        "fromTokenAddress": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "toTokenAddress": "0x6b175474e89094c44da98b954eedeac495271d0f",
        "amount": "100000",
        "walletAddress": "0x0000000000000000000000000000000000000000",
        "enableEstimate": "false",
        "fee": "100",
        "isPermit2": "0x",
        "permit": "0x"
    }

    body = {
        "auctionDuration": 0,
        "auctionStartAmount": 0,
        "auctionEndAmount": 0,
        "points": ["string"]
    }

    headers = {
        "Content-Type": "application/json",
        "Authorization": "Bearer YOUR_API_KEY"
    }

    try:
        response = requests.post(url, json=body, params=params, headers=headers)
        print(response.json())
    except Exception as error:
        print(error)
```

#### cURL

```bash
curl -X POST "https://api.1inch.dev/fusion/quoter/v2.0/1/quote/receive?fromTokenAddress=0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount=100000&walletAddress=0x0000000000000000000000000000000000000000&enableEstimate=false&fee=100&isPermit2=0x&permit=0x" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "auctionDuration": 0,
    "auctionStartAmount": 0,
    "auctionEndAmount": 0,
    "points": ["string"]
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

type CustomPresetInput struct {
    AuctionDuration   int      `json:"auctionDuration"`
    AuctionStartAmount int     `json:"auctionStartAmount"`
    AuctionEndAmount   int     `json:"auctionEndAmount"`
    Points            []string `json:"points"`
}

func httpCall() {
    url := "https://api.1inch.dev/fusion/quoter/v2.0/1/quote/receive?fromTokenAddress=0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount=100000&walletAddress=0x0000000000000000000000000000000000000000&enableEstimate=false&fee=100&isPermit2=0x&permit=0x"

    body := CustomPresetInput{
        AuctionDuration:   0,
        AuctionStartAmount: 0,
        AuctionEndAmount:   0,
        Points:            []string{"string"},
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

## Data Models

### `CustomPresetInput`

| Field                | Type    | Required | Description                                        |
| -------------------- | ------- | -------- | -------------------------------------------------- |
| `auctionDuration`    | number  | Yes      | Duration of the Dutch auction in seconds           |
| `auctionStartAmount` | integer | Yes      | Starting amount for the auction                    |
| `auctionEndAmount`   | integer | Yes      | Final amount after the auction completes           |
| `points`             | array   | Yes      | Array of auction curve points defining price decay |

### `GetQuoteOutput`

| Field                | Type              | Description                            |
| -------------------- | ----------------- | -------------------------------------- |
| `quoteId`            | object            | Unique identifier for the quote        |
| `fromTokenAmount`    | string            | Amount of source tokens                |
| `toTokenAmount`      | string            | Expected amount of destination tokens  |
| `feeToken`           | string            | Token used for fee payment             |
| `fee`                | ResolverFee       | Fee structure and configuration        |
| `presets`            | QuotePresetsClass | Dutch auction presets including custom |
| `settlementAddress`  | string            | Contract address for order settlement  |
| `whitelist`          | array             | List of whitelisted addresses          |
| `recommended_preset` | string            | Recommended auction preset             |
| `suggested`          | boolean           | Whether this is a suggested quote      |
| `prices`             | TokenPairValue    | USD price information                  |
| `volume`             | TokenPairValue    | Trading volume information             |
| `surplusFee`         | number            | Surplus fee amount                     |

---

## Workflow Integration

### **Custom Preset Quote Flow:**

1. **Define Custom Parameters** → Set auction duration, amounts, curve points
2. **Get Custom Quote** → **This endpoint** with custom preset input
3. **Review Custom Preset** → Check the `custom` preset in response
4. **Sign Intent** → User signs order with custom parameters
5. **Submit Order** → Use submit endpoint with custom quoteId

### **Key Differences from Standard Quote:**

- **Custom auction duration** instead of predefined presets
- **User-defined auction amounts** for start/end points
- **Custom curve points** for price decay behavior
- **Flexible auction strategy** for advanced users

---

## Usage Notes

### **Custom Preset Configuration:**

#### **Auction Duration**

- **Short duration**: Quick execution, higher competition
- **Long duration**: More time for price discovery, potentially better rates

#### **Auction Amounts**

- **Start amount**: Initial price offered to resolvers
- **End amount**: Final price after auction decay
- **Curve points**: Define how price decreases over time

#### **Curve Points**

- **Delay**: Time offset for each point
- **Coefficient**: Price multiplier at each point
- **Custom strategy**: Implement complex price decay patterns

### **When to Use Custom Presets:**

- **Large orders**: Require specific timing and pricing
- **Advanced strategies**: Custom auction curves
- **Institutional users**: Need precise control over execution
- **Special requirements**: Non-standard auction parameters

### **Integration with Standard Presets:**

- Response includes **all presets** (fast, medium, slow, custom)
- **Compare custom** with standard presets
- **Choose optimal** strategy based on requirements

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
