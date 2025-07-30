import { useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useActor } from "@/contexts/Actors";
import { useToast } from "@/hooks/use-toast";

export function GreetBetter() {
  const { actor } = useActor();
  const { toast } = useToast();

  const [name, setName] = useState("ICP");
  const [greeting, setGreeting] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  console.log("GreetBetter component rendered, actor:", actor);

  const handleGreet = async () => {
    console.log("GreetBetter handleGreet called, actor:", actor);

    if (!actor) {
      console.log("GreetBetter: Actor is undefined, returning early");
      return;
    }

    setIsLoading(true);
    try {
      console.log("GreetBetter: Calling actor.greet with name:", name);
      const result = await actor.greet(name);
      console.log("GreetBetter result:", result);
      setGreeting(result);
      toast({ description: "Greeting sent!" });
    } catch (err) {
      console.error("GreetBetter failed:", err);
      toast({ variant: "destructive", description: "Failed to greet" });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex flex-col items-center gap-4">
      <Input
        value={name}
        onChange={(e) => {
          setName(e.target.value);
        }}
      />
      <Button onClick={handleGreet} disabled={isLoading}>
        {isLoading ? "Greeting..." : "Greet"}
      </Button>
      {greeting && <p className="text-lg">{greeting}</p>}
    </div>
  );
}
