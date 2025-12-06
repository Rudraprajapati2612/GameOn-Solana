use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;


#[derive(Accounts)]

pub struct  RequestWithdrawal<'info>{

    #[account(mut)]
    pub user : Signer<'info>,
    #[account(
        seeds = [GLOBAL_VAULT_SEED],
        bump= global_vault.bump
    )]
    pub global_vault : Account<'info,GlobalVault>,

    #[account(
        mut,
        seeds = [USER_VAULT_SEED,user.key().as_ref()],
        bump = user_vault.bump,
        constraint = user_vault.owner == user.key()  @ VaultError::Unauthorized
    )]
    pub user_vault : Account<'info,UserVault>
}


pub fn handler (ctx:Context<RequestWithdrawal>,degen_amount:u64)-> Result<()>{
    let global_vault = &mut ctx.accounts.global_vault;
    let user_vault = &mut ctx.accounts.user_vault;
    
    let clock = Clock::get()?;

    require!(!global_vault.paused,VaultError::VaultPaused);
    require!(degen_amount>0,VaultError::InvalidWithdrawalAmount);

    require!(!user_vault.has_pending_withdrawal(),VaultError::WithdrawalAlreadyPending);

    require!(
        user_vault.total_degen_balance >= degen_amount,
        VaultError::InsufficientBalance
    );

    //  DEGEN TOken * Lamport / conversion rate 
    // user has 5000 Token * 1 lamport(10^9)
    // conversion rate = 10000
    // 5*10^12/10000
    // 500,000,000 in lamport ==0.5sol 
    let sol_amount = degen_amount
        .checked_mul(LAMPORTS_PER_SOL)
        .ok_or(VaultError::ArithmeticOverflow)?
        .checked_div(global_vault.conversion_rate)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    // Check vault has enough SOL
    require!(
        global_vault.current_sol_balance >= sol_amount,
        VaultError::VaultInsufficientFunds
    );


    // now set withdrawal request 
    user_vault.pending_withdrawal_amount = degen_amount;
    user_vault.withdrawal_requested_at = clock.unix_timestamp;
    user_vault.withdrawal_unlock_ts=clock.unix_timestamp.checked_add(WITHDRAWAL_TIMELOCK_SECONDS).ok_or(VaultError::ArithmeticOverflow)?;
    msg!("Withdrawal requested!");
    msg!("User: {}", ctx.accounts.user.key());
    msg!("DEGEN amount: {}", degen_amount);
    msg!("Unlock time: {} (in 24 hours)", user_vault.withdrawal_unlock_ts);
    Ok(())
}