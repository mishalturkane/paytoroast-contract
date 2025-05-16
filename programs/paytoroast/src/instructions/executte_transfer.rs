
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;



// Context: Execute the transfer of funds
#[derive(Accounts)]
pub struct ExecuteTransfer<'info> {
    #[account(mut)]
    pub roast_escrow: Account<'info, RoastEscrow>,
    /// CHECK: This is the escrow account where funds are held.
    #[account(mut, seeds = [b"escrow", roast_escrow.roaster.as_ref(), roast_escrow.receiver.as_ref(), &roast_escrow.seed_id.to_le_bytes()], bump = roast_escrow.bump)]
    pub escrow_pda: AccountInfo<'info>,
    #[account(mut, associated_token::mint = roast_escrow.mint, associated_token::authority = escrow_pda)]
    pub escrow_pda_token_account: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = roast_escrow.mint, associated_token::authority = roast_escrow.roaster)]
    pub roaster_token_account: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = roast_escrow.mint, associated_token::authority = roast_escrow.receiver)]
    pub receiver_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

  // Instruction: Execute the transfer of funds based on receiver's response
    pub fn execute_transfer(ctx: Context<ExecuteTransfer>) -> Result<()> {
        let roast_escrow = &ctx.accounts.roast_escrow;
        let escrow_pda_info = &ctx.accounts.escrow_pda;
        let token_program_info = ctx.accounts.token_program.to_account_info();
        let program_id = ctx.program_id;
        let seeds = &[
            b"escrow",
            roast_escrow.roaster.as_ref(),
            roast_escrow.receiver.as_ref(),
            roast_escrow.seed_id.to_le_bytes().as_ref(),
            &[roast_escrow.bump],
        ];
        let signer = &[&seeds[..]];

        if roast_escrow.status == RoastStatus::Accepted {
            let transfer_checked_instruction = TransferChecked {
                from: ctx.accounts.escrow_pda_token_account.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: escrow_pda_info.clone(),
                mint: ctx.accounts.mint.to_account_info(),
                amount: roast_escrow.amount,
                decimals: ctx.accounts.mint.decimals,
            };
            let cpi_accounts = TransferCheckedContext {
                accounts: transfer_checked_instruction,
                signers: signer,
            };
            token::transfer_checked(cpi_accounts, roast_escrow.amount, ctx.accounts.mint.decimals)?;
        } else if roast_escrow.status == RoastStatus::Rejected {
            let transfer_checked_instruction = TransferChecked {
                from: ctx.accounts.escrow_pda_token_account.to_account_info(),
                to: ctx.accounts.roaster_token_account.to_account_info(),
                authority: escrow_pda_info.clone(),
                mint: ctx.accounts.mint.to_account_info(),
                amount: roast_escrow.amount,
                decimals: ctx.accounts.mint.decimals,
            };
            let cpi_accounts = TransferCheckedContext {
                accounts: transfer_checked_instruction,
                signers: signer,
            };
            token::transfer_checked(cpi_accounts, roast_escrow.amount, ctx.accounts.mint.decimals)?;
        }

        // Close the escrow account
        let close_instruction = CloseAccount {
            account: ctx.accounts.escrow_pda_token_account.to_account_info(),
            destination: if roast_escrow.status == RoastStatus::Accepted {
                ctx.accounts.receiver.to_account_info()
            } else {
                ctx.accounts.roaster.to_account_info()
            },
            authority: escrow_pda_info.clone(),
        };
        let cpi_accounts = CloseAccountContext {
            accounts: close_instruction,
            signers: signer,
        };
        token::close_account(cpi_accounts)?;

        Ok(())
    }
