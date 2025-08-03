const hre = require("hardhat");
const { ethers } = require("ethers");
require("dotenv").config();

async function unwrapWETH() {
  console.log("🔄 Unwrapping WETH back to ETH...\n");

  // Load environment variables
  const MAKER_PRIVATE_KEY = process.env.MAKER_PRIVATE_KEY;
  const MAKER_ADDRESS = process.env.MAKER_ADDRESS;

  if (!MAKER_PRIVATE_KEY || !MAKER_ADDRESS) {
    console.error(
      "❌ Error: Missing MAKER_PRIVATE_KEY or MAKER_ADDRESS in .env file"
    );
    console.log("📝 Please add to your .env file:");
    console.log("   MAKER_PRIVATE_KEY=0x...");
    console.log("   MAKER_ADDRESS=0x...");
    process.exit(1);
  }

  console.log(`🎯 Maker Address: ${MAKER_ADDRESS}`);
  console.log(`🔑 Maker Private Key: ${MAKER_PRIVATE_KEY.substring(0, 10)}...`);

  // WETH contract address on Base Sepolia
  const WETH_ADDRESS = "0x4200000000000000000000000000000000000006";

  // Create provider and wallet
  const provider = new ethers.JsonRpcProvider("https://sepolia.base.org");
  const makerWallet = new ethers.Wallet(MAKER_PRIVATE_KEY, provider);

  // Get WETH contract
  const wethContract = new ethers.Contract(
    WETH_ADDRESS,
    [
      "function withdraw(uint256 amount) external",
      "function balanceOf(address owner) view returns (uint256)",
      "function symbol() view returns (string)",
    ],
    makerWallet
  );

  // Check current balances
  const ethBalance = await provider.getBalance(MAKER_ADDRESS);
  const wethBalance = await wethContract.balanceOf(MAKER_ADDRESS);

  console.log("💰 Current Balances:");
  console.log(`   ETH: ${ethers.formatEther(ethBalance)} ETH`);
  console.log(`   WETH: ${ethers.formatEther(wethBalance)} WETH`);

  if (wethBalance == 0) {
    console.log("\n❌ Error: No WETH to unwrap!");
    console.log("💡 You need to wrap some ETH to WETH first");
    return {
      success: false,
      error: "No WETH balance to unwrap"
    };
  }

  // Amount to unwrap (all available WETH)
  const unwrapAmount = wethBalance;

  console.log(`\n🔄 Unwrapping ${ethers.formatEther(unwrapAmount)} WETH to ETH...`);

  try {
    // Unwrap WETH to ETH
    const tx = await wethContract.withdraw(unwrapAmount, {
      gasLimit: 200000,
    });

    console.log(`📝 Transaction hash: ${tx.hash}`);
    console.log(`⏳ Waiting for confirmation...`);

    // Wait for confirmation
    const receipt = await tx.wait();

    console.log(`\n✅ WETH unwrapped successfully!`);
    console.log(`📦 Block number: ${receipt.blockNumber}`);
    console.log(`💰 Gas used: ${receipt.gasUsed.toString()}`);

    // Check new balances
    const newEthBalance = await provider.getBalance(MAKER_ADDRESS);
    const newWethBalance = await wethContract.balanceOf(MAKER_ADDRESS);

    console.log(`\n💰 New Balances:`);
    console.log(`   ETH: ${ethers.formatEther(newEthBalance)} ETH`);
    console.log(`   WETH: ${ethers.formatEther(newWethBalance)} WETH`);

    console.log(`\n🎉 You got your ETH back!`);
    console.log(
      `🔗 View transaction: https://sepolia.basescan.org/tx/${tx.hash}`
    );

    return {
      success: true,
      txHash: tx.hash,
      receipt,
      ethGained: unwrapAmount,
    };
  } catch (error) {
    console.error("❌ Error unwrapping WETH:", error.message);

    if (error.message.includes("insufficient funds")) {
      console.log("💡 Solution: Add more ETH to the address for gas fees");
    }

    return {
      success: false,
      error: error.message,
    };
  }
}

async function main() {
  try {
    const result = await unwrapWETH();

    if (result.success) {
      console.log("\n🎉 WETH unwrapped successfully!");
      console.log("💎 You now have your ETH back!");
    } else {
      console.log("\n💥 Failed to unwrap WETH");
    }
  } catch (error) {
    console.error("❌ Error:", error);
    process.exit(1);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 