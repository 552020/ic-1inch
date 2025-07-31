import EthBadge from "./EthBadge";
import SessionButton from "./SessionButton";
import TestModeToggle from "./TestModeToggle";

interface HeaderProps {
  testMode?: boolean;
  onTestModeChange?: (enabled: boolean) => void;
}

export default function Header({ testMode, onTestModeChange }: HeaderProps) {
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
          <EthBadge />
          <SessionButton />
        </div>
      </div>
    </header>
  );
}
