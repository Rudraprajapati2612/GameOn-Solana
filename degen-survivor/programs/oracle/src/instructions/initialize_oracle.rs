use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::OracleError;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeOracle<'info> {
    /// Admin who will manage oracle configuration
    #[account(mut)]
    pub admin: Signer<'info>,

    /// OracleConfig account (global configuration)
    #[account(
        init,
        payer = admin,
        space = OracleConfig::SIZE,
        seeds = [ORACLE_CONFIG_SEED],
        bump
    )]
    pub oracle_config: Account<'info, OracleConfig>,

    /// System program
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeOracle>,
    btc_price_feed: Pubkey,
    sol_price_feed: Pubkey,
    staleness_threshold: i64,
    confidence_threshold: u64,
    min_publishers: u8,
) -> Result<()> {
    let oracle_config = &mut ctx.accounts.oracle_config;
    let clock = Clock::get()?;
    
    // Validate parameters
    require!(
        staleness_threshold > 0,
        OracleError::InvalidStalenessThreshold
    );
    
    require!(
        confidence_threshold > 0,
        OracleError::InvalidConfidenceThreshold
    );
    
    // Initialize configuration
    oracle_config.admin = ctx.accounts.admin.key();
    oracle_config.btc_price_feed = btc_price_feed;
    oracle_config.sol_price_feed = sol_price_feed;
    oracle_config.staleness_threshold = staleness_threshold;
    oracle_config.confidence_threshold = confidence_threshold;
    oracle_config.min_publishers = min_publishers;
    oracle_config.emergency_pause = false;
    oracle_config.last_updated = clock.unix_timestamp;
    oracle_config.bump = ctx.bumps.oracle_config;
    
    msg!("Oracle initialized!");
    msg!("Admin: {}", oracle_config.admin);
    msg!("BTC feed: {}", btc_price_feed);
    msg!("SOL feed: {}", sol_price_feed);
    msg!("Staleness threshold: {}s", staleness_threshold);
    msg!("Confidence threshold: {}", confidence_threshold);
    msg!("Min publishers: {}", min_publishers);
    
    Ok(())
}