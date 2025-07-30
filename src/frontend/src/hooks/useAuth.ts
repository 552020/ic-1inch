import { useState } from "react";

export interface AuthState {
  isAuthenticated: boolean;
  userPrincipal?: string;
  loading: boolean;
  error: string | null;
}

export function useAuth() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [userPrincipal, setUserPrincipal] = useState<string>();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const login = async () => {
    setLoading(true);
    setError(null);
    try {
      // Simulate Internet Identity login
      // In a real app, this would integrate with Internet Identity
      await new Promise((resolve) => setTimeout(resolve, 1000));
      setIsAuthenticated(true);
      setUserPrincipal("rdmx6-jaaaa-aaaah-qcaiq-cai");
    } catch {
      setError("Failed to authenticate");
    } finally {
      setLoading(false);
    }
  };

  const logout = () => {
    setIsAuthenticated(false);
    setUserPrincipal(undefined);
    setError(null);
  };

  return {
    isAuthenticated,
    userPrincipal,
    loading,
    error,
    login,
    logout,
  };
}
