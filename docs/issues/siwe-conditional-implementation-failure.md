# SIWE Conditional Implementation Failure

## Problem Statement

We attempted to implement a conditional SIWE (Sign-In With Ethereum) setup that would use SIWE authentication when available and fall back to anonymous identity when not. This approach was intended to make the frontend flexible - working both with and without SIWE authentication.

## Initial Approach

The user suggested this pattern:

```typescript
import { useSiwe } from "ic-siwe-js/react";
import { AnonymousIdentity } from "@dfinity/agent";

export default function Actors({ children }: { children: ReactNode }) {
  const siwe = useSiwe(); // ✅ always call the hook
  const identity = siwe?.identity ?? new AnonymousIdentity(); // ✅ fallback

  return (
    <ActorProvider<_SERVICE>
      canisterId={canisterId}
      context={actorContext}
      identity={identity}
      idlFactory={idlFactory}
    >
      {children}
    </ActorProvider>
  );
}
```

## Why This Failed

### 1. **`useSiwe()` Throws Error When Provider Missing**

The `useSiwe()` hook from `ic-siwe-js/react` does not gracefully return `undefined` when the `SiweIdentityProvider` is missing from the component tree. Instead, it throws an error:

```
Uncaught Error: useSiwe must be used within a SiweIdentityProvider
```

This means the hook internally checks for the provider context and throws if not found, making the conditional approach impossible.

### 2. **React Rules of Hooks Enforcement**

React's Rules of Hooks are enforced at runtime, and the `useSiwe()` hook is designed to fail fast when used incorrectly. Unlike some other hooks that might return `undefined` or `null` when their context is missing, `useSiwe()` explicitly throws an error.

### 3. **Library Design Intent**

The `ic-siwe-js` library is designed to require explicit setup of the `SiweIdentityProvider`. This is intentional for security reasons - SIWE authentication should be explicitly configured rather than silently falling back to anonymous access.

### 4. **TypeScript Linter Errors**

When implementing the "correct pattern" that calls `useSiwe()` unconditionally, we encountered TypeScript linter errors:

```
Line 39: Unnecessary optional chain on a non-nullish value.
Line 40: Unnecessary conditional, expected left-hand side of `??` operator to be possibly null or undefined.
Line 40: Unnecessary optional chain on a non-nullish value.
```

These errors indicate that TypeScript's type system recognizes that `useSiwe()` will always throw an error when the provider is missing, rather than returning a potentially undefined value. The linter is essentially telling us that the optional chaining (`?.`) and nullish coalescing (`??`) operators are unnecessary because the function will never return a value to check.

This reinforces the point that `useSiwe()` is designed to fail fast and cannot be used conditionally, even with proper React hooks patterns.

## Alternative Solutions

### Option 1: Remove SIWE Completely

```typescript
import { AnonymousIdentity } from "@dfinity/agent";

export default function Actors({ children }: { children: ReactNode }) {
  const identity = new AnonymousIdentity();

  return (
    <ActorProvider<_SERVICE>
      canisterId={canisterId}
      context={actorContext}
      identity={identity}
      idlFactory={idlFactory}
    >
      {children}
    </ActorProvider>
  );
}
```

### Option 2: Proper SIWE Setup

Set up the complete SIWE provider chain in `main.tsx`:

```typescript
import { SiweIdentityProvider } from "ic-siwe-js/react";

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <SiweIdentityProvider canisterId={canisterId}>
      <QueryClientProvider client={queryClient}>
        <Actors>
          <App />
          <Toaster />
        </Actors>
      </QueryClientProvider>
    </SiweIdentityProvider>
  </React.StrictMode>
);
```

### Option 3: Environment-Based Configuration

Use environment variables to conditionally render SIWE:

```typescript
const useSIWE = process.env.REACT_APP_USE_SIWE === "true";

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    {useSIWE ? (
      <SiweIdentityProvider canisterId={canisterId}>
        <App />
      </SiweIdentityProvider>
    ) : (
      <App />
    )}
  </React.StrictMode>
);
```

## Lessons Learned

1. **React Hooks Are Not Always Graceful**: Some hooks are designed to fail fast when used incorrectly, especially authentication-related hooks.

2. **Library Design Matters**: The `ic-siwe-js` library is designed for explicit setup, not conditional usage.

3. **Security Implications**: Authentication should typically be explicitly configured rather than silently falling back to anonymous access.

4. **TypeScript Type System**: The linter errors reveal that TypeScript understands the hook's behavior - it will always throw rather than return undefined.

5. **Alternative Patterns**: When conditional authentication is needed, consider:
   - Environment-based configuration
   - Feature flags
   - Separate authentication providers
   - Explicit setup/teardown patterns

## Current Status

For the MVP, we've opted to use `AnonymousIdentity` without SIWE to simplify the setup and focus on core functionality. SIWE can be added later when proper authentication is required.
