use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("6iEFedqxjyCaJPcBZzp1w7D3TT8wPRCdvveMZe8FcgCm"); // Replace with your program ID


#[program]
pub mod roast_escrow {
    use super::*;

    pub fn initialize_roast(ctx: Context<InitializeRoast>, roast_message: String, amount: u64, mint: Pubkey, seed_id: u64) -> Result<()> {
        instructions::initialize_roast(ctx, roast_message, amount, mint, seed_id)
    }

    pub fn deposit_in_escrow(ctx: Context<DepositInEscrow>, amount: u64) -> Result<()> {
        instructions::deposit_in_escrow(ctx, amount)
    }

    pub fn receiver_responds(ctx: Context<ReceiverResponds>, accepted: bool) -> Result<()> {
        instructions::receiver_responds(ctx, accepted)
    }

    pub fn execute_transfer(ctx: Context<ExecuteTransfer>) -> Result<()> {
        instructions::execute_transfer(ctx)
    }

    pub fn close_roast_escrow(ctx: Context<CloseRoastEscrow>) -> Result<()> {
        instructions::close_roast_escrow(ctx)
    }
}
