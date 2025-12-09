use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GameError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]

pub struct FinalizedLeaderboard<'info>{
    #[account(mut)]
    pub creator : Signer<'info>,
    
    #[account(
        mut,
        seeds = [GAME_SEED, game_id.to_le_bytes().as_ref()],
        bump = game_state.bump,
        constraint = game_state.creator == creator.key() @ GameError::Unauthorized,
    )]
    pub game_state: Account<'info, GameState>,

    /// PlayerState account to set rank (passed in by backend)
    #[account(
        mut,
        seeds = [PLAYER_SEED, game_id.to_le_bytes().as_ref(), player_state.player.key().as_ref()],
        bump = player_state.bump,
    )]
    pub player_state: Account<'info, PlayerState>,
}

pub fn handler(ctx:Context<FinalizedLeaderboard>, _game_id:u64,rank:u16)->Result<()>{
    let game_state = &mut ctx.accounts.game_state;
    let player_state = &mut ctx.accounts.player_state;
    let clock = Clock::get()?;
    
    // Validate game is active
    require!(
        game_state.status == GameStatus::Active,
        GameError::InvalidGameStatus
    );
    // check for all round is completed 
    require!(
        player_state.all_round_completed,
        GameError::PlayersNotEvaluated
    );

    player_state.final_rank = Some(rank);

    let prize_amount = calculate_prize_amount(
        rank,
        game_state.prize_pool,
        game_state.platform_fee_bps,
    );

    player_state.prize_amount = prize_amount;

    if rank == 1 {
        game_state.top_scorer = Some(player_state.player);
        game_state.higest_score = player_state.total_score;
    }

    msg!("Player ranked!");
    msg!("Player: {}", player_state.player);
    msg!("Username: {}", player_state.username);
    msg!("Rank: {}", rank);
    msg!("Total score: {}", player_state.total_score);
    msg!("Prize: {}", prize_amount);
    Ok(())
}

fn calculate_prize_amount(rank: u16, prize_pool: u64, platform_fee_bps: u16) -> u64 {
    // Calculate pool after platform fee
    let platform_fee = (prize_pool as u128)
        .checked_mul(platform_fee_bps as u128)
        .and_then(|v| v.checked_div(BPS_DIVISOR as u128))
        .and_then(|v| u64::try_from(v).ok())
        .unwrap_or(0);
    
    let distributable_pool = prize_pool.saturating_sub(platform_fee);
    
    // Calculate prize based on rank
    let prize_bps = match rank {
        1 => PRIZE_RANK_1_BPS,
        2 => PRIZE_RANK_2_BPS,
        3 => PRIZE_RANK_3_BPS,
        4 | 5 => PRIZE_RANK_4_5_BPS,
        6..=10 => PRIZE_RANK_6_10_BPS,
        _ => 0,
    };
    
    (distributable_pool as u128)
        .checked_mul(prize_bps as u128)
        .and_then(|v| v.checked_div(BPS_DIVISOR as u128))
        .and_then(|v| u64::try_from(v).ok())
        .unwrap_or(0)
}