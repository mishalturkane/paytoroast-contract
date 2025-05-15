# paytoroast 
### Pay to Roast Get Paid to Be Roasted.
The first Web3 platform where you can send spicy roasts with crypto attached. Accept the roast, get the cash. Reject it, and the sender gets refunded.

## 1 Accounts
``` roast_state.rs ``` that stores the pda 
``` bash 
pub struct RoastState {
    pub roaster: Pubkey,
    pub receiver: Pubkey,
    pub roast_message: Vec<u8>,
    pub amount: u64,
    pub mint: Pubkey,
    pub escrow_pda: Pubkey,
    pub status: RoastStatus,
}
``` 