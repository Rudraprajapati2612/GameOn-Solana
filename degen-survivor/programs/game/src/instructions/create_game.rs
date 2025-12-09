use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;
use crate::errors::GameError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct CreateGame<'info> {
    /// Backend authority creating the game
    #[account(mut)]
    pub creator: Signer<'info>,

    /// GameState account
    #[account(
        init,
        payer = creator,
        space = GameState::SIZE,
        seeds = [GAME_SEED, game_id.to_le_bytes().as_ref()],
        bump
    )]
    pub game_state: Account<'info, GameState>,

    /// DEGEN token mint
    pub token_mint: Account<'info, Mint>,

    /// Prize pool token account (holds all entry fees)
    #[account(
        init,
        payer = creator,
        token::mint = token_mint,
        token::authority = game_state,
    )]
    pub prize_pool_token_account: Account<'info, TokenAccount>,

    /// Token program
    pub token_program: Program<'info, Token>,

    /// System program
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateGame>,
    game_id: u64,
    game_type: GameType,
    start_time: i64,
    entry_fee: u64,
) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let clock = Clock::get()?;
    
    // Validate start time is in future
    require!(
        start_time > clock.unix_timestamp,
        GameError::InvalidStartTime
    );
    
    // Initialize game state
    game_state.game_id = game_id;
    game_state.game_type = game_type;
    game_state.creator = ctx.accounts.creator.key();
    game_state.status = GameStatus::Pending;
    game_state.created_at = clock.unix_timestamp;
    game_state.start_time = start_time;
    game_state.actual_start_time = None;
    game_state.end_time = None;
    game_state.current_round = 0;
    game_state.total_round = TOTAL_ROUNDS;
    
    // Calculate round deadlines (start + 0min, +2min, +4min, +6min, +8min)
    game_state.round_deadline = [
        start_time + ROUND_DURATION_SECONDS,
        start_time + (ROUND_GAP_SECONDS * 1) + ROUND_DURATION_SECONDS,
        start_time + (ROUND_GAP_SECONDS * 2) + ROUND_DURATION_SECONDS,
        start_time + (ROUND_GAP_SECONDS * 3) + ROUND_DURATION_SECONDS,
        start_time + (ROUND_GAP_SECONDS * 4) + ROUND_DURATION_SECONDS,
    ];
    
    game_state.entry_fee = entry_fee;
    game_state.prize_pool = 0;
    game_state.prize_pool_token_account = ctx.accounts.prize_pool_token_account.key();
    game_state.platform_fee_bps = PLATFORM_FEE_BPS;
    game_state.prize_pool_distributed = false;
    game_state.total_player = 0;
    game_state.max_player = MAX_PLAYER;
    game_state.player_finalized = false;
    
    // Define round types for this game
    game_state.round_types = [
        RoundType::PriceDirection,
        RoundType::Magnitude,
        RoundType::Comperative,
        RoundType::Range,
        RoundType::Trend,
    ];
    
    game_state.leaderboard_finalized = false;
    game_state.top_scorer = None;
    game_state.higest_score = 0;
    game_state.bump = ctx.bumps.game_state;
    
    msg!("Game created!");
    msg!("Game ID: {}", game_id);
    msg!("Type: {:?}", game_type);
    msg!("Start time: {}", start_time);
    msg!("Entry fee: {}", entry_fee);
    
    Ok(())
}