import { useState } from "react";
import { Button } from "./components/ui/button";
import AppLegacy from "./pages/AppLegacy";
import { Greet } from "./components/Greet";
import { GreetBetter } from "./components/GreetBetter";

function App() {
  const [showLegacy, setShowLegacy] = useState(false);

  if (showLegacy) {
    return <AppLegacy />;
  }

  return (
    <div className="min-h-screen flex flex-col items-center justify-center gap-8 p-8">
      <Greet />
      <GreetBetter />
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
