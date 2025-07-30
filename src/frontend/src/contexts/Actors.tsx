/* eslint-disable react-refresh/only-export-components */
import {
  ActorProvider,
  InterceptorErrorData,
  InterceptorRequestData,
  createActorContext,
  createUseActorHook,
} from "ic-use-actor";
import { canisterId, idlFactory } from "../../../declarations/backend";
import { ReactNode } from "react";
import { _SERVICE } from "../../../declarations/backend/backend.did";
import { useSiwe } from "ic-siwe-js/react";
import { useToast } from "@/hooks/use-toast";
import { Principal } from "@dfinity/principal";
import { AnonymousIdentity } from "@dfinity/agent";

const actorContext = createActorContext<_SERVICE>();
export const useActor = createUseActorHook<_SERVICE>(actorContext);

// Hook to get backend principal
export const useBackendPrincipal = () => {
  try {
    return Principal.fromText(canisterId);
  } catch (error) {
    console.error("Failed to create backend principal:", error);
    return null;
  }
};

export default function Actors({ children }: { children: ReactNode }) {
  const { toast } = useToast();
  const { identity: siweIdentity, clear } = useSiwe();

  // Use SIWE identity if available, otherwise fallback to anonymous
  const identity = siweIdentity || new AnonymousIdentity();

  const errorToast = (error: unknown) => {
    if (typeof error === "object" && error !== null && "message" in error) {
      toast({
        variant: "destructive",
        description: error.message as string,
      });
    }
  };

  const handleResponseError = (data: InterceptorErrorData) => {
    const { error } = data;
    console.error("onResponseError", error);
    if (
      error instanceof Error &&
      (error.message.includes("Invalid delegation") ||
        error.message.includes("Specified sender delegation has expired") ||
        error.message.includes("Invalid certificate"))
    ) {
      toast({
        variant: "destructive",
        description: "Invalid delegation. Please log in again.",
      });
      setTimeout(() => {
        clear(); // Will trigger SIWE logout
        window.location.reload();
      }, 1000);
      return;
    }

    if (typeof data === "object" && "message" in data) {
      errorToast(data);
    }
  };

  const handleRequest = (data: InterceptorRequestData) => {
    console.log("onRequest", data.args, data.methodName);
    return data.args;
  };

  return (
    <ActorProvider<_SERVICE>
      canisterId={canisterId}
      context={actorContext}
      identity={identity}
      idlFactory={idlFactory}
      onRequest={handleRequest}
      onRequestError={errorToast}
      onResponseError={handleResponseError}
    >
      {children}
    </ActorProvider>
  );
}
