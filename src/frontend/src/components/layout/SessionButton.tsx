import React from "react";
import { Button } from "../ui/button";
import { User } from "lucide-react";
import { useSiwe } from "ic-siwe-js/react";

export default function SessionButton() {
  const { identity } = useSiwe();

  const truncatePrincipal = (principal: string) => {
    if (principal.length <= 10) return principal;
    return `${principal.slice(0, 5)}...${principal.slice(-5)}`;
  };

  return (
    <Button
      variant="outline"
      size="sm"
      disabled={!identity}
      className="flex items-center gap-2"
    >
      <User className="w-4 h-4" />
      <span className="hidden sm:inline">
        {identity
          ? truncatePrincipal(identity.getPrincipal().toString())
          : "Not Connected"}
      </span>
    </Button>
  );
}
