use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GameError;
use crate::program::Game;
use crate::state::*;


#[derive(Accounts)]
#[instruction(game_id : u64 , round_number : u8)]


pub struct SumbitPredection <'info> {

    pub player :  Signer<'info>,

    #[account(
    seeds = [GAME_SEED,game_id.to_le_bytes().as_ref()],
    bump = game_state.bump
    )]

    pub game_state : Account<'info,GameState>,

    #[account(
        mut,
        seeds = [PLAYER_SEED , game_id.to_le_bytes().as_ref(),player.key().as_ref()],
        bump = player_state.bump,
        constraint = player_state.player == player.key() @ GameError::Unauthorized,
    )]
    pub player_state :Account<'info,PlayerState>,

    #[account(
        seeds = [ROUND_RESULT_SEED, game_id.to_le_bytes().as_ref(), &[round_number]],
        bump = round_result.bump,
    )]
    pub round_result: Account<'info, RoundResult>,
}

pub fn handeler(ctx:Context<SumbitPredection>,_game_id:u64,round_number:u8,choice : PredectionChoice)->Result<()>{
    let game_state = &mut ctx.accounts.game_state;
    let player_state = &mut ctx.accounts.player_state;
    let round_result = &mut ctx.accounts.round_result;
    let clock = Clock::get()?;
    // validate game is active 
    require!(game_state.status == GameStatus::Active, GameError::InvalidGameStatus);

    // validate round number 
    require!(round_number >=1 && round_number<=5, GameError::InvalidRoundNumber);
    
    require!(
        round_number == game_state.current_round,
        GameError::InvalidRoundNumber
    );
    // check if player is already predicted
    
    require!(!player_state.has_predicted(round_number),GameError::AlreadyPredicted);
    // check predection window is open 
    require!(
        clock.unix_timestamp < round_result.round_end_ts,
        GameError::PredictionWindowClosed
    );
    let lockout_start = round_result.round_end_ts
    .checked_sub(PREDICTION_LOCKOUT_SECONDS)
    .ok_or(GameError::ArithmeticOverflow)?;

    require!(
        clock.unix_timestamp < lockout_start,
        GameError::PredictionTooLate
    );

    let response_time  = clock.unix_timestamp.checked_sub(round_result.round_start_ts).ok_or(GameError::ArithmeticOverflow)? as u32;
    let predection = RoundPrediction{
        round : round_number,
        choice,
        sumbited_at : clock.unix_timestamp,
        response_time,
        point_earned : 0 ,// filled during evualaltion time 
        is_correct : false

    };

   let index = (round_number-1 ) as usize;
   player_state.predection[index] = Some(predection);

    player_state.total_reponse_time = player_state.total_reponse_time.checked_add(response_time as u64)
    .ok_or(GameError ::ArithmeticOverflow)?;

    //  if tie braker then chekcs for first round question 
    if round_number==1{
        player_state.first_prediction_ts = clock.unix_timestamp;
    }

    msg!("Prediction submitted!");
    msg!("Player: {}", ctx.accounts.player.key());
    msg!("Round: {}", round_number);
    msg!("Choice: {:?}", choice);
    msg!("Response time: {}ms", response_time * 1000);
    Ok(())
}