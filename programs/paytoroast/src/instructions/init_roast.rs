

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;

// Context: Initialize a new roast interaction
#[derive(Accounts)]
#[instruction(roast_message: String, amount: u64, mint: Pubkey, seed_id: u64)]
pub struct InitializeRoast<'info> {
    #[account(mut, signer)]
    pub roaster: Signer<'info>,
    #[account(
        init,
        payer = roaster,
        space = 8 + 32 + 32 + 4 + (MAX_ROAST_LENGTH as usize) + 8 + 32 + 1 + 8 + 1,
        seeds = [b"roast_escrow", roaster.key().as_ref(), receiver.key().as_ref(), &seed_id.to_le_bytes()],
        bump,
    )]
    pub roast_escrow: Account<'info, RoastEscrow>,
    /// CHECK: This is the escrow account where funds will be held. It will be owned by the program.
    #[account(
        init,
        payer = roaster,
        seeds = [b"escrow", roaster.key().as_ref(), receiver.key().as_ref(), &seed_id.to_le_bytes()],
        bump,
        space = 8, // Minimum rent-exempt balance
        owner = crate::id(),
    )]
    pub escrow_pda: AccountInfo<'info>,
    #[account(
        init,
        payer = roaster,
        associated_token::mint = mint,
        associated_token::authority = escrow_pda,
    )]
    pub escrow_pda_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub receiver: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

 // Instruction: Initialize a new roast interaction
    pub fn initialize_roast(
        ctx: Context<InitializeRoast>,
        roast_message: String,
        amount: u64,
        mint: Pubkey,
        seed_id: u64,
    ) -> Result<()> {
        let roast_escrow = &mut ctx.accounts.roast_escrow;
        roast_escrow.roaster = ctx.accounts.roaster.key();
        roast_escrow.receiver = ctx.accounts.receiver.key();
        roast_escrow.amount = amount;
        roast_escrow.mint = mint;
        roast_escrow.escrow_pda = ctx.accounts.escrow_pda.key();
        roast_escrow.status = RoastStatus::Pending;
        roast_escrow.seed_id = seed_id;
        roast_escrow.bump = *ctx.bumps.get("roast_escrow").unwrap();

        // Store the roast message, ensuring it doesn't exceed the maximum length
        let message_bytes = roast_message.as_bytes();
        if message_bytes.len() > MAX_ROAST_LENGTH {
            return err!(ErrorCode::RoastMessageTooLong);
        }
        roast_escrow.roast_message.clear();
        roast_escrow.roast_message.extend_from_slice(message_bytes);

        Ok(())
    }