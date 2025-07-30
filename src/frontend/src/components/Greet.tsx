// src/components/Greet.tsx
import { useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useActor } from "@/contexts/Actors";

export function Greet() {
  const { actor } = useActor();
  const [name, setName] = useState("ICP");
  const [greeting, setGreeting] = useState("");

  console.log("Greet component rendered, actor:", actor);

  const handleGreet = async () => {
    console.log("handleGreet called, actor:", actor);

    if (!actor) {
      console.log("Actor is undefined, returning early");
      return;
    }

    try {
      console.log("Calling actor.greet with name:", name);
      const result = await actor.greet(name);
      console.log("Greet result:", result);
      setGreeting(result);
    } catch (err) {
      console.error("Greet failed:", err);
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
      <Button onClick={handleGreet}>Greet</Button>
      {greeting && <p className="text-lg">{greeting}</p>}
    </div>
  );
}
