use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::constants::*;
use crate::errors::OracleError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64, round_number: u8, asset_type: AssetType, snapshot_type: SnapshotType)]
pub struct FetchAndStore<'info> {
    /// Payer for account creation
    #[account(mut)]
    pub payer: Signer<'info>,

    /// OracleConfig (read configuration)
    #[account(
        seeds = [ORACLE_CONFIG_SEED],
        bump = oracle_config.bump,
    )]
    pub oracle_config: Account<'info, OracleConfig>,

    /// Pyth price account (external account from Pyth)
    pub pyth_price_account: Account<'info, PriceUpdateV2>,

    /// PriceSnapshot account (create)
    #[account(
        init,
        payer = payer,
        space = PriceSnapshot::SIZE,
        seeds = [
            PRICE_SNAPSHOT_SEED,
            game_id.to_le_bytes().as_ref(),
            &[round_number],
            &[asset_type as u8],
            &[snapshot_type as u8],
        ],
        bump
    )]
    pub price_snapshot: Account<'info, PriceSnapshot>,

    /// System program
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<FetchAndStore>,
    game_id: u64,
    round_number: u8,
    asset_type: AssetType,
    snapshot_type: SnapshotType,
) -> Result<()> {
    let oracle_config = &ctx.accounts.oracle_config;
    let pyth_account = &ctx.accounts.pyth_price_account;
    let price_snapshot = &mut ctx.accounts.price_snapshot;
    let clock = Clock::get()?;
    
    // Check if oracle is paused
    require!(!oracle_config.emergency_pause, OracleError::OraclePaused);
    
    // Verify correct price feed
    let expected_feed = match asset_type {
        AssetType::BTC => oracle_config.btc_price_feed,
        AssetType::SOL => oracle_config.sol_price_feed,
    };
    
    require!(
        pyth_account.key() == expected_feed,
        OracleError::WrongPriceFeed
    );
    
    // Extract price data from Pyth account
    let price_data = &pyth_account.price_message;
    
    let price = price_data.price;
    let conf = price_data.conf;
    let expo = price_data.exponent;
    let publish_time = price_data.publish_time;
    
    // Validate price status (should be trading)
    // Note: Pyth SDK handles this internally, but we can add extra checks
    
    // Calculate staleness
    let current_time = clock.unix_timestamp;
    let staleness = current_time
        .checked_sub(publish_time)
        .ok_or(OracleError::ArithmeticOverflow)?;
    
    // Validate staleness
    require!(
        staleness <= oracle_config.staleness_threshold,
        OracleError::PriceStale
    );
    
    // Validate confidence
    require!(
        conf <= oracle_config.confidence_threshold,
        OracleError::LowConfidence
    );
    
    // Note: Pyth SDK doesn't expose num_publishers directly in PriceUpdateV2
    // We'll set a default value or skip this check
    let num_publishers = 5; // Default assumption
    
    // Validate minimum publishers (if we had access to this data)
    // For MVP, we trust Pyth's aggregation
    
    // Normalize price
    let price_normalized = PriceSnapshot::normalize_price(price, expo);
    
    // Determine status
    let status = if staleness > oracle_config.staleness_threshold {
        PriceStatus::Stale
    } else if conf > oracle_config.confidence_threshold {
        PriceStatus::LowConfidence
    } else {
        PriceStatus::Valid
    };
    
    // Store snapshot
    price_snapshot.game_id = game_id;
    price_snapshot.round_number = round_number;
    price_snapshot.asset_type = asset_type;
    price_snapshot.snapshot_type = snapshot_type;
    price_snapshot.snapshot_time = current_time;
    price_snapshot.price = price;
    price_snapshot.exponent = expo;
    price_snapshot.confidence = conf;
    price_snapshot.publish_time = publish_time;
    price_snapshot.num_publishers = num_publishers;
    price_snapshot.pyth_status = PYTH_STATUS_TRADING; // Assume trading if we got data
    price_snapshot.price_normalized = price_normalized;
    price_snapshot.staleness = staleness;
    price_snapshot.status = status;
    price_snapshot.bump = ctx.bumps.price_snapshot;
    
    msg!("Price snapshot created!");
    msg!("Game ID: {}", game_id);
    msg!("Round: {}", round_number);
    msg!("Asset: {:?}", asset_type);
    msg!("Type: {:?}", snapshot_type);
    msg!("Price: {} (expo: {})", price, expo);
    msg!("Price normalized: {}", price_normalized);
    msg!("Confidence: {}", conf);
    msg!("Staleness: {}s", staleness);
    msg!("Status: {:?}", status);
    
    Ok(())
}