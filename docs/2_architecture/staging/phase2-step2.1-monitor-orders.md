# Phase 2 Step 2.1: Monitor Orders

_Resolver monitors 1inch API for new orders to fill_

---

## Overview

Resolver needs to monitor the 1inch Fusion+ API for new ICP orders that they can fill.

---

## Required Inputs

### **From Phase 1:**

- **Order hash** (unique identifier)
- **Order status** (active/pending)
- **Secret hash** (for escrow creation)

---

## Monitoring Approaches

### **✅ Option 1: Manual Monitoring** _(MVP)_

- **Process:** Resolver manually checks API endpoint
- **Tool:** `curl` commands to check active orders
- **Frequency:** Manual checks when needed
- **Advantage:** Simple, no automation needed
- **Disadvantage:** Requires manual intervention

### **✅ Option 2: Automated Polling** _(Stretch Goal)_

- **Process:** Script automatically polls API
- **Tool:** Scheduled scripts or services
- **Frequency:** Regular intervals (e.g., every 30 seconds)
- **Advantage:** Automated, catches orders quickly
- **Disadvantage:** Requires automation setup

### **✅ Option 3: Webhook Integration** _(Stretch Goal)_

- **Process:** 1inch API notifies resolver of new orders
- **Tool:** Webhook endpoint for notifications
- **Frequency:** Real-time notifications
- **Advantage:** Instant notification, most efficient
- **Disadvantage:** Requires webhook infrastructure

---

## API Endpoints

### **Option 1: Intent Swaps (Fusion) API**

```
GET /fusion/orders/v2.0/{chain}/order/active
```

- **Purpose:** Get gasless swap active orders
- **Parameters:** `chain`, `page`, `limit`, `version`
- **Documentation:** [order_get-active-orders.md](docs/apis/swap/intent_swaps_fusion/endpoints/order_get-active-orders.md)

### **Option 2: Fusion+ API**

```
GET /fusion-plus/orders/v1.0/order/active
```

- **Purpose:** Get cross-chain swap active orders
- **Parameters:** `page`, `limit`, `srcChain`, `dstChain`
- **Documentation:** [order_get-cross-chain-swap-active-orders.md](docs/apis/swap/fusion-plus/order_get-cross-chain-swap-active-orders.md)

### **Which to Use:**

- **For ICP integration:** Likely Fusion+ API (cross-chain)
- **For testing:** Intent Swaps API (single chain)
- **Need to verify:** Which API supports ICP orders

---

## Outputs

### **For Step 2.2:**

- **Order details** (amounts, tokens, timelock)
- **Secret hash** (for escrow creation)
- **Order parameters** (for escrow setup)

---

## MVP Recommendation

### **Manual Monitoring:**

- **Resolver uses `curl`** to check active orders
- **Manual process** - no automation needed
- **Simple and working** for MVP demo
