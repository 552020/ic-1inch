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
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { ArrowRight } from "lucide-react";

interface OrderFormProps {
  fromToken: string;
  toToken: string;
  fromAmount: string;
  toAmount: string;
  isCreating: boolean;
  testMode: boolean;
  onFromTokenChange: (token: string) => void;
  onToTokenChange: (token: string) => void;
  onFromAmountChange: (amount: string) => void;
  onToAmountChange: (amount: string) => void;
  onSwapDirection: () => void;
  onCreateOrder: () => void;
}

export function OrderForm({
  fromToken,
  toToken,
  fromAmount,
  toAmount,
  isCreating,
  testMode,
  onFromTokenChange,
  onToTokenChange,
  onFromAmountChange,
  onToAmountChange,
  onSwapDirection,
  onCreateOrder,
}: OrderFormProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Create Cross-Chain Order</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Token Selection */}
        <div className="flex items-center gap-2">
          <div className="flex-1">
            <Label htmlFor="fromToken">From</Label>
            <Select value={fromToken} onValueChange={onFromTokenChange}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="ICP">ICP</SelectItem>
                <SelectItem value="ETH">ETH</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <Button
            variant="outline"
            size="icon"
            onClick={onSwapDirection}
            className="mt-6"
          >
            <ArrowRight className="h-4 w-4" />
          </Button>
          <div className="flex-1">
            <Label htmlFor="toToken">To</Label>
            <Select value={toToken} onValueChange={onToTokenChange}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="ICP">ICP</SelectItem>
                <SelectItem value="ETH">ETH</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>

        {/* Amount Inputs */}
        <div className="space-y-3">
          <div>
            <Label htmlFor="fromAmount">Amount ({fromToken})</Label>
            <Input
              id="fromAmount"
              type="number"
              placeholder={`Enter ${fromToken} amount`}
              value={fromAmount}
              onChange={(e) => onFromAmountChange(e.target.value)}
            />
          </div>
          <div>
            <Label htmlFor="toAmount">Amount ({toToken})</Label>
            <Input
              id="toAmount"
              type="number"
              placeholder={`Enter ${toToken} amount`}
              value={toAmount}
              onChange={(e) => onToAmountChange(e.target.value)}
            />
          </div>
        </div>

        {/* Create Order Button */}
        <Button
          onClick={onCreateOrder}
          disabled={!fromAmount || !toAmount || isCreating}
          className="w-full"
        >
          {isCreating ? "Creating Order..." : "Create Order"}
        </Button>

        {/* Test Mode Info */}
        {testMode && (
          <div className="text-xs text-muted-foreground space-y-1">
            <p>💡 Test Mode: Try amount &gt; 1000 to test error handling</p>
            <p>💡 ICP → ETH: Atomic order creation + token locking</p>
            <p>
              💡 ETH → ICP: Standard order creation (resolver handles locking)
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
