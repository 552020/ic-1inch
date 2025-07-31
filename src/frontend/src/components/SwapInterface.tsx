import { useState, useEffect } from "react";
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
import { ArrowRight, TrendingUp, AlertCircle, CheckCircle } from "lucide-react";
import { Alert, AlertDescription } from "./ui/alert";

interface SwapInterfaceProps {
  onOrderCreated?: (order: any) => void;
}

interface MarketRate {
  pair: string;
  rate: number;
  change24h: number;
  lastUpdated: string;
}

export default function SwapInterface({ onOrderCreated }: SwapInterfaceProps) {
  const [fromToken, setFromToken] = useState<string>("ICP");
  const [toToken, setToToken] = useState<string>("ETH");
  const [fromAmount, setFromAmount] = useState<string>("");
  const [toAmount, setToAmount] = useState<string>("");
  const [isCreating, setIsCreating] = useState(false);
  const [showConfirmation, setShowConfirmation] = useState(false);
  const [marketRates, setMarketRates] = useState<MarketRate[]>([]);
  const [isLoadingRates, setIsLoadingRates] = useState(false);

  // Mock market rate fetching (in real app, this would call DEX APIs)
  const fetchMarketRates = async () => {
    setIsLoadingRates(true);
    try {
      // Simulate API call delay
      await new Promise((resolve) => setTimeout(resolve, 1000));

      // Mock market rates (in real app, fetch from CoinGecko, DEX APIs, etc.)
      const mockRates: MarketRate[] = [
        {
          pair: "ICP/ETH",
          rate: 0.00285, // 1 ICP = 0.00285 ETH
          change24h: 2.34,
          lastUpdated: new Date().toISOString(),
        },
        {
          pair: "ETH/ICP",
          rate: 350.88, // 1 ETH = 350.88 ICP
          change24h: -2.29,
          lastUpdated: new Date().toISOString(),
        },
      ];

      setMarketRates(mockRates);
    } catch (error) {
      console.error("Failed to fetch market rates:", error);
    } finally {
      setIsLoadingRates(false);
    }
  };

  useEffect(() => {
    fetchMarketRates();
    // Refresh rates every 30 seconds
    const interval = setInterval(fetchMarketRates, 30000);
    return () => clearInterval(interval);
  }, []);

  const getCurrentMarketRate = () => {
    const pair = `${fromToken}/${toToken}`;
    return marketRates.find((rate) => rate.pair === pair);
  };

  const calculateMarketPrice = (amount: string) => {
    const rate = getCurrentMarketRate();
    if (!rate || !amount) return "";
    return (parseFloat(amount) * rate.rate).toFixed(6);
  };

  const handleSwapDirection = () => {
    const tempToken = fromToken;
    const tempAmount = fromAmount;
    setFromToken(toToken);
    setToToken(tempToken);
    setFromAmount(toAmount);
    setToAmount(tempAmount);
  };

  const handleFromAmountChange = (value: string) => {
    setFromAmount(value);
    // Auto-calculate market price for reference
    if (value && !toAmount) {
      const marketPrice = calculateMarketPrice(value);
      if (marketPrice) {
        setToAmount(marketPrice);
      }
    }
  };

  const handleCreateOrder = () => {
    if (!fromAmount || !toAmount) return;
    setShowConfirmation(true);
  };

  const handleConfirmOrder = async () => {
    setIsCreating(true);
    try {
      const currentRate = getCurrentMarketRate();
      const userRate = parseFloat(toAmount) / parseFloat(fromAmount);

      const order = {
        id: `fusion_${Date.now()}`,
        fromToken,
        toToken,
        fromAmount: parseFloat(fromAmount),
        toAmount: parseFloat(toAmount),
        userRate,
        marketRate: currentRate?.rate || 0,
        priceImpact: currentRate
          ? ((userRate - currentRate.rate) / currentRate.rate) * 100
          : 0,
        status: "pending" as const,
        createdAt: new Date().toISOString(),
        expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(), // 24 hours
      };

      onOrderCreated?.(order);

      // Reset form
      setFromAmount("");
      setToAmount("");
      setShowConfirmation(false);
    } catch (error) {
      console.error("Failed to create order:", error);
    } finally {
      setIsCreating(false);
    }
  };

  const getPriceImpact = () => {
    const currentRate = getCurrentMarketRate();
    if (!currentRate || !fromAmount || !toAmount) return 0;

    const userRate = parseFloat(toAmount) / parseFloat(fromAmount);
    return ((userRate - currentRate.rate) / currentRate.rate) * 100;
  };

  const getPriceImpactColor = (impact: number) => {
    if (Math.abs(impact) < 1) return "text-green-600";
    if (Math.abs(impact) < 5) return "text-yellow-600";
    return "text-red-600";
  };

  if (showConfirmation) {
    const priceImpact = getPriceImpact();
    const currentRate = getCurrentMarketRate();

    return (
      <div className="max-w-md mx-auto space-y-4">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <CheckCircle className="w-5 h-5 text-green-600" />
              Confirm Order
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Order Summary */}
            <div className="space-y-2">
              <div className="flex justify-between">
                <span className="text-muted-foreground">You pay:</span>
                <span className="font-medium">
                  {fromAmount} {fromToken}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">You receive:</span>
                <span className="font-medium">
                  {toAmount} {toToken}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Your rate:</span>
                <span className="font-medium">
                  1 {fromToken} ={" "}
                  {(parseFloat(toAmount) / parseFloat(fromAmount)).toFixed(6)}{" "}
                  {toToken}
                </span>
              </div>
              {currentRate && (
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Market rate:</span>
                  <span className="font-medium">
                    1 {fromToken} = {currentRate.rate.toFixed(6)} {toToken}
                  </span>
                </div>
              )}
              {Math.abs(priceImpact) > 0.1 && (
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Price impact:</span>
                  <span
                    className={`font-medium ${getPriceImpactColor(
                      priceImpact
                    )}`}
                  >
                    {priceImpact > 0 ? "+" : ""}
                    {priceImpact.toFixed(2)}%
                  </span>
                </div>
              )}
            </div>

            {/* Warnings */}
            {Math.abs(priceImpact) > 5 && (
              <Alert>
                <AlertCircle className="h-4 w-4" />
                <AlertDescription>
                  High price impact! Your order rate differs significantly from
                  market rate.
                </AlertDescription>
              </Alert>
            )}

            <Alert>
              <AlertCircle className="h-4 w-4" />
              <AlertDescription>
                This is a limit order. It will be filled when a resolver accepts
                your terms. Your {fromToken} tokens will be locked until the
                order is filled or expires.
              </AlertDescription>
            </Alert>

            {/* Action Buttons */}
            <div className="flex gap-2">
              <Button
                variant="outline"
                onClick={() => setShowConfirmation(false)}
                className="flex-1"
                disabled={isCreating}
              >
                Back
              </Button>
              <Button
                onClick={handleConfirmOrder}
                disabled={isCreating}
                className="flex-1"
              >
                {isCreating ? "Creating..." : "Confirm Order"}
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="max-w-md mx-auto space-y-4">
      {/* Market Rates Display */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm flex items-center gap-2">
            <TrendingUp className="w-4 h-4" />
            Market Rates (Reference Only)
          </CardTitle>
        </CardHeader>
        <CardContent className="pt-0">
          {isLoadingRates ? (
            <div className="text-sm text-muted-foreground">
              Loading rates...
            </div>
          ) : (
            <div className="grid grid-cols-2 gap-2 text-sm">
              {marketRates.map((rate) => (
                <div key={rate.pair} className="flex justify-between">
                  <span className="text-muted-foreground">{rate.pair}:</span>
                  <div className="text-right">
                    <div className="font-medium">{rate.rate.toFixed(6)}</div>
                    <div
                      className={`text-xs ${
                        rate.change24h >= 0 ? "text-green-600" : "text-red-600"
                      }`}
                    >
                      {rate.change24h >= 0 ? "+" : ""}
                      {rate.change24h.toFixed(2)}%
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
          <div className="text-xs text-muted-foreground mt-2">
            Updated:{" "}
            {marketRates[0]?.lastUpdated
              ? new Date(marketRates[0].lastUpdated).toLocaleTimeString()
              : "Never"}
          </div>
        </CardContent>
      </Card>

      {/* Swap Interface */}
      <Card>
        <CardContent className="p-6 space-y-4">
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
                onChange={(e) => handleFromAmountChange(e.target.value)}
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
            <div className="space-y-2 p-3 bg-muted rounded-lg">
              <div className="text-sm">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Your rate:</span>
                  <span className="font-medium">
                    1 {fromToken} ={" "}
                    {(parseFloat(toAmount) / parseFloat(fromAmount)).toFixed(6)}{" "}
                    {toToken}
                  </span>
                </div>
                {getCurrentMarketRate() && (
                  <>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">
                        Market rate:
                      </span>
                      <span>
                        {getCurrentMarketRate()!.rate.toFixed(6)} {toToken}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Difference:</span>
                      <span className={getPriceImpactColor(getPriceImpact())}>
                        {getPriceImpact() > 0 ? "+" : ""}
                        {getPriceImpact().toFixed(2)}%
                      </span>
                    </div>
                  </>
                )}
              </div>
            </div>
          )}

          {/* Create Order Button */}
          <Button
            onClick={handleCreateOrder}
            disabled={!fromAmount || !toAmount || fromToken === toToken}
            className="w-full"
          >
            Create Limit Order
          </Button>

          {fromToken === toToken && (
            <div className="text-sm text-red-600 text-center">
              Please select different tokens
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
