use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GameError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct ClaimPrize<'info> {
    /// Player claiming prize
    #[account(mut)]
    pub player: Signer<'info>,

    /// GameState account
    #[account(
        seeds = [GAME_SEED, game_id.to_le_bytes().as_ref()],
        bump = game_state.bump,
    )]
    pub game_state: Account<'info, GameState>,

    /// PlayerState account
    #[account(
        mut,
        seeds = [PLAYER_SEED, game_id.to_le_bytes().as_ref(), player.key().as_ref()],
        bump = player_state.bump,
        constraint = player_state.player == player.key() @ GameError::Unauthorized,
    )]
    pub player_state: Account<'info, PlayerState>,
    
    // Note: Prize distribution will be handled by Prize Distributor program
    // via CPI. For now, we just mark as claimed.
}

pub fn handler(ctx: Context<ClaimPrize>, _game_id: u64) -> Result<()> {
    let game_state = &ctx.accounts.game_state;
    let player_state = &mut ctx.accounts.player_state;
    
    // Validate game is completed
    require!(
        game_state.status == GameStatus::Completed,
        GameError::InvalidGameStatus
    );
    
    // Validate leaderboard is finalized
    require!(
        game_state.leaderboard_finalized,
        GameError::LeaderboardNotFinalized
    );
    
    // Validate player has a rank
    let rank = player_state.final_rank
        .ok_or(GameError::LeaderboardNotFinalized)?;
    
    // Validate player is a winner (top 10)
    require!(rank <= 10, GameError::NotAWinner);
    
    // Validate not already claimed
    require!(
        !player_state.prize_claimed,
        GameError::PrizeAlreadyClaimed
    );
    
    // Mark as claimed
    player_state.prize_claimed = true;
    
    // TODO: Add CPI to Prize Distributor program here
    // For now, just mark as claimed
    
    msg!("Prize claimed!");
    msg!("Player: {}", player_state.player);
    msg!("Rank: {}", rank);
    msg!("Prize amount: {}", player_state.prize_amount);
    
    Ok(())
}