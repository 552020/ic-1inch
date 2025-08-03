# 1inch API Endpoints Overview

_Comprehensive overview of all documented 1inch API endpoints for ICP integration_

---

## Overview

This document provides a complete overview of all 1inch API endpoints documented in our internal APIs directory. These endpoints are essential for implementing cross-chain swaps between ICP and other blockchains using the 1inch Fusion+ protocol.

---

## API Categories

### **1. Cross-Chain Swaps (Fusion+)**

The Fusion+ API enables cross-chain atomic swaps between different blockchains.

#### **Orders Endpoints:**

| Method | Endpoint                                                    | Description                                | Status        |
| ------ | ----------------------------------------------------------- | ------------------------------------------ | ------------- |
| `GET`  | `/fusion-plus/orders/v1.0/order/active`                     | Get cross chain swap active orders         | ✅ Documented |
| `GET`  | `/fusion-plus/orders/v1.0/order/escrow`                     | Get actual escrow factory contract address | ✅ Documented |
| `GET`  | `/fusion-plus/orders/v1.0/order/{orderHash}/auction/status` | Check Dutch auction price and availability | ✅ Documented |

#### **Quoter Endpoints:**

| Method | Endpoint                                       | Description                           | Status     |
| ------ | ---------------------------------------------- | ------------------------------------- | ---------- |
| `GET`  | `/fusion-plus/quoter/v1.0/quote`               | Get quote details based on input data | ⚠️ Missing |
| `POST` | `/fusion-plus/quoter/v1.0/quote/custom-preset` | Get quote with custom preset details  | ⚠️ Missing |

#### **Relayer Endpoints:**

| Method | Endpoint                                       | Description                            | Status     |
| ------ | ---------------------------------------------- | -------------------------------------- | ---------- |
| `POST` | `/fusion-plus/relayer/v1.0/order/submit`       | Submit cross chain order for resolvers | ⚠️ Missing |
| `POST` | `/fusion-plus/relayer/v1.0/order/submit-batch` | Submit multiple cross chain orders     | ⚠️ Missing |
| `POST` | `/fusion-plus/relayer/v1.0/secret/submit`      | Submit secret for order execution      | ⚠️ Missing |

### **2. Intent Swaps (Fusion)**

The Fusion API enables gasless swaps on single chains with Dutch auction mechanics.

#### **Orders Endpoints:**

| Method | Endpoint                                         | Description                      | Status        |
| ------ | ------------------------------------------------ | -------------------------------- | ------------- |
| `GET`  | `/fusion/orders/v2.0/{chain}/order/active`       | Get gasless swap active orders   | ✅ Documented |
| `GET`  | `/fusion/orders/v2.0/{chain}/order/{orderHash}`  | Get order by hash                | ⚠️ Missing    |
| `POST` | `/fusion/orders/v2.0/{chain}/order/submit`       | Submit limit order for resolvers | ✅ Documented |
| `POST` | `/fusion/orders/v2.0/{chain}/order/submit-batch` | Submit multiple limit orders     | ⚠️ Missing    |

#### **Quoter Endpoints:**

| Method | Endpoint                                          | Description                           | Status        |
| ------ | ------------------------------------------------- | ------------------------------------- | ------------- |
| `GET`  | `/fusion/quoter/v2.0/{chain}/quote`               | Get quote details based on input data | ✅ Documented |
| `POST` | `/fusion/quoter/v2.0/{chain}/quote/custom-preset` | Get quote with custom preset details  | ✅ Documented |
| `POST` | `/fusion/quoter/v2.0/{chain}/order/build`         | Build order by given quote            | ⚠️ Missing    |

#### **Settlement Endpoints:**

| Method | Endpoint                                   | Description                            | Status     |
| ------ | ------------------------------------------ | -------------------------------------- | ---------- |
| `GET`  | `/fusion/settlement/v2.0/{chain}/contract` | Get actual settlement contract address | ⚠️ Missing |

---

## Implementation Status

### **✅ Fully Documented Endpoints:**

1. **Cross-Chain Swap Active Orders** - Essential for monitoring orders
2. **Escrow Factory Address** - Required for creating escrows
3. **Auction Status** - Critical for resolver workflow
4. **Gasless Swap Active Orders** - For single-chain monitoring
5. **Limit Order Submission** - For submitting orders
6. **Quote Details** - For pricing information
7. **Custom Preset Quotes** - For advanced pricing strategies

### **⚠️ Missing Documentation:**

1. **Cross-Chain Quote Endpoints** - Needed for pricing cross-chain swaps
2. **Relayer Submission Endpoints** - Critical for resolver bidding
3. **Secret Submission Endpoints** - Required for order execution
4. **Order Details Endpoints** - For getting specific order information
5. **Settlement Contract Endpoints** - For single-chain execution

---

## Endpoint Usage by Phase

### **Phase 1: Announcement**

| Endpoint                                      | Purpose                  | User     |
| --------------------------------------------- | ------------------------ | -------- |
| `GET /fusion-plus/quoter/v1.0/quote`          | Get cross-chain pricing  | Maker    |
| `POST /fusion-plus/relayer/v1.0/order/submit` | Submit cross-chain order | Maker    |
| `GET /fusion-plus/orders/v1.0/order/active`   | Monitor for new orders   | Resolver |

### **Phase 2: Deposit**

| Endpoint                                                        | Purpose              | User     |
| --------------------------------------------------------------- | -------------------- | -------- |
| `GET /fusion-plus/orders/v1.0/order/{orderHash}/auction/status` | Check if won auction | Resolver |
| `GET /fusion-plus/orders/v1.0/order/escrow`                     | Get factory address  | Resolver |
| `POST /fusion-plus/relayer/v1.0/order/submit`                   | Submit resolver bid  | Resolver |

### **Phase 3: Execution**

| Endpoint                                       | Purpose                     | User     |
| ---------------------------------------------- | --------------------------- | -------- |
| `POST /fusion-plus/relayer/v1.0/secret/submit` | Submit secret for execution | Resolver |

---

## Authentication

All endpoints require **API Key authentication** via Bearer token:

```bash
Authorization: Bearer YOUR_API_KEY
```

---

## Base URLs

- **Production:** `https://api.1inch.dev`
- **Testnet:** `https://testnet.api.1inch.dev` (if available)

---

## Rate Limits

- **Standard:** 100 requests per minute
- **Premium:** 1000 requests per minute
- **Enterprise:** Custom limits

---

## Error Handling

All endpoints return standard HTTP status codes:

- **200** - Success
- **400** - Bad Request
- **401** - Unauthorized
- **404** - Not Found
- **429** - Rate Limited
- **500** - Server Error

---

## Next Steps

### **Priority 1 - Critical Missing Endpoints:**

1. **Cross-Chain Quote Endpoints** - Essential for pricing
2. **Relayer Submission Endpoints** - Required for resolver bidding
3. **Secret Submission Endpoints** - Needed for execution

### **Priority 2 - Nice to Have:**

1. **Order Details Endpoints** - For better monitoring
2. **Settlement Contract Endpoints** - For single-chain operations
3. **Batch Submission Endpoints** - For efficiency

---

## Notes

- **Cross-chain endpoints** are the primary focus for ICP integration
- **Single-chain endpoints** are useful for testing and development
- **Resolver endpoints** are critical for the Dutch auction workflow
- **Quote endpoints** are essential for pricing and user experience
