import React from "react";

export function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="bg-background border-t border-border mt-auto">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        <div className="text-center space-y-2">
          <div className="text-sm text-muted-foreground">
            © {currentYear} ICP Limit Orders Protocol • MVP for ChainFusion+
          </div>
          <div className="text-xs text-muted-foreground">
            Powered by Internet Computer • Zero gas fees
          </div>
        </div>
      </div>
    </footer>
  );
}
