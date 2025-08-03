const hre = require("hardhat");
const { ethers } = require("ethers");

async function checkOrderStatus() {
    console.log("🔍 Checking Giveaway Order Status...\n");

    // Contract address
    const LOP_ADDRESS = "0xdfC365795F146a6755998C5e916a592A9706eDC6";
    
    // Get the contract instance
    const lop = await hre.ethers.getContractAt("LimitOrderProtocol", LOP_ADDRESS);
    
    // The giveaway order details
    const giveawayOrder = {
        salt: "0x94861106fbd44d75f10ee28c0409f002654b038664c4c8e4ff8a2b3a6dac8718",
        maker: "0x8CB80b37cc7193D0f055b1189F25eB903D888D3A",
        receiver: "0x8CB80b37cc7193D0f055b1189F25eB903D888D3A",
        makerAsset: "0x4200000000000000000000000000000000000006", // WETH
        takerAsset: "0x0000000000000000000000000000000000000000", // Zero address (nothing)
        makingAmount: ethers.parseEther("0.01").toString(), // 0.01 ETH
        takingAmount: "0", // Zero - you want nothing back!
        makerTraits: "0x0000000000000000000000000000000000000000000000000000000000000000"
    };
    
    console.log("📋 Order Details:");
    console.log(`   Maker: ${giveawayOrder.maker}`);
    console.log(`   Maker Asset: ${giveawayOrder.makerAsset} (WETH)`);
    console.log(`   Making Amount: ${ethers.formatEther(giveawayOrder.makingAmount)} ETH`);
    console.log(`   Taking Amount: ${giveawayOrder.takingAmount} (ZERO!)`);
    
    // Hash the order
    const orderHash = await lop.hashOrder(giveawayOrder);
    console.log(`\n📝 Order Hash: ${orderHash}`);
    
    // Check if contract is paused
    console.log("\n⏸️  Contract Status:");
    try {
        const isPaused = await lop.paused();
        console.log(`   Contract paused: ${isPaused}`);
    } catch (error) {
        console.log(`   ❌ Error checking pause status: ${error.message}`);
    }
    
    // Check maker's WETH balance
    console.log("\n💰 Maker WETH Balance:");
    try {
        const wethContract = new ethers.Contract(giveawayOrder.makerAsset, [
            "function balanceOf(address owner) view returns (uint256)"
        ], hre.ethers.provider);
        
        const makerWethBalance = await wethContract.balanceOf(giveawayOrder.maker);
        console.log(`   Maker WETH: ${ethers.formatEther(makerWethBalance)} WETH`);
        
        if (makerWethBalance < ethers.parseEther("0.01")) {
            console.log("   ⚠️  Warning: Maker doesn't have enough WETH!");
        }
    } catch (error) {
        console.log(`   ❌ Error checking WETH balance: ${error.message}`);
    }
    
    // Check maker's ETH balance
    console.log("\n💰 Maker ETH Balance:");
    try {
        const makerEthBalance = await hre.ethers.provider.getBalance(giveawayOrder.maker);
        console.log(`   Maker ETH: ${ethers.formatEther(makerEthBalance)} ETH`);
    } catch (error) {
        console.log(`   ❌ Error checking ETH balance: ${error.message}`);
    }
    
    // Check bit invalidator for this order
    console.log("\n🔢 Bit Invalidator Status:");
    try {
        const invalidator = await lop.bitInvalidatorForOrder(giveawayOrder.maker, 0);
        console.log(`   Bit Invalidator for slot 0: ${invalidator}`);
        
        if (invalidator != 0) {
            console.log("   ⚠️  Warning: Order might be invalidated!");
        }
    } catch (error) {
        console.log(`   ❌ Error checking invalidator: ${error.message}`);
    }
    
    // Check remaining amount for this order
    console.log("\n📊 Remaining Amount:");
    try {
        const remaining = await lop.remainingInvalidatorForOrder(giveawayOrder.maker, orderHash);
        console.log(`   Remaining amount: ${ethers.formatEther(remaining)} ETH`);
        
        if (remaining == 0) {
            console.log("   ⚠️  Warning: Order might be fully filled!");
        }
    } catch (error) {
        console.log(`   ❌ Error checking remaining: ${error.message}`);
    }
    
    console.log("\n✅ Order status check completed!");
}

async function main() {
    try {
        await checkOrderStatus();
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