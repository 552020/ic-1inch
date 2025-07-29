import { useState } from "react";
import { backend } from "../../declarations/backend";
import { MainLayout } from "./components/layout/MainLayout";
import { CreateOrderForm } from "./components/maker/CreateOrderForm";
import { OrderBook } from "./components/taker/OrderBook";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

function App() {
  // Authentication state
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [userPrincipal, setUserPrincipal] = useState<string>();

  // View management
  const [currentView, setCurrentView] = useState<"maker" | "taker" | "relayer">(
    "maker"
  );

  // Connection test state (for testing backend connectivity)
  const [greeting, setGreeting] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Mock authentication functions
  const handleLogin = async () => {
    setLoading(true);
    try {
      // Simulate Internet Identity login
      // In a real app, this would integrate with Internet Identity
      await new Promise((resolve) => setTimeout(resolve, 1000));
      setIsAuthenticated(true);
      setUserPrincipal("rdmx6-jaaaa-aaaah-qcaiq-cai");
    } catch {
      setError("Failed to authenticate");
    } finally {
      setLoading(false);
    }
  };

  const handleLogout = () => {
    setIsAuthenticated(false);
    setUserPrincipal(undefined);
    setCurrentView("maker");
  };

  // Order creation handler
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const handleCreateOrder = async (orderData: any) => {
    setLoading(true);
    setError(null);
    try {
      console.log("Creating order:", orderData);
      // This would call the actual backend create_order function
      // const result = await backend.create_order(...);

      // Simulate order creation
      await new Promise((resolve) => setTimeout(resolve, 2000));
      alert("Order created successfully! (This is a simulation)");
    } catch (err) {
      console.error("Error creating order:", err);
      setError("Failed to create order. Make sure backend is running.");
    } finally {
      setLoading(false);
    }
  };

  // Order filling handler
  const handleFillOrder = async (orderId: string) => {
    setLoading(true);
    setError(null);
    try {
      console.log("Filling order:", orderId);
      // This would call the actual backend fill_order function
      // const result = await backend.fill_order(orderId);

      // Simulate order filling
      await new Promise((resolve) => setTimeout(resolve, 1500));
      alert(`Order ${orderId} filled successfully! (This is a simulation)`);
    } catch (err) {
      console.error("Error filling order:", err);
      setError("Failed to fill order. Make sure backend is running.");
    } finally {
      setLoading(false);
    }
  };

  // Test backend connection
  const testConnection = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await backend.greet("ICP Limit Orders");
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

  // Render different views based on currentView
  const renderMainContent = () => {
    if (!isAuthenticated) {
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
                  Create limit orders with zero gas fees using ICP&apos;s
                  reverse gas model
                </p>
                <Button
                  onClick={() => void handleLogin()}
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
                    onClick={() => void testConnection()}
                    variant="outline"
                    disabled={loading}
                    className="w-full"
                  >
                    {loading ? "Testing..." : "Test Backend Connection"}
                  </Button>

                  {error && (
                    <div className="p-3 bg-destructive/10 border border-destructive/20 rounded-lg">
                      <p className="text-sm text-destructive">{error}</p>
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

    // Authenticated user views
    switch (currentView) {
      case "maker":
        return (
          <div className="space-y-6">
            <div className="text-center">
              <h1 className="text-3xl font-bold">Create Limit Order</h1>
              <p className="text-muted-foreground mt-2">
                Set your desired exchange rate and let takers fill your order
              </p>
            </div>
            <CreateOrderForm
              onSubmit={(data) => void handleCreateOrder(data)}
              isLoading={loading}
            />
          </div>
        );

      case "taker":
        return (
          <div className="space-y-6">
            <div className="text-center">
              <h1 className="text-3xl font-bold">Order Book</h1>
              <p className="text-muted-foreground mt-2">
                Browse and fill available limit orders
              </p>
            </div>
            <OrderBook
              onFillOrder={(orderId) => void handleFillOrder(orderId)}
              isLoading={loading}
            />
          </div>
        );

      case "relayer":
        return (
          <div className="space-y-6">
            <div className="text-center">
              <h1 className="text-3xl font-bold">Analytics Dashboard</h1>
              <p className="text-muted-foreground mt-2">
                Monitor system performance and statistics
              </p>
            </div>
            <Card>
              <CardContent className="p-8 text-center">
                <div className="space-y-4">
                  <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mx-auto">
                    <span className="text-2xl">📊</span>
                  </div>
                  <h3 className="text-xl font-semibold">
                    Analytics Coming Soon
                  </h3>
                  <p className="text-muted-foreground">
                    Advanced analytics and monitoring tools are being developed
                  </p>
                  <Badge variant="secondary">Under Development</Badge>
                </div>
              </CardContent>
            </Card>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <MainLayout
      currentView={currentView}
      onViewChange={setCurrentView}
      isAuthenticated={isAuthenticated}
      userPrincipal={userPrincipal}
      onLogin={() => void handleLogin()}
      onLogout={handleLogout}
    >
      {renderMainContent()}
    </MainLayout>
  );
}

export default App;
