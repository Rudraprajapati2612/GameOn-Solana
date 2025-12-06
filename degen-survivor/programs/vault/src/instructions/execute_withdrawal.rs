use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::constants::*;
use crate::errors::VaultError;
use crate::state::*;


#[derive(Accounts)]

pub struct  ExecuteWithdrawal<'info>{
    #[account(mut)]

    pub user : Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED,global_vault.key().as_ref()],
        bump = global_vault.bump
    )]

    pub global_vault : Account<'info,GlobalVault>,

    pub token_mint : Account<'info,Mint>,
    
    #[account(
        mut,
        seeds = [USER_VAULT_SEED, user.key().as_ref()],
        bump = user_vault.bump,
        constraint = user_vault.owner == user.key() @ VaultError::Unauthorized,
    )]
    pub user_vault: Account<'info, UserVault>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub fee_collector : AccountInfo<'info>,

    pub system_program : Program<'info,System>,

    pub token_program : Program<'info,Token>
}

pub fn handeler (ctx:Context<ExecuteWithdrawal>)->Result<()>{
    let global_vault = &mut ctx.accounts.global_vault;
    let user_vault = &mut ctx.accounts.user_vault;

    let clock = Clock::get()?;
    // whenever the global vault is false &&  user has pending withdrawal && has Withdrawal state  is ready  to execute
    require!(!global_vault.paused, VaultError::VaultPaused);
    require!(
        user_vault.has_pending_withdrawal(),
        VaultError::NoPendingWithdrawal
    );
    require!(
        user_vault.is_withdrawal_ready(clock.unix_timestamp),
        VaultError::WithdrawalTimelockActive
    );
    Ok(())
}