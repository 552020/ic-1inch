import { useState } from "react";
import { Button } from "./components/ui/button";
import { Card, CardContent } from "./components/ui/card";
import { Separator } from "./components/ui/separator";
import AppLegacy from "./pages/AppLegacy";
import { GreetBetter } from "./components/GreetBetter";
import LoginPage from "./components/layout/LoginPage";
import Header from "./components/layout/Header";

function App() {
  const [showLegacy, setShowLegacy] = useState(false);

  if (showLegacy) {
    return <AppLegacy onBackToMain={() => setShowLegacy(false)} />;
  }

  return (
    <div className="min-h-screen bg-background">
      <Header />

      <main className="container mx-auto px-4 py-8">
        <div className="max-w-4xl mx-auto space-y-8">
          {/* Login Section */}
          <Card>
            <CardContent className="p-6">
              <LoginPage />
            </CardContent>
          </Card>

          <Separator />

          {/* Test Components Section */}
          <Card>
            <CardContent className="p-6">
              <GreetBetter />
            </CardContent>
          </Card>

          <Separator />

          {/* Legacy App Access */}
          <div className="text-center">
            <Button
              variant="outline"
              size="lg"
              onClick={() => {
                setShowLegacy(true);
              }}
            >
              Go to Legacy App
            </Button>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
