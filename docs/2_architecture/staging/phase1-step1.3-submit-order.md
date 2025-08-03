# Phase 1 Step 1.3: Submit Order

_All possible approaches for submitting signed orders to 1inch Fusion+ API_

---

## Overview

Step 1.3 involves submitting the signed order (from Step 1.2) to the 1inch Fusion+ API. This step broadcasts the order to the resolver network and **initiates a Dutch auction on 1inch servers** for competitive resolver selection.

### **Dutch Auction Process**

After order submission, the following happens on 1inch servers:

1. **Order Broadcast:** The signed order is shared with all authorized resolvers
2. **Dutch Auction:** 1inch servers run a Dutch auction among resolvers
3. **Resolver Selection:** The winning resolver gets exclusive rights to fill the order
4. **Order Assignment:** The selected resolver receives the order details for execution

**⚠️ Important:** The Dutch auction is handled entirely by 1inch infrastructure - our implementation only needs to submit orders and monitor for assignments.

---

## Required Inputs

### **From Step 1.2 (Sign Intent):**

- **Signed order JSON** (complete structure with signature)
- **Order data** (maker, taker, amounts, etc.)
- **EIP-712 signature** (cryptographic proof)
- **QuoteId** (reference to original quote)

### **Additional Requirements:**

- **API key** (1inch authentication)
- **Network endpoint** (chain-specific API)
- **HTTP client** (for API communication)

---

## Submission Approaches

### **✅ Option 1: Command-Line Tools** _(Recommended for MVP)_

#### **curl Commands:**

- **Tool:** Standard `curl` command
- **Input:** Signed order JSON file
- **Output:** API response (order hash, status)
- **Authentication:** Bearer token in headers
- **Advantage:** Simple, universal, no dependencies
- **Disadvantage:** Manual process, limited error handling

#### **HTTPie:**

- **Tool:** `httpie` command-line HTTP client
- **Input:** Signed order JSON file
- **Output:** Formatted API response
- **Authentication:** Bearer token in headers
- **Advantage:** Better formatting, easier debugging
- **Disadvantage:** Additional dependency

#### **wget:**

- **Tool:** `wget` command-line tool
- **Input:** Signed order JSON file
- **Output:** API response file
- **Authentication:** Bearer token in headers
- **Advantage:** Available on most systems
- **Disadvantage:** Less user-friendly than curl

### **✅ Option 2: Scripting Languages**

#### **Node.js with axios/fetch:**

- **Library:** `axios` or native `fetch`
- **Input:** Signed order object
- **Output:** API response object
- **Authentication:** Bearer token in headers
- **Advantage:** JavaScript ecosystem, good error handling
- **Disadvantage:** Requires Node.js runtime

#### **Python with requests:**

- **Library:** `requests` HTTP library
- **Input:** Signed order dictionary
- **Output:** API response object
- **Authentication:** Bearer token in headers
- **Advantage:** Python ecosystem, excellent error handling
- **Disadvantage:** Requires Python runtime

#### **Go with http client:**

- **Library:** Standard `net/http` package
- **Input:** Signed order struct
- **Output:** API response struct
- **Authentication:** Bearer token in headers
- **Advantage:** Compiled binary, fast execution
- **Disadvantage:** Requires Go compilation

### **✅ Option 3: Backend Services**

#### **API Gateway:**

- **Architecture:** Centralized API gateway
- **Input:** Signed order via internal API
- **Output:** API response via internal API
- **Authentication:** Internal service authentication
- **Advantage:** Centralized management, monitoring
- **Disadvantage:** Additional infrastructure layer

#### **Microservice:**

- **Architecture:** Dedicated order submission service
- **Input:** Signed order via API call
- **Output:** API response via API call
- **Authentication:** Service-to-service authentication
- **Advantage:** Isolated responsibility, scalable
- **Disadvantage:** More complex architecture

### **✅ Option 4: SDK Integration**

#### **1inch SDK:**

- **Library:** Official 1inch SDK
- **Input:** Signed order object
- **Output:** API response object
- **Authentication:** SDK handles authentication
- **Advantage:** Official support, best practices
- **Disadvantage:** SDK dependency, version management

#### **Web3 Libraries:**

- **Library:** Web3.js, ethers.js with HTTP capabilities
- **Input:** Signed order object
- **Output:** API response object
- **Authentication:** Bearer token in headers
- **Advantage:** Web3 ecosystem integration
- **Disadvantage:** Additional complexity

### **✅ Option 5: Automation Tools**

#### **CI/CD Pipelines:**

