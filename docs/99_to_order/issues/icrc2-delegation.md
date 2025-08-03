Yes, on the Internet Computer (ICP), it is possible to delegate the ability for someone else (such as a DEX or a swap canister) to transfer tokens from your account to another account, using the ICRC-2 token standard. This is achieved through the approve-transfer-from pattern, which is similar to the ERC-20 standard on Ethereum.

Here’s how it works:

1. **Approve**: You, as the token owner, call the `icrc2_approve` method on the token ledger, specifying the amount and the delegate (e.g., the swap canister or the eventual winner of the bid). This allows the delegate to transfer up to the approved amount from your account.
2. **Transfer From**: The delegate (whoever wins the bid or the swap canister acting on their behalf) can then call `icrc2_transfer_from` to move the approved amount from your account to their own, or to another specified account.

This mechanism allows you to "lock" your ICP in the sense that only the approved delegate can transfer the tokens, and only up to the approved amount. However, you must specify the delegate (the principal) in advance. If you don't know who will win the bid, you would typically approve the swap canister to act as the delegate, and the canister's logic would ensure that only the winner can claim the funds.

This pattern is commonly used in decentralized exchanges and token swap canisters on ICP, as shown in the [ICRC-2 token swap example](https://github.com/dfinity/examples/tree/master/motoko/icrc2-swap) and described in the documentation:

> The `approve` method allows an account owner to delegate token transfers to a third party on the owner's behalf, while the `transfer_from` method enables approved token transactions to be initiated. These workflows are common in the Ethereum ecosystem through the ERC-20 token standard.  
> ...  
> The user transfers up to X tokens from account A to any account through the `icrc2_transfer_from` method. The number of transfers is not limited as long as the total amount spent is below X.  
> [Docs > Defi > Token standards](https://internetcomputer.org/docs/defi/token-standards#icrc-2)

So, in your scenario, you would approve the swap canister to transfer your ICP, and the canister would handle the logic of awarding the funds to the winning bidder. This is a secure and standard approach for such use cases on ICP.

**Note:** If you want to ensure that only the winner can claim the funds and you don't know their identity in advance, the swap logic must be implemented in a canister you trust (ideally immutable or with trusted controllers), as the canister will have the delegated authority to move your tokens once approved. Always review the canister's code and controllers for security, as highlighted in the documentation [here](https://medium.com/dfinity/defi-boom-coming-internet-computer-smart-contracts-can-now-transfer-icp-tokens-c9916ede1060#3869).
