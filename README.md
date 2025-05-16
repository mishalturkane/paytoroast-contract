# PaytoRoast [https://paytoroast.vercel.app/](https://paytoroast.vercel.app/)
```Pay to Roast Get Paid to Be Roasted.```
paytoroast is the first Web3 platform where you can send spicy roasts with crypto attached. Accept the roast, get the cash. Reject it, and the sender gets refunded.
We built **DARE**, a novel **Decision-based Action Release Escrow** protocol on Solana,
 enabling permissionless roast-based payouts.

---
 ## How It Works💡
[Roaster] ->
Roaster initiates the interaction by submitting:

A roast message

A token/SOL amount

Funds are securely locked in a PDA-based escrow vault.


[Reciever] ->
The receiver receives a link to accept or reject the interaction.

Based on the receiver’s response:

✅ Accept → The roast is posted (e.g., via Twitter API) and the locked amount is transferred to the receiver.

❌ Reject → The locked amount is refunded to the original sender (roaster).

---
## 1 Accounts
``` roast_state.rs ``` that stores the pda 
``` bash 
#[account]
pub struct RoastEscrow {
    pub roaster: Pubkey,
    pub receiver: Pubkey,
    pub roast_message: Vec<u8>,
    pub amount: u64,
    pub mint: Pubkey,
    pub escrow_pda: Pubkey,
    pub status: RoastStatus,
    pub seed_id: u64,
    pub bump: u8,
}
``` 
---
## 2 Instructions

- ```initialize_roast.rs```: Initializes the ```RoastEscrow``` account and **escrow PDA** with provided details.

- ```deposit_in_escrow.rs```: Transfers tokens from the roaster's account to the escrow PDA.

- ```receiver_responds.rs```: Records the receiver's decision (accept/reject) for the roast.

- ```execute_transfer.rs```: Distributes funds based on the receiver's decision and closes the escrow PDA's token account.
- ```close_roast_escrow.rs```: Close the RoastEscrow account adn refunds rent.

---

## Progress:
- Smartcontract ✅
- Testing ✅
- Frontend 🏗️ 50% and 50% done✅[https://paytoroast.vercel.app/](https://paytoroast.vercel.app/)
---