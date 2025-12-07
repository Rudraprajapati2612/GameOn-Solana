use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::VaultError;
use crate::state::*;

/// Pause/unpause vault operations
#[derive(Accounts)]
pub struct SetPaused<'info> {
    /// Admin authority
    pub admin: Signer<'info>,

    /// GlobalVault PDA
    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED],
        bump = global_vault.bump,
        constraint = global_vault.admin == admin.key() @ VaultError::Unauthorized,
    )]
    pub global_vault: Account<'info, GlobalVault>,
}

pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
    let global_vault = &mut ctx.accounts.global_vault;
    global_vault.paused = paused;
    
    msg!("Vault paused status: {}", paused);
    Ok(())
}

/// Update conversion rate
#[derive(Accounts)]
pub struct UpdateConversionRate<'info> {
    /// Admin authority
    pub admin: Signer<'info>,

    /// GlobalVault PDA
    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED],
        bump = global_vault.bump,
        constraint = global_vault.admin == admin.key() @ VaultError::Unauthorized,
    )]
    pub global_vault: Account<'info, GlobalVault>,
}

pub fn update_conversion_rate(ctx: Context<UpdateConversionRate>, new_rate: u64) -> Result<()> {
    require!(new_rate > 0, VaultError::InvalidConversionRate);
    
    let global_vault = &mut ctx.accounts.global_vault;
    global_vault.conversion_rate = new_rate;
    
    msg!("Conversion rate updated to: {} DEGEN per SOL", new_rate);
    Ok(())
}

/// Update withdrawal fee
#[derive(Accounts)]
pub struct UpdateWithdrawalFee<'info> {
    /// Admin authority
    pub admin: Signer<'info>,

    /// GlobalVault PDA
    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED],
        bump = global_vault.bump,
        constraint = global_vault.admin == admin.key() @ VaultError::Unauthorized,
    )]
    pub global_vault: Account<'info, GlobalVault>,
}

pub fn update_withdrawal_fee(ctx: Context<UpdateWithdrawalFee>, new_fee_bps: u16) -> Result<()> {
    require!(
        new_fee_bps <= MAX_WITHDRAWAL_FEE_BPS,
        VaultError::FeeExceedsMaximum
    );
    
    let global_vault = &mut ctx.accounts.global_vault;
    global_vault.withdrawal_fee_bps = new_fee_bps;
    
    msg!("Withdrawal fee updated to: {}%", new_fee_bps / 100);
    Ok(())
}

/// Transfer admin authority to new admin
#[derive(Accounts)]
pub struct TransferAdmin<'info> {
    /// Current admin
    pub admin: Signer<'info>,

    /// New admin
    /// CHECK: This will become the new admin
    pub new_admin: AccountInfo<'info>,

    /// GlobalVault PDA
    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED],
        bump = global_vault.bump,
        constraint = global_vault.admin == admin.key() @ VaultError::Unauthorized,
    )]
    pub global_vault: Account<'info, GlobalVault>,
}

pub fn transfer_admin(ctx: Context<TransferAdmin>) -> Result<()> {
    let global_vault = &mut ctx.accounts.global_vault;
    global_vault.admin = ctx.accounts.new_admin.key();
    
    msg!("Admin transferred to: {}", global_vault.admin);
    Ok(())
}