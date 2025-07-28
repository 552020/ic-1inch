import React, { useState } from "react";
import { backend } from "../../declarations/backend";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Badge } from "@/components/ui/badge";

function App() {
  const [greeting, setGreeting] = useState("");
  const [name, setName] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      const result = await backend.greet(name);
      setGreeting(result);
    } catch (err) {
      console.error("Error calling backend:", err);
      setError(
        "Failed to connect to backend. Make sure dfx is running and deployed."
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800">
      <div className="container mx-auto p-8">
        <div className="max-w-4xl mx-auto">
          {/* Header */}
          <div className="text-center mb-12">
            <div className="flex items-center justify-center gap-3 mb-4">
              <div className="w-12 h-12 bg-gradient-to-r from-blue-600 to-purple-600 rounded-xl flex items-center justify-center">
                <span className="text-white font-bold text-xl">1🚀</span>
              </div>
              <h1 className="text-4xl font-bold bg-gradient-to-r from-slate-900 to-slate-600 dark:from-white dark:to-slate-300 bg-clip-text text-transparent">
                IC-1inch Limit Order Protocol
              </h1>
            </div>
            <p className="text-lg text-muted-foreground mb-4">
              MVP Implementation on Internet Computer
            </p>
            <Badge variant="secondary" className="text-sm">
              🔗 ChainFusion+ Ready
            </Badge>
          </div>

          {/* Main Content */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            {/* Connection Test Card */}
            <Card className="shadow-lg">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  🧪 Backend Connection Test
                </CardTitle>
                <CardDescription>
                  Test the connection to your IC backend canister
                </CardDescription>
              </CardHeader>
              <CardContent>
                <form
                  onSubmit={(e) => {
                    void handleSubmit(e);
                  }}
                  className="space-y-4"
                >
                  <div className="space-y-2">
                    <Label htmlFor="name">Enter your name</Label>
                    <Input
                      id="name"
                      type="text"
                      value={name}
                      onChange={(e) => {
                        setName(e.target.value);
                      }}
                      placeholder="Your name here..."
                      disabled={loading}
                      className="transition-all"
                    />
                  </div>
                  <Button
                    type="submit"
                    disabled={loading || !name.trim()}
                    className="w-full"
                    size="lg"
                  >
                    {loading ? "Connecting..." : "🚀 Test Connection"}
                  </Button>
                </form>

                {error && (
                  <div className="mt-4 p-3 bg-destructive/10 border border-destructive/20 rounded-lg">
                    <p className="text-sm text-destructive">{error}</p>
                  </div>
                )}

                {greeting && (
                  <div className="mt-4 p-4 bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800 rounded-lg">
                    <p className="text-green-800 dark:text-green-200 font-medium">
                      ✅ {greeting}
                    </p>
                  </div>
                )}
              </CardContent>
            </Card>

            {/* Features Card */}
            <Card className="shadow-lg">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  ⚡ Protocol Features
                </CardTitle>
                <CardDescription>
                  What&apos;s implemented in this MVP
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div className="flex items-center gap-3 p-3 bg-muted/50 rounded-lg">
                    <div className="w-8 h-8 bg-blue-100 dark:bg-blue-900 rounded-full flex items-center justify-center">
                      📝
                    </div>
                    <div>
                      <p className="font-medium">Create Limit Orders</p>
                      <p className="text-sm text-muted-foreground">
                        On-chain order creation
                      </p>
                    </div>
                  </div>

                  <div className="flex items-center gap-3 p-3 bg-muted/50 rounded-lg">
                    <div className="w-8 h-8 bg-green-100 dark:bg-green-900 rounded-full flex items-center justify-center">
                      🔄
                    </div>
                    <div>
                      <p className="font-medium">Fill Orders</p>
                      <p className="text-sm text-muted-foreground">
                        Atomic token swaps
                      </p>
                    </div>
                  </div>

                  <div className="flex items-center gap-3 p-3 bg-muted/50 rounded-lg">
                    <div className="w-8 h-8 bg-purple-100 dark:bg-purple-900 rounded-full flex items-center justify-center">
                      🌉
                    </div>
                    <div>
                      <p className="font-medium">ICRC Integration</p>
                      <p className="text-sm text-muted-foreground">
                        Native ICP token support
                      </p>
                    </div>
                  </div>

                  <div className="flex items-center gap-3 p-3 bg-muted/50 rounded-lg">
                    <div className="w-8 h-8 bg-orange-100 dark:bg-orange-900 rounded-full flex items-center justify-center">
                      🔮
                    </div>
                    <div>
                      <p className="font-medium">ChainFusion+ Ready</p>
                      <p className="text-sm text-muted-foreground">
                        Cross-chain extension points
                      </p>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Status Footer */}
          <div className="mt-12 text-center">
            <p className="text-sm text-muted-foreground">
              🔧 Development Status: <Badge variant="outline">MVP Phase</Badge>
            </p>
            <p className="text-xs text-muted-foreground mt-2">
              Once connection test passes, the limit order interface will be
              available
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
