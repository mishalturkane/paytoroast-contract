use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RoastState {
    pub roaster: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    #[max_len = 280] // Example: Maximum length of 280 characters
    pub roast_message: String,
    pub escrow_vault: Pubkey,
    pub status: RoastStatus, // Agar aapne status enum add kiya hai toh
    pub escrow_bump: u8,    // Agar aapne escrow bump add kiya hai toh
    pub state_bump: u8,     // Agar aapne state bump add kiya hai toh
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)] // InitSpace for enum bhi chahiye agar aap ise use kar rahe hain
pub enum RoastStatus {
    Pending,
    Accepted,
    Rejected,
}