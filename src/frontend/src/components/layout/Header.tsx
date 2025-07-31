import EthBadge from "./EthBadge";
import SessionButton from "./SessionButton";
import TestModeToggle from "./TestModeToggle";
import ConnectButton from "./ConnectButton";
import LoginButton from "./LoginButton";
import { useAccount } from "wagmi";
import { useChainId } from "wagmi";
import { isChainIdSupported } from "../../wagmi/is-chain-id-supported";
import { Button } from "@/components/ui/button";
import { Activity } from "lucide-react";

interface HeaderProps {
  testMode?: boolean;
  onTestModeChange?: (enabled: boolean) => void;
}

export default function Header({ testMode, onTestModeChange }: HeaderProps) {
  const { isConnected } = useAccount();
  const chainId = useChainId();

  return (
    <header className="border-b p-4">
      <div className="flex justify-between items-center">
        {/* Left side - Test Mode Toggle */}
        {testMode !== undefined && onTestModeChange && (
          <TestModeToggle
            testMode={testMode}
            onTestModeChange={onTestModeChange}
          />
        )}

        {/* Right side - User controls */}
        <div className="flex items-center space-x-4">
          {/* Wallet Connection */}
          {!isConnected && <ConnectButton />}
          {isConnected && isChainIdSupported(chainId) && <EthBadge />}
          {isConnected && !isChainIdSupported(chainId) && (
            <Button disabled variant="outline">
              <Activity className="w-4 h-4 mr-2" />
              Unsupported Network
            </Button>
          )}

          {/* Authentication */}
          <LoginButton />

          {/* Session Info */}
          <SessionButton />
        </div>
      </div>
    </header>
  );
}
