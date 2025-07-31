import { ethers } from "hardhat";

async function main() {
  console.log("🚀 Deploying Fusion+ Mechanical Turk contracts...");

  // Get the deployer account
  const [deployer] = await ethers.getSigners();
  console.log("📝 Deploying contracts with account:", deployer.address);
  console.log("💰 Account balance:", (await deployer.getBalance()).toString());

  // Deploy FusionEscrow
  console.log("\n📦 Deploying FusionEscrow...");
  const FusionEscrow = await ethers.getContractFactory("FusionEscrow");
  const fusionEscrow = await FusionEscrow.deploy();
  await fusionEscrow.waitForDeployment();
  const fusionEscrowAddress = await fusionEscrow.getAddress();
  console.log("✅ FusionEscrow deployed to:", fusionEscrowAddress);

  // Deploy TestICP
  console.log("\n📦 Deploying TestICP...");
  const TestICP = await ethers.getContractFactory("TestICP");
  const testICP = await TestICP.deploy();
  await testICP.waitForDeployment();
  const testICPAddress = await testICP.getAddress();
  console.log("✅ TestICP deployed to:", testICPAddress);

  // Deploy TestETH
  console.log("\n📦 Deploying TestETH...");
  const TestETH = await ethers.getContractFactory("TestETH");
  const testETH = await TestETH.deploy();
  await testETH.waitForDeployment();
  const testETHAddress = await testETH.getAddress();
  console.log("✅ TestETH deployed to:", testETHAddress);

  // Mint some test tokens to the deployer
  console.log("\n🪙 Minting test tokens...");
  const mintAmount = ethers.parseEther("1000"); // 1000 tokens each
  
  await testICP.mint(deployer.address, mintAmount);
  console.log("✅ Minted 1000 TestICP to deployer");
  
  await testETH.mint(deployer.address, mintAmount);
  console.log("✅ Minted 1000 TestETH to deployer");

  // Authorize deployer as a resolver (for testing)
  console.log("\n🔐 Authorizing deployer as resolver...");
  await fusionEscrow.authorizeResolver(deployer.address);
  console.log("✅ Deployer authorized as resolver");

  console.log("\n🎉 Deployment complete!");
  console.log("\n📋 Contract Addresses:");
  console.log("FusionEscrow:", fusionEscrowAddress);
  console.log("TestICP:", testICPAddress);
  console.log("TestETH:", testETHAddress);
  console.log("\n🔑 Deployer (Resolver):", deployer.address);

  // Save deployment info for frontend
  const deploymentInfo = {
    network: "sepolia",
    deployer: deployer.address,
    contracts: {
      fusionEscrow: fusionEscrowAddress,
      testICP: testICPAddress,
      testETH: testETHAddress,
    },
    timestamp: new Date().toISOString(),
  };

  console.log("\n💾 Deployment info:", JSON.stringify(deploymentInfo, null, 2));
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("❌ Deployment failed:", error);
    process.exit(1);
  }); 