/**
 * 1inch Fusion+ Resolver - Event Listener
 *
 * Listens for `SrcEscrowCreated` events from the EscrowFactory contract on Base Sepolia.
 * Extracts the `deployedAt` timestamp needed to compute `srcCancellationTimestamp`
 * for creating the destination escrow on ICP. Events are saved to events.json for
 * persistence and resumption after restarts.
 */

import { ethers } from "ethers";
import dotenv from "dotenv";
import fs from "fs";
import path from "path";
dotenv.config();

// Load config
const config = JSON.parse(
  fs.readFileSync(path.join(process.cwd(), "config.json"), "utf8")
);
const network = config.baseSepolia;

// Constants
const escrowFactoryAddress = network.contracts.escrowFactory;
// For better event listening, let's try a different approach
const rpcUrl = "https://sepolia.base.org"; // Public RPC for testing
const provider = new ethers.JsonRpcProvider(rpcUrl);

console.log("🔗 Connected to Base Sepolia");
console.log("📍 EscrowFactory:", escrowFactoryAddress);
const eventsFile = path.join(process.cwd(), "events.json");

// Event signature (from logs)
const SRC_ESCROW_CREATED =
  "event SrcEscrowCreated((address maker,address token,uint256 amount,uint256 timelock,bytes32 hashlock),address resolver,bytes32 salt)";
const iface = new ethers.Interface([SRC_ESCROW_CREATED]);
const topic = iface.getEvent("SrcEscrowCreated").topicHash;

// Main
async function main() {
  console.log("Checking for recent SrcEscrowCreated events...");

  try {
    // Get recent events from the last 1000 blocks
    const currentBlock = await provider.getBlockNumber();
    const fromBlock = Math.max(0, currentBlock - 1000);

    console.log(`📊 Scanning blocks ${fromBlock} to ${currentBlock}`);

    const logs = await provider.getLogs({
      address: escrowFactoryAddress,
      topics: [topic],
      fromBlock: fromBlock,
      toBlock: "latest",
    });

    console.log(`🔍 Found ${logs.length} events`);

    for (const log of logs) {
      const parsed = iface.parseLog(log);
      const { salt } = parsed.args;

      const block = await provider.getBlock(log.blockNumber);
      const deployedAt = block.timestamp;

      console.log("📦 SrcEscrowCreated:");
      console.log("  Salt:          ", salt);
      console.log("  Block:         ", log.blockNumber);
      console.log("  TxHash:        ", log.transactionHash);
      console.log(
        "  Deployed At:   ",
        new Date(deployedAt * 1000).toISOString()
      );

      // Save event data
      const eventData = {
        txHash: log.transactionHash,
        salt,
        deployedAt,
        blockNumber: log.blockNumber,
        timestamp: new Date().toISOString(),
        processed: false,
      };

      // Append to events.json
      let events = [];
      try {
        events = JSON.parse(fs.readFileSync(eventsFile, "utf8"));
      } catch (e) {
        events = [];
      }

      events.push(eventData);
      fs.writeFileSync(eventsFile, JSON.stringify(events, null, 2));
      console.log("  ✅ Saved to events.json");
      console.log("");
    }

    if (logs.length === 0) {
      console.log("❌ No recent SrcEscrowCreated events found");
      console.log("💡 Try creating a test escrow to generate events");
    }
  } catch (error) {
    console.error("❌ Error:", error.message);
  }
}

main();
