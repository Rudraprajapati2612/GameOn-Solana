use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::{constants::*, cpi};
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

    let degen_amount = user_vault.pending_withdrawal_amount;

    // calculate Sol to return to the user 

    let total_sol = degen_amount.checked_mul(LAMPORTS_PER_SOL).ok_or(VaultError::ArithmeticOverflow)?.
    checked_div(global_vault.conversion_rate).ok_or(VaultError::ArithmeticOverflow)?;


    // calculate 5% platform fees 

    let fee_amount = total_sol.checked_mul(global_vault.withdrawal_fee_bps as u64).ok_or(VaultError::ArithmeticOverflow)?

    .checked_div(BPS_DIVISOR).ok_or(VaultError::ArithmeticOverflow)?;

    let user_recive = total_sol.checked_sub(fee_amount).ok_or(VaultError::ArithmeticOverflow)?;

    // Now Checks Vault has enough 

    require!(
        global_vault.current_sol_balance >= total_sol,
        VaultError::VaultInsufficientFunds
    );
    // we will burn the degen token 

    let seeds = &[GLOBAL_VAULT_SEED , &[global_vault.bump]];

    let signer_seed = &[&seeds[..]];

    let burn_ctx_account = anchor_spl::token::Burn{
        mint : ctx.accounts.token_mint.to_account_info(),
        from  : ctx.accounts.user_token_account.to_account_info(),
        authority:ctx.accounts.user.to_account_info()
    };

    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), burn_ctx_account, signer_seed);

    anchor_spl::token::burn(cpi_ctx, degen_amount)?;

    // transfer sol to the user 

    **global_vault.to_account_info().try_borrow_mut_lamports()? -=user_recive;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()?+=user_recive;

    if fee_amount > 0 {
        **global_vault.to_account_info().try_borrow_mut_lamports()? -= fee_amount;
        **ctx.accounts.fee_collector.to_account_info().try_borrow_mut_lamports()? += fee_amount;
    }

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
    msg!("SOL returned to user: {} lamports", user_recive);
    msg!("Fee collected: {} lamports", fee_amount);

    Ok(())
}