import { useState } from "react";
import { Button } from "./components/ui/button";
import AppLegacy from "./pages/AppLegacy";

function App() {
  const [showLegacy, setShowLegacy] = useState(false);

  if (showLegacy) {
    return <AppLegacy />;
  }

  return (
    <div className="min-h-screen flex items-center justify-center">
      <Button
        onClick={() => {
          setShowLegacy(true);
        }}
      >
        Go to Legacy App
      </Button>
    </div>
  );
}

export default App;
