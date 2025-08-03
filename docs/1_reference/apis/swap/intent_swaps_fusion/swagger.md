# 1inch Intent Swaps (Fusion) API - Swagger Overview

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/intent-swaps-fusion/swagger)_

---

## Overview

The 1inch Intent Swaps (Fusion) API provides endpoints for managing gasless, intent-based swaps with MEV protection through Dutch auctions. This document provides an overview of all available endpoints organized by category.

> **Note**: This documentation contains only the endpoint names and descriptions from the 1inch portal. The actual endpoint paths, parameters, response schemas, and code examples are not available and would need to be obtained from the official 1inch documentation.

---

## API Categories

### **Orders**

Endpoints for managing gasless swap orders and their lifecycle:

- **GET** Get gasless swap active orders
- **GET** Get actual settlement contract address
- **GET** OrderApiController_getOrderByOrderHash_v2.0
- **POST** Get orders by hashes
- **GET** OrderApiController_getOrdersByMaker_v2.0

### **Quoter**

Endpoints for price discovery and quote generation:

- **GET** Get quote details based on input data
- **POST** Get quote with custom preset details

### **Relayer**

Endpoints for order submission and execution:

- **POST** Submit a limit order that resolvers will be able to fill
- **POST** Submit a list of limit orders which resolvers will be able to fill

---

## Base URL

All endpoints use the base URL: `https://api.1inch.dev`

---

## Authentication

**Authorization - API KEY**

All endpoints require API key authentication. Sign in for automatic API key authentication.

---

## Common Response Codes

| Code  | Description                               |
| ----- | ----------------------------------------- |
| `200` | Success                                   |
| `400` | Bad Request - Input data is invalid       |
| `401` | Unauthorized - Invalid or missing API key |
| `403` | Forbidden - Insufficient permissions      |
| `429` | Too Many Requests - Rate limit exceeded   |
| `500` | Internal Server Error                     |

---

## Rate Limits

The Intent Swaps API implements rate limiting to ensure fair usage:

- **Free Tier**: Limited requests per minute
- **Paid Tier**: Higher rate limits based on subscription
- **Enterprise**: Custom rate limits available

Rate limit headers are included in responses:

- `X-RateLimit-Limit`: Maximum requests per window
- `X-RateLimit-Remaining`: Remaining requests in current window
- `X-RateLimit-Reset`: Time when the rate limit resets

---

## Key Features

### **Gasless Execution**

- Users don't pay gas fees upfront
- Resolvers cover all gas costs
- MEV protection through Dutch auctions

### **Intent-Based Orders**

- Users sign off-chain intents
- Orders are broadcast to resolver network
- Dutch auction ensures competitive pricing

### **Partial Fill Support**

- Large orders can be split into multiple parts
- Different resolvers can fill different portions
- Optimizes execution efficiency

---

## Detailed Documentation

For detailed information about specific endpoints, refer to the individual endpoint documentation:

- **[Get Gasless Swap Active Orders](order_get-gasless-swap-active-orders.md)** - Detailed documentation for the active orders endpoint

---

## SDK and Tools

1inch provides official SDKs and tools for integrating with the Intent Swaps API:

- **Fusion SDK** - Official SDK for Fusion integration
- **Solana SDK** - Specialized SDK for Solana integration
- **EVM SDK** - SDK for Ethereum Virtual Machine chains
- **Swagger UI** - Interactive API documentation
- **Code Examples** - Language-specific implementation examples

---

## Error Handling

### Error Response Format

```json
{
  "error": {
    "code": "string",
    "message": "string",
    "details": "object"
  }
}
```

### Common Error Scenarios

- **Invalid Parameters**: Check parameter types and required fields
- **Authentication Issues**: Verify API key is valid and properly included
- **Rate Limiting**: Implement exponential backoff for rate limit errors
- **Network Issues**: Handle timeouts and connection errors gracefully

---

## Best Practices

### **API Usage**

- Implement proper error handling and retry logic
- Use pagination for large result sets
- Cache responses when appropriate
- Monitor rate limits and implement backoff strategies

### **Order Management**

- Monitor order status regularly
- Implement proper cancellation logic
- Use partial fills for large orders
- Monitor Dutch auction progress

### **Security**

- Never expose API keys in client-side code
- Use HTTPS for all API communications
- Validate all input parameters
- Implement proper input sanitization

---

## Migration Information

For information about migrating between API versions, refer to the [Migration Guide](migration/v1.0-to-v2.0.md).

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
