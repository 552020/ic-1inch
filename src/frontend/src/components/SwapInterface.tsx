import { useState } from "react";
import { Button } from "./ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { AlertCircle } from "lucide-react";
import { Alert, AlertDescription } from "./ui/alert";
import { useTestMode, OrderData, Order } from "../hooks/useTestMode";
import { useOrderCreation } from "../hooks/useOrderCreation";
import { OrderForm } from "./OrderForm";
import { OrderStatus } from "./OrderStatus";

interface SwapInterfaceProps {
  onOrderCreated?: (order: Order) => void;
  testMode?: boolean;
}

export default function SwapInterface({
  onOrderCreated,
  testMode = true,
}: SwapInterfaceProps) {
  const [fromToken, setFromToken] = useState<string>("ICP");
  const [toToken, setToToken] = useState<string>("ETH");
  const [fromAmount, setFromAmount] = useState<string>("");
  const [toAmount, setToAmount] = useState<string>("");
  const [isCreating, setIsCreating] = useState(false);
  const [showConfirmation, setShowConfirmation] = useState(false);
  const [currentOrder, setCurrentOrder] = useState<Order | null>(null);
  const [showOrderStatus, setShowOrderStatus] = useState(false);
  const [secret, setSecret] = useState("");
  const [error, setError] = useState<string | null>(null);

  const {
    simulateOrderCreation,
    simulateTokenLocking,
    simulateRelayerVerification,
    simulateOrderRollback,
  } = useTestMode();

  const {
    createOrder,
    getOrderStatus,
    isCreating: isCreatingReal,
    error: realError,
    setError: setRealError,
  } = useOrderCreation();

  const handleSwapDirection = () => {
    const tempToken = fromToken;
    const tempAmount = fromAmount;
    setFromToken(toToken);
    setToToken(tempToken);
    setFromAmount(toAmount);
    setToAmount(tempAmount);
  };

  const handleCreateOrder = () => {
    if (!fromAmount || !toAmount) return;
    setError(null);
    setShowConfirmation(true);
  };

  const handleConfirmOrder = async () => {
    setIsCreating(true);
    setError(null);
    setRealError(null);

    const orderData: OrderData = {
      fromToken,
      toToken,
      fromAmount,
      toAmount,
    };

    try {
      let order: Order;

      if (testMode) {
        // Use simulation in test mode
        order = await simulateOrderCreation(orderData);

        // Step 2: Simulate token locking (atomic step for ICP → ETH)
        if (fromToken === "ICP" && toToken === "ETH") {
          await simulateTokenLocking(orderData);
          // Update order status to show tokens are locked
          order.status = "accepted" as const;
          order.resolver = "0x742d35Cc6634C0532925a3b8D4C0532925a3b8D4";
        }

        // Step 3: Simulate relayer verification and confirmation request
        const verifiedOrder = await simulateRelayerVerification(order);
        setCurrentOrder(verifiedOrder);
      } else {
        // Use real canister calls
        order = await createOrder(orderData);
        setCurrentOrder(order);
      }

      onOrderCreated?.(order);

      // Show order status instead of resetting form
      setShowConfirmation(false);
      setShowOrderStatus(true);

      // For ETH → ICP orders in real mode, poll for status updates
      if (!testMode && fromToken === "ETH" && toToken === "ICP") {
        // Poll for order status updates
        const pollInterval = setInterval(async () => {
          if (order.id) {
            const updatedOrder = await getOrderStatus(order.id);
            if (updatedOrder && updatedOrder.status !== order.status) {
              setCurrentOrder(updatedOrder);
              if (updatedOrder.status === "accepted") {
                clearInterval(pollInterval);
              }
            }
          }
        }, 5000); // Poll every 5 seconds

        // Clear interval after 10 minutes
        setTimeout(() => clearInterval(pollInterval), 10 * 60 * 1000);
      }
    } catch (error) {
      console.error("Failed to create order:", error);
      const errorMessage =
        error instanceof Error ? error.message : "Unknown error occurred";
      setError(errorMessage);
      setRealError(errorMessage);

      // If token locking failed in test mode, simulate rollback
      if (
        testMode &&
        error instanceof Error &&
        error.message.includes("Token locking failed")
      ) {
        await simulateOrderRollback(orderData);
      }
    } finally {
      setIsCreating(false);
    }
  };

  const handleConfirmSwap = async () => {
    if (!currentOrder) return;

    setIsCreating(true);
    try {
      if (testMode) {
        // Simulate confirmation delay
        await new Promise((resolve) => setTimeout(resolve, 1000));

        // Move to secret sharing phase
        const awaitingSecretOrder = {
          ...currentOrder,
          status: "awaiting_secret" as const,
        };
        setCurrentOrder(awaitingSecretOrder);
      } else {
        // In real mode, this would trigger the actual swap completion
        // For now, just simulate the transition
        const awaitingSecretOrder = {
          ...currentOrder,
          status: "awaiting_secret" as const,
        };
        setCurrentOrder(awaitingSecretOrder);
      }
    } catch (error) {
      console.error("Failed to confirm swap:", error);
    } finally {
      setIsCreating(false);
    }
  };

  const handleShareSecret = async () => {
    if (!currentOrder || !secret.trim()) return;

    setIsCreating(true);
    try {
      // Simulate secret sharing
      await new Promise((resolve) => setTimeout(resolve, 1500));

      const completedOrder = {
        ...currentOrder,
        status: "completed" as const,
        secret: secret.trim(),
      };
      setCurrentOrder(completedOrder);

      // Auto-reset after completion
      setTimeout(() => {
        setShowOrderStatus(false);
        setCurrentOrder(null);
        setSecret("");
        setFromAmount("");
        setToAmount("");
      }, 5000);
    } catch (error) {
      console.error("Failed to share secret:", error);
    } finally {
      setIsCreating(false);
    }
  };

  const handleNewOrder = () => {
    setShowOrderStatus(false);
    setCurrentOrder(null);
    setSecret("");
    setFromAmount("");
    setToAmount("");
  };

  // Order Status View
  if (showOrderStatus && currentOrder) {
    return (
      <OrderStatus
        order={currentOrder}
        secret={secret}
        isCreating={isCreating || isCreatingReal}
        onSecretChange={setSecret}
        onShareSecret={handleShareSecret}
        onConfirmSwap={handleConfirmSwap}
        onNewOrder={handleNewOrder}
      />
    );
  }

  // Main Swap Interface
  return (
    <div className="max-w-md mx-auto space-y-4">
      {/* Test Mode Indicator */}
      {testMode && (
        <Alert className="border-yellow-200 bg-yellow-50">
          <AlertCircle className="h-4 w-4 text-yellow-600" />
          <AlertDescription className="text-yellow-800">
            Test Mode - Simulating atomic order creation and token locking
          </AlertDescription>
        </Alert>
      )}

      {/* Error Display */}
      {(error || realError) && (
        <Alert className="border-red-200 bg-red-50">
          <AlertCircle className="h-4 w-4 text-red-600" />
          <AlertDescription className="text-red-800">
            {error || realError}
          </AlertDescription>
        </Alert>
      )}

      <OrderForm
        fromToken={fromToken}
        toToken={toToken}
        fromAmount={fromAmount}
        toAmount={toAmount}
        isCreating={isCreating || isCreatingReal}
        testMode={testMode}
        onFromTokenChange={setFromToken}
        onToTokenChange={setToToken}
        onFromAmountChange={setFromAmount}
        onToAmountChange={setToAmount}
        onSwapDirection={handleSwapDirection}
        onCreateOrder={handleCreateOrder}
      />

      {/* Confirmation Dialog */}
      {showConfirmation && (
        <Card>
          <CardHeader>
            <CardTitle>Confirm Order</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2 p-3 bg-muted rounded-lg">
              <div className="flex justify-between">
                <span>You pay:</span>
                <span className="font-medium">
                  {fromAmount} {fromToken}
                </span>
              </div>
              <div className="flex justify-between">
                <span>You receive:</span>
                <span className="font-medium">
                  {toAmount} {toToken}
                </span>
              </div>
              {fromToken === "ICP" && toToken === "ETH" && (
                <div className="text-xs text-blue-600 bg-blue-50 p-2 rounded">
                  ⚡ ICP → ETH: Tokens will be locked immediately during order
                  creation
                </div>
              )}
            </div>
            <div className="flex gap-2">
              <Button
                onClick={() => setShowConfirmation(false)}
                variant="outline"
                className="flex-1"
              >
                Cancel
              </Button>
              <Button
                onClick={handleConfirmOrder}
                disabled={isCreating || isCreatingReal}
                className="flex-1"
              >
                {isCreating || isCreatingReal ? "Creating..." : "Confirm"}
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
