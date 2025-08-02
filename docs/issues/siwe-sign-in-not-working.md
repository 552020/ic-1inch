# SIWE Sign-In Not Working

## Issue Description

The SIWE (Sign-In with Ethereum) sign-in button is not working properly. Users can connect their wallet successfully, but when they click "Sign in", the authentication process fails or doesn't complete as expected.

## Expected Sign-In Flow

When a user clicks the "Sign in" button, the following should happen:

### 1. **Preparation Phase**

- Button shows "Preparing" state with loading spinner
- SIWE provider prepares the sign-in message
- Message includes domain, URI, chain ID, and other parameters

### 2. **Signature Request**

- MetaMask (or wallet) prompts user to sign a SIWE message
- Message format follows EIP-4361 standard:

  ```
  127.0.0.1 wants you to sign in with your Ethereum account:
  0x742d35Cc6634C0532925a3b8D39c5A6b3b6e4D4e

  Login to the Fusion+ Mechanical Turk demo app

  URI: http://127.0.0.1:5174
  Version: 1
  Chain ID: 8453
  Nonce: [random nonce]
  Issued At: [timestamp]
  Expiration Time: [timestamp + 5 minutes]
  ```

### 3. **Authentication Phase**

- Button shows "Signing in" state with loading spinner
- Signed message is sent to SIWE provider canister
- Canister verifies the signature and creates a session
- Delegation is created linking ETH address to ICP principal

### 4. **Success State**

- User is authenticated and can access the application
- Identity is available via `useSiwe().identity`
- User can interact with ICP canisters using their ETH-derived identity

## Current State

### ✅ Working Components

- ✅ Wallet connection (MetaMask integration)
- ✅ SIWE provider canister deployed (`ufxgi-4p777-77774-qaadq-cai`)
- ✅ Environment variables properly configured
- ✅ Frontend can access canister ID
- ✅ Sign-in button renders and is clickable

### ❌ Issues to Investigate

- ❌ Sign-in process completion
- ❌ SIWE message generation
- ❌ Signature verification
- ❌ Session creation
- ❌ Identity establishment

## Debug Information Added

Added comprehensive logging to `LoginButton.tsx`:

```typescript
// State logging
console.log("🔍 LoginButton Debug:", {
  isConnected,
  chainId,
  isChainIdSupported: isChainIdSupported(chainId),
  isLoggingIn,
  isPreparingLogin,
  loginError: loginError?.message,
  hasIdentity: !!identity,
});

// Click handler logging
const handleLogin = async () => {
  console.log("🚀 Sign-in button clicked!");
  console.log("🔍 Pre-login state:", { ... });

  try {
    console.log("🔄 Calling login()...");
    await login();
    console.log("✅ Login completed successfully");
  } catch (error) {
    console.error("❌ Login failed:", error);
  }
};
```

## Investigation Steps

### 1. Check Browser Console

Look for debug messages when clicking sign-in:

- `🚀 Sign-in button clicked!`
- `🔄 Calling login()...`
- `✅ Login completed successfully` OR `❌ Login failed:`

### 2. Check SIWE Provider Configuration

Verify the SIWE provider canister is properly configured:

```bash
dfx canister call ic_siwe_provider get_settings
```

### 3. Check Network Connectivity

Ensure the frontend can communicate with the local replica:

```bash
curl -X POST http://127.0.0.1:4943/api/v2/canister/ufxgi-4p777-77774-qaadq-cai/query
```

### 4. Check Wallet Connection

Verify MetaMask is connected to the correct network (Base/8453 or local testnet).

## Potential Root Causes

### 1. **SIWE Provider Configuration Issues**

- Incorrect domain/URI in canister settings
- Wrong chain ID configuration
- Missing or incorrect targets in canister init

### 2. **Network/Connectivity Issues**

- Frontend can't reach ICP replica
- CORS issues with canister calls
- Network mismatch between wallet and application

### 3. **Signature/Verification Issues**

- SIWE message format incorrect
- Signature verification failing in canister
- Nonce or timestamp validation issues

### 4. **Session/Identity Issues**

- Session creation failing
- Delegation not being established
- Identity not being stored correctly

## Expected Debug Output

### Successful Flow:

```
🔍 LoginButton Debug: { isConnected: true, chainId: 8453, ... }
🚀 Sign-in button clicked!
🔍 Pre-login state: { isConnected: true, disabled: false, ... }
🔄 Calling login()...
✅ Login completed successfully
🔍 LoginButton Debug: { hasIdentity: true, ... }
```

### Failed Flow:

```
🔍 LoginButton Debug: { isConnected: true, chainId: 8453, ... }
🚀 Sign-in button clicked!
🔍 Pre-login state: { isConnected: true, disabled: false, ... }
🔄 Calling login()...
❌ Login failed: [Error details]
🔍 LoginButton Debug: { loginError: "Error message", ... }
```

## Next Steps

1. **Test sign-in flow** with debug logging enabled
2. **Analyze console output** to identify failure point
3. **Check SIWE provider canister** configuration and status
4. **Verify network connectivity** between frontend and replica
5. **Test with different wallets/networks** if needed

## Related Files

- `src/frontend/src/components/layout/LoginButton.tsx` - Sign-in button with debug logging
- `src/frontend/src/components/layout/LoginPage.tsx` - Login page container
- `src/frontend/src/contexts/AuthSync.tsx` - Authentication synchronization
- `src/frontend/src/main.tsx` - SIWE provider setup
- `scripts/mechanical-turk/deploy-siwe-provider.sh` - SIWE deployment script

## Priority

**High** - Authentication is required for users to interact with the application and test the mechanical turk flow.
