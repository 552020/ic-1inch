import { CreateOrderForm } from "../components/maker/CreateOrderForm";

interface MakerPageProps {
  onCreateOrder: (data: any) => Promise<void>;
  loading: boolean;
}

export function MakerPage({ onCreateOrder, loading }: MakerPageProps) {
  const handleSubmit = async (data: any) => {
    try {
      await onCreateOrder(data);
      // TODO: Show success message
      alert("Order created successfully!");
    } catch (err) {
      // Error is handled in the hook
      console.error("Failed to create order:", err);
    }
  };

  return (
    <div className="space-y-6">
      <div className="text-center">
        <h1 className="text-3xl font-bold">Create Limit Order</h1>
        <p className="text-muted-foreground mt-2">
          Set your desired exchange rate and let takers fill your order
        </p>
      </div>
      <CreateOrderForm onSubmit={handleSubmit} isLoading={loading} />
    </div>
  );
}
