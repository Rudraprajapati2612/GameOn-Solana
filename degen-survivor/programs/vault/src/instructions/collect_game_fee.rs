use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::VaultError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct CollectGameFee<'info> {
    /// Platform admin collecting fee
    #[account(mut)]
    pub admin: Signer<'info>,

    /// GlobalVault PDA
    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED],
        bump = global_vault.bump,
        constraint = global_vault.admin == admin.key() @ VaultError::Unauthorized,
    )]
    pub global_vault: Account<'info, GlobalVault>,

    /// Platform wallet receiving SOL
    /// CHECK: This is where platform fee SOL goes
    #[account(mut)]
    pub platform_wallet: AccountInfo<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CollectGameFee>,
    game_id: u64,
    total_entry_fees_degen: u64,
) -> Result<()> {
    let global_vault = &mut ctx.accounts.global_vault;
    
    // Calculate platform fee in DEGEN (6% of entry fees)
    // Example: 50 players × 500 DEGEN = 25,000 DEGEN
    // Platform fee: 25,000 × 6% = 1,500 DEGEN
    let platform_fee_degen = (total_entry_fees_degen as u128)
        .checked_mul(600) // 6% = 600 basis points
        .and_then(|v| v.checked_div(10_000))
        .and_then(|v| u64::try_from(v).ok())
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    // Convert DEGEN to SOL
    // Formula: (platform_fee_degen × LAMPORTS_PER_SOL) / conversion_rate
    // Example: (1,500 × 1,000,000,000) / 10,000 = 150,000,000 lamports = 0.15 SOL
    let platform_fee_sol = platform_fee_degen
        .checked_mul(LAMPORTS_PER_SOL)
        .and_then(|v| v.checked_div(global_vault.conversion_rate))
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    // Verify vault has enough SOL
    let vault_lamports = global_vault.to_account_info().lamports();
    require!(
        vault_lamports >= platform_fee_sol,
        VaultError::VaultInsufficientFunds
    );
    
    // Transfer SOL from vault to platform wallet
    **global_vault.to_account_info().try_borrow_mut_lamports()? -= platform_fee_sol;
    **ctx.accounts.platform_wallet.try_borrow_mut_lamports()? += platform_fee_sol;
    
    // Update vault state
    global_vault.total_sol_withdrawal = global_vault
        .total_sol_withdrawal
        .checked_add(platform_fee_sol)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    global_vault.current_sol_balance = global_vault
        .current_sol_balance
        .checked_sub(platform_fee_sol)
        .ok_or(VaultError::ArithmeticOverflow)?;
    
    msg!("Platform game fee collected!");
    msg!("Game ID: {}", game_id);
    msg!("Entry fees (DEGEN): {}", total_entry_fees_degen);
    msg!("Platform fee (DEGEN equivalent): {}", platform_fee_degen);
    msg!("Platform fee (SOL): {} lamports", platform_fee_sol);
    msg!("Platform wallet: {}", ctx.accounts.platform_wallet.key());
    
    Ok(())
}