# 1inch Fusion+ API - Swagger Overview

_Source: [1inch Developer Portal](https://portal.1inch.dev/documentation/apis/swap/fusion-plus/swagger)_

---

## Overview

The 1inch Fusion+ API provides endpoints for managing cross-chain swap orders, escrow contracts, and order lifecycle operations. This document provides an overview of all available endpoints organized by category.

---

## API Categories

### **Orders**

Endpoints for managing cross-chain swap orders and their lifecycle:

- **GET** `/fusion-plus/orders/v1.0/order/active` - Get cross chain swap active orders
- **GET** `/fusion-plus/orders/v1.0/escrow-factory` - Get actual escrow factory contract address
- **GET** `/fusion-plus/orders/v1.0/orders/{makerAddress}` - Get orders by maker address
- **GET** `/fusion-plus/orders/v1.0/order/{orderHash}/withdrawal-data` - Get all data to perform withdrawal and cancellation
- **GET** `/fusion-plus/orders/v1.0/order/{orderHash}/secrets` - Get idx of each secret that is ready for submission for specific order
- **GET** `/fusion-plus/orders/v1.0/secrets` - Get idx of each secret that is ready for submission for all orders
- **GET** `/fusion-plus/orders/v1.0/public-periods` - Get all data to perform a cancellation or withdrawal on public periods
- **GET** `/fusion-plus/orders/v1.0/order/{orderHash}` - Get order by hash
- **POST** `/fusion-plus/orders/v1.0/orders` - Get orders by hashes

### **Quoter**

Endpoints for price discovery and quote generation:

- **GET** `/fusion-plus/quoter/v1.0/quote` - Get quote details based on input data
- **POST** `/fusion-plus/quoter/v1.0/quote` - Get quote with custom preset details
- **POST** `/fusion-plus/quoter/v1.0/order` - Build order by given quote

### **Relayer**

Endpoints for order submission and execution:

- **POST** `/fusion-plus/relayer/v1.0/order` - Submit a cross chain order that resolvers will be able to fill
- **POST** `/fusion-plus/relayer/v1.0/orders` - Submit many cross chain orders that resolvers will be able to fill
- **POST** `/fusion-plus/relayer/v1.0/secret` - Submit a secret for order fill after SrcEscrow and DstEscrow deployed and DstChain finality lock passed

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

The Fusion+ API implements rate limiting to ensure fair usage:

- **Free Tier**: Limited requests per minute
- **Paid Tier**: Higher rate limits based on subscription
- **Enterprise**: Custom rate limits available

Rate limit headers are included in responses:

- `X-RateLimit-Limit`: Maximum requests per window
- `X-RateLimit-Remaining`: Remaining requests in current window
- `X-RateLimit-Reset`: Time when the rate limit resets

---

## Detailed Documentation

For detailed information about specific endpoints, refer to the individual endpoint documentation:

- **[Get Cross Chain Swap Active Orders](order_get-cross-chain-swap-active-orders.md)** - Detailed documentation for the active orders endpoint

---

## SDK and Tools

1inch provides official SDKs and tools for integrating with the Fusion+ API:

- **Fusion+ SDK** - Official SDK for Fusion+ integration
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

### **Security**

- Never expose API keys in client-side code
- Use HTTPS for all API communications
- Validate all input parameters
- Implement proper input sanitization

### **Performance**

- Use appropriate pagination parameters
- Implement caching for frequently accessed data
- Monitor API response times
- Optimize request frequency based on your use case

---

_This documentation is based on the 1inch Developer Portal and may be updated. For the most current information, please refer to the official documentation._
