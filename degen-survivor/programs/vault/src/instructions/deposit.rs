use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;
use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED],
        bump = global_vault.bump
    )]
    pub global_vault: Account<'info, GlobalVault>,

    #[account(
        init_if_needed,
        payer = user,
        space = UserVault::SIZE,
        seeds = [USER_VAULT_SEED, user.key().as_ref()],
        bump
    )]
    pub user_vault: Account<'info, UserVault>,
    
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        address = global_vault.token_mint
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>
}

pub fn handler(ctx: Context<Deposit>, sol_amount: u64) -> Result<()> {
    let global_vault = &mut ctx.accounts.global_vault;
    let user_vault = &mut ctx.accounts.user_vault;

    // Check if vault is paused
    require!(!global_vault.paused, VaultError::VaultPaused);
    
    // Validate deposit amount
    require!(
        sol_amount >= MIN_DEPOSIT_LAMPORTS,
        VaultError::DepositTooSmall
    );
    require!(
        sol_amount <= MAX_DEPOSIT_LAMPORTS,
        VaultError::DepositTooLarge
    );

    // Initialize user vault if needed
    if user_vault.owner == Pubkey::default() {
        user_vault.owner = ctx.accounts.user.key();
        user_vault.total_degen_balance = 0;
        user_vault.total_deposite = 0;
        user_vault.total_withdrawal = 0;
        user_vault.pending_withdrawal_amount = 0;
        user_vault.withdrawal_requested_at = 0;
        user_vault.withdrawal_unlock_ts = 0;
        user_vault.bump = ctx.bumps.user_vault;
    }

    // Calculate DEGEN tokens to mint based on conversion rate
    let degen_amount = sol_amount
        .checked_mul(global_vault.conversion_rate)
        .ok_or(VaultError::ArithmeticOverflow)?
        .checked_div(LAMPORTS_PER_SOL)
        .ok_or(VaultError::ArithmeticOverflow)?;

    // Transfer SOL from user to Global Vault PDA
    let transfer_accounts = system_program::Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: global_vault.to_account_info()
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_accounts
    );

    system_program::transfer(cpi_ctx, sol_amount)?;

    // Prepare PDA signer seeds for minting
    let seeds = &[GLOBAL_VAULT_SEED, &[global_vault.bump]];
    let signer_seeds = &[&seeds[..]];

    // Mint DEGEN tokens to user's token account
    let mint_to_accounts = anchor_spl::token::MintTo {
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: global_vault.to_account_info()
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        mint_to_accounts,
        signer_seeds
    );

    anchor_spl::token::mint_to(cpi_ctx, degen_amount)?;

    // Update global vault balances
    global_vault.total_sol_deposited = global_vault
        .total_sol_deposited
        .checked_add(sol_amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    global_vault.current_sol_balance = global_vault
        .current_sol_balance
        .checked_add(sol_amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    // Update user vault balances
    user_vault.total_deposite = user_vault
        .total_deposite
        .checked_add(sol_amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    user_vault.total_degen_balance = user_vault
        .total_degen_balance
        .checked_add(degen_amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    // Emit success logs
    msg!("Deposit successful!");
    msg!("User: {}", ctx.accounts.user.key());
    msg!("SOL deposited: {} lamports", sol_amount);
    msg!("DEGEN minted: {}", degen_amount);
    
    Ok(())
}