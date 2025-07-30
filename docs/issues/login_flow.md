# Login Flow Documentation

This document describes the complete login flow implementation in the Secretus project, which uses Ethereum wallet authentication with the Internet Computer (ICP) platform.

## Overview

The login flow implements **Sign-In with Ethereum (SIWE)** authentication, allowing users to authenticate using their Ethereum wallet and receive an Internet Computer identity. The flow involves connecting an Ethereum wallet, signing a SIWE message, and obtaining a delegated identity for the ICP platform.

## Key Components

### 1. Provider Setup (`main.tsx`)

The application is wrapped with several providers that enable the login flow:

```typescript
// src/frontend/src/main.tsx
<WagmiProvider config={wagmiConfig}>
  <QueryClientProvider client={queryClient}>
    <SiweIdentityProvider canisterId={canisterId}>
      <Actors>
        <AuthGuard>
          <App />
        </AuthGuard>
      </Actors>
    </SiweIdentityProvider>
  </QueryClientProvider>
</WagmiProvider>
```

- **WagmiProvider**: Provides Ethereum wallet connection functionality
- **SiweIdentityProvider**: Manages SIWE authentication state and provides the `useSiwe` hook
- **Actors**: Provides backend actor context for ICP canister interactions
- **AuthGuard**: Protects routes and manages authentication state

### 2. Authentication Guard (`AuthGuard.tsx`)

The `AuthGuard` component manages the authentication state and ensures users are properly authenticated:

```typescript
// src/frontend/src/contexts/AuthGuard.tsx
export default function AuthGuard({ children }: AuthGuardProps) {
  const { isConnected, address } = useAccount();
  const chainId = useChainId();
  const { clear, isInitializing, identity, identityAddress } = useSiwe();

  // Auto-clear session when wallet disconnects
  useEffect(() => {
    if (!isConnected && identity) {
      clear();
    }
  }, [isConnected, clear, identity]);

  // Auto-clear session when switching to unsupported network
  useEffect(() => {
    if (!isChainIdSupported(chainId)) {
      clear();
    }
  }, [chainId, clear]);

  // Auto-clear session when switching Ethereum addresses
  useEffect(() => {
    if (identityAddress && address && address !== identityAddress) {
      clear();
    }
  }, [address, clear, identityAddress]);

  if (isInitializing) return null;
  if (!isConnected || !identity) return <LoginPage />;

  return <>{children}</>;
}
```

**Key Features:**

- Automatically clears session when wallet disconnects
- Clears session when switching to unsupported networks
- Clears session when switching Ethereum addresses
- Shows login page when not authenticated
- Shows main app when authenticated

### 3. Login Page (`LoginPage.tsx`)

The main login interface that guides users through the authentication process:

```typescript
// src/frontend/src/components/login/LoginPage.tsx
export default function LoginPage(): React.ReactElement {
  const { isConnected } = useAccount();
  const chainId = useChainId();
  const { loginError } = useSiwe();
  const { toast } = useToast();

  // Show error toast if login fails
  useEffect(() => {
    if (loginError) {
      toast({
        variant: "destructive",
        description: loginError.message,
      });
    }
  }, [loginError, toast]);

  return (
    <div className="flex flex-col gap-5 w-full h-screen items-center justify-center">
      {/* Step 1: Connect Wallet */}
      <div className="flex items-center justify-center w-full gap-5">
        <div className="items-center justify-center hidden w-8 h-8 text-xl font-bold rounded-full md:flex bg-primary text-primary-foreground">
          1
        </div>
        <div>
          {!isConnected && <ConnectButton />}
          {isConnected && isChainIdSupported(chainId) && <EthBadge />}
          {isConnected && !isChainIdSupported(chainId) && (
            <Button disabled variant="outline">
              <Activity />
              Unsupported Network
            </Button>
          )}
        </div>
      </div>

      {/* Step 2: Sign In */}
      <div className="flex items-center justify-center w-full gap-5">
        <div className="items-center justify-center hidden w-8 h-8 text-xl font-bold rounded-full md:flex bg-primary text-primary-foreground">
          2
        </div>
        <div>
          <LoginButton />
        </div>
      </div>
    </div>
  );
}
```

**Two-Step Process:**

1. **Connect Wallet**: User connects their Ethereum wallet
2. **Sign In**: User signs a SIWE message to authenticate with ICP

### 4. Connect Button (`ConnectButton.tsx`)

Handles the wallet connection process:

