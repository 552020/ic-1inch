import { ethers } from "hardhat";

async function main() {
  console.log("🔧 Setting up test environment...");

  // Get the deployer account
  const [deployer] = await ethers.getSigners();
  console.log("📝 Setup account:", deployer.address);

  // Get contract instances
  const fusionEscrowAddress = process.env.FUSION_ESCROW_ADDRESS;
  const testICPAddress = process.env.TEST_ICP_ADDRESS;
  const testETHAddress = process.env.TEST_ETH_ADDRESS;

  if (!fusionEscrowAddress || !testICPAddress || !testETHAddress) {
    console.error("❌ Missing contract addresses in environment variables");
    console.log("Please set:");
    console.log("- FUSION_ESCROW_ADDRESS");
    console.log("- TEST_ICP_ADDRESS");
    console.log("- TEST_ETH_ADDRESS");
    process.exit(1);
  }

  const fusionEscrow = await ethers.getContractAt("FusionEscrow", fusionEscrowAddress);
  const testICP = await ethers.getContractAt("TestICP", testICPAddress);
  const testETH = await ethers.getContractAt("TestETH", testETHAddress);

  console.log("\n📋 Contract Addresses:");
  console.log("FusionEscrow:", fusionEscrowAddress);
  console.log("TestICP:", testICPAddress);
  console.log("TestETH:", testETHAddress);

  // Check if deployer is authorized as resolver
  const isAuthorized = await fusionEscrow.authorizedResolvers(deployer.address);
  console.log("\n🔐 Deployer authorized as resolver:", isAuthorized);

  if (!isAuthorized) {
    console.log("⚠️  Deployer not authorized as resolver. This is needed for testing.");
  }

  // Check token balances
  const icpBalance = await testICP.balanceOf(deployer.address);
  const ethBalance = await testETH.balanceOf(deployer.address);
  
  console.log("\n💰 Token Balances:");
  console.log("TestICP:", ethers.formatEther(icpBalance));
  console.log("TestETH:", ethers.formatEther(ethBalance));

  // Check ETH balance
  const ethBalanceWei = await deployer.getBalance();
  console.log("ETH:", ethers.formatEther(ethBalanceWei));

  console.log("\n✅ Test environment setup complete!");
  console.log("\n📝 Next steps:");
  console.log("1. Run tests: npx hardhat test");
  console.log("2. Deploy to Sepolia: npx hardhat run scripts/deploy.ts --network sepolia");
  console.log("3. Verify contracts on Etherscan");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("❌ Setup failed:", error);
    process.exit(1);
  }); 