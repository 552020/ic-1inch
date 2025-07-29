import { useState, useEffect } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import { Button } from "../ui/button";
import { Badge } from "../ui/badge";
import { Input } from "../ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "../ui/table";
import {
  ArrowUpDown,
  Filter,
  Loader2,
  RefreshCw,
  TrendingUp,
  Clock,
  Coins,
} from "lucide-react";

// Types for order data (matching backend types)
interface Order {
  id: string;
  maker: string;
  receiver: string;
  maker_asset: string;
  taker_asset: string;
  making_amount: number;
  taking_amount: number;
  expiration: number;
  created_at: number;
  allowed_taker?: string;
}

interface OrderBookProps {
  onFillOrder?: (orderId: string) => void;
  isLoading?: boolean;
}

export function OrderBook({
  onFillOrder,
  isLoading: externalLoading,
}: OrderBookProps) {
  // State management
  const [orders, setOrders] = useState<Order[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [lastUpdate, setLastUpdate] = useState<Date>(new Date());

  // Filtering and sorting state
  const [searchFilter, setSearchFilter] = useState("");
  const [assetFilter, setAssetFilter] = useState<string>("all");
  const [sortBy, setSortBy] = useState<"price" | "amount" | "expiration">(
    "price"
  );
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("asc");

  // Fetch orders from backend
  const fetchOrders = async () => {
    setLoading(true);
    setError(null);
    try {
      // For MVP, we'll simulate data since backend get_active_orders might not be implemented yet
      // TODO: Replace with actual backend call when available
      // const result = await backend.get_active_orders();

      // Mock data for development
      const mockOrders: Order[] = [
        {
          id: "1",
          maker: "rdmx6-jaaaa-aaaah-qcaiq-cai",
          receiver: "rdmx6-jaaaa-aaaah-qcaiq-cai",
          maker_asset: "ICP",
          taker_asset: "ckBTC",
          making_amount: 1000000000, // 10 ICP (8 decimals)
          taking_amount: 50000, // 0.0005 ckBTC (8 decimals)
          expiration: Date.now() + 24 * 60 * 60 * 1000, // 24 hours from now
          created_at: Date.now() - 2 * 60 * 60 * 1000, // 2 hours ago
        },
        {
          id: "2",
          maker: "rdmx6-jaaaa-aaaah-qcaiq-cai",
          receiver: "rdmx6-jaaaa-aaaah-qcaiq-cai",
          maker_asset: "ckBTC",
          taker_asset: "ICP",
          making_amount: 100000, // 0.001 ckBTC
          taking_amount: 2000000000, // 20 ICP
          expiration: Date.now() + 12 * 60 * 60 * 1000, // 12 hours from now
          created_at: Date.now() - 1 * 60 * 60 * 1000, // 1 hour ago
        },
        {
          id: "3",
          maker: "rdmx6-jaaaa-aaaah-qcaiq-cai",
          receiver: "rdmx6-jaaaa-aaaah-qcaiq-cai",
          maker_asset: "ICP",
          taker_asset: "ckUSDC",
          making_amount: 500000000, // 5 ICP
          taking_amount: 6500000, // 65 USDC (6 decimals)
          expiration: Date.now() + 6 * 60 * 60 * 1000, // 6 hours from now
          created_at: Date.now() - 30 * 60 * 1000, // 30 minutes ago
        },
      ];

      setOrders(mockOrders);
      setLastUpdate(new Date());
    } catch (err) {
      console.error("Error fetching orders:", err);
      setError("Failed to fetch orders. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  // Initial load and refresh timer
  useEffect(() => {
    void fetchOrders();

    // Auto-refresh every 30 seconds
    const interval = setInterval(() => {
      void fetchOrders();
    }, 30000);

    return () => clearInterval(interval);
  }, []);

  // Format amounts for display
  const formatAmount = (amount: number, decimals: number = 8): string => {
    return (amount / Math.pow(10, decimals)).toFixed(decimals === 6 ? 2 : 4);
  };

  // Calculate price ratio
  const calculatePrice = (order: Order): number => {
    return order.taking_amount / order.making_amount;
  };

  // Format expiration time
  const formatExpiration = (timestamp: number): string => {
    const expDate = new Date(timestamp);
    const now = new Date();
    const hoursLeft = Math.ceil(
      (expDate.getTime() - now.getTime()) / (1000 * 60 * 60)
    );

    if (hoursLeft < 1) return "< 1h";
    if (hoursLeft < 24) return `${hoursLeft}h`;
    return `${Math.ceil(hoursLeft / 24)}d`;
  };

  // Filter and sort orders
  const filteredOrders = orders
    .filter((order) => {
      // Text search filter
      if (searchFilter) {
        const searchLower = searchFilter.toLowerCase();
        return (
          order.maker_asset.toLowerCase().includes(searchLower) ||
          order.taker_asset.toLowerCase().includes(searchLower) ||
          order.id.toLowerCase().includes(searchLower)
        );
      }
      return true;
    })
    .filter((order) => {
      // Asset pair filter
      if (assetFilter === "all") return true;
      return (
        order.maker_asset === assetFilter || order.taker_asset === assetFilter
      );
    })
    .sort((a, b) => {
      let comparison = 0;

      switch (sortBy) {
        case "price":
          comparison = calculatePrice(a) - calculatePrice(b);
          break;
        case "amount":
          comparison = a.making_amount - b.making_amount;
          break;
        case "expiration":
          comparison = a.expiration - b.expiration;
          break;
      }

      return sortOrder === "asc" ? comparison : -comparison;
    });

  // Get unique assets for filter dropdown
  const uniqueAssets = Array.from(
    new Set([
      ...orders.map((o) => o.maker_asset),
      ...orders.map((o) => o.taker_asset),
    ])
  );

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle className="flex items-center gap-2">
              <Coins className="size-5" />
              Active Orders
            </CardTitle>
            <CardDescription>
              {filteredOrders.length} active limit orders • Last updated{" "}
              {lastUpdate.toLocaleTimeString()}
            </CardDescription>
          </div>
          <Button
            variant="outline"
            size="sm"
            onClick={() => void fetchOrders()}
            disabled={loading}
          >
            {loading ? (
              <Loader2 className="size-4 animate-spin" />
            ) : (
              <RefreshCw className="size-4" />
            )}
            Refresh
          </Button>
        </div>
      </CardHeader>

      <CardContent className="space-y-4">
        {/* Filters */}
        <div className="flex flex-col sm:flex-row gap-4">
          <div className="flex-1">
            <Input
              placeholder="Search by token or order ID..."
              value={searchFilter}
              onChange={(e) => setSearchFilter(e.target.value)}
              className="w-full"
            />
          </div>

          <Select value={assetFilter} onValueChange={setAssetFilter}>
            <SelectTrigger className="w-full sm:w-40">
              <Filter className="size-4 mr-2" />
              <SelectValue placeholder="Filter asset" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Assets</SelectItem>
              {uniqueAssets.map((asset) => (
                <SelectItem key={asset} value={asset}>
                  {asset}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          <Select
            value={`${sortBy}-${sortOrder}`}
            onValueChange={(value) => {
              const [newSortBy, newSortOrder] = value.split("-") as [
                typeof sortBy,
                typeof sortOrder
              ];
              setSortBy(newSortBy);
              setSortOrder(newSortOrder);
            }}
          >
            <SelectTrigger className="w-full sm:w-48">
              <ArrowUpDown className="size-4 mr-2" />
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="price-asc">Price: Low to High</SelectItem>
              <SelectItem value="price-desc">Price: High to Low</SelectItem>
              <SelectItem value="amount-desc">Amount: High to Low</SelectItem>
              <SelectItem value="amount-asc">Amount: Low to High</SelectItem>
              <SelectItem value="expiration-asc">Expires Soon</SelectItem>
              <SelectItem value="expiration-desc">Expires Later</SelectItem>
            </SelectContent>
          </Select>
        </div>

        {/* Error state */}
        {error && (
          <div className="p-4 bg-destructive/10 border border-destructive/20 rounded-lg">
            <p className="text-sm text-destructive">{error}</p>
          </div>
        )}

        {/* Orders table */}
        {loading ? (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="size-6 animate-spin mr-2" />
            <span>Loading orders...</span>
          </div>
        ) : filteredOrders.length === 0 ? (
          <div className="text-center py-8">
            <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mx-auto mb-4">
              <Coins className="size-8 text-muted-foreground" />
            </div>
            <h3 className="text-lg font-semibold mb-2">No orders found</h3>
            <p className="text-muted-foreground">
              {searchFilter || assetFilter !== "all"
                ? "Try adjusting your filters"
                : "Be the first to create a limit order!"}
            </p>
          </div>
        ) : (
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Pair</TableHead>
                  <TableHead>You Give</TableHead>
                  <TableHead>You Get</TableHead>
                  <TableHead>Price</TableHead>
                  <TableHead>Expires</TableHead>
                  <TableHead className="text-right">Action</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {filteredOrders.map((order) => (
                  <TableRow key={order.id} className="hover:bg-muted/50">
                    <TableCell>
                      <div className="flex items-center gap-2">
                        <Badge variant="outline" className="text-xs">
                          {order.taker_asset} → {order.maker_asset}
                        </Badge>
                      </div>
                    </TableCell>

                    <TableCell>
                      <div className="font-medium">
                        {formatAmount(
                          order.taking_amount,
                          order.taker_asset === "ckUSDC" ? 6 : 8
                        )}{" "}
                        {order.taker_asset}
                      </div>
                    </TableCell>

                    <TableCell>
                      <div className="font-medium">
                        {formatAmount(order.making_amount)} {order.maker_asset}
                      </div>
                    </TableCell>

                    <TableCell>
                      <div className="flex items-center gap-1">
                        <TrendingUp className="size-3 text-green-500" />
                        <span className="font-mono text-sm">
                          {calculatePrice(order).toFixed(6)}
                        </span>
                      </div>
                    </TableCell>

                    <TableCell>
                      <div className="flex items-center gap-1">
                        <Clock className="size-3 text-orange-500" />
                        <span className="text-sm">
                          {formatExpiration(order.expiration)}
                        </span>
                      </div>
                    </TableCell>

                    <TableCell className="text-right">
                      <Button
                        size="sm"
                        onClick={() => {
                          if (
                            confirm(
                              `Fill order for ${formatAmount(
                                order.making_amount
                              )} ${order.maker_asset}?`
                            )
                          ) {
                            onFillOrder?.(order.id);
                          }
                        }}
                        disabled={externalLoading}
                      >
                        Fill Order
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
