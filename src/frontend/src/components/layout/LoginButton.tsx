import { useAccount, useChainId } from "wagmi";

import { Button } from "../ui/Button";
import { isChainIdSupported } from "../../wagmi/is-chain-id-supported";
import { useSiwe } from "ic-siwe-js/react";
import { LoaderCircle } from "lucide-react";

export default function LoginButton() {
  const { isConnected } = useAccount();
  const chainId = useChainId();
  const { login, clear, isLoggingIn, isPreparingLogin, loginError, identity } =
    useSiwe();

  // Debug logging
  console.log("🔍 LoginButton Debug:", {
    isConnected,
    chainId,
    isChainIdSupported: isChainIdSupported(chainId),
    isLoggingIn,
    isPreparingLogin,
    loginError: loginError?.message,
    hasIdentity: !!identity,
  });

  const text = () => {
    if (isLoggingIn) {
      return "Signing in";
    }
    if (isPreparingLogin) {
      return "Preparing";
    }
    // If user has identity, show sign out
    if (identity) {
      return "Sign out";
    }
    return "Sign in";
  };

  const icon =
    isLoggingIn || isPreparingLogin ? (
      <LoaderCircle className="animate-spin" />
    ) : undefined;

  const disabled =
    isLoggingIn ||
    isPreparingLogin ||
    // If user has identity, only disable during loading states
    (identity ? false : !isChainIdSupported(chainId) || !isConnected);

  const handleClick = async () => {
    // If user has identity, logout
    if (identity) {
      console.log("🚀 Sign-out button clicked!");
      console.log("🔍 Pre-logout state:", {
        identity: !!identity,
        hasSession: !!identity,
      });

      try {
        console.log("🔄 Calling clear()...");
        clear();
        console.log("✅ Logout completed successfully");
        console.log("🔍 Post-logout state:", {
          identity: !!identity,
          hasSession: !!identity,
        });
      } catch (error) {
        console.error("❌ Logout failed:", error);
      }
      return;
    }

    // Otherwise, login
    console.log("🚀 Sign-in button clicked!");
    console.log("🔍 Pre-login state:", {
      isConnected,
      chainId,
      isChainIdSupported: isChainIdSupported(chainId),
      disabled,
      identity: !!identity,
      loginError: loginError?.message,
    });

    try {
      console.log("🔄 Calling login()...");
      const result = await login();
      console.log("✅ Login completed successfully:", result);
      console.log("🔍 Post-login state:", {
        identity: !!identity,
        hasSession: !!identity,
      });
    } catch (error) {
      console.error("❌ Login failed:", error);
      console.error("❌ Error details:", {
        message: error?.message,
        cause: error?.cause,
        stack: error?.stack,
      });
    }
  };

  return (
    <Button className="w-44" disabled={disabled} onClick={handleClick}>
      {icon}
      {text()}
    </Button>
  );
}
