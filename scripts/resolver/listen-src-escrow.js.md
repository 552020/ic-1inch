# Event Listener Implementation Notes

## Issues Encountered

1. **Alchemy API Authentication**:

   - Initial typo in `.env`: `ALCHEME_API_KEY` → `ALCHEMY_API_KEY`
   - API key was invalid/expired, causing 401 Unauthorized errors

2. **Public RPC Filter Issues**:
   - Base Sepolia public RPC (`https://sepolia.base.org`) doesn't support persistent event filters
   - Error: `filter not found` when using `provider.on()` for real-time event listening

## Solution Changes

**Original Design**: Real-time event listening with `provider.on()`

```js
provider.on({ address: factory, topics: [topic] }, callback);
```

**Updated Design**: Historical log scanning with `provider.getLogs()`

```js
const logs = await provider.getLogs({
  address: escrowFactoryAddress,
  topics: [topic],
  fromBlock: currentBlock - 1000,
  toBlock: "latest",
});
```

## Result

- ✅ Works with public RPC endpoints
- ✅ Scans last 1000 blocks for events
- ✅ Saves events to `events.json` for persistence
- 🔄 Can be run periodically for near real-time monitoring
