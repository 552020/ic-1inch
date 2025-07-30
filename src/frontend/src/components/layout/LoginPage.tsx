import ConnectButton from "./ConnectButton";
import LoginButton from "./LoginButton";
import { isChainIdSupported } from "../../wagmi/is-chain-id-supported";
import { useAccount } from "wagmi";
import { useChainId } from "wagmi";
import { useEffect } from "react";
import { useSiwe } from "ic-siwe-js/react";
import { useToast } from "@/hooks/use-toast";
import { Button } from "@/components/ui/button";
import { Activity } from "lucide-react";
import EthBadge from "./EthBadge";

export default function LoginPage(): React.ReactElement {
  const { isConnected } = useAccount();
  const chainId = useChainId();
  const { loginError } = useSiwe();
  const { toast } = useToast();

  /**
   * Show an error toast if the login call fails.
   */
  useEffect(() => {
    if (loginError) {
      toast({
        variant: "destructive",
        description: loginError.message,
      });
    }
  }, [loginError, toast]);

  return (
    <div className="flex flex-col items-center justify-center space-y-6">
      {/* Connection Steps */}
      <div className="w-full max-w-md space-y-4">
        {/* Step 1: Connect Wallet */}
        <div className="flex items-center justify-center gap-4 p-4 border rounded-lg">
          <div className="flex items-center justify-center w-8 h-8 text-sm font-bold rounded-full bg-primary text-primary-foreground">
            1
          </div>
          <div>
            {!isConnected && <ConnectButton />}
            {isConnected && isChainIdSupported(chainId) && <EthBadge />}
            {isConnected && !isChainIdSupported(chainId) && (
              <Button disabled variant="outline">
                <Activity className="w-4 h-4 mr-2" />
                Unsupported Network
              </Button>
            )}
          </div>
        </div>

        {/* Step 2: Sign In */}
        <div className="flex items-center justify-center gap-4 p-4 border rounded-lg">
          <div className="flex items-center justify-center w-8 h-8 text-sm font-bold rounded-full bg-primary text-primary-foreground">
            2
          </div>
          <div>
            <LoginButton />
          </div>
        </div>
      </div>
    </div>
  );
}
