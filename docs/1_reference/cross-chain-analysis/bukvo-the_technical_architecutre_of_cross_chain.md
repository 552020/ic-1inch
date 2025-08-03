# The Technical Architecture of Cross-Chain in 1inch

_Transcript of Anton Bukvo's presentation on 1inch's cross-chain architecture_

---

## Introduction

**Kik:** Welcome Anton.

**Anton:** Hi Kik. Thank you.

**Anton:** Hi everyone. I'm here to reveal a high-level architecture of cross-chain stuff in 1inch. We actually use three different layers. The bottom layer we call the "intents layer" — right now it's the 1inch Limit Order Protocol. This layer is able to use user funds by their digital signatures.

The next layer, built on top of the bottom layer, is the price discovery layer — what we call 1inch Fusion. It basically adds Dutch auctions. And then there's one more layer on top, called Fusion Plus, which is the atomic swaps layer. We have escrow layers implemented.

---

## Layer 1: Limit Order Protocol

We'll start from the Limit Order Protocol. This isn't a usual architecture diagram — it's a diagram of value flows. You can see how different assets — or rather, rights to those assets — are transferred among entities. On this slide, you can also see a transaction that the resolver (or "taker") executes to fill orders, while signers (or "makers") only release an order via digital signature. We call it a "signed order" here.

So, those who want to create orders do so off-chain, and those who want to fill them bring them on-chain. That's how it works.

### Key Features

**Predicates:** The Limit Order Protocol has features that developers can use to build interesting mechanisms. For example, it has predicates — a predicate is a static call that checks arbitrary conditions and returns true or false, allowing or disallowing an order to be filled. So, if you integrate an oracle, it could work like a stop-loss: it checks the price, and the order can only be filled if the price goes down. You can't place the order at that price now because it would be filled immediately — but that's the idea behind predicates.

**Interactions:** There are also interactions. The Limit Order Protocol handles two transfer-froms: one from maker to taker and one from taker to maker. It also supports pre-interaction, mid-interaction, and post-interaction logic between the two transfers.

**Dynamic Pricing:** Another cool feature is dynamic pricing. You can write an arbitrary smart contract to form the price. It's not limited to fixed pricing. You can build things like exponential auctions, oracles, stop-loss orders, or anything you come up with.

**Flags:** The protocol also supports various flags. For instance, if you want the order to be as cheap as possible, you can enable options like allowing only single-fill or only full-fill orders. Enabling one of these options saves about 15k gas by using bit invalidation. There are many such flags worth checking out.

---

## Layer 2: 1inch Fusion (Dutch Auctions)

Now, Fusion. Fusion is Dutch auctions built on top of the Limit Order Protocol. Each order has its own auction, and resolvers participate in them. Since these are Dutch auctions, they participate passively — just waiting as the price improves over time. They compete to fill orders. When the exchange rate becomes barely profitable, they can't wait anymore because someone else might take it. That's the price discovery mechanism.

Here's an example: a nearly $13 million swap from USDT to wrapped ETH. You can see how the price declined during the auction, and multiple fills occurred in less than one minute. This resulted in a $200,000 surplus compared to using a market order.

A Dutch auction moves from a price good for the maker toward a worse price for them but improves for takers (resolvers) over time.

---

## Layer 3: Fusion Plus (Atomic Swaps)

Fusion Plus builds on top of 1inch Fusion. So we have the Limit Order Protocol → Dutch auction → atomic swaps and escrows. HTLC stands for Hashed Timelock Contracts.

Here's how it works. Some arrows represent transactions or interactions, others show how funds and rights move between entities.

### How Atomic Swaps Work

The user creates a signed order with the hash of a secret key and also generates the secret key — which they don't reveal immediately. When the resolver sees a good price in the Dutch auction, they submit the signed order to the source chain's Limit Order Protocol. Funds get locked in the source escrow, secured by both a timer and the secret key.