```typescript
// src/frontend/src/components/login/ConnectButton.tsx
export default function ConnectButton() {
  const { isConnecting } = useAccount();
  const [connectDialogOpen, setConnectDialogOpen] = useState(false);

  const handleClick = () => {
    if (isConnecting) return;
    setConnectDialogOpen(true);
  };

  return (
    <>
      <Button className="w-44" disabled={isConnecting} onClick={handleClick}>
        {isConnecting ? <LoaderCircle /> : <EthereumIcon />}
        {isConnecting ? "Connecting" : "Connect wallet"}
      </Button>
      <ConnectDialog isOpen={connectDialogOpen} setIsOpen={setConnectDialogOpen} />
    </>
  );
}
```

**Features:**

- Shows loading state while connecting
- Opens wallet selection dialog
- Disabled during connection process

### 5. Connect Dialog (`ConnectDialog.tsx`)

Displays available wallet connectors:

```typescript
// src/frontend/src/components/ConnectDialog.tsx
export default function ConnectDialog({
  isOpen,
  setIsOpen,
}: {
  isOpen: boolean;
  setIsOpen: (isOpen: boolean) => void;
}) {
  const { connect, connectors, error, isPending, variables, reset } = useConnect();
  const { isConnected } = useAccount();

  useEffect(() => {
    if (isOpen) reset();
  }, [isOpen, reset]);

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogContent className="w-64">
        <DialogHeader>Connect Wallet</DialogHeader>
        {connectors.map((connector) => (
          <Button
            className="justify-between w-52"
            disabled={isConnected || isPending}
            key={connector.id}
            onClick={() => connect({ connector })}
            variant="outline"
          >
            {isPending && connector.id === variables.connector?.id ? (
              <LoaderCircle className="animate-spin" />
            ) : undefined}
            {connector.name}
            <img className="w-4 h-4" src={iconSource(connector)} />
          </Button>
        ))}
        {error && <div className="p-2 text-center text-white bg-red-500">{error.message}</div>}
      </DialogContent>
    </Dialog>
  );
}
```

**Features:**

- Lists available wallet connectors (MetaMask, WalletConnect, etc.)
- Shows loading state during connection
- Displays connection errors
- Auto-resets state when dialog opens

### 6. Login Button (`LoginButton.tsx`)

Handles the SIWE authentication process:

```typescript
// src/frontend/src/components/login/LoginButton.tsx
export default function LoginButton() {
  const { isConnected } = useAccount();
  const chainId = useChainId();
  const { login, isLoggingIn, isPreparingLogin } = useSiwe();

  const text = () => {
    if (isLoggingIn) return "Signing in";
    if (isPreparingLogin) return "Preparing";
    return "Sign in";
  };

  const disabled = !isChainIdSupported(chainId) || isLoggingIn || !isConnected || isPreparingLogin;

  return (
    <Button className="w-44" disabled={disabled} onClick={() => void login()}>
      {isLoggingIn || isPreparingLogin ? <LoaderCircle className="animate-spin" /> : undefined}
      {text()}
    </Button>
  );
}
```

**Features:**

- Disabled when wallet not connected
- Disabled on unsupported networks
- Shows loading states during preparation and signing
- Triggers SIWE authentication flow

### 7. Session Management

#### Session Button (`SessionButton.tsx`)

Provides access to session information and logout:

```typescript
// src/frontend/src/components/header/SessionButton.tsx
export default function SessionButton() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      <Button onClick={() => setIsOpen(true)}>
        <User />
      </Button>
      <SessionDialog isOpen={isOpen} setIsOpen={setIsOpen} />
    </>
  );
}
```

#### Session Dialog (`SessionDialog.tsx`)

Displays session information and logout functionality:

```typescript
// src/frontend/src/components/header/SessionDialog.tsx
export default function SessionDialog({ isOpen, setIsOpen }: SessionDialogProps) {
  const { clear, identity, delegationChain } = useSiwe();

  const logout = () => {
    clear();
    setIsOpen(false);
  };

  if (!identity) return null;

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogContent className="w-96">
        <DialogHeader>Session</DialogHeader>
        <div className="px-4 py-2 text-xs rounded-lg bg-muted">
          <pre>
            {delegationChain?.delegations.map((delegation) => {
              const pubKey = arrayBufferToHex(delegation.delegation.pubkey);
              const expiration = new Date(Number(delegation.delegation.expiration / 1000000n));
              return (
                <div key={pubKey}>
                  Internet Identity: {identity.getPrincipal().toString().slice(0, 8)}...
                  {identity.getPrincipal().toString().slice(-8)}
                  <br />
                  Pubkey: {pubKey.slice(0, 8)}...{pubKey.slice(-8)}
                  <br />
                  Expiration: {expiration.toLocaleDateString()} {expiration.toLocaleTimeString()}
                </div>
              );
            })}
          </pre>
        </div>
        <div className="flex gap-3 w-full">
          <Button variant="outline" className="w-full" onClick={() => setIsOpen(false)}>
            Close
          </Button>
          <Button onClick={logout} className="w-full">
            Logout
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
```

**Features:**

