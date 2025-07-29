import { useState } from "react";
import { Button } from "../ui/button";
import { Badge } from "../ui/badge";
import {
  User,
  LogOut,
  Settings,
  TrendingUp,
  BookOpen,
  BarChart3,
  Menu,
  X,
} from "lucide-react";

interface HeaderProps {
  isAuthenticated: boolean;
  userPrincipal?: string;
  onLogin: () => void;
  onLogout: () => void;
  currentView: "maker" | "taker" | "relayer";
  onViewChange: (view: "maker" | "taker" | "relayer") => void;
}

export function Header({
  isAuthenticated,
  userPrincipal,
  onLogin,
  onLogout,
  currentView,
  onViewChange,
}: HeaderProps) {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const truncatePrincipal = (principal: string) => {
    if (principal.length <= 10) return principal;
    return `${principal.slice(0, 5)}...${principal.slice(-5)}`;
  };

  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
    } catch (err) {
      console.error("Failed to copy: ", err);
    }
  };

  const navigation = [
    { id: "maker", label: "Create Orders", icon: TrendingUp },
    { id: "taker", label: "Order Book", icon: BookOpen },
    { id: "relayer", label: "Analytics", icon: BarChart3 },
  ];

  return (
    <header className="bg-white border-b border-gray-200 shadow-sm">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          {/* Logo and Brand */}
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <h1 className="text-xl font-bold text-gray-900">
                ICP Limit Orders
              </h1>
              <p className="text-xs text-gray-500">
                Powered by Internet Computer
              </p>
            </div>
          </div>

          {/* Desktop Navigation */}
          <nav className="hidden md:flex space-x-8">
            {navigation.map((item) => {
              const Icon = item.icon;
              return (
                <button
                  key={item.id}
                  onClick={() => {
                    onViewChange(item.id as "maker" | "taker" | "relayer");
                  }}
                  className={`flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                    currentView === item.id
                      ? "bg-blue-100 text-blue-700"
                      : "text-gray-600 hover:text-gray-900 hover:bg-gray-100"
                  }`}
                >
                  <Icon className="w-4 h-4 mr-2" />
                  {item.label}
                </button>
              );
            })}
          </nav>

          {/* User Authentication */}
          <div className="flex items-center space-x-4">
            {isAuthenticated ? (
              <div className="flex items-center space-x-3">
                {/* User Info */}
                <div className="hidden sm:flex items-center space-x-2">
                  <Badge
                    variant="outline"
                    className="flex items-center space-x-1"
                  >
                    <User className="w-3 h-3" />
                    <span
                      className="cursor-pointer text-xs"
                      onClick={() => {
                        if (userPrincipal) void copyToClipboard(userPrincipal);
                      }}
                      title={userPrincipal}
                    >
                      {userPrincipal
                        ? truncatePrincipal(userPrincipal)
                        : "Unknown"}
                    </span>
                  </Badge>
                  <Badge variant="secondary" className="text-xs">
                    Connected
                  </Badge>
                </div>

                {/* Settings Button */}
                <Button variant="ghost" size="sm">
                  <Settings className="w-4 h-4" />
                </Button>

                {/* Logout Button */}
                <Button
                  variant="outline"
                  size="sm"
                  onClick={onLogout}
                  className="flex items-center space-x-1"
                >
                  <LogOut className="w-4 h-4" />
                  <span className="hidden sm:inline">Logout</span>
                </Button>
              </div>
            ) : (
              <Button onClick={onLogin} className="flex items-center space-x-2">
                <User className="w-4 h-4" />
                <span>Connect Wallet</span>
              </Button>
            )}

            {/* Mobile menu button */}
            <div className="md:hidden">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  setMobileMenuOpen(!mobileMenuOpen);
                }}
              >
                {mobileMenuOpen ? (
                  <X className="w-5 h-5" />
                ) : (
                  <Menu className="w-5 h-5" />
                )}
              </Button>
            </div>
          </div>
        </div>

        {/* Mobile Navigation */}
        {mobileMenuOpen && (
          <div className="md:hidden border-t border-gray-200">
            <div className="px-2 pt-2 pb-3 space-y-1">
              {navigation.map((item) => {
                const Icon = item.icon;
                return (
                  <button
                    key={item.id}
                    onClick={() => {
                      onViewChange(item.id as "maker" | "taker" | "relayer");
                      setMobileMenuOpen(false);
                    }}
                    className={`flex items-center w-full px-3 py-2 rounded-md text-base font-medium transition-colors ${
                      currentView === item.id
                        ? "bg-blue-100 text-blue-700"
                        : "text-gray-600 hover:text-gray-900 hover:bg-gray-100"
                    }`}
                  >
                    <Icon className="w-5 h-5 mr-3" />
                    {item.label}
                  </button>
                );
              })}

              {/* Mobile User Info */}
              {isAuthenticated && userPrincipal && (
                <div className="px-3 py-2 border-t border-gray-200 mt-2">
                  <p className="text-sm text-gray-500">Connected as:</p>
                  <p
                    className="text-sm font-mono text-gray-900 cursor-pointer"
                    onClick={() => {
                      void copyToClipboard(userPrincipal);
                    }}
                  >
                    {truncatePrincipal(userPrincipal)}
                  </p>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </header>
  );
}
