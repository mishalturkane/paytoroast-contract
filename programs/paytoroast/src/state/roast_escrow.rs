// State account: Roast Escrow
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

impl Space for RoastEscrow {
    const INIT_SPACE: usize = 8 + // Discriminator
        32 + // roaster: Pubkey
        32 + // receiver: Pubkey
        4 + (MAX_ROAST_LENGTH as usize) + // roast_message: Vec<u8>
        8 + // amount: u64
        32 + // mint: Pubkey
        32 + // escrow_pda: Pubkey
        1 + // status: RoastStatus
        8 + // seed_id: u64
        1;   // bump: u8
}

// Enum: Roast Status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RoastStatus {
    Pending,
    Accepted,
    Rejected,
}