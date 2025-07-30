import React, { useEffect } from "react";
import { useAccount, useChainId } from "wagmi";

import { isChainIdSupported } from "@/wagmi/is-chain-id-supported";
import { useSiwe } from "ic-siwe-js/react";

export default function AuthSync({ children }: { children: React.ReactNode }) {
	const { isConnected, address } = useAccount();
	const chainId = useChainId();
	const { clear, isInitializing, identity, identityAddress } = useSiwe();
  
	useEffect(() => {
	  if (!isConnected && identity) clear();
	}, [isConnected, identity, clear]);
  
	useEffect(() => {
	  if (!isChainIdSupported(chainId)) clear();
	}, [chainId, clear]);
  
	useEffect(() => {
	  if (identityAddress && address && address !== identityAddress) clear();
	}, [address, identityAddress, clear]);
  
	if (isInitializing) return null;
  
	return <>{children}</>; // ✅ safe passive wrapper
  }
  