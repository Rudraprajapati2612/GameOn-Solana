use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::constants::*;
use crate::errors::VaultError;
use crate::state::*;

#[derive(Accounts)]
pub struct ExecuteWithdrawal<'info> {
    /// User executing withdrawal
    #[account(mut)]
    pub user: Signer<'info>,

    /// GlobalVault PDA - sends SOL
    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED],
        bump = global_vault.bump,
    )]
    pub global_vault: Account<'info, GlobalVault>,

    /// UserVault PDA
    #[account(
        mut,
        seeds = [USER_VAULT_SEED, user.key().as_ref()],
        bump = user_vault.bump,
        constraint = user_vault.owner == user.key() @ VaultError::Unauthorized,
    )]
    pub user_vault: Account<'info, UserVault>,

    /// DEGEN token mint
    #[account(
        mut,
        address = global_vault.token_mint,
    )]
    pub token_mint: Account<'info, Mint>,

    /// User's DEGEN token account (ATA) - tokens will be burned from here
    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// Fee collector account (receives 5% of SOL)
    /// CHECK: This is the admin's account for collecting fees
    #[account(mut)]
    pub fee_collector: AccountInfo<'info>,

    /// SPL Token Program
    pub token_program: Program<'info, Token>,
    
    /// System Program
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ExecuteWithdrawal>) -> Result<()> {
    let global_vault = &mut ctx.accounts.global_vault;
    let user_vault = &mut ctx.accounts.user_vault;
    let clock = Clock::get()?;
    
    // Security checks
    require!(!global_vault.paused, VaultError::VaultPaused);
    require!(
        user_vault.has_pending_withdrawal(),
        VaultError::NoPendingWithdrawal
    );
    require!(
        user_vault.is_withdrawal_ready(clock.unix_timestamp),
        VaultError::WithdrawalTimelockActive
    );
    
    let degen_amount = user_vault.pending_withdrawal_amount;
    
    // Calculate SOL to return
    let total_sol = degen_amount
        .checked_mul(LAMPORTS_PER_SOL)
        .ok_or(VaultError::ArithmeticOverflow)?
        .checked_div(global_vault.conversion_rate)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    // Calculate fee (5%)
    let fee_amount = total_sol
        .checked_mul(global_vault.withdrawal_fee_bps as u64)
        .ok_or(VaultError::ArithmeticOverflow)?
        .checked_div(BPS_DIVISOR)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    let user_receives = total_sol
        .checked_sub(fee_amount)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    // Verify vault has enough SOL
    require!(
        global_vault.current_sol_balance >= total_sol,
        VaultError::VaultInsufficientFunds
    );
    
    // Burn DEGEN tokens from user
    let seeds = &[GLOBAL_VAULT_SEED, &[global_vault.bump]];
    let signer = &[&seeds[..]];
    
    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
            signer,
        ),
        degen_amount,
    )?;
    
    // Transfer SOL from GlobalVault PDA to user
    let vault_lamports = global_vault.to_account_info().lamports();
    
    // Verify vault has enough SOL
    require!(
        vault_lamports >= total_sol,
        VaultError::VaultInsufficientFunds
    );
    
    // Transfer SOL to user (95%)
    **global_vault.to_account_info().try_borrow_mut_lamports()? -= user_receives;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += user_receives;
    
    // Transfer fee to fee collector (5%)
    if fee_amount > 0 {
        **global_vault.to_account_info().try_borrow_mut_lamports()? -= fee_amount;
        **ctx.accounts.fee_collector.to_account_info().try_borrow_mut_lamports()? += fee_amount;
    }
    
    // Update vault balances
    global_vault.total_sol_withdrawal = global_vault
        .total_sol_withdrawal
        .checked_add(total_sol)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    global_vault.current_sol_balance = global_vault
        .current_sol_balance
        .checked_sub(total_sol)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    // Update user vault
    user_vault.total_withdrawal = user_vault
        .total_withdrawal
        .checked_add(total_sol)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    user_vault.total_degen_balance = user_vault
        .total_degen_balance
        .checked_sub(degen_amount)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    // Clear withdrawal request
    user_vault.pending_withdrawal_amount = 0;
    user_vault.withdrawal_unlock_ts = 0;
    user_vault.withdrawal_requested_at = 0;
    
    msg!("Withdrawal executed!");
    msg!("User: {}", ctx.accounts.user.key());
    msg!("DEGEN burned: {}", degen_amount);
    msg!("SOL returned to user: {} lamports", user_receives);
    msg!("Fee collected: {} lamports", fee_amount);
    
    Ok(())
}