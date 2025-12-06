use anchor_lang::prelude::*;
use::anchor_spl::token::{Mint,Token,TokenAccount};

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = GlobalVault::SIZE,
        seeds = [GLOBAL_VAULT_SEED],
        bump
    )]
    pub global_vault: Account<'info, GlobalVault>,

    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer = admin,
        mint::decimals=TOKEN_DECIMALS,
        mint::authority = global_vault
    )]
    pub token_mint : Account<'info,Mint>,

    pub token_program : Program<'info,Token>,

    

}



pub fn handler(ctx:Context<Initialize>)->Result<()>{
    let global_vault = &mut ctx.accounts.global_vault;
    
    global_vault.admin = ctx.accounts.admin.key();
    global_vault.total_sol_deposited=0;
    global_vault.total_sol_withdrawal=0;
    global_vault.current_sol_balance = 0;

    global_vault.conversion_rate = DEFAULT_CONVERSION_RATE;
    global_vault.withdrawal_fee_bps = DEFAULT_WITHDRAWAL_FEE_BPS;
    global_vault.paused = false;
    global_vault.bump = ctx.bumps.global_vault;

    msg!("Vault initialized!");
    msg!("Admin: {}", global_vault.admin);
    msg!("Token Mint: {}", global_vault.token_mint);
    msg!("Conversion Rate: {} DEGEN per SOL", global_vault.conversion_rate);
    msg!("Withdrawal Fee: {}%", global_vault.withdrawal_fee_bps / 100);
    Ok(())
}