Then the resolver interacts with the Limit Order Protocol on the destination chain, locking destination funds in another escrow. The resolver waits for the user to reveal the secret key. Once the user is satisfied with the destination escrow's parameters and finality — which on EVM chains can be verified with a single hash computation — they reveal the secret. This unlocks both escrows on both chains.

If the user doesn't reveal the secret, the destination escrow can be canceled after its timer expires — it's designed to expire first, so the resolver's funds are safe.

---

## Cross-Chain Compatibility

That's the basic idea. The Limit Order Protocol and escrow logic works well on many chains. But not all — for example, Bitcoin doesn't have smart contracts. Its UTXO model is different: outputs are small programs that must be satisfied to spend.

The architecture of underlying blockchains varies. But most, including Bitcoin, are capable of supporting HTLCs. Atomic swaps were first discussed on the Bitcoin Talk forum back in 2013 — 12 years ago. At that time, only Bitcoin-like chains existed. So, if it's doable on Bitcoin, it's likely doable on most chains.

We believe atomic swaps are the only self-custodial scheme. This is a long-term architectural direction. We want this to succeed and last. If something is built at the architectural level with atomic swaps for self-custody, it should endure.

Custodial solutions are not a long-term vision. There are too many risks. The more things we build permissionlessly, the better. Custody is easier to build, but not the future we want.

Let me check if there's anything else in my slides... I think that's it.

---

## Q&A Session

**Kik:** Anton, thank you so much for that. One interesting thing is that Sergey is hijacking the Q&A on the livestream right now, talking directly with hackers. But I'll pause for a second to let anyone on the livestream ask questions to Anton, especially about the architecture.

The first question is if we can get a link to your slides — we'll share them with attendees.

**Anton:** Yes, I sent them to you, but we should make it a proper link.

**Kik:** We'll do that.

Now my question: when talking to people building on Bitcoin who want to integrate DeFi from EVM, what are their wishes and complaints? What's working, what's not?

**Anton:** So, computation functionality is very limited on Bitcoin-like chains. Some Bitcoin-like chains have more opcodes, but many are disabled on Bitcoin for security — even if we don't always understand the reasons.

What I've seen is that many teams try different Layer 2 solutions. But there's a key difference between Layer 2s on EVM and Bitcoin. On EVM, Layer 2s are governed by smart contracts on Layer 1. These contracts guarantee safety and correctness of computation.

That's not possible on Bitcoin. Bitcoin-based Layer 2s use it only for data availability and ordering. They embed EVM-like transactions into Bitcoin as raw data. Then they extract and execute them on a separate blockchain. But Bitcoin doesn't verify or guarantee the correctness of that logic.

So, Bitcoin can support only a few specific use cases — like multi-signature, payment channels (like Lightning Network), and atomic swaps. But I don't expect dozens of diverse apps.

If the only tool you have is HTLC, you're limited to those things.

**Kik:** Final question: is there anything on your wish list that you hope hackers will build?

**Anton:** When I saw the list of blockchains here, each with different environments, languages, and VMs — I was amazed. I normally work within the EVM ecosystem and only touch others slightly. But this is like 15–20 different chains — it's massive. No company can cover all of them alone. Only hackers at a hackathon can do that. We need cumulative effort.

**Kik:** Amazing. Anton, thanks again for the great presentation. We've shared the slides already. We'll see you in two weeks when submissions are reviewed.

**Anton:** Thank you, Karthik. See you.

---

## Closing Remarks

**Kik:** To everyone else — some final thoughts:

As you begin your 10-day hackathon journey, pace yourself. The reason we run longer hackathons is so you can spend the first few days learning without rushing into building blindly. Take time to read documentation, ask questions, attend workshops, and think through what's possible.

Then use the rest of the time to build, iterate, and improve. Prioritize sleep and learning. Don't exhaust yourself — this is meant to be fun and educational.

If you get closer to your goal by the end, that's great progress.

Happy hacking! See you on Discord. Ask questions there or join the live feedback sessions with the 1inch team. We're excited to see all the DeFi integrations you'll build.

See you next week.

_[Music]_
