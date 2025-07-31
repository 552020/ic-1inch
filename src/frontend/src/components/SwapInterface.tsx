import { useState } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { ArrowRight } from "lucide-react";

interface SwapInterfaceProps {
  onOrderCreated?: (order: any) => void;
}

export default function SwapInterface({ onOrderCreated }: SwapInterfaceProps) {
  const [fromToken, setFromToken] = useState<string>("ICP");
  const [toToken, setToToken] = useState<string>("ETH");
  const [fromAmount, setFromAmount] = useState<string>("");
  const [toAmount, setToAmount] = useState<string>("");
  const [isCreating, setIsCreating] = useState(false);

  const handleSwapDirection = () => {
    const tempToken = fromToken;
    const tempAmount = fromAmount;
    setFromToken(toToken);
    setToToken(tempToken);
    setFromAmount(toAmount);
    setToAmount(tempAmount);
  };

  const handleCreateOrder = async () => {
    if (!fromAmount || !toAmount) return;

    setIsCreating(true);
    try {
      const order = {
        id: `order_${Date.now()}`,
        fromToken,
        toToken,
        fromAmount: parseFloat(fromAmount),
        toAmount: parseFloat(toAmount),
        status: "pending",
        createdAt: new Date().toISOString(),
      };

      onOrderCreated?.(order);
      setFromAmount("");
      setToAmount("");
    } catch (error) {
      console.error("Failed to create order:", error);
    } finally {
      setIsCreating(false);
    }
  };

  return (
    <div className="max-w-md mx-auto space-y-4">
      {/* From Token */}
      <div className="space-y-2">
        <Label>From</Label>
        <div className="flex gap-2">
          <Select value={fromToken} onValueChange={setFromToken}>
            <SelectTrigger className="w-24">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ICP">ICP</SelectItem>
              <SelectItem value="ETH">ETH</SelectItem>
            </SelectContent>
          </Select>
          <Input
            type="number"
            placeholder="0.0"
            value={fromAmount}
            onChange={(e) => setFromAmount(e.target.value)}
            className="flex-1"
          />
        </div>
      </div>

      {/* Swap Direction */}
      <div className="flex justify-center">
        <Button
          variant="outline"
          size="sm"
          onClick={handleSwapDirection}
          className="rounded-full p-2"
        >
          <ArrowRight className="w-4 h-4" />
        </Button>
      </div>

      {/* To Token */}
      <div className="space-y-2">
        <Label>To</Label>
        <div className="flex gap-2">
          <Select value={toToken} onValueChange={setToToken}>
            <SelectTrigger className="w-24">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ICP">ICP</SelectItem>
              <SelectItem value="ETH">ETH</SelectItem>
            </SelectContent>
          </Select>
          <Input
            type="number"
            placeholder="0.0"
            value={toAmount}
            onChange={(e) => setToAmount(e.target.value)}
            className="flex-1"
          />
        </div>
      </div>

      {/* Rate Display */}
      {fromAmount && toAmount && (
        <div className="text-sm text-muted-foreground text-center">
          1 {fromToken} ={" "}
          {(parseFloat(toAmount) / parseFloat(fromAmount)).toFixed(6)} {toToken}
        </div>
      )}

      {/* Create Order Button */}
      <Button
        onClick={handleCreateOrder}
        disabled={!fromAmount || !toAmount || isCreating}
        className="w-full"
      >
        {isCreating ? "Creating..." : "Create Order"}
      </Button>
    </div>
  );
}
