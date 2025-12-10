use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("B4yqJH8RCYohHEbsZZXX7Pty84wYDG6kkav5U14zzvRb");

#[program]
pub mod oracle {
    use super::*;

    /// Initialize oracle configuration
    /// Sets up Pyth feed addresses and validation thresholds
    /// Should be called once during deployment
    pub fn initialize_oracle(
        ctx: Context<InitializeOracle>,
        btc_price_feed: Pubkey,
        sol_price_feed: Pubkey,
        staleness_threshold: i64,
        confidence_threshold: u64,
        min_publishers: u8,
    ) -> Result<()> {
        instructions::initialize_oracle::handler(
            ctx,
            btc_price_feed,
            sol_price_feed,
            staleness_threshold,
            confidence_threshold,
            min_publishers,
        )
    }

    /// Fetch price from Pyth and store validated snapshot
    /// Called by backend at start and end of each round
    /// Validates price quality before storing
    pub fn fetch_and_store(
        ctx: Context<FetchAndStore>,
        game_id: u64,
        round_number: u8,
        asset_type: AssetType,
        snapshot_type: SnapshotType,
    ) -> Result<()> {
        instructions::fetch_and_store::handler(ctx, game_id, round_number, asset_type, snapshot_type)
    }

    /// Update oracle configuration
    /// Only admin can call this
    /// Allows updating feed addresses, thresholds, or emergency pause
    pub fn update_oracle(
        ctx: Context<UpdateOracle>,
        new_btc_feed: Option<Pubkey>,
        new_sol_feed: Option<Pubkey>,
        new_staleness_threshold: Option<i64>,
        new_confidence_threshold: Option<u64>,
        new_min_publishers: Option<u8>,
        emergency_pause: Option<bool>,
    ) -> Result<()> {
        instructions::update_oracle::handler(
            ctx,
            new_btc_feed,
            new_sol_feed,
            new_staleness_threshold,
            new_confidence_threshold,
            new_min_publishers,
            emergency_pause,
        )
    }
}