use anchor_lang::prelude::*;

//Error Codes
#[error_code]
pub enum ErrorCode {
    #[msg("The roast message is too long.")]
    RoastMessageTooLong,
    #[msg("The roast is not yet completed.")]
    RoastNotCompleted,
    #[msg("Invalid status for transfer.")]
    InvalidStatusForTransfer,
    #[msg("Incorrect escrow PDA.")]
    IncorrectEscrowPda,
    #[msg("Unauthorized action.")]
    UnauthorizedAction,
    #[msg("Mint Mismatch.")]
    MintMismatch,
    #[msg("Amount must be greater than zero.")]
    AmountMustBeGreaterThanZero,
}