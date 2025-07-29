import React from "react";
import { Badge } from "../ui/badge";
import { Button } from "../ui/button";
import { Separator } from "../ui/separator";
import {
  Github,
  BookOpen,
  Activity,
  Clock,
  Shield,
  Zap,
  ExternalLink,
} from "lucide-react";

interface FooterProps {
  systemStatus?: {
    blockHeight: number;
    lastUpdate: string;
    cycleBalance: string;
    networkLatency: number;
  };
}

export function Footer({ systemStatus }: FooterProps) {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="bg-background border-t border-border">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* System Status Bar */}
        <div className="py-3 border-b border-border">
          <div className="flex flex-wrap items-center justify-between gap-4 text-sm">
            <div className="flex items-center gap-6">
              <div className="flex items-center gap-2">
                <div className="size-2 bg-green-500 rounded-full animate-pulse"></div>
                <span className="text-muted-foreground">ICP Network</span>
                <Badge variant="secondary" className="text-xs">
                  Online
                </Badge>
              </div>

              {systemStatus && (
                <>
                  <div className="flex items-center gap-2">
                    <Activity className="size-3 text-blue-500" />
                    <span className="text-muted-foreground">
                      Block: {systemStatus.blockHeight.toLocaleString()}
                    </span>
                  </div>

                  <div className="flex items-center gap-2">
                    <Clock className="size-3 text-green-500" />
                    <span className="text-muted-foreground">
                      Updated: {systemStatus.lastUpdate}
                    </span>
                  </div>

                  <div className="flex items-center gap-2">
                    <Zap className="size-3 text-yellow-500" />
                    <span className="text-muted-foreground">
                      Cycles: {systemStatus.cycleBalance}
                    </span>
                  </div>

                  <div className="flex items-center gap-2">
                    <Shield className="size-3 text-purple-500" />
                    <span className="text-muted-foreground">
                      Latency: {systemStatus.networkLatency}ms
                    </span>
                  </div>
                </>
              )}
            </div>

            <div className="flex items-center gap-2">
              <Badge variant="outline" className="text-xs">
                v1.0.0-MVP
              </Badge>
              <Badge variant="secondary" className="text-xs">
                Testnet
              </Badge>
            </div>
          </div>
        </div>

        {/* Main Footer Content */}
        <div className="py-8">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
            {/* Protocol Info */}
            <div className="space-y-4">
              <div>
                <h3 className="font-semibold text-foreground">
                  ICP Limit Orders
                </h3>
                <p className="text-sm text-muted-foreground mt-1">
                  Decentralized limit order protocol on Internet Computer
                </p>
              </div>
              <div className="space-y-2">
                <p className="text-xs text-muted-foreground">
                  {`Powered by ICP's reverse gas model`}
                </p>
                <p className="text-xs text-muted-foreground">
                  Zero fees for order creation
                </p>
                <p className="text-xs text-muted-foreground">
                  ICRC-1/ICRC-2 token support
                </p>
              </div>
            </div>

            {/* Resources */}
            <div className="space-y-4">
              <h4 className="font-medium text-foreground">Resources</h4>
              <div className="space-y-2">
                <Button
                  variant="ghost"
                  size="sm"
                  className="justify-start p-0 h-auto"
                >
                  <BookOpen className="size-3 mr-2" />
                  <span className="text-sm">Documentation</span>
                  <ExternalLink className="size-3 ml-1" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  className="justify-start p-0 h-auto"
                >
                  <Github className="size-3 mr-2" />
                  <span className="text-sm">GitHub Repository</span>
                  <ExternalLink className="size-3 ml-1" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  className="justify-start p-0 h-auto"
                >
                  <Activity className="size-3 mr-2" />
                  <span className="text-sm">API Reference</span>
                  <ExternalLink className="size-3 ml-1" />
                </Button>
              </div>
            </div>

            {/* Protocol Stats */}
            <div className="space-y-4">
              <h4 className="font-medium text-foreground">Protocol Stats</h4>
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Total Orders</span>
                  <span className="font-medium">1,247</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">24h Volume</span>
                  <span className="font-medium">$125K</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Active Users</span>
                  <span className="font-medium">89</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Uptime</span>
                  <span className="font-medium text-green-600">99.9%</span>
                </div>
              </div>
            </div>

            {/* Technical Info */}
            <div className="space-y-4">
              <h4 className="font-medium text-foreground">Technical</h4>
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Canister ID</span>
                  <span className="font-mono text-xs">rdmx6-jaaaa...</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Network</span>
                  <span className="font-medium">Internet Computer</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Protocol</span>
                  <span className="font-medium">ICRC-1/ICRC-2</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Security</span>
                  <span className="font-medium text-green-600">Audited</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <Separator />

        {/* Bottom Footer */}
        <div className="py-4">
          <div className="flex flex-col sm:flex-row justify-between items-center gap-4 text-sm text-muted-foreground">
            <div className="flex items-center gap-4">
              <span>© {currentYear} ICP Limit Orders Protocol</span>
              <span>•</span>
              <span>Built for ChainFusion+</span>
            </div>

            <div className="flex items-center gap-4">
              <Button variant="ghost" size="sm" className="h-auto p-0">
                Privacy Policy
              </Button>
              <Button variant="ghost" size="sm" className="h-auto p-0">
                Terms of Service
              </Button>
              <Button variant="ghost" size="sm" className="h-auto p-0">
                Support
              </Button>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
}
