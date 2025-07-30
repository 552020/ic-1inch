import { createConfig, http } from "wagmi";

import { base, baseSepolia } from "wagmi/chains";
import { walletConnect } from "wagmi/connectors";

// Replace with your own WalletConnect project ID
// Register for free at https://walletconnect.com/
const INFURA_PROJECT_ID: string =
  import.meta.env.VITE_INFURA_PROJECT_ID || "69b32fc409c14871bf13bf44487aa20b";

const WALLETCONNECT_PROJECT_ID: string =
  import.meta.env.VITE_WALLETCONNECT_PROJECT_ID ||
  "3936b3795b20eea5fe9282a3a80be958";

// RPC URLs with fallback to public endpoints if no Infura key is provided
const getRpcUrl = (network: string, fallback: string): string => {
  if (INFURA_PROJECT_ID) {
    return `https://${network}.infura.io/v3/${INFURA_PROJECT_ID}`;
  }
  console.warn(
    `No Infura project ID found for ${network}. Using public endpoint.`
  );
  return fallback;
};

export const wagmiConfig = createConfig({
  chains: [base, baseSepolia], // Support both Base Mainnet and Base Sepolia
  connectors: [walletConnect({ projectId: WALLETCONNECT_PROJECT_ID })],
  transports: {
    [base.id]: http(getRpcUrl("base-mainnet", "https://mainnet.base.org")),
    [baseSepolia.id]: http(
      getRpcUrl("base-sepolia", "https://sepolia.base.org")
    ),
  },
});

// Log configuration
console.log(`🔗 Infura configured: ${INFURA_PROJECT_ID ? "Yes" : "No"}`);
