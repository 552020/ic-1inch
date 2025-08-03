# 1inch Intent Swaps (Fusion) API - Get Quote Details

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/swagger)_

---

## Overview

This endpoint retrieves quote details for a gasless swap, including pricing information, auction presets, and Dutch auction parameters. It's the first step in the Intent Swaps (Fusion) workflow, providing the necessary data to create a signed order.

## API Endpoint

#### **GET** `/fusion/quoter/v2.0/{chain}/quote/receive`

**Get quote details based on input data**

Retrieves comprehensive quote information including token amounts, fees, auction presets, and Dutch auction configuration for gasless swaps.

**Base URL:** `https://api.1inch.dev`

**Endpoint:** `GET /fusion/quoter/v2.0/{chain}/quote/receive`

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
| `slippage`               | object  | No       | -                                                                                                 | -                                            |
| `source`                 | object  | No       | -                                                                                                 | -                                            |

---

## Response Codes

| Code      | Description                               |
| --------- | ----------------------------------------- |
| `200`     | Returns presets with slippage and quoteId |
| `400`     | Input data is invalid                     |
| `default` | Other error responses                     |

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
    },
    "medium": {
      /* Same structure as fast */
    },
    "slow": {
      /* Same structure as fast */
    },
    "custom": {
      /* Same structure as fast */
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

    headers = {
        "Authorization": "Bearer YOUR_API_KEY"
    }

    try:
        response = requests.get(url, params=params, headers=headers)
        print(response.json())
    except Exception as error:
        print(error)
```

#### cURL

```bash
curl -X GET "https://api.1inch.dev/fusion/quoter/v2.0/1/quote/receive?fromTokenAddress=0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount=100000&walletAddress=0x0000000000000000000000000000000000000000&enableEstimate=false&fee=100&isPermit2=0x&permit=0x" \
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
    url := "https://api.1inch.dev/fusion/quoter/v2.0/1/quote/receive?fromTokenAddress=0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount=100000&walletAddress=0x0000000000000000000000000000000000000000&enableEstimate=false&fee=100&isPermit2=0x&permit=0x"

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

### `GetQuoteOutput`

| Field                | Type              | Description                                        |
| -------------------- | ----------------- | -------------------------------------------------- |
| `quoteId`            | object            | Unique identifier for the quote                    |
| `fromTokenAmount`    | string            | Amount of source tokens                            |
| `toTokenAmount`      | string            | Expected amount of destination tokens              |
| `feeToken`           | string            | Token used for fee payment                         |
| `fee`                | ResolverFee       | Fee structure and configuration                    |
| `presets`            | QuotePresetsClass | Dutch auction presets (fast, medium, slow, custom) |
| `settlementAddress`  | string            | Contract address for order settlement              |
| `whitelist`          | array             | List of whitelisted addresses                      |
| `recommended_preset` | string            | Recommended auction preset                         |
| `suggested`          | boolean           | Whether this is a suggested quote                  |
| `prices`             | TokenPairValue    | USD price information                              |
| `volume`             | TokenPairValue    | Trading volume information                         |
| `surplusFee`         | number            | Surplus fee amount                                 |

### `ResolverFee`

| Field                      | Type   | Description                               |
| -------------------------- | ------ | ----------------------------------------- |
| `receiver`                 | string | Fee recipient address                     |
| `bps`                      | number | Fee in basis points                       |
| `whitelistDiscountPercent` | number | Discount percentage for whitelisted users |
| `integratorFee`            | number | Fee for integrators                       |

### `PresetClass`

| Field                | Type               | Description                          |
| -------------------- | ------------------ | ------------------------------------ |
| `bankFee`            | string             | Bank fee amount                      |
| `auctionDuration`    | number             | Duration of Dutch auction in seconds |
| `startAuctionIn`     | number             | Delay before auction starts          |
| `initialRateBump`    | number             | Initial rate increase percentage     |
| `auctionStartAmount` | string             | Starting amount for auction          |
| `startAmount`        | string             | Initial swap amount                  |
| `auctionEndAmount`   | string             | Final amount after auction           |
| `exclusiveResolver`  | object             | Exclusive resolver configuration     |
| `tokenFee`           | string             | Token fee amount                     |
| `estP`               | number             | Estimated probability                |
| `points`             | array              | Auction curve points                 |
| `allowPartialFills`  | boolean            | Whether partial fills are allowed    |
| `allowMultipleFills` | boolean            | Whether multiple fills are allowed   |
| `gasCost`            | GasCostConfigClass | Gas cost configuration               |

### `AuctionPointClass`

| Field         | Type   | Description                      |
| ------------- | ------ | -------------------------------- |
| `delay`       | number | Time delay in seconds            |
| `coefficient` | number | Price coefficient for this point |

### `GasCostConfigClass`

| Field              | Type   | Description              |
| ------------------ | ------ | ------------------------ |
| `gasBumpEstimate`  | number | Estimated gas price bump |
| `gasPriceEstimate` | string | Estimated gas price      |

---

## Workflow Integration

### **Complete Quote-to-Order Flow:**

1. **Get Quote** → **This endpoint** provides pricing and auction parameters
2. **Select Preset** → Choose fast/medium/slow/custom auction settings
3. **Sign Intent** → User signs order with wallet (EIP-712)
4. **Submit Order** → Use submit endpoint with quoteId and signature
5. **Monitor Status** → Track order through Orders API

### **Key Integration Points:**

- **`quoteId`** from response is required for order submission
- **`presets`** determine Dutch auction behavior
- **`settlementAddress`** is used in order creation
- **`fee`** structure affects order parameters

---

## Usage Notes

### **Quote Generation Process:**

1. **Specify tokens** and amounts
2. **Set wallet address** for order creation
3. **Choose auction preset** (fast/medium/slow/custom)
4. **Review pricing** and fees
5. **Use quoteId** for order submission

### **Auction Presets:**

- **Fast**: Quick execution, higher fees
- **Medium**: Balanced speed and cost
- **Slow**: Lower fees, longer wait time
- **Custom**: User-defined parameters

### **Fee Structure:**

- **Basis points (bps)**: 1% = 100 bps
- **Whitelist discounts**: Available for eligible users
- **Integrator fees**: For platform integrations

### **Chain Support:**

- Specify chain ID in the URL path
- Example: `/fusion/quoter/v2.0/1/quote/receive` for Ethereum mainnet

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
