import React, { useState } from "react";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import { Badge } from "../ui/badge";
import { Separator } from "../ui/separator";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import { Switch } from "../ui/switch";
import {
  ArrowLeftRight,
  DollarSign,
  Calendar as CalendarIcon,
  Info,
  Zap,
  RefreshCw,
  Calculator,
  AlertTriangle,
  CheckCircle,
} from "lucide-react";

// Simple date helpers
const addDays = (date: Date, days: number): Date => {
  const result = new Date(date);
  result.setDate(result.getDate() + days);
  return result;
};

const formatDate = (date: Date): string => {
  return date.toLocaleDateString("en-US", {
    weekday: "long",
    year: "numeric",
    month: "long",
    day: "numeric",
  });
};

interface CreateOrderFormProps {
  onSubmit: (orderData: any) => void;
  isLoading?: boolean;
}

interface TokenBalance {
  symbol: string;
  balance: string;
  canisterId: string;
  decimals: number;
}

export function CreateOrderForm({
  onSubmit,
  isLoading = false,
}: CreateOrderFormProps) {
  // Form state
  const [makerToken, setMakerToken] = useState<string>("");
  const [takerToken, setTakerToken] = useState<string>("");
  const [makingAmount, setMakingAmount] = useState<string>("");
  const [takingAmount, setTakingAmount] = useState<string>("");
  const [expiration, setExpiration] = useState<Date | undefined>(
    addDays(new Date(), 7)
  );
  const [isPrivateOrder, setIsPrivateOrder] = useState(false);
  const [allowedTaker, setAllowedTaker] = useState<string>("");
  const [receiver, setReceiver] = useState<string>("");

  // Mock token data - in a real app, this would come from your token registry
  const availableTokens: TokenBalance[] = [
    {
      symbol: "ICP",
      balance: "1,250.50",
      canisterId: "rdmx6-jaaaa-aaaah-qcaiq-cai",
      decimals: 8,
    },
    {
      symbol: "ckBTC",
      balance: "0.0245",
      canisterId: "mxzaz-hqaaa-aaaar-qaada-cai",
      decimals: 8,
    },
    {
      symbol: "ckETH",
      balance: "2.85",
      canisterId: "ss2fx-dyaaa-aaaar-qacoq-cai",
      decimals: 18,
    },
    {
      symbol: "CHAT",
      balance: "15,000",
      canisterId: "2ouva-viaaa-aaaaq-aaamq-cai",
      decimals: 8,
    },
  ];

  // Calculate exchange rate
  const exchangeRate =
    makingAmount &&
    takingAmount &&
    parseFloat(makingAmount) > 0 &&
    parseFloat(takingAmount) > 0
      ? (parseFloat(takingAmount) / parseFloat(makingAmount)).toFixed(6)
      : "0";

  // Get token info
  const getMakerTokenInfo = () =>
    availableTokens.find((t) => t.canisterId === makerToken);
  const getTakerTokenInfo = () =>
    availableTokens.find((t) => t.canisterId === takerToken);

  // Validation
  const isFormValid = () => {
    return (
      makerToken &&
      takerToken &&
      makerToken !== takerToken &&
      makingAmount &&
      takingAmount &&
      parseFloat(makingAmount) > 0 &&
      parseFloat(takingAmount) > 0 &&
      expiration &&
      (!isPrivateOrder || allowedTaker)
    );
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!isFormValid()) return;

    const orderData = {
      receiver: receiver || null, // Use current user if not specified
      makerAsset: makerToken,
      takerAsset: takerToken,
      makingAmount: parseFloat(makingAmount),
      takingAmount: parseFloat(takingAmount),
      expiration: expiration!.getTime() * 1000000, // Convert to nanoseconds
      allowedTaker: isPrivateOrder ? allowedTaker : null,
    };

    onSubmit(orderData);
  };

  const swapTokens = () => {
    const tempToken = makerToken;
    const tempAmount = makingAmount;
    setMakerToken(takerToken);
    setTakerToken(tempToken);
    setMakingAmount(takingAmount);
    setTakingAmount(tempAmount);
  };

  return (
    <Card className="w-full max-w-2xl mx-auto">
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle className="flex items-center gap-2">
              <DollarSign className="size-5 text-primary" />
              Create Limit Order
            </CardTitle>
            <CardDescription>
              Set your desired exchange rate and let takers fill your order
            </CardDescription>
          </div>
          <Badge variant="secondary" className="flex items-center gap-1">
            <Zap className="size-3" />
            Zero Gas Fees
          </Badge>
        </div>
      </CardHeader>

      <CardContent className="space-y-6">
        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Token Pair Selection */}
          <div className="space-y-4">
            <Label className="text-base font-medium">Token Pair</Label>

            {/* Maker Token (You Give) */}
            <div className="space-y-2">
              <Label
                htmlFor="maker-token"
                className="text-sm text-muted-foreground"
              >
                You Give (Maker Asset)
              </Label>
              <div className="flex gap-2">
                <Select value={makerToken} onValueChange={setMakerToken}>
                  <SelectTrigger className="flex-1">
                    <SelectValue placeholder="Select token to give" />
                  </SelectTrigger>
                  <SelectContent>
                    {availableTokens.map((token) => (
                      <SelectItem
                        key={token.canisterId}
                        value={token.canisterId}
                      >
                        <div className="flex items-center justify-between w-full">
                          <span className="font-medium">{token.symbol}</span>
                          <span className="text-sm text-muted-foreground ml-2">
                            Balance: {token.balance}
                          </span>
                        </div>
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Input
                  type="number"
                  placeholder="Amount"
                  value={makingAmount}
                  onChange={(e) => setMakingAmount(e.target.value)}
                  className="w-32"
                  step="any"
                />
              </div>
              {getMakerTokenInfo() && (
                <div className="flex items-center justify-between text-sm text-muted-foreground">
                  <span>
                    Available: {getMakerTokenInfo()!.balance}{" "}
                    {getMakerTokenInfo()!.symbol}
                  </span>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() =>
                      setMakingAmount(
                        getMakerTokenInfo()!.balance.replace(",", "")
                      )
                    }
                  >
                    Max
                  </Button>
                </div>
              )}
            </div>

            {/* Swap Button */}
            <div className="flex justify-center">
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={swapTokens}
                className="rounded-full p-2"
              >
                <ArrowLeftRight className="size-4" />
              </Button>
            </div>

            {/* Taker Token (You Want) */}
            <div className="space-y-2">
              <Label
                htmlFor="taker-token"
                className="text-sm text-muted-foreground"
              >
                You Want (Taker Asset)
              </Label>
              <div className="flex gap-2">
                <Select value={takerToken} onValueChange={setTakerToken}>
                  <SelectTrigger className="flex-1">
                    <SelectValue placeholder="Select token to receive" />
                  </SelectTrigger>
                  <SelectContent>
                    {availableTokens.map((token) => (
                      <SelectItem
                        key={token.canisterId}
                        value={token.canisterId}
                      >
                        <div className="flex items-center justify-between w-full">
                          <span className="font-medium">{token.symbol}</span>
                          <span className="text-sm text-muted-foreground ml-2">
                            Balance: {token.balance}
                          </span>
                        </div>
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Input
                  type="number"
                  placeholder="Amount"
                  value={takingAmount}
                  onChange={(e) => setTakingAmount(e.target.value)}
                  className="w-32"
                  step="any"
                />
              </div>
              {getTakerTokenInfo() && (
                <div className="text-sm text-muted-foreground">
                  Available: {getTakerTokenInfo()!.balance}{" "}
                  {getTakerTokenInfo()!.symbol}
                </div>
              )}
            </div>
          </div>

          <Separator />

          {/* Exchange Rate Display */}
          {exchangeRate !== "0" &&
            getMakerTokenInfo() &&
            getTakerTokenInfo() && (
              <Card>
                <CardContent className="pt-4">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Calculator className="size-4 text-muted-foreground" />
                      <span className="text-sm font-medium">Exchange Rate</span>
                    </div>
                    <div className="text-right">
                      <div className="font-medium">
                        1 {getMakerTokenInfo()!.symbol} = {exchangeRate}{" "}
                        {getTakerTokenInfo()!.symbol}
                      </div>
                      <div className="text-sm text-muted-foreground">
                        {(1 / parseFloat(exchangeRate)).toFixed(6)}{" "}
                        {getMakerTokenInfo()!.symbol} per{" "}
                        {getTakerTokenInfo()!.symbol}
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            )}

          {/* Expiration */}
          <div className="space-y-2">
            <Label className="text-base font-medium">Order Expiration</Label>
            <div className="relative">
              <CalendarIcon className="absolute left-3 top-3 size-4 text-muted-foreground" />
              <Input
                type="datetime-local"
                value={expiration ? expiration.toISOString().slice(0, 16) : ""}
                onChange={(e) =>
                  setExpiration(
                    e.target.value ? new Date(e.target.value) : undefined
                  )
                }
                className="pl-10"
                min={new Date().toISOString().slice(0, 16)}
              />
            </div>
            {expiration && (
              <p className="text-sm text-muted-foreground">
                Order expires: {formatDate(expiration)}
              </p>
            )}
            <div className="flex gap-2">
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={() => setExpiration(addDays(new Date(), 1))}
              >
                1 Day
              </Button>
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={() => setExpiration(addDays(new Date(), 7))}
              >
                1 Week
              </Button>
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={() => setExpiration(addDays(new Date(), 30))}
              >
                1 Month
              </Button>
            </div>
          </div>

          <Separator />

          {/* Advanced Options */}
          <div className="space-y-4">
            <Label className="text-base font-medium">Advanced Options</Label>

            {/* Private Order */}
            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label htmlFor="private-order" className="text-sm font-medium">
                  Private Order
                </Label>
                <p className="text-sm text-muted-foreground">
                  Restrict filling to a specific taker
                </p>
              </div>
              <Switch
                id="private-order"
                checked={isPrivateOrder}
                onCheckedChange={setIsPrivateOrder}
              />
            </div>

            {isPrivateOrder && (
              <div className="space-y-2">
                <Label htmlFor="allowed-taker">Allowed Taker Principal</Label>
                <Input
                  id="allowed-taker"
                  placeholder="rdmx6-jaaaa-aaaah-qcaiq-cai"
                  value={allowedTaker}
                  onChange={(e) => setAllowedTaker(e.target.value)}
                />
              </div>
            )}

            {/* Custom Receiver */}
            <div className="space-y-2">
              <Label htmlFor="receiver">Custom Receiver (Optional)</Label>
              <Input
                id="receiver"
                placeholder="Leave empty to use your principal"
                value={receiver}
                onChange={(e) => setReceiver(e.target.value)}
              />
              <p className="text-sm text-muted-foreground">
                If specified, tokens will be sent to this principal instead of
                yours
              </p>
            </div>
          </div>

          <Separator />

          {/* Submit Button */}
          <div className="space-y-4">
            {/* Warning/Info */}
            <Card className="border-orange-200 bg-orange-50">
              <CardContent className="pt-4">
                <div className="flex items-start gap-3">
                  <Info className="size-5 text-orange-600 mt-0.5" />
                  <div className="space-y-1">
                    <p className="text-sm font-medium text-orange-800">
                      Order Creation is Free
                    </p>
                    <p className="text-sm text-orange-700">
                      Thanks to ICP's reverse gas model, creating orders costs
                      you nothing. Your order will be stored on-chain and
                      discoverable by takers immediately.
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Button
              type="submit"
              className="w-full"
              size="lg"
              disabled={!isFormValid() || isLoading}
            >
              {isLoading ? (
                <>
                  <RefreshCw className="mr-2 size-4 animate-spin" />
                  Creating Order...
                </>
              ) : (
                <>
                  <CheckCircle className="mr-2 size-4" />
                  Create Limit Order
                </>
              )}
            </Button>

            {!isFormValid() && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <AlertTriangle className="size-4" />
                <span>
                  Please fill all required fields to create your order
                </span>
              </div>
            )}
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
