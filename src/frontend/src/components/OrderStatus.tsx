import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { AlertCircle, CheckCircle, Clock, Key, Shield } from "lucide-react";
import { Alert, AlertDescription } from "./ui/alert";
import { Order } from "../hooks/useTestMode";

interface OrderStatusProps {
  order: Order;
  secret: string;
  isCreating: boolean;
  onSecretChange: (secret: string) => void;
  onShareSecret: () => void;
  onConfirmSwap: () => void;
  onNewOrder: () => void;
}

export function OrderStatus({
  order,
  secret,
  isCreating,
  onSecretChange,
  onShareSecret,
  onConfirmSwap,
  onNewOrder,
}: OrderStatusProps) {
  const getStatusIcon = (status: string) => {
    switch (status) {
      case "pending":
        return <Clock className="w-5 h-5 text-yellow-600" />;
      case "accepted":
        return <CheckCircle className="w-5 h-5 text-blue-600" />;
      case "awaiting_confirmation":
        return <Shield className="w-5 h-5 text-orange-600" />;
      case "awaiting_secret":
        return <Key className="w-5 h-5 text-orange-600" />;
      case "completed":
        return <CheckCircle className="w-5 h-5 text-green-600" />;
      default:
        return <AlertCircle className="w-5 h-5 text-red-600" />;
    }
  };

  const getStatusMessage = (status: string) => {
    switch (status) {
      case "pending":
        return order.fromToken === "ICP" && order.toToken === "ETH"
          ? "Creating order and locking tokens..."
          : "Waiting for a resolver to accept your order...";
      case "accepted":
        return order.fromToken === "ICP" && order.toToken === "ETH"
          ? "Order created with tokens locked! Relayer is verifying both chains..."
          : "Resolver found! They are preparing the swap...";
      case "awaiting_confirmation":
        return "Relayer has verified all assets are locked. Please confirm to proceed with the swap.";
      case "awaiting_secret":
        return "Ready to complete! Please share your secret/password to finalize the swap.";
      case "completed":
        return "Swap completed successfully! 🎉";
      default:
        return "Something went wrong.";
    }
  };

  return (
    <div className="max-w-md mx-auto space-y-4">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            {getStatusIcon(order.status)}
            Order Status
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Order Details */}
          <div className="space-y-2 p-3 bg-muted rounded-lg">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Order ID:</span>
              <span className="font-mono text-sm">{order.id}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">You pay:</span>
              <span className="font-medium">
                {order.fromAmount} {order.fromToken}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">You receive:</span>
              <span className="font-medium">
                {order.toAmount} {order.toToken}
              </span>
            </div>
            {order.resolver && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Resolver:</span>
                <span className="font-mono text-sm">
                  {order.resolver.slice(0, 10)}...
                </span>
              </div>
            )}
          </div>

          {/* Relayer Message */}
          {order.relayerMessage && (
            <Alert className="border-green-200 bg-green-50">
              <Shield className="h-4 w-4 text-green-600" />
              <AlertDescription className="text-green-800">
                {order.relayerMessage}
              </AlertDescription>
            </Alert>
          )}

          {/* Status Message */}
          <Alert>
            <AlertCircle className="h-4 w-4" />
            <AlertDescription>
              {getStatusMessage(order.status)}
            </AlertDescription>
          </Alert>

          {/* Confirmation Button */}
          {order.status === "awaiting_confirmation" && (
            <Button
              onClick={onConfirmSwap}
              disabled={isCreating}
              className="w-full"
            >
              {isCreating ? "Confirming..." : "Confirm Swap"}
            </Button>
          )}

          {/* Secret Sharing Interface */}
          {order.status === "awaiting_secret" && (
            <div className="space-y-3">
              <Label htmlFor="secret">Enter your secret/password:</Label>
              <Input
                id="secret"
                type="password"
                placeholder="Enter secret to complete swap"
                value={secret}
                onChange={(e) => onSecretChange(e.target.value)}
              />
              <Button
                onClick={onShareSecret}
                disabled={!secret.trim() || isCreating}
                className="w-full"
              >
                {isCreating ? "Sharing..." : "Share Secret"}
              </Button>
            </div>
          )}

          {/* New Order Button */}
          {order.status === "completed" && (
            <Button onClick={onNewOrder} className="w-full">
              Create New Order
            </Button>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
