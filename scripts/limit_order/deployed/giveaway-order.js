const hre = require("hardhat");
const { ethers } = require("ethers");

async function createGiveawayOrder() {
  console.log("🎁 Creating Giveaway Order on Base Sepolia...\n");

  // Contract address
  const LOP_ADDRESS = "0xdfC365795F146a6755998C5e916a592A9706eDC6";

  // Get the contract instance
  const lop = await hre.ethers.getContractAt("LimitOrderProtocol", LOP_ADDRESS);

  // Get your signer
  const [signer] = await hre.ethers.getSigners();
  console.log(`🎯 Your address: ${signer.address}`);

  // Create giveaway order
  const giveawayOrder = {
    salt: ethers.hexlify(ethers.randomBytes(32)),
    maker: signer.address,
    receiver: signer.address,
    makerAsset: "0x4200000000000000000000000000000000000006", // WETH
    takerAsset: "0x0000000000000000000000000000000000000000", // Zero address (nothing)
    makingAmount: ethers.parseEther("0.01").toString(), // 0.01 ETH
    takingAmount: "0", // Zero - you want nothing back!
    makerTraits:
      "0x0000000000000000000000000000000000000000000000000000000000000000",
  };

  console.log("📋 Giveaway Order Details:");
  console.log(`   Maker: ${giveawayOrder.maker}`);
  console.log(`   Maker Asset: ${giveawayOrder.makerAsset} (WETH)`);
  console.log(`   Taker Asset: ${giveawayOrder.takerAsset} (NOTHING!)`);
  console.log(
    `   Making Amount: ${ethers.formatEther(giveawayOrder.makingAmount)} ETH`
  );
  console.log(`   Taking Amount: ${giveawayOrder.takingAmount} (ZERO!)`);
  console.log(`   Salt: ${giveawayOrder.salt}`);

  // Get domain separator
  const domainSeparator = await lop.DOMAIN_SEPARATOR();
  console.log(`\n🔐 Domain Separator: ${domainSeparator}`);

  // Hash the order
  const orderHash = await lop.hashOrder(giveawayOrder);
  console.log(`📝 Order Hash: ${orderHash}`);

  // Sign the order
  const signature = await signer.signTypedData(
    {
      name: "1inch Limit Order Protocol",
      version: "4",
      chainId: 84532, // Base Sepolia
      verifyingContract: LOP_ADDRESS,
    },
    {
      Order: [
        { name: "salt", type: "uint256" },
        { name: "maker", type: "address" },
        { name: "receiver", type: "address" },
        { name: "makerAsset", type: "address" },
        { name: "takerAsset", type: "address" },
        { name: "makingAmount", type: "uint256" },
        { name: "takingAmount", type: "uint256" },
        { name: "makerTraits", type: "uint256" },
      ],
    },
    {
      salt: giveawayOrder.salt,
      maker: giveawayOrder.maker,
      receiver: giveawayOrder.receiver,
      makerAsset: giveawayOrder.makerAsset,
      takerAsset: giveawayOrder.takerAsset,
      makingAmount: giveawayOrder.makingAmount,
      takingAmount: giveawayOrder.takingAmount,
      makerTraits: giveawayOrder.makerTraits,
    }
  );

  console.log(`✍️  Signature: ${signature}`);

  // Parse signature components
  const sig = ethers.Signature.from(signature);
  const r = sig.r;
  const vs = sig.s + (sig.v === 27 ? "1b" : "1c");

  console.log(`\n🔧 Signature Components:`);
  console.log(`   r: ${r}`);
  console.log(`   vs: ${vs}`);

  console.log("\n✅ Giveaway order created and signed!");
  console.log("🎁 Anyone can now take your 0.01 ETH for free!");
  console.log("\n📋 To fill this order, someone would call:");
  console.log(`   lop.fillOrder(order, "${r}", "${vs}", amount, takerTraits)`);

  return {
    order: giveawayOrder,
    signature,
    r,
    vs,
    orderHash,
  };
}

// Function to fill the giveaway order (for testing)
async function fillGiveawayOrder(order, r, vs) {
  console.log("\n🎁 Filling Giveaway Order...");

  const [signer] = await hre.ethers.getSigners();
  const lop = await hre.ethers.getContractAt(
    "LimitOrderProtocol",
    "0xdfC365795F146a6755998C5e916a592A9706eDC6"
  );

  const amount = order.makingAmount; // Fill the full amount
  const takerTraits =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

  try {
    const tx = await lop.fillOrder(
      order,
      r,
      vs,
      amount,
      takerTraits,
      { value: amount } // Send ETH with the transaction
    );

    console.log(`📝 Transaction hash: ${tx.hash}`);
    const receipt = await tx.wait();
    console.log(`✅ Giveaway completed in block ${receipt.blockNumber}`);

    return receipt;
  } catch (error) {
    console.error("❌ Error filling giveaway order:", error.message);
    throw error;
  }
}

async function main() {
  try {
    const result = await createGiveawayOrder();

    // Uncomment to test filling the order
    // await fillGiveawayOrder(result.order, result.r, result.vs);
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
