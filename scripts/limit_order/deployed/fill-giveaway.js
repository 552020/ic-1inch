const hre = require("hardhat");
const { ethers } = require("ethers");
require("dotenv").config();

async function fillGiveawayOrder() {
  console.log("🎁 Filling Giveaway Order on Base Sepolia...\n");

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

  console.log(`🎯 Taker Address: ${MAKER_ADDRESS}`);
  console.log(`🔑 Taker Private Key: ${MAKER_PRIVATE_KEY.substring(0, 10)}...`);

  // Contract address
  const LOP_ADDRESS = "0xdfC365795F146a6755998C5e916a592A9706eDC6";

  // Create provider and wallet
  const provider = new ethers.JsonRpcProvider("https://sepolia.base.org");
  const takerWallet = new ethers.Wallet(MAKER_PRIVATE_KEY, provider);

  // Get the contract instance
  const lop = new ethers.Contract(
    LOP_ADDRESS,
    [
      "function fillOrder(tuple(uint256 salt, address maker, address receiver, address makerAsset, address takerAsset, uint256 makingAmount, uint256 takingAmount, uint256 makerTraits) order, bytes32 r, bytes32 vs, uint256 amount, uint256 takerTraits) external payable returns(uint256 makingAmount, uint256 takingAmount, bytes32 orderHash)",
    ],
    takerWallet
  );

  // The giveaway order details (from giveaway-1.txt)
  const giveawayOrder = {
    salt: "0x94861106fbd44d75f10ee28c0409f002654b038664c4c8e4ff8a2b3a6dac8718",
    maker: "0x8CB80b37cc7193D0f055b1189F25eB903D888D3A",
    receiver: "0x8CB80b37cc7193D0f055b1189F25eB903D888D3A",
    makerAsset: "0x4200000000000000000000000000000000000006", // WETH
    takerAsset: "0x0000000000000000000000000000000000000000", // Zero address (nothing)
    makingAmount: ethers.parseEther("0.01").toString(), // 0.01 ETH
    takingAmount: "0", // Zero - you want nothing back!
    makerTraits:
      "0x0000000000000000000000000000000000000000000000000000000000000000",
  };

  // Signature components from the giveaway order
  const r =
    "0xde244154d111984ccee9a38dca3f6430eeaff05256df7e802220764e05530926";
  const vs =
    "0x59b15dca28a74a14a150512e7812ff2ee9bf410f90496edc8e809de1357bf0e3";

  console.log("📋 Giveaway Order Details:");
  console.log(`   Maker: ${giveawayOrder.maker}`);
  console.log(`   Taker: ${MAKER_ADDRESS}`);
  console.log(`   Maker Asset: ${giveawayOrder.makerAsset} (WETH)`);
  console.log(
    `   Making Amount: ${ethers.formatEther(giveawayOrder.makingAmount)} ETH`
  );
  console.log(`   Taking Amount: ${giveawayOrder.takingAmount} (ZERO!)`);

  // Check taker's ETH balance
  const takerBalance = await provider.getBalance(MAKER_ADDRESS);
  console.log(
    `\n💰 Taker ETH Balance: ${ethers.formatEther(takerBalance)} ETH`
  );

  if (takerBalance < ethers.parseEther("0.001")) {
    console.log("⚠️  Warning: Low ETH balance for gas fees!");
    console.log("💡 Consider getting some ETH from Base Sepolia faucet");
  }

  // Fill amount (full order)
  const amount = giveawayOrder.makingAmount;
  const takerTraits =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

  console.log(`\n🎯 Filling Order:`);
  console.log(`   Amount: ${ethers.formatEther(amount)} ETH`);
  console.log(
    `   Value: 0 ETH (giveaway - no ETH needed, only gas fees)`
  );

  try {
    console.log("\n🚀 Executing transaction...");

    // Fill the order
    const tx = await lop.fillOrder(giveawayOrder, r, vs, amount, takerTraits, {
      value: "0", // No ETH needed for giveaway - only gas fees
      gasLimit: 500000, // Adjust if needed
    });

    console.log(`📝 Transaction hash: ${tx.hash}`);
    console.log(`⏳ Waiting for confirmation...`);

    // Wait for confirmation
    const receipt = await tx.wait();

    console.log(`\n✅ Giveaway completed successfully!`);
    console.log(`📦 Block number: ${receipt.blockNumber}`);
    console.log(`💰 Gas used: ${receipt.gasUsed.toString()}`);
    console.log(
      `💸 Gas price: ${ethers.formatUnits(receipt.gasPrice, "gwei")} gwei`
    );

    // Check the result
    const logs = receipt.logs;
    console.log(`📊 Transaction logs: ${logs.length} events`);

    // Check new balance
    const newBalance = await provider.getBalance(MAKER_ADDRESS);
    console.log(
      `\n💰 New Taker ETH Balance: ${ethers.formatEther(newBalance)} ETH`
    );
    console.log(`📈 Balance change: +${ethers.formatEther(amount)} ETH`);

    console.log(`\n🎁 SUCCESS! ${MAKER_ADDRESS} received 0.01 ETH for free!`);
    console.log(
      `🔗 View transaction: https://sepolia.basescan.org/tx/${tx.hash}`
    );

    return {
      success: true,
      txHash: tx.hash,
      receipt,
      amountReceived: amount,
    };
  } catch (error) {
    console.error("❌ Error filling giveaway order:", error.message);

    if (error.message.includes("insufficient funds")) {
      console.log(
        "💡 Solution: Add more ETH to the taker address for gas fees"
      );
    } else if (error.message.includes("InvalidatedOrder")) {
      console.log("💡 Solution: Order already filled or cancelled");
    } else if (error.message.includes("BadSignature")) {
      console.log("💡 Solution: Signature verification failed");
    }

    return {
      success: false,
      error: error.message,
    };
  }
}

async function main() {
  try {
    const result = await fillGiveawayOrder();

    if (result.success) {
      console.log("\n🎉 Giveaway order filled successfully!");
    } else {
      console.log("\n💥 Failed to fill giveaway order");
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
