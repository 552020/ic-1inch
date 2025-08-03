/**
 * Extract SrcEscrowCreated event data for createDstEscrow() call
 *
 * Reads events.json and extracts deployedAt timestamp and escrow parameters
 * needed to call createDstEscrow() on the destination chain.
 */

import fs from "fs";
import path from "path";

const eventsFile = path.join(process.cwd(), "events.json");

function extractEventData(txHash) {
  try {
    const events = JSON.parse(fs.readFileSync(eventsFile, "utf8"));
    const event = events.find((e) => e.txHash === txHash);

    if (!event) {
      console.error(`Event not found for txHash: ${txHash}`);
      return null;
    }

    // Extract deployedAt from timelocks (this needs to be parsed from the actual event data)
    const deployedAt = event.deployedAt;
    const srcCancellationTimestamp = deployedAt + 3600; // Add timelock duration

    return {
      deployedAt,
      srcCancellationTimestamp,
      salt: event.salt,
      blockNumber: event.blockNumber,
      txHash: event.txHash,
    };
  } catch (error) {
    console.error("Error reading events:", error.message);
    return null;
  }
}

function listUnprocessedEvents() {
  try {
    const events = JSON.parse(fs.readFileSync(eventsFile, "utf8"));
    const unprocessed = events.filter((e) => !e.processed);

    console.log(`Found ${unprocessed.length} unprocessed events:`);
    unprocessed.forEach((event) => {
      console.log(`  TxHash: ${event.txHash}`);
      console.log(`  Block:  ${event.blockNumber}`);
      console.log(
        `  Time:   ${new Date(event.deployedAt * 1000).toISOString()}`
      );
      console.log("");
    });

    return unprocessed;
  } catch (error) {
    console.error("Error reading events:", error.message);
    return [];
  }
}

// CLI usage
const command = process.argv[2];
const txHash = process.argv[3];

if (command === "extract" && txHash) {
  const data = extractEventData(txHash);
  if (data) {
    console.log("Extracted event data:");
    console.log(JSON.stringify(data, null, 2));
  }
} else if (command === "list") {
  listUnprocessedEvents();
} else {
  console.log("Usage:");
  console.log("  node extract-event-data.js list");
  console.log("  node extract-event-data.js extract <txHash>");
}
