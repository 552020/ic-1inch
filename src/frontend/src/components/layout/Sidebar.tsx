import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuBadge,
  useSidebar,
} from "../ui/sidebar";
import { Badge } from "../ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
import { Button } from "../ui/button";
import { Separator } from "../ui/separator";
import {
  TrendingUp,
  BookOpen,
  BarChart3,
  PlusCircle,
  List,
  DollarSign,
  Clock,
  CheckCircle,
  Activity,
  Wallet,
  RefreshCw,
  User,
  Settings,
} from "lucide-react";

interface AppSidebarProps {
  currentView: "maker" | "taker" | "relayer";
  onViewChange: (view: "maker" | "taker" | "relayer") => void;
  userStats?: {
    activeOrders: number;
    filledOrders: number;
    totalVolume: string;
    tokenBalance: string;
  };
  systemStats?: {
    totalOrders: number;
    ordersToday: number;
    totalVolume: string;
    activeUsers: number;
  };
  isAuthenticated: boolean;
  userPrincipal?: string;
}

export function AppSidebar({
  currentView,
  onViewChange,
  userStats,
  systemStats,
  isAuthenticated,
  userPrincipal,
}: AppSidebarProps) {
  const { state } = useSidebar();

  const truncatePrincipal = (principal: string) => {
    if (principal.length <= 10) return principal;
    return `${principal.slice(0, 5)}...${principal.slice(-5)}`;
  };

  const makerMenuItems = [
    { id: "create", label: "Create Order", icon: PlusCircle, badge: null },
    {
      id: "my-orders",
      label: "My Orders",
      icon: List,
      badge: userStats?.activeOrders || 0,
    },
    { id: "portfolio", label: "Portfolio", icon: Wallet, badge: null },
  ];

  const takerMenuItems = [
    { id: "order-book", label: "Order Book", icon: BookOpen, badge: 247 },
    {
      id: "market-analysis",
      label: "Market Analysis",
      icon: TrendingUp,
      badge: null,
    },
    {
      id: "fill-history",
      label: "Fill History",
      icon: CheckCircle,
      badge: null,
    },
  ];

  const relayerMenuItems = [
    {
      id: "system-overview",
      label: "System Overview",
      icon: BarChart3,
      badge: null,
    },
    { id: "monitoring", label: "Monitoring", icon: Activity, badge: null },
    { id: "api-docs", label: "API Documentation", icon: BookOpen, badge: null },
  ];

  const getMenuItems = () => {
    switch (currentView) {
      case "maker":
        return makerMenuItems;
      case "taker":
        return takerMenuItems;
      case "relayer":
        return relayerMenuItems;
      default:
        return makerMenuItems;
    }
  };

  const getViewIcon = (view: "maker" | "taker" | "relayer") => {
    switch (view) {
      case "maker":
        return TrendingUp;
      case "taker":
        return BookOpen;
      case "relayer":
        return BarChart3;
    }
  };

  return (
    <Sidebar collapsible="icon">
      <SidebarHeader>
        <div className="flex items-center gap-2 px-2 py-2">
          <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
            <TrendingUp className="size-4" />
          </div>
          <div className="grid flex-1 text-left text-sm leading-tight">
            <span className="truncate font-semibold">ICP Limit Orders</span>
            <span className="truncate text-xs capitalize">
              {currentView} Dashboard
            </span>
          </div>
        </div>
      </SidebarHeader>

      <SidebarContent>
        {/* View Switcher */}
        <div className="px-3 py-2">
          <h4 className="mb-2 px-2 text-xs font-semibold tracking-tight">
            View Mode
          </h4>
          <SidebarMenu>
            {(["maker", "taker", "relayer"] as const).map((view) => {
              const Icon = getViewIcon(view);
              return (
                <SidebarMenuItem key={view}>
                  <SidebarMenuButton
                    onClick={() => onViewChange(view)}
                    isActive={currentView === view}
                    className="w-full"
                  >
                    <Icon className="size-4" />
                    <span className="capitalize">{view}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              );
            })}
          </SidebarMenu>
        </div>

        <Separator />

        {/* Navigation Menu */}
        <div className="px-3 py-2">
          <h4 className="mb-2 px-2 text-xs font-semibold tracking-tight">
            Navigation
          </h4>
          <SidebarMenu>
            {getMenuItems().map((item) => {
              const Icon = item.icon;
              return (
                <SidebarMenuItem key={item.id}>
                  <SidebarMenuButton>
                    <Icon className="size-4" />
                    <span>{item.label}</span>
                    {item.badge !== null && item.badge > 0 && (
                      <SidebarMenuBadge>{item.badge}</SidebarMenuBadge>
                    )}
                  </SidebarMenuButton>
                </SidebarMenuItem>
              );
            })}
          </SidebarMenu>
        </div>

        <Separator />

        {/* Quick Stats - Only show when expanded */}
        {state === "expanded" && (
          <div className="px-3 py-2">
            <div className="flex items-center justify-between mb-2 px-2">
              <h4 className="text-xs font-semibold tracking-tight">
                Quick Stats
              </h4>
              <Button variant="ghost" size="sm">
                <RefreshCw className="size-3" />
              </Button>
            </div>

            {/* Maker Stats */}
            {currentView === "maker" && userStats && (
              <Card className="mb-3">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm">Your Trading</CardTitle>
                </CardHeader>
                <CardContent className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <Clock className="size-3 text-blue-500" />
                      <span className="text-muted-foreground">Active</span>
                    </div>
                    <Badge variant="secondary">{userStats.activeOrders}</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <CheckCircle className="size-3 text-green-500" />
                      <span className="text-muted-foreground">Filled</span>
                    </div>
                    <Badge variant="secondary">{userStats.filledOrders}</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <DollarSign className="size-3 text-yellow-500" />
                      <span className="text-muted-foreground">Volume</span>
                    </div>
                    <Badge variant="outline">{userStats.totalVolume}</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <Wallet className="size-3 text-purple-500" />
                      <span className="text-muted-foreground">Balance</span>
                    </div>
                    <Badge variant="outline">{userStats.tokenBalance}</Badge>
                  </div>
                </CardContent>
              </Card>
            )}

            {/* Taker Stats */}
            {currentView === "taker" && (
              <Card className="mb-3">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm">Market Overview</CardTitle>
                </CardHeader>
                <CardContent className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <BookOpen className="size-3 text-blue-500" />
                      <span className="text-muted-foreground">Available</span>
                    </div>
                    <Badge variant="secondary">247</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <TrendingUp className="size-3 text-green-500" />
                      <span className="text-muted-foreground">Best Spread</span>
                    </div>
                    <Badge variant="outline">0.1%</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <Activity className="size-3 text-orange-500" />
                      <span className="text-muted-foreground">24h Volume</span>
                    </div>
                    <Badge variant="outline">$125K</Badge>
                  </div>
                </CardContent>
              </Card>
            )}

            {/* Relayer Stats */}
            {currentView === "relayer" && systemStats && (
              <Card className="mb-3">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm">System Health</CardTitle>
                </CardHeader>
                <CardContent className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <BarChart3 className="size-3 text-blue-500" />
                      <span className="text-muted-foreground">
                        Total Orders
                      </span>
                    </div>
                    <Badge variant="secondary">{systemStats.totalOrders}</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <Clock className="size-3 text-green-500" />
                      <span className="text-muted-foreground">Today</span>
                    </div>
                    <Badge variant="secondary">{systemStats.ordersToday}</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <DollarSign className="size-3 text-yellow-500" />
                      <span className="text-muted-foreground">
                        Total Volume
                      </span>
                    </div>
                    <Badge variant="outline">{systemStats.totalVolume}</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <Activity className="size-3 text-purple-500" />
                      <span className="text-muted-foreground">
                        Active Users
                      </span>
                    </div>
                    <Badge variant="outline">{systemStats.activeUsers}</Badge>
                  </div>
                </CardContent>
              </Card>
            )}
          </div>
        )}
      </SidebarContent>

      <SidebarFooter>
        {/* Connection Status */}
        <div className="px-3 py-2">
          <div className="flex items-center justify-between text-sm">
            <div className="flex items-center gap-2">
              <div className="size-2 bg-green-500 rounded-full"></div>
              <span className="text-muted-foreground">ICP Network</span>
            </div>
            <Badge variant="secondary" className="text-xs">
              Connected
            </Badge>
          </div>

          {/* User Info */}
          {isAuthenticated && userPrincipal && state === "expanded" && (
            <div className="mt-2 pt-2 border-t">
              <div className="flex items-center gap-2">
                <User className="size-4 text-muted-foreground" />
                <div className="flex-1 min-w-0">
                  <p className="text-xs text-muted-foreground">Connected as:</p>
                  <p className="text-xs font-mono truncate">
                    {truncatePrincipal(userPrincipal)}
                  </p>
                </div>
                <Button variant="ghost" size="sm">
                  <Settings className="size-3" />
                </Button>
              </div>
            </div>
          )}
        </div>
      </SidebarFooter>
    </Sidebar>
  );
}
