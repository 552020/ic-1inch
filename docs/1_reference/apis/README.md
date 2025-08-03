# 1inch API Documentation

This folder contains a copy of the official 1inch API documentation.

**Purpose:** Since 1inch doesn't provide a public repository or easy way to inject their API documentation into LLM context, this is a local copy for reference during development.

## API Endpoints Overview

### **Cross-Chain Swaps (Fusion+)**

- `GET /fusion-plus/orders/v1.0/order/active` - Get cross chain swap active orders
- `GET /fusion-plus/orders/v1.0/order/escrow` - Get actual escrow factory contract address
- `GET /fusion-plus/orders/v1.0/order/{orderHash}/auction/status` - Check Dutch auction price and availability

### **Intent Swaps (Fusion)**

- `GET /fusion/orders/v2.0/{chain}/order/active` - Get gasless swap active orders
- `POST /fusion/orders/v2.0/{chain}/order/submit` - Submit limit order for resolvers
- `GET /fusion/quoter/v2.0/{chain}/quote` - Get quote details based on input data

**Contents:**

- Official 1inch Fusion+ API endpoints
- Swagger documentation
- API usage examples

**Source:** Official 1inch API documentation
