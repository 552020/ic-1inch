import { Switch } from "../ui/switch";
import { Label } from "../ui/label";
import { TestTube, Database } from "lucide-react";

interface TestModeToggleProps {
  testMode: boolean;
  onTestModeChange: (enabled: boolean) => void;
}

export default function TestModeToggle({
  testMode,
  onTestModeChange,
}: TestModeToggleProps) {
  return (
    <div className="flex items-center space-x-2">
      <div className="flex items-center space-x-2">
        {testMode ? (
          <TestTube className="h-4 w-4 text-yellow-600" />
        ) : (
          <Database className="h-4 w-4 text-blue-600" />
        )}
        <Label
          htmlFor="test-mode"
          className="text-sm font-medium cursor-pointer"
        >
          {testMode ? "Test Mode" : "Real Mode"}
        </Label>
      </div>
      <Switch
        id="test-mode"
        checked={testMode}
        onCheckedChange={onTestModeChange}
        className="data-[state=checked]:bg-yellow-600"
      />
    </div>
  );
}
