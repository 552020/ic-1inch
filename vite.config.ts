import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import dotenv from "dotenv";
import environment from "vite-plugin-environment";
import path from "path";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

dotenv.config({ path: ".env" });

// Debug environment loading
console.log("🔍 Vite Config Debug:");
console.log("  • Loading .env from:", ".env");
console.log(
  "  • CANISTER_ID_IC_SIWE_PROVIDER:",
  process.env.CANISTER_ID_IC_SIWE_PROVIDER
);
console.log("  • All CANISTER_ env vars:");
Object.keys(process.env).forEach((key) => {
  if (key.startsWith("CANISTER_")) {
    console.log(`    - ${key}:`, process.env[key]);
  }
});

// https://vite.dev/config/
export default defineConfig({
  define: {
    "process.env.CANISTER_ID_IC_SIWE_PROVIDER": JSON.stringify(process.env.CANISTER_ID_IC_SIWE_PROVIDER),
    "process.env.CANISTER_ID_ORDERBOOK": JSON.stringify(process.env.CANISTER_ID_ORDERBOOK),
    "process.env.CANISTER_ID_ESCROW": JSON.stringify(process.env.CANISTER_ID_ESCROW),
    "process.env.CANISTER_ID_TEST_TOKEN_ICP": JSON.stringify(process.env.CANISTER_ID_TEST_TOKEN_ICP),
    "process.env.CANISTER_ID_TEST_TOKEN_ETH": JSON.stringify(process.env.CANISTER_ID_TEST_TOKEN_ETH),
    "process.env.DFX_NETWORK": JSON.stringify(process.env.DFX_NETWORK),
  },
  root: "src/frontend",
  build: {
    outDir: "dist",
    target: "ES2022",
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
      },
    },
  },
  plugins: [
    wasm(),
    topLevelAwait(),
    react(),
    environment("all", { prefix: "CANISTER_", defineOn: "import.meta.env" }),
    environment("all", { prefix: "DFX_", defineOn: "import.meta.env" }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src/frontend/src"),
    },
  },
});
