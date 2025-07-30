import { OrderBook } from "../components/taker/OrderBook";

interface TakerPageProps {
  onFillOrder: (orderId: string) => Promise<void>;
  loading: boolean;
}

export function TakerPage({ onFillOrder, loading }: TakerPageProps) {
  const handleFillOrder = async (orderId: string) => {
    try {
      await onFillOrder(orderId);
      // TODO: Show success message
      alert(`Order ${orderId} filled successfully!`);
    } catch (err) {
      // Error is handled in the hook
      console.error("Failed to fill order:", err);
    }
  };

  return (
    <div className="space-y-6">
      <div className="text-center">
        <h1 className="text-3xl font-bold">Order Book</h1>
        <p className="text-muted-foreground mt-2">
          Browse and fill available limit orders
        </p>
      </div>
      <OrderBook onFillOrder={handleFillOrder} isLoading={loading} />
    </div>
  );
}
