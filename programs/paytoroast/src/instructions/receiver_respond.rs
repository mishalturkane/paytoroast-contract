
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;


// Context: Receiver responds to the roast
#[derive(Accounts)]
pub struct ReceiverResponds<'info> {
    #[account(mut, signer)]
    pub receiver: Signer<'info>,
    #[account(mut, has_one = receiver)]
    pub roast_escrow: Account<'info, RoastEscrow>,
}


 
    // Instruction: Receiver responds to the roast (accept/reject)
    pub fn receiver_responds(ctx: Context<ReceiverResponds>, accepted: bool) -> Result<()> {
        let roast_escrow = &mut ctx.accounts.roast_escrow;
        roast_escrow.status = if accepted { RoastStatus::Accepted } else { RoastStatus::Rejected };
        Ok(())
    }
