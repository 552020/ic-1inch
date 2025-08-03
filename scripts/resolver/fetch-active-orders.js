const axios = require("axios");
const fs = require("fs");
const path = require("path");

async function fetchActiveOrders() {
  const url = "https://api.1inch.dev/fusion-plus/orders/v1.0/order/active";

  const config = {
    headers: {
      Authorization: `Bearer ${process.env.INCH_API_KEY}`,
      Accept: "application/json",
    },
    params: {},
    paramsSerializer: {
      indexes: null,
    },
  };

  try {
    console.log("🔍 Fetching active orders from 1inch Fusion+ API...");
    const response = await axios.get(url, config);

    // Save raw response
    const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
    const outputDir = path.join(__dirname, "data");

    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }

    const filename = `active-orders-${timestamp}.json`;
    const filepath = path.join(outputDir, filename);

    fs.writeFileSync(filepath, JSON.stringify(response.data, null, 2));

    console.log(
      `✅ Fetched ${response.data.orders?.length || 0} active orders`
    );
    console.log(`📁 Saved to: ${filepath}`);

    // Also save latest version
    const latestFile = path.join(outputDir, "active-orders-latest.json");
    fs.writeFileSync(latestFile, JSON.stringify(response.data, null, 2));
    console.log(`📁 Latest saved to: ${latestFile}`);

    return response.data;
  } catch (error) {
    console.error("❌ Error fetching active orders:", error.message);
    if (error.response) {
      console.error("Response status:", error.response.status);
      console.error("Response data:", error.response.data);
    }
    throw error;
  }
}

// Export for use in other scripts
module.exports = { fetchActiveOrders };

// Run if called directly
if (require.main === module) {
  fetchActiveOrders()
    .then(() => {
      console.log("✅ Script completed successfully");
      process.exit(0);
    })
    .catch((error) => {
      console.error("❌ Script failed:", error.message);
      process.exit(1);
    });
}
