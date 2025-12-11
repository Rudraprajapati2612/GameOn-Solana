use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("8CLxjoAivuuxrFNK8aanU2f6Nw7L6tBT2xEgNNoUMmNE");

#[program]
pub mod game {
    use super::*;

    /// Create a new game
    /// Called by backend at scheduled time (e.g., 9:00 AM for 10:00 AM game)
    pub fn create_game(
        ctx: Context<CreateGame>,
        game_id: u64,
        game_type: GameType,
        start_time: i64,
        entry_fee: u64,
    ) -> Result<()> {
        instructions::create_game::handler(ctx, game_id, game_type, start_time, entry_fee)
    }

    /// Player joins an upcoming game
    /// Burns entry fee and creates PlayerState account
    pub fn join_game(
        ctx: Context<JoinGame>,
        game_id: u64,
        username: String,
    ) -> Result<()> {
        instructions::join_game::handler(ctx, game_id, username)
    }

    /// Start the game (called by backend at exactly start_time)
    /// Changes status to Active and initializes Round 1
    pub fn start_game(
        ctx: Context<StartGame>,
        game_id: u64,
        start_price_btc: Option<u64>,
        start_price_sol: Option<u64>,
    ) -> Result<()> {
        instructions::start_game::handler(ctx, start_price_btc, start_price_sol,game_id)
    }

    /// Player submits prediction for current round
    /// Stores choice and timestamp on-chain
    pub fn submit_prediction(
        ctx: Context<SumbitPredection>,
        game_id: u64,
        round_number: u8,
        choice: PredectionChoice,
    ) -> Result<()> {
        instructions::submit_prediction::handler(ctx, game_id, round_number, choice)
    }

    /// Evaluate a player's prediction for a completed round
    /// Called by backend after round ends, calculates points
    pub fn evaluate_round(
        ctx: Context<EvualatedRound>,
        game_id: u64,
        round_number: u8,
    ) -> Result<()> {
        instructions::evaluate_round::handler(ctx, game_id, round_number)
    }

    /// Advance to next round
    /// Called by backend after all players evaluated for current round
    pub fn advance_round(
        ctx: Context<AdvanceRound>,
        game_id: u64,
        next_round: u8,
        start_price_btc: Option<u64>,
        start_price_sol: Option<u64>,
    ) -> Result<()> {
        instructions::advance_round::handler(ctx, game_id, next_round, start_price_btc, start_price_sol)
    }

    /// Set rank and prize amount for a player
    /// Called by backend for each player after all rounds complete
    pub fn finalize_leaderboard(
        ctx: Context<FinalizedLeaderboard>,
        game_id: u64,
        rank: u16,
    ) -> Result<()> {
        instructions::finalize_leaderboard::handler(ctx, game_id, rank)
    }
    
    /// Complete the game (mark as Completed status)
    /// Called by backend after all players ranked
    pub fn complete_game(ctx: Context<CompleteGame>, _game_id: u64) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        let clock = Clock::get()?;
        
        game_state.status = GameStatus::Completed;
        game_state.leaderboard_finalized = true;
        game_state.end_time = Some(clock.unix_timestamp);
        
        msg!("Game completed!");
        msg!("Final players: {}", game_state.total_player);
        msg!("Prize pool: {}", game_state.prize_pool);
        msg!("Top scorer: {:?}", game_state.top_scorer);
        msg!("Highest score: {}", game_state.higest_score);
        
        Ok(())
    }

    /// Player claims their prize
    /// Can only be called by winners (rank 1-10)
    pub fn claim_prize(
        ctx: Context<ClaimPrize>,
        game_id: u64,
    ) -> Result<()> {
        instructions::claim_prize::handler(ctx, game_id)
    }
    
    /// Update round result with end prices and correct answer
    /// Called by backend after round ends (before evaluation)
    pub fn update_round_result(
        ctx: Context<UpdateRoundResult>,
        _game_id: u64,
        _round_number: u8,
        end_price_btc: Option<u64>,
        end_price_sol: Option<u64>,
        correct_answer: PredectionChoice,
    ) -> Result<()> {
        let round_result = &mut ctx.accounts.round_result;
        let clock = Clock::get()?;
        
        round_result.end_price_btc = end_price_btc;
        round_result.end_price_sol = end_price_sol;
        round_result.correct_answer = Some(correct_answer);
        round_result.evaluation_ts = Some(clock.unix_timestamp);
        
        // Calculate price changes
        if let (Some(start_btc), Some(end_btc)) = (round_result.start_price_btc, end_price_btc) {
            round_result.price_change_btc = (end_btc as i64) - (start_btc as i64);
        }
        
        if let (Some(start_sol), Some(end_sol)) = (round_result.start_price_sol, end_price_sol) {
            round_result.price_change_sol = (end_sol as i64) - (start_sol as i64);
        }
        
        msg!("Round result updated!");
        msg!("End price BTC: {:?}", end_price_btc);
        msg!("End price SOL: {:?}", end_price_sol);
        msg!("Correct answer: {:?}", correct_answer);
        
        Ok(())
    }
}

// Additional account contexts

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct CompleteGame<'info> {
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"game", game_id.to_le_bytes().as_ref()],
        bump = game_state.bump,
    )]
    pub game_state: Account<'info, GameState>,
}

#[derive(Accounts)]
#[instruction(game_id: u64, round_number: u8)]
pub struct UpdateRoundResult<'info> {
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"round-result", game_id.to_le_bytes().as_ref(), &[round_number]],
        bump = round_result.bump,
    )]
    pub round_result: Account<'info, RoundResult>,
}