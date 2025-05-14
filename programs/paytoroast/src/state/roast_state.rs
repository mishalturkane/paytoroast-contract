use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RoastState {
    pub roaster: Pubkey,
    pub receiver: Pubkey,
    pub roast_message: Vec<u8>,
    pub amount: u64,
    pub mint: Pubkey,
    pub escrow_pda: Pubkey,
    pub status: RoastStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RoastStatus {
    Pending,
    Accepted,
    Rejected,
}

// Implement the Space trait for RoastState
impl Space for RoastState {
    const INIT_SPACE: usize = 
        8 + // Discriminator
        32 + // roaster: Pubkey
        32 + // receiver: Pubkey
        4 + // roast_message: Vec<u8> prefix (4 bytes for the length)
        (4 * 280) + // roast_message: Assuming a maximum length of 280 bytes (adjust as needed)
        8 + // amount: u64
        32 + // mint: Pubkey
        32 + // escrow_pda: Pubkey
        1;  // status: RoastStatus (enum, typically 1 byte)
}
