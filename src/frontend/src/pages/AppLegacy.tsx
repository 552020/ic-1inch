import { useState } from "react";
import { Button } from "../components/ui/button";
import { ArrowLeft } from "lucide-react";
import { MainLayout } from "../components/layout/MainLayout";
import { LoginScreen } from "../components/auth/LoginScreen";
import { MakerPage } from "./MakerPage";
import { TakerPage } from "./TakerPage";
import { RelayerPage } from "./RelayerPage";
import { useAuth } from "../hooks/useAuth";
import { useBackend } from "../hooks/useBackend";

interface AppLegacyProps {
  onBackToMain: () => void;
}

function AppLegacy({ onBackToMain }: AppLegacyProps) {
  const [currentView, setCurrentView] = useState<"maker" | "taker" | "relayer">(
    "maker"
  );
  const auth = useAuth();
  const backend = useBackend();

  const renderContent = () => {
    if (!auth.isAuthenticated) {
      return (
        <LoginScreen
          onLogin={auth.login}
          onTestConnection={backend.testConnection}
          loading={auth.loading || backend.loading}
          error={auth.error || backend.error}
        />
      );
    }

    switch (currentView) {
      case "maker":
        return (
          <MakerPage
            onCreateOrder={backend.createOrder}
            loading={backend.loading}
          />
        );
      case "taker":
        return (
          <TakerPage
            onFillOrder={backend.fillOrder}
            loading={backend.loading}
          />
        );
      case "relayer":
        return <RelayerPage />;
      default:
        return null;
    }
  };

  return (
    <div>
      {/* Back to Main App Button */}
      <div className="p-4 border-b">
        <Button
          variant="ghost"
          size="sm"
          onClick={onBackToMain}
          className="flex items-center gap-2"
        >
          <ArrowLeft className="w-4 h-4" />
          Back to Main App
        </Button>
      </div>

      <MainLayout
        currentView={currentView}
        onViewChange={setCurrentView}
        isAuthenticated={auth.isAuthenticated}
        userPrincipal={auth.userPrincipal}
        onLogin={auth.login}
        onLogout={auth.logout}
      >
        {renderContent()}
      </MainLayout>
    </div>
  );
}

export default AppLegacy;