- Displays Internet Computer principal
- Shows delegation public key
- Shows session expiration time
- Provides logout functionality

## SIWE Authentication Flow

The authentication process follows the SIWE standard:

### 1. Prepare Login

- Frontend calls `siwe_prepare_login(eth_address)` on the ICP canister
- Canister returns a SIWE message with nonce
- Frontend prompts user to sign the message with their Ethereum wallet

### 2. Login

- User signs the SIWE message with their wallet
- Frontend calls `siwe_login(eth_address, signature, session_identity)` on the canister
- Canister verifies the signature and Ethereum address
- Canister prepares delegation for the next step

### 3. Get Delegation

- Frontend calls `siwe_get_delegation(delegation_expires)` on the canister
- Canister returns a delegation
- Frontend creates a delegation identity for ICP interactions

## Error Handling

The application includes comprehensive error handling:

- **Connection Errors**: Displayed in ConnectDialog
- **Login Errors**: Shown as toast notifications in LoginPage
- **Network Errors**: Auto-clear session when switching to unsupported networks
- **Address Changes**: Auto-clear session when switching Ethereum addresses
- **Delegation Errors**: Handled in Actors context with automatic logout

## Security Features

- **Session Uniqueness**: Each application context has unique session identities
- **Timebound Sessions**: Delegations have expiration times
- **Address Validation**: Ensures Ethereum address matches identity
- **Network Validation**: Only allows supported networks
- **Auto-clear**: Automatically clears sessions on wallet disconnect or network change

## File Structure

```
src/frontend/src/
├── components/
│   ├── login/
│   │   ├── LoginPage.tsx      # Main login interface
│   │   ├── ConnectButton.tsx  # Wallet connection button
│   │   └── LoginButton.tsx    # SIWE authentication button
│   ├── header/
│   │   ├── SessionButton.tsx  # Session access button
│   │   └── SessionDialog.tsx  # Session info and logout
│   └── ConnectDialog.tsx      # Wallet selection dialog
├── contexts/
│   ├── AuthGuard.tsx          # Authentication protection
│   └── Actors.tsx             # Backend actor context
└── main.tsx                   # Provider setup
```

## Dependencies

- **wagmi**: Ethereum wallet connection and interaction
- **ic-siwe-js**: SIWE authentication for Internet Computer
- **@dfinity/agent**: Internet Computer agent functionality
- **react-query**: Data fetching and caching
- **lucide-react**: Icons
- **tailwindcss**: Styling

This login flow provides a secure, user-friendly authentication system that bridges Ethereum wallets with the Internet Computer platform, enabling seamless cross-chain identity management.

## Appendix: Moving Login to Header

If you want to move the login buttons from the `LoginPage` to the header for a more integrated experience, here's how to implement it:

### 1. Create Header Login Component

Create a new component `HeaderLogin.tsx` in the header directory:

```typescript
// src/frontend/src/components/header/HeaderLogin.tsx
import { useAccount, useChainId } from "wagmi";
import { isChainIdSupported } from "@/wagmi/is-chain-id-supported";
import { useSiwe } from "ic-siwe-js/react";
import { useToast } from "@/hooks/use-toast";
import { Button } from "@/components/ui/button";
import { Activity } from "lucide-react";
import ConnectButton from "../login/ConnectButton";
import LoginButton from "../login/LoginButton";
import EthBadge from "../EthBadge";

export default function HeaderLogin() {
  const { isConnected } = useAccount();
  const chainId = useChainId();
  const { loginError } = useSiwe();
  const { toast } = useToast();

  // Show error toast if login fails
  useEffect(() => {
    if (loginError) {
      toast({
        variant: "destructive",
        description: loginError.message,
      });
    }
  }, [loginError, toast]);

  return (
    <div className="flex items-center gap-2">
      {/* Step 1: Connect Wallet */}
      {!isConnected && <ConnectButton />}
      {isConnected && isChainIdSupported(chainId) && <EthBadge />}
      {isConnected && !isChainIdSupported(chainId) && (
        <Button disabled variant="outline" size="sm">
          <Activity className="w-4 h-4" />
          Unsupported Network
        </Button>
      )}

      {/* Step 2: Sign In */}
      {isConnected && isChainIdSupported(chainId) && <LoginButton />}
    </div>
  );
}
```

### 2. Update Header Component

Modify the existing `Header.tsx` to include the login component:

```typescript
// src/frontend/src/components/header/Header.tsx
import SessionButton from "./SessionButton";
import HeaderLogin from "./HeaderLogin";
import { useSiwe } from "ic-siwe-js/react";

export default function Header() {
  const { identity } = useSiwe();

  return (
    <header className="flex items-center justify-between w-full p-4 border-b">
      <div className="flex items-center gap-4">
        <img alt="ic" className="w-8 h-8" src="/icp-logo.png" />
        <h1 className="text-xl font-bold">Secretus</h1>
      </div>

      <div className="flex items-center gap-4">
        {/* Show login buttons when not authenticated */}
        {!identity && <HeaderLogin />}

        {/* Show session button when authenticated */}
        {identity && <SessionButton />}
      </div>
    </header>
  );
}
```

