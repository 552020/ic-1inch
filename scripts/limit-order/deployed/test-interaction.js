const hre = require("hardhat");

async function main() {
  console.log(
    "🔍 Testing LimitOrderProtocol interactions on Base Sepolia...\n"
  );

  // Contract address from our deployment
  const LOP_ADDRESS = "0xdfC365795F146a6755998C5e916a592A9706eDC6";

  // Get the contract instance
  const lop = await hre.ethers.getContractAt("LimitOrderProtocol", LOP_ADDRESS);

  console.log("📋 Contract Details:");
  console.log(`   Address: ${LOP_ADDRESS}`);
  console.log(`   Network: ${hre.network.name}`);
  console.log(
    `   Chain ID: ${await hre.ethers.provider
      .getNetwork()
      .then((n) => n.chainId)}`
  );

  // Test 1: Get domain separator
  console.log("\n🔐 Testing DOMAIN_SEPARATOR...");
  try {
    const domainSeparator = await lop.DOMAIN_SEPARATOR();
    console.log(`   ✅ Domain Separator: ${domainSeparator}`);
  } catch (error) {
    console.log(`   ❌ Error getting domain separator: ${error.message}`);
  }

  // Test 2: Check if contract is paused
  console.log("\n⏸️  Testing pause status...");
  try {
    const isPaused = await lop.paused();
    console.log(`   📊 Contract paused: ${isPaused}`);
  } catch (error) {
    console.log(`   ❌ Error checking pause status: ${error.message}`);
  }

  // Test 3: Get owner
  console.log("\n👑 Testing owner...");
  try {
    const owner = await lop.owner();
    console.log(`   🏆 Owner: ${owner}`);
  } catch (error) {
    console.log(`   ❌ Error getting owner: ${error.message}`);
  }

  // Test 4: Get WETH address
  console.log("\n💎 Testing WETH address...");
  try {
    // We need to call the internal _WETH() function or check the constructor
    // Let's try to get it from the deployment info
    const deployment = await hre.deployments.get("LimitOrderProtocol");
    console.log(`   📦 Deployment args: ${JSON.stringify(deployment.args)}`);
  } catch (error) {
    console.log(`   ❌ Error getting WETH info: ${error.message}`);
  }

  // Test 5: Check contract balance
  console.log("\n💰 Testing contract balance...");
  try {
    const balance = await hre.ethers.provider.getBalance(LOP_ADDRESS);
    console.log(`   💰 ETH Balance: ${hre.ethers.formatEther(balance)} ETH`);
  } catch (error) {
    console.log(`   ❌ Error getting balance: ${error.message}`);
  }

  // Test 6: Test bitInvalidatorForOrder (should return 0 for new maker)
  console.log("\n🔢 Testing bitInvalidatorForOrder...");
  try {
    const [signer] = await hre.ethers.getSigners();
    const invalidator = await lop.bitInvalidatorForOrder(signer.address, 0);
    console.log(
      `   📊 Bit Invalidator for ${signer.address}, slot 0: ${invalidator}`
    );
  } catch (error) {
    console.log(`   ❌ Error getting bit invalidator: ${error.message}`);
  }

  console.log("\n✅ Basic interaction tests completed!");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("❌ Error:", error);
    process.exit(1);
  });
