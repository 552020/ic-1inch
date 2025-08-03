# What is 1inch Fusion+, and how does it work?

_Everything you need to know about 1inch's innovative cross-chain swaps_

**Written by:** Valeriia Nikitina  
**Updated:** Over a week ago

---

## In this article, we will cover:

- [Phases of a Fusion+ swap](#phases-of-a-fusion-swap)
- [How to complete a 1inch Fusion+ Swap](#how-to-complete-a-1inch-fusion-swap)
- [How to become a resolver](#how-to-become-a-fusion-resolver)
- [1inch Fusion+ FAQ](#fusion-faq)

1inch Fusion+ is a powerful solution for secure and efficient intent-based atomic swaps in DeFi, using a creative architecture of Dutch auctions and automated execution, all without relying on a single centralized custodian.

For a comprehensive technical overview, please refer to the [1inch Fusion+ Whitepaper](https://1inch.io/assets/1inch-fusion-plus.pdf).

---

## Phases of a Fusion+ Swap

The process typically has three phases and involves two main participants: the maker, who creates the order, and the resolver, who fills it.

If any problems arise, there's an optional 4th "Recovery phase" that can be used as a last resort.

### 1. Announcement Phase

The maker initiates the process by signing a 1inch Fusion+ order and broadcasting it to 1inch. This signals their intent to execute a cross-chain swap and sets the process in motion.

**Dutch Auction:** The order is distributed to all resolvers, triggering a [Dutch auction](https://help.1inch.io/en/articles/6796085-what-is-1inch-fusion-and-how-does-it-work). Resolvers compete by offering progressively better prices as the auction continues until a resolver locks in the order by initiating an escrow on the source chain.

### 2. Deposit Phase

The winning resolver deposits the maker's assets into an escrow contract on the source chain, and then deposits the corresponding assets into an escrow on the destination chain. Both escrows are linked by a secret hash, ensuring that assets can only be unlocked once the swap is completed. A small safety deposit is also assigned to each escrow, incentivizing the resolver to successfully complete the order.

### 3. Withdrawal Phase

Once both escrows are verified by the relayer, the secret is revealed, allowing the resolver to unlock the assets on the destination chain for the maker. The resolver then uses the same secret to retrieve their newly acquired assets on the source chain, finalizing the swap.

### Recovery Phase (Optional)

In the event of a failed swap (e.g., if a party becomes unresponsive), the protocol includes a recovery mechanism. After the time-lock expires, any resolver or any participating entity can cancel the swap and return the assets to their original owners. The safety deposit in each escrow is transferred to any resolver who steps in to complete the swap during this phase.

---

## How to complete a 1inch Fusion+ Swap

**\*\*\*DO NOT CLOSE THE TAB AFTER SUBMITTING YOUR SWAP \*\*\***

To ensure your cross-chain swap is executed smoothly, please follow the steps below regarding push notifications:

- **Keep the tab open:** Closing the page or letting it go to sleep can result in the cancellation of your cross-chain order.
- **Enable browser notifications:** Make sure notifications are turned on in your browser settings.
- **Enable OS notifications:** Notifications must also be enabled in your operating system settings (macOS or Windows). Instructions vary by OS version.
- **Check "Do Not Disturb" or "Focus" modes:** If these modes are active, you won't receive notifications, even if they are enabled elsewhere.

Following these steps will help you stay informed and prevent any issues with your cross-chain swaps.

### How to swap

In this example, we'll be swapping USDT on Arbitrum (source) to aUSDC on Ethereum Mainnet (destination).

#### Step 1

Connect your wallet to the 1inch dApp.

#### Step 2

Select the source and destination chains, source and destination tokens, and source token amount that you would like to swap.

_Note: When "All networks" is selected, you can view the available balance for each of your wallet's tokens across all chains._

#### Step 3

Once the desired networks, tokens, and amount have been selected, click the "Swap" button.

#### Step 4

Review the information on the swap confirmation page to ensure accuracy. If everything looks good, click the "Confirm Swap" button.

**Remember to keep the swap page tab open during the transaction!** This is required to share your secret with the resolver and complete the swap.

#### Step 5

After submitting the swap, you can click "View pending transaction" to check the status of your swap.

You can also view the pending transaction (along with history) by clicking your wallet address in the upper right corner of the screen.

Selecting the pending transaction itself will allow you to view the status of the order's fulfillment.

### Partial fills

The partial fill feature ensures that when an order is filled in parts by different resolvers, each part is protected by a unique secret to prevent other resolvers from claiming the remaining portions unfairly. This is done using a Merkle tree structure, where secrets are assigned to specific portions of the order, ensuring that only the intended participant can complete their part without exposing the rest of the order.

Below is an example of an order that was partially filled by multiple resolvers.

---

## How to become a Fusion+ resolver

To become a resolver, you need a balance of staked 1INCH (st1INCH) tokens, meeting the minimum requirement of 5% of the total supply of Unicorn Power (UP). The maximum limit of listed resolvers is 10. Holding st1INCH tokens grants users UP, which can be utilized for farming programs or various activities. The duration of the token lock, ranging from 1 month to 2 years, directly correlates to the voting power gained as a resolver.

To uphold the integrity of the 1inch Fusion mode, a smart contract-enforced maximum gas fee rule is in place. This cap on priority gas fees prevents any attempt to bypass the limit, including direct payments to a block builder's coinbase. Violations will incur penalties.

**Priority gas fee caps are determined by the baseFee:**

- For baseFee <10.6 gwei, the priorityFee cap is 70% of the baseFee.
- For 10.6 gwei ≤ baseFee ≤ 104.1 gwei, the priorityFee cap is 50% of the baseFee.
- For baseFee >104.1 gwei, the priorityFee cap is 65% of the baseFee.

**Violators of the gas fee cap will face the following penalties:**

- First offense: official warning
- Second offense: one-day block from filling orders
- Third offense: seven-day block from filling orders
- Fourth offense: thirty-day block from filling orders
- Fifth offense: three-hundred sixty-five-day block from filling orders

---

## Fusion+ FAQ

**What happens if there is an issue during the Fusion+ swap?**

If there are any issues throughout the swap, you can always click the "Cancel" button, located next to the pending transaction in your wallet history.

**How long does it take for a Fusion+ swap to complete?**

It depends on the blockchains that are selected, due to each having a different hash-lock period for escrow. Generally speaking, most swaps complete in less than 5 minutes.

**Do I need to pay gas fees for a Fusion+ swap?**

No gas fees needed! With 1inch Fusion+ 100% of the gas fees are covered by the resolvers who fill the trade.

**What happens if I didn't receive the correct amount of tokens for the swap?**

This can happen due the default "partial fill" feature. With partial fill, only a part of your swap may be completed. This feature exists to minimize failed swaps.

**Can my transaction be front-run or "sandwiched" during the process?**

No, by design, 1inch Fusion+ swaps are MEV resistant, meaning that your transaction will not be frontrun or sandwiched.

---

## Questions, comments, concerns?

**Feel free to reach out to us in the live support chat!**

---

_Source: [1inch Help Center](https://help.1inch.io/en/articles/9842591-what-is-1inch-fusion-and-how-does-it-work)_
