use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::OracleError;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateOracle<'info> {
    /// Admin updating configuration
    pub admin: Signer<'info>,

    /// OracleConfig account
    #[account(
        mut,
        seeds = [ORACLE_CONFIG_SEED],
        bump = oracle_config.bump,
        constraint = oracle_config.admin == admin.key() @ OracleError::Unauthorized,
    )]
    pub oracle_config: Account<'info, OracleConfig>,
}

pub fn handler(
    ctx: Context<UpdateOracle>,
    new_btc_feed: Option<Pubkey>,
    new_sol_feed: Option<Pubkey>,
    new_staleness_threshold: Option<i64>,
    new_confidence_threshold: Option<u64>,
    new_min_publishers: Option<u8>,
    emergency_pause: Option<bool>,
) -> Result<()> {
    let oracle_config = &mut ctx.accounts.oracle_config;
    let clock = Clock::get()?;
    
    // Update fields if provided
    if let Some(btc_feed) = new_btc_feed {
        oracle_config.btc_price_feed = btc_feed;
        msg!("Updated BTC feed: {}", btc_feed);
    }
    
    if let Some(sol_feed) = new_sol_feed {
        oracle_config.sol_price_feed = sol_feed;
        msg!("Updated SOL feed: {}", sol_feed);
    }
    
    if let Some(staleness) = new_staleness_threshold {
        require!(staleness > 0, OracleError::InvalidStalenessThreshold);
        oracle_config.staleness_threshold = staleness;
        msg!("Updated staleness threshold: {}s", staleness);
    }
    
    if let Some(confidence) = new_confidence_threshold {
        require!(confidence > 0, OracleError::InvalidConfidenceThreshold);
        oracle_config.confidence_threshold = confidence;
        msg!("Updated confidence threshold: {}", confidence);
    }
    
    if let Some(min_pubs) = new_min_publishers {
        oracle_config.min_publishers = min_pubs;
        msg!("Updated min publishers: {}", min_pubs);
    }
    
    if let Some(pause) = emergency_pause {
        oracle_config.emergency_pause = pause;
        msg!("Emergency pause: {}", pause);
    }
    
    oracle_config.last_updated = clock.unix_timestamp;
    
    msg!("Oracle configuration updated!");
    
    Ok(())
}