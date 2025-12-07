use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {

    #[msg("Vault is currently paused by admin")]
    VaultPaused,

    #[msg("Deposit amount is too small")]
    DepositTooSmall,
    
    #[msg("Deposite Amount is too large")]
    DepositTooLarge,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Withdrawal timelock is still active")]
    WithdrawalTimelockActive,

    #[msg("No pending withdrawal request found")]
    NoPendingWithdrawal,

    #[msg("User already has a pending withdrawal")]
    WithdrawalAlreadyPending,

    #[msg("Withdrawal amount cannot be zero")]
    InvalidWithdrawalAmount,

    #[msg("Vault has insufficient SOL for withdrawal")]
    VaultInsufficientFunds,

    #[msg("Fee percentage exceeds maximum allowed (10%)")]
    FeeExceedsMaximum,

    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,

    #[msg("Invalid conversion rate (must be > 0)")]
    InvalidConversionRate,

    #[msg("Unauthorized: Only admin can perform this action")]
    Unauthorized,

    #[msg("Invalid token mint provided")]
    InvalidTokenMint,

    #[msg("Token account does not belong to user")]
    InvalidTokenAccount,
}
