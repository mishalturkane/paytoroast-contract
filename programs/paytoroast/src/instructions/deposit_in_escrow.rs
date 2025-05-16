




use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;





// Context: Deposit funds into the escrow account
#[derive(Accounts)]
pub struct DepositInEscrow<'info> {
    #[account(mut, signer)]
    pub roaster: Signer<'info>,
    #[account(mut, has_one = roaster)]
    pub roast_escrow: Account<'info, RoastEscrow>,
    /// CHECK: This is the escrow account where funds are deposited.
    #[account(mut, seeds = [b"escrow", roast_escrow.roaster.as_ref(), roast_escrow.receiver.as_ref(), &roast_escrow.seed_id.to_le_bytes()], bump = roast_escrow.bump)]
    pub escrow_pda: AccountInfo<'info>,
    #[account(mut, associated_token::mint = roast_escrow.mint, associated_token::authority = roaster)]
    pub roaster_token_account: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = roast_escrow.mint, associated_token::authority = escrow_pda)]
    pub escrow_pda_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}


 // Instruction: Deposit funds into the escrow account
    pub fn deposit_in_escrow(ctx: Context<DepositInEscrow>, amount: u64) -> Result<()> {
        let transfer_checked_instruction = TransferChecked {
            from: ctx.accounts.roaster_token_account.to_account_info(),
            to: ctx.accounts.escrow_pda_token_account.to_account_info(),
            authority: ctx.accounts.roaster.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            amount,
            decimals: ctx.accounts.mint.decimals, // Use decimals from the mint account
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = TransferCheckedContext {
            accounts: transfer_checked_instruction,
            signers: &[ctx.accounts.roaster.to_account_info()],
        };
        token::transfer_checked(cpi_accounts, amount, ctx.accounts.mint.decimals)?;
        Ok(())
    }
