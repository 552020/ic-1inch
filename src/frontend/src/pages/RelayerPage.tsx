import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

export function RelayerPage() {
  return (
    <div className="space-y-6">
      <div className="text-center">
        <h1 className="text-3xl font-bold">Analytics Dashboard</h1>
        <p className="text-muted-foreground mt-2">
          Monitor system performance and statistics
        </p>
      </div>
      <Card>
        <CardContent className="p-8 text-center">
          <div className="space-y-4">
            <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mx-auto">
              <span className="text-2xl">📊</span>
            </div>
            <h3 className="text-xl font-semibold">Analytics Coming Soon</h3>
            <p className="text-muted-foreground">
              Advanced analytics and monitoring tools are being developed
            </p>
            <Badge variant="secondary">Under Development</Badge>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