- **Platform:** GitHub Actions, GitLab CI, Jenkins
- **Input:** Signed order from previous step
- **Output:** API response and status
- **Authentication:** Secure environment variables
- **Advantage:** Automated testing, deployment integration
- **Disadvantage:** Requires CI/CD setup

#### **Scheduled Jobs:**

- **Platform:** Cron jobs, systemd timers
- **Input:** Signed order from file/database
- **Output:** API response to file/database
- **Authentication:** System environment variables
- **Advantage:** Automated execution, monitoring
- **Disadvantage:** Requires scheduling setup

---

## API Endpoint Details

### **Endpoint:**

```
POST /fusion/relayer/v2.0/{chain}/order/submit
```

### **Base URL:**

```
https://api.1inch.dev
```

### **Authentication:**

```
Authorization: Bearer {API_KEY}
```

### **Content-Type:**

```
application/json
```

---

## Request Structure

### **Complete Request Body:**

```json
{
  "order": {
    "salt": "string",
    "makerAsset": "string",
    "takerAsset": "string",
    "maker": "string",
    "receiver": "string",
    "makingAmount": "string",
    "takingAmount": "string",
    "makerTraits": "string"
  },
  "signature": "string",
  "extension": "0x",
  "quoteId": "string"
}
```

### **Required Fields:**

- **order:** Complete order data structure
- **signature:** EIP-712 signature
- **extension:** Additional parameters (default: "0x")
- **quoteId:** Reference to original quote

---

## Response Handling

### **Success Response (201):**

```json
{
  "statusCode": 201,
  "message": "Order submitted successfully"
}
```

### **Error Response (400):**

```json
{
  "statusCode": 400,
  "message": "string",
  "error": "Bad Request"
}
```

### **Common Error Scenarios:**

- **Invalid signature** → Re-sign order
- **Expired quote** → Get new quote
- **Invalid order data** → Validate and fix
- **Network errors** → Retry with backoff

---

## Validation Requirements

### **Pre-Submission Validation:**

- **Signature verification** (optional double-check)
- **Quote ID validation** (matches original quote)
- **Order data integrity** (no tampering)
- **Timestamp validation** (quote not expired)

### **Post-Submission Validation:**

- **Response status code** (201 for success)
- **Order hash generation** (if provided)
- **Error message parsing** (if failure)
- **Network confirmation** (order broadcast)

---

## Error Handling Strategies

### **Retry Logic:**

- **Exponential backoff** for network errors
- **Maximum retry attempts** (3-5 times)
- **Timeout handling** (30-60 seconds)
- **Circuit breaker** for repeated failures

### **Fallback Options:**

- **Alternative endpoints** (if available)
- **Different submission methods** (curl → script)
- **Manual submission** (last resort)
- **Error reporting** (logging, monitoring)

---

## Integration with Phase 2

### **Output for Phase 2:**

- **Order hash** (unique identifier)
- **Order status** (active/pending)
- **Submission timestamp** (for monitoring)
- **Network confirmation** (broadcast success)

### **Dutch Auction Results:**

- **Resolver assignment** (if our resolver wins the auction)
- **Order details** (amounts, tokens, timelock for escrow creation)
- **Secret hash** (for escrow creation)
- **Auction timeout** (if no resolver accepts the order)

### **Monitoring Requirements:**

- **Order status tracking** (active → filled/expired)
- **Resolver activity** (when resolvers pick up order)
- **Timeout handling** (if order expires)
- **Success confirmation** (when swap completes)

---

## MVP Recommendation

### **Phase 1: Command-Line Tools**

- **curl commands** for simple submission
- **JSON file input** from Step 1.2
- **Basic error handling** and validation
- **Manual monitoring** of order status

### **Phase 2: Scripting Integration** _(Stretch Goal)_

- **Node.js/Python scripts** for automation
- **Integrated workflow** (Step 1.1 → 1.2 → 1.3)
- **Enhanced error handling** and retry logic
- **Automated monitoring** and status tracking

### **Phase 3: Backend Services** _(Stretch Goal)_

- **API gateway** for centralized management
- **Production monitoring** and alerting
- **Scalable architecture** for high volume
- **Advanced error handling** and recovery

---

## Security Considerations

### **API Key Management:**

- **Environment variables** for development
- **Secure storage** (vault, HSM) for production
- **Key rotation** policies
- **Access logging** and monitoring

### **Request Validation:**

- **Input sanitization** (prevent injection)
- **Rate limiting** (prevent abuse)
- **Request signing** (optional additional security)
- **Network security** (HTTPS, VPN)

### **Response Validation:**

- **Response integrity** (verify authenticity)
- **Error message parsing** (avoid information leakage)
- **Status code validation** (handle all cases)
- **Timeout handling** (prevent hanging requests)
