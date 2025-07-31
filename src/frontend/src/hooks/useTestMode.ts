import { useState } from "react";

export interface OrderData {
  fromToken: string;
  toToken: string;
  fromAmount: string;
  toAmount: string;
}

export interface Order {
  id: string;
  fromToken: string;
  toToken: string;
  fromAmount: number;
  toAmount: number;
  status:
    | "pending"
    | "accepted"
    | "awaiting_confirmation"
    | "awaiting_secret"
    | "completed"
    | "failed";
  createdAt: string;
  expiresAt: string;
  resolver?: string;
  secret?: string;
  relayerMessage?: string;
}

// Test mode configuration
const TEST_MODE_ENABLED = true; // Default to test mode for development
const REALISTIC_DELAY = () => 1000 + Math.random() * 1000; // 1-2 seconds
const RELAYER_VERIFICATION_DELAY = 3000; // 3 seconds for relayer verification
const INSUFFICIENT_BALANCE_THRESHOLD = 1000; // Simulate error when amount > 1000

export const useTestMode = () => {
  const [testMode, setTestMode] = useState(TEST_MODE_ENABLED);

  // Simulate realistic order creation with atomic flow
  const simulateOrderCreation = async (
    orderData: OrderData
  ): Promise<Order> => {
    // Simulate network delay
    await new Promise((resolve) => setTimeout(resolve, REALISTIC_DELAY()));

    // Basic error simulation for test mode
    if (
      testMode &&
      parseFloat(orderData.fromAmount) > INSUFFICIENT_BALANCE_THRESHOLD
    ) {
      throw new Error("Insufficient balance");
    }

    return {
      id: `fusion_${Date.now().toString()}_${Math.random()
        .toString(36)
        .substring(2, 11)}`,
      fromToken: orderData.fromToken,
      toToken: orderData.toToken,
      fromAmount: parseFloat(orderData.fromAmount),
      toAmount: parseFloat(orderData.toAmount),
      status: "pending",
      createdAt: new Date().toISOString(),
      expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(),
    };
  };

  // Simulate realistic token locking (atomic step)
  const simulateTokenLocking = async (_orderData: OrderData): Promise<void> => {
    // Simulate network delay for token locking
    await new Promise((resolve) => setTimeout(resolve, REALISTIC_DELAY()));

    // Simulate token locking failure (10% chance in test mode)
    if (testMode && Math.random() < 0.1) {
      throw new Error("Token locking failed");
    }
  };

  // Simulate relayer verification and confirmation request
  const simulateRelayerVerification = async (order: Order): Promise<Order> => {
    // Simulate relayer checking both chains
    await new Promise((resolve) =>
      setTimeout(resolve, RELAYER_VERIFICATION_DELAY)
    );

    return {
      ...order,
      status: "awaiting_confirmation" as const,
      relayerMessage:
        "✅ All assets verified and locked on both chains. Please confirm and share your secret to complete the swap.",
    };
  };

  // Simulate order rollback if token locking fails
  const simulateOrderRollback = async (
    _orderData: OrderData
  ): Promise<void> => {
    await new Promise((resolve) => setTimeout(resolve, 500));
    console.log("Order rollback simulated");
  };

  return {
    testMode,
    setTestMode,
    simulateOrderCreation,
    simulateTokenLocking,
    simulateRelayerVerification,
    simulateOrderRollback,
  };
};
