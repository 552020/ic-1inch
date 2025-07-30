import React from "react";
import { SidebarProvider, SidebarTrigger } from "../ui/sidebar";
import { HeaderLegacy } from "./HeaderLegacy";
import { AppSidebar } from "./Sidebar";
import { Footer } from "./Footer";

interface MainLayoutProps {
  children: React.ReactNode;
  currentView: "maker" | "taker" | "relayer";
  onViewChange: (view: "maker" | "taker" | "relayer") => void;
  isAuthenticated: boolean;
  userPrincipal?: string;
  onLogin: () => void;
  onLogout: () => void;
}

export function MainLayout({
  children,
  currentView,
  onViewChange,
  isAuthenticated,
  userPrincipal,
  onLogin,
  onLogout,
}: MainLayoutProps) {
  // Mock data - in a real app, this would come from your state management/API
  const mockUserStats = {
    activeOrders: 3,
    filledOrders: 12,
    totalVolume: "$2.5K",
    tokenBalance: "1,250 ICP",
  };

  const mockSystemStats = {
    totalOrders: 1247,
    ordersToday: 89,
    totalVolume: "$125K",
    activeUsers: 234,
  };

  return (
    <SidebarProvider defaultOpen={true}>
      <div className="flex min-h-screen w-full">
        {/* Sidebar */}
        <AppSidebar
          currentView={currentView}
          onViewChange={onViewChange}
          userStats={mockUserStats}
          systemStats={mockSystemStats}
          isAuthenticated={isAuthenticated}
          userPrincipal={userPrincipal}
        />

        {/* Main Content Area */}
        <div className="flex flex-1 flex-col">
          {/* Header */}
          <HeaderLegacy
            isAuthenticated={isAuthenticated}
            userPrincipal={userPrincipal}
            onLogin={onLogin}
            onLogout={onLogout}
            currentView={currentView}
            onViewChange={onViewChange}
          />

          {/* Main Content */}
          <main className="flex-1 bg-muted/20">
            {/* Content with sidebar trigger for mobile */}
            <div className="p-4 md:p-6">
              <div className="mb-4 md:hidden">
                <SidebarTrigger />
              </div>
              {children}
            </div>
          </main>

          {/* Footer */}
          <Footer />
        </div>
      </div>
    </SidebarProvider>
  );
}
