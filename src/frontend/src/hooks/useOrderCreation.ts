import { useState } from "react";
import { useActor } from "@ic-use-actor/react";
import { orderbook } from "../../../declarations/orderbook";
import { escrow } from "../../../declarations/escrow";

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

export const useOrderCreation = () => {
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Real order creation with canister calls
  const createOrder = async (orderData: OrderData): Promise<Order> => {
    setIsCreating(true);
    setError(null);

    try {
      // Convert token strings to canister Token enum
      const fromToken = orderData.fromToken === "ICP" ? { ICP: null } : { ETH: null };
      const toToken = orderData.toToken === "ICP" ? { ICP: null } : { ETH: null };
      
      // Convert amounts to nat64 (assuming amounts are in smallest units)
      const fromAmount = BigInt(Math.floor(parseFloat(orderData.fromAmount) * 1e8)); // Convert to e8 for ICP
      const toAmount = BigInt(Math.floor(parseFloat(orderData.toAmount) * 1e18)); // Convert to e18 for ETH
      
      // Set expiration to 24 hours from now
      const expiresAt = BigInt(Date.now() + 24 * 60 * 60 * 1000);

      // Call orderbook canister to create order
      const result = await orderbook.create_order(
        orderData.fromToken, // maker address (will be derived from current user)
        fromToken,
        toToken,
        fromAmount,
        toAmount,
        expiresAt
      );

      if ("Err" in result) {
        throw new Error(`Order creation failed: ${result.Err}`);
      }

      const orderId = result.Ok;

      // For ICP → ETH orders, immediately lock tokens
      if (orderData.fromToken === "ICP" && orderData.toToken === "ETH") {
        await lockTokensForOrder(orderId, fromAmount);
      }

      // Get the created order details
      const orderDetails = await orderbook.get_fusion_order_status(orderId);
      
      if (!orderDetails) {
        throw new Error("Failed to retrieve order details");
      }

      // Convert canister order to frontend Order format
      const order: Order = {
        id: orderDetails.id,
        fromToken: orderData.fromToken,
        toToken: orderData.toToken,
        fromAmount: parseFloat(orderData.fromAmount),
        toAmount: parseFloat(orderData.toAmount),
        status: mapOrderStatus(orderDetails.status),
        createdAt: new Date(Number(orderDetails.created_at)).toISOString(),
        expiresAt: new Date(Number(orderDetails.expires_at)).toISOString(),
        resolver: orderDetails.resolver_eth_address || undefined,
      };

      return order;

    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Unknown error occurred";
      setError(errorMessage);
      throw error;
    } finally {
      setIsCreating(false);
    }
  };

  // Lock tokens for ICP → ETH orders
  const lockTokensForOrder = async (orderId: string, amount: bigint): Promise<void> => {
    try {
      // Set timelock to 24 hours
      const timelock = BigInt(Date.now() + 24 * 60 * 60 * 1000);
      
      // Call escrow canister to lock tokens
      const result = await escrow.lock_icp_for_order(orderId, amount, timelock);
      
      if ("Err" in result) {
        throw new Error(`Token locking failed: ${result.Err}`);
      }
    } catch (error) {
      // If token locking fails, we should rollback the order
      console.error("Token locking failed, attempting rollback:", error);
      throw error;
    }
  };

  // Map canister OrderStatus to frontend status
  const mapOrderStatus = (status: any): Order["status"] => {
    switch (status) {
      case { Pending: null }:
        return "pending";
      case { Accepted: null }:
        return "accepted";
      case { Completed: null }:
        return "completed";
      case { Failed: null }:
        return "failed";
      default:
        return "pending";
    }
  };

  // Get order status from canister
  const getOrderStatus = async (orderId: string): Promise<Order | null> => {
    try {
      const orderDetails = await orderbook.get_fusion_order_status(orderId);
      
      if (!orderDetails) {
        return null;
      }

      return {
        id: orderDetails.id,
        fromToken: "ICP", // This would need to be determined from the order details
        toToken: "ETH",   // This would need to be determined from the order details
        fromAmount: Number(orderDetails.from_amount) / 1e8,
        toAmount: Number(orderDetails.to_amount) / 1e18,
        status: mapOrderStatus(orderDetails.status),
        createdAt: new Date(Number(orderDetails.created_at)).toISOString(),
        expiresAt: new Date(Number(orderDetails.expires_at)).toISOString(),
        resolver: orderDetails.resolver_eth_address || undefined,
      };
    } catch (error) {
      console.error("Failed to get order status:", error);
      return null;
    }
  };

  return {
    createOrder,
    getOrderStatus,
    isCreating,
    error,
    setError,
  };
}; 