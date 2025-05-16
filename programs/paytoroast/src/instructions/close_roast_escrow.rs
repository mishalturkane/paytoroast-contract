
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;


// Context: Close the roast escrow account
#[derive(Accounts)]
pub struct CloseRoastEscrow<'info> {
    #[account(mut, close = roaster)]
    pub roast_escrow: Account<'info, RoastEscrow>,
    /// CHECK: Receiver to receive rent if accepted, else roaster.
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,
    #[account(mut)]
    pub roaster: Signer<'info>,
}


 // Instruction: Close the roast escrow account (optional, for cleanup)
    pub fn close_roast_escrow(ctx: Context<CloseRoastEscrow>) -> Result<()> {
        let roast_escrow = &ctx.accounts.roast_escrow;
        if roast_escrow.status != RoastStatus::Pending {
            let receiver = if roast_escrow.status == RoastStatus::Accepted {
                ctx.accounts.receiver.to_account_info()
            } else {
                ctx.accounts.roaster.to_account_info()
            };
            let account_to_close = ctx.accounts.roast_escrow.to_account_info();
            let lamports = account_to_close.lamports();
            **receiver.try_borrow_mut_lamports()? += lamports;
            **account_to_close.try_borrow_mut_lamports()? = 0;
            Ok(())
        } else {
            return err!(ErrorCode::RoastNotCompleted);
        }
    }