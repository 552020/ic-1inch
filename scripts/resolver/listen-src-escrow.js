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

// Constants
const provider = new ethers.JsonRpcProvider(process.env.RPC_URL);
const escrowFactoryAddress = process.env.ESCROW_FACTORY;
const eventsFile = path.join(process.cwd(), "events.json");

// Event signature (from logs)
const SRC_ESCROW_CREATED =
  "event SrcEscrowCreated((address maker,address token,uint256 amount,uint256 timelock,bytes32 hashlock),address resolver,bytes32 salt)";
const iface = new ethers.Interface([SRC_ESCROW_CREATED]);
const topic = iface.getEvent("SrcEscrowCreated").topicHash;

// Main
async function main() {
  console.log("Listening for SrcEscrowCreated events...");

  provider.on(
    {
      address: escrowFactoryAddress,
      topics: [topic],
    },
    async (log) => {
      const parsed = iface.parseLog(log);
      const { salt } = parsed.args;

      const block = await provider.getBlock(log.blockNumber);
      const deployedAt = block.timestamp;

      console.log("📦 SrcEscrowCreated:");
      console.log("  Salt:          ", salt);
      console.log("  Block:         ", log.blockNumber);
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
      const events = JSON.parse(
        fs.readFileSync(eventsFile, "utf8").catch(() => "[]") || "[]"
      );
      events.push(eventData);
      fs.writeFileSync(eventsFile, JSON.stringify(events, null, 2));
      console.log("  Saved to events.json");

      // TODO: Call ICP escrow_manager.createDstEscrow() here
      // const srcCancellationTimestamp = deployedAt + srcTimelock;
    }
  );
}

main();
