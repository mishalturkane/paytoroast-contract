# paytoroast 
### Pay to Roast Get Paid to Be Roasted.
The first Web3 platform where you can send spicy roasts with crypto attached. Accept the roast, get the cash. Reject it, and the sender gets refunded.

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

- ```deposit_in_escrow```: Transfers tokens from the roaster's account to the escrow PDA.

- ```receiver_responds```: Records the receiver's decision (accept/reject) for the roast.

- ```execute_transfer```: Distributes funds based on the receiver's decision and closes the escrow PDA's token account.

```close_roast_escrow```: Closes the RoastEscrow account and refunds rent
---