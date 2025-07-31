import { useState } from "react";
import { Button } from "./components/ui/button";
import { Card, CardContent } from "./components/ui/card";
import { Separator } from "./components/ui/separator";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./components/ui/tabs";
import AppLegacy from "./pages/AppLegacy";
import { GreetBetter } from "./components/GreetBetter";
import LoginPage from "./components/layout/LoginPage";
import Header from "./components/layout/Header";
import SwapInterface from "./components/SwapInterface";
import OrderBook from "./components/OrderBook";
import { Order } from "./hooks/useTestMode";

function App() {
  const [showLegacy, setShowLegacy] = useState(false);
  const [orders, setOrders] = useState<Order[]>([]);
  const [userRole, setUserRole] = useState<"maker" | "resolver">("maker");
  const [testMode, setTestMode] = useState(true); // Default to test mode

  const handleOrderCreated = (order: Order) => {
    setOrders((prev) => [order, ...prev]);
  };

  const handleAcceptOrder = (orderId: string) => {
    setOrders((prev) =>
      prev.map((order) =>
        order.id === orderId ? { ...order, status: "accepted" as const } : order
      )
    );
  };

  if (showLegacy) {
    return <AppLegacy onBackToMain={() => setShowLegacy(false)} />;
  }

  return (
    <div className="min-h-screen bg-background">
      <Header testMode={testMode} onTestModeChange={setTestMode} />

      <main className="container mx-auto px-4 py-8">
        <div className="max-w-6xl mx-auto space-y-8">
          {/* Login Section */}
          <Card>
            <CardContent className="p-6">
              <LoginPage />
            </CardContent>
          </Card>

          <Separator />

          {/* Fusion Swap Interface */}
          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between mb-4">
                <h2 className="text-2xl font-bold">Fusion+ Mechanical Turk</h2>
                <div className="flex gap-2">
                  <Button
                    variant={userRole === "maker" ? "default" : "outline"}
                    size="sm"
                    onClick={() => setUserRole("maker")}
                  >
                    Maker
                  </Button>
                  <Button
                    variant={userRole === "resolver" ? "default" : "outline"}
                    size="sm"
                    onClick={() => setUserRole("resolver")}
                  >
                    Resolver
                  </Button>
                </div>
              </div>

              <Tabs defaultValue="swap" className="w-full">
                <TabsList className="grid w-full grid-cols-2">
                  <TabsTrigger value="swap">Create Swap</TabsTrigger>
                  <TabsTrigger value="orders">Order Book</TabsTrigger>
                </TabsList>

                <TabsContent value="swap" className="mt-6">
                  <SwapInterface
                    onOrderCreated={handleOrderCreated}
                    testMode={testMode}
                  />
                </TabsContent>

                <TabsContent value="orders" className="mt-6">
                  <OrderBook
                    orders={orders}
                    onAcceptOrder={handleAcceptOrder}
                    userRole={userRole}
                  />
                </TabsContent>
              </Tabs>
            </CardContent>
          </Card>

          <Separator />

          {/* Test Components Section */}
          <Card>
            <CardContent className="p-6">
              <GreetBetter />
            </CardContent>
          </Card>

          <Separator />

          {/* Legacy App Access */}
          <div className="text-center">
            <Button
              variant="outline"
              size="lg"
              onClick={() => {
                setShowLegacy(true);
              }}
            >
              Go to Legacy App
            </Button>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
