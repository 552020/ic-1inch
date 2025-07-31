import { useState, useEffect } from "react";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import { Clock, ArrowRight, CheckCircle } from "lucide-react";

interface Order {
  id: string;
  fromToken: string;
  toToken: string;
  fromAmount: number;
  toAmount: number;
  status: "pending" | "accepted" | "completed" | "failed";
  createdAt: string;
  maker?: string;
  resolver?: string;
}

interface OrderBookProps {
  orders?: Order[];
  onAcceptOrder?: (orderId: string) => void;
  userRole?: "maker" | "resolver";
}

export default function OrderBook({
  orders = [],
  onAcceptOrder,
  userRole,
}: OrderBookProps) {
  const [activeOrders, setActiveOrders] = useState<Order[]>(orders);

  useEffect(() => {
    setActiveOrders(orders);
  }, [orders]);

  const getStatusBadge = (status: Order["status"]) => {
    switch (status) {
      case "pending":
        return (
          <Badge variant="secondary" className="flex items-center gap-1">
            <Clock className="w-3 h-3" /> Pending
          </Badge>
        );
      case "accepted":
        return (
          <Badge variant="default" className="flex items-center gap-1">
            <CheckCircle className="w-3 h-3" /> Accepted
          </Badge>
        );
      case "completed":
        return (
          <Badge variant="default" className="flex items-center gap-1">
            <CheckCircle className="w-3 h-3" /> Completed
          </Badge>
        );
      default:
        return <Badge variant="outline">Unknown</Badge>;
    }
  };

  const formatAmount = (amount: number, token: string) => {
    return `${amount.toFixed(6)} ${token}`;
  };

  if (activeOrders.length === 0) {
    return (
      <div className="text-center py-8 text-muted-foreground">
        <Clock className="w-8 h-8 mx-auto mb-2 opacity-50" />
        <p>No active orders</p>
      </div>
    );
  }

  return (
    <div className="space-y-3 max-h-96 overflow-y-auto">
      {activeOrders.map((order) => (
        <div key={order.id} className="border rounded-lg p-4">
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-2">
                <span className="font-medium">
                  {formatAmount(order.fromAmount, order.fromToken)}
                </span>
                <ArrowRight className="w-4 h-4 text-muted-foreground" />
                <span className="font-medium">
                  {formatAmount(order.toAmount, order.toToken)}
                </span>
              </div>

              <div className="text-sm text-muted-foreground">
                1 {order.fromToken} ={" "}
                {(order.toAmount / order.fromAmount).toFixed(6)} {order.toToken}
              </div>
            </div>

            <div className="flex flex-col items-end gap-2">
              {getStatusBadge(order.status)}

              {userRole === "resolver" && order.status === "pending" && (
                <Button size="sm" onClick={() => onAcceptOrder?.(order.id)}>
                  Accept
                </Button>
              )}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
