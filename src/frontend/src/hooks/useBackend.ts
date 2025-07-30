import { useState } from "react";
import { backend } from "../../../declarations/backend";

export function useBackend() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const testConnection = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await backend.greet("ICP Limit Orders");
      return result;
    } catch (err) {
      console.error("Error calling backend:", err);
      const errorMessage =
        "Failed to connect to backend. Make sure dfx is running and deployed.";
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  };

  const createOrder = async (orderData: any): Promise<void> => {
    setLoading(true);
    setError(null);
    try {
      console.log("Creating order:", orderData);
      // TODO: Call actual backend.create_order(...);

      // Simulate order creation for now
      await new Promise((resolve) => setTimeout(resolve, 2000));
    } catch (err) {
      console.error("Error creating order:", err);
      const errorMessage =
        "Failed to create order. Make sure backend is running.";
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  };

  const fillOrder = async (orderId: string): Promise<void> => {
    setLoading(true);
    setError(null);
    try {
      console.log("Filling order:", orderId);
      // TODO: Call actual backend.fill_order(orderId);

      // Simulate order filling for now
      await new Promise((resolve) => setTimeout(resolve, 1500));
    } catch (err) {
      console.error("Error filling order:", err);
      const errorMessage =
        "Failed to fill order. Make sure backend is running.";
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  };

  return {
    loading,
    error,
    testConnection,
    createOrder,
    fillOrder,
  };
}
