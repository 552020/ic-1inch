import { useState } from "react";
import { MainLayout } from "../components/layout/MainLayout";
import { LoginScreen } from "../components/auth/LoginScreen";
import { MakerPage } from "./MakerPage";
import { TakerPage } from "./TakerPage";
import { RelayerPage } from "./RelayerPage";
import { useAuth } from "../hooks/useAuth";
import { useBackend } from "../hooks/useBackend";

function AppLegacy() {
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
  );
}

export default AppLegacy;