### 3. Update AuthGuard

Modify the `AuthGuard` to show a simplified login page or redirect:

```typescript
// src/frontend/src/contexts/AuthGuard.tsx
export default function AuthGuard({ children }: AuthGuardProps) {
  const { isConnected, address } = useAccount();
  const chainId = useChainId();
  const { clear, isInitializing, identity, identityAddress } = useSiwe();

  // ... existing useEffect hooks ...

  if (isInitializing) {
    return null;
  }

  // If wallet is not connected or there is no identity, show minimal login page
  if (!isConnected || !identity) {
    return (
      <div className="flex flex-col items-center justify-center w-full h-screen gap-10">
        <div className="text-center">
          <h1 className="text-3xl font-bold mb-4">Welcome to Secretus</h1>
          <p className="text-muted-foreground mb-8">
            Connect your Ethereum wallet and sign in to start sending encrypted files
          </p>
          <div className="flex flex-col items-center gap-4">
            <HeaderLogin />
          </div>
        </div>
      </div>
    );
  }

  return <>{children}</>;
}
```

### 4. Alternative: Inline Header Login

For a more compact header, you can inline the login buttons directly:

```typescript
// src/frontend/src/components/header/Header.tsx
import { useAccount, useChainId } from "wagmi";
import { isChainIdSupported } from "@/wagmi/is-chain-id-supported";
import { useSiwe } from "ic-siwe-js/react";
import { Button } from "@/components/ui/button";
import { Activity, User } from "lucide-react";
import ConnectDialog from "../ConnectDialog";
import SessionDialog from "./SessionDialog";
import { useState } from "react";

export default function Header() {
  const { isConnected, isConnecting } = useAccount();
  const chainId = useChainId();
  const { identity, login, isLoggingIn, isPreparingLogin } = useSiwe();
  const [connectDialogOpen, setConnectDialogOpen] = useState(false);
  const [sessionDialogOpen, setSessionDialogOpen] = useState(false);

  const handleConnect = () => {
    if (isConnecting) return;
    setConnectDialogOpen(true);
  };

  const handleLogin = () => {
    if (!isConnected || !isChainIdSupported(chainId) || isLoggingIn || isPreparingLogin) return;
    void login();
  };

  return (
    <header className="flex items-center justify-between w-full p-4 border-b">
      <div className="flex items-center gap-4">
        <img alt="ic" className="w-8 h-8" src="/icp-logo.png" />
        <h1 className="text-xl font-bold">Secretus</h1>
      </div>

      <div className="flex items-center gap-2">
        {!identity ? (
          <>
            {/* Connect Button */}
            <Button size="sm" disabled={isConnecting} onClick={handleConnect}>
              {isConnecting ? "Connecting..." : "Connect"}
            </Button>

            {/* Login Button */}
            {isConnected && isChainIdSupported(chainId) && (
              <Button size="sm" disabled={isLoggingIn || isPreparingLogin} onClick={handleLogin}>
                {isLoggingIn ? "Signing..." : isPreparingLogin ? "Preparing..." : "Sign In"}
              </Button>
            )}

            {/* Network Warning */}
            {isConnected && !isChainIdSupported(chainId) && (
              <Button disabled size="sm" variant="outline">
                <Activity className="w-4 h-4 mr-1" />
                Wrong Network
              </Button>
            )}

            <ConnectDialog isOpen={connectDialogOpen} setIsOpen={setConnectDialogOpen} />
          </>
        ) : (
          <>
            <Button size="sm" onClick={() => setSessionDialogOpen(true)}>
              <User className="w-4 h-4" />
            </Button>
            <SessionDialog isOpen={sessionDialogOpen} setIsOpen={setSessionDialogOpen} />
          </>
        )}
      </div>
    </header>
  );
}
```

### 5. Benefits of Header Login

- **Better UX**: Login is always accessible without navigating to a separate page
- **Space Efficient**: Saves screen real estate by integrating login into existing header
- **Consistent Design**: Maintains header presence throughout the application
- **Progressive Disclosure**: Shows login options only when needed

### 6. Considerations

- **Mobile Responsiveness**: Ensure buttons work well on smaller screens
- **State Management**: Handle loading states and errors appropriately
- **Accessibility**: Maintain proper focus management and keyboard navigation
- **Error Handling**: Consider how to display login errors in the header context

This approach provides a more integrated login experience while maintaining all the security and functionality of the original implementation.
