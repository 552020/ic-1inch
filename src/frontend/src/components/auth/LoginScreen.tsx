import { useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";

interface LoginScreenProps {
  onLogin: () => Promise<void>;
  onTestConnection: () => Promise<string>;
  loading: boolean;
  error: string | null;
}

export function LoginScreen({
  onLogin,
  onTestConnection,
  loading,
  error,
}: LoginScreenProps) {
  const [greeting, setGreeting] = useState("");
  const [connectionError, setConnectionError] = useState<string | null>(null);

  const handleTestConnection = async () => {
    try {
      setConnectionError(null);
      const result = await onTestConnection();
      setGreeting(result);
    } catch (err) {
      setConnectionError(
        err instanceof Error ? err.message : "Connection failed"
      );
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <Card>
        <CardHeader className="text-center">
          <CardTitle className="text-2xl">
            Welcome to ICP Limit Orders
          </CardTitle>
          <CardDescription>
            Connect your Internet Identity to start trading
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="text-center space-y-4">
            <div className="w-16 h-16 bg-gradient-to-r from-blue-600 to-purple-600 rounded-xl flex items-center justify-center mx-auto">
              <span className="text-white font-bold text-2xl">🚀</span>
            </div>
            <p className="text-muted-foreground">
              Create limit orders with zero gas fees using ICP&apos;s reverse
              gas model
            </p>
            <Button
              onClick={onLogin}
              disabled={loading}
              size="lg"
              className="w-full"
            >
              {loading ? "Connecting..." : "Connect Wallet"}
            </Button>
          </div>

          {/* Backend Connection Test */}
          <div className="pt-6 border-t">
            <div className="space-y-4">
              <h3 className="font-medium text-center">
                Backend Connection Test
              </h3>
              <Button
                onClick={handleTestConnection}
                variant="outline"
                disabled={loading}
                className="w-full"
              >
                {loading ? "Testing..." : "Test Backend Connection"}
              </Button>

              {(error || connectionError) && (
                <div className="p-3 bg-destructive/10 border border-destructive/20 rounded-lg">
                  <p className="text-sm text-destructive">
                    {error || connectionError}
                  </p>
                </div>
              )}

              {greeting && (
                <div className="p-3 bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800 rounded-lg">
                  <p className="text-green-800 dark:text-green-200 text-sm">
                    ✅ {greeting}
                  </p>
                </div>
              )}
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
