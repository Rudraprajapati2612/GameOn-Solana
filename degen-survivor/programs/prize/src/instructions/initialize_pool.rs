use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct InitializePrizePool<'info> {
    /// Backend authority or admin
    #[account(mut)]
    pub admin: Signer<'info>,

    /// PrizePool account
    #[account(
        init,
        payer = admin,
        space = PrizePool::SIZE,
        seeds = [PRIZE_POOL_SEED, game_id.to_le_bytes().as_ref()],
        bump
    )]
    pub prize_pool: Account<'info, PrizePool>,

    /// Game state account (from game program)
    /// CHECK: We verify this matches the game_id
    pub game_state: AccountInfo<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializePrizePool>,
    game_id: u64,
    total_pool: u64,
    game_state_bump: u8,
) -> Result<()> {
    let prize_pool = &mut ctx.accounts.prize_pool;
    let clock = Clock::get()?;
    
    // Calculate platform fee (6% of total pool)
    let platform_fee = (total_pool as u128)
        .checked_mul(PLATFORM_FEE_BPS as u128)
        .and_then(|v| v.checked_div(BPS_DIVISOR as u128))
        .and_then(|v| u64::try_from(v).ok())
        .unwrap_or(0);
    
    // Initialize prize pool
    prize_pool.game_id = game_id;
    prize_pool.game_state = ctx.accounts.game_state.key();
    prize_pool.total_pool = total_pool;
    prize_pool.distributed_amount = 0;
    prize_pool.platform_fee = platform_fee;
    prize_pool.platform_fee_collected = false;
    prize_pool.claims_processed = 0;
    prize_pool.total_winners = TOTAL_WINNERS;
    prize_pool.fully_distributed = false;
    prize_pool.created_at = clock.unix_timestamp;
    prize_pool.first_claim_at = None;
    prize_pool.last_claim_at = None;
    prize_pool.admin = ctx.accounts.admin.key();
    prize_pool.game_state_bump = game_state_bump;
    prize_pool.bump = ctx.bumps.prize_pool;
    
    msg!("Prize pool initialized!");
    msg!("Game ID: {}", game_id);
    msg!("Total pool: {}", total_pool);
    msg!("Platform fee: {} ({}%)", platform_fee, PLATFORM_FEE_BPS / 100);
    msg!("Distributable pool: {}", total_pool - platform_fee);
    
    Ok(())
}