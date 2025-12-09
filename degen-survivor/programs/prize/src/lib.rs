use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("BfCM2AwLS8UXboAExYh1p5Csx6RX8tb7QKYgNKvbAUje");

#[program]
pub mod prize {
    use super::*;

    /// Initialize prize pool for a game
    /// Called by backend after game is completed
    /// Sets up prize distribution accounts
    pub fn initialize_prize_pool(
        ctx: Context<InitializePrizePool>,
        game_id: u64,
        total_pool: u64,
        game_state_bump: u8,
    ) -> Result<()> {
        instructions::initialize_pool::handler(ctx, game_id, total_pool, game_state_bump)
    }

    /// Player claims their prize
    /// Can only be called by winners (rank 1-10)
    /// Transfers DEGEN tokens to winner's account
    pub fn claim_prize(
        ctx: Context<ClaimPrize>,
        game_id: u64,
        rank: u16,
        prize_amount: u64,
    ) -> Result<()> {
        instructions::claim_prize::handler(ctx, game_id, rank, prize_amount)
    }

    /// Admin collects platform fee from a game
    /// 6% of total prize pool
    /// Can only be called once per game
    pub fn collect_platform_fee(
        ctx: Context<CollectPlatformFee>,
        game_id: u64,
    ) -> Result<()> {
        instructions::collect_platform_fee::handler(ctx, game_id)
    }
}