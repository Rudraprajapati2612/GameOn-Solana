use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GameError;
use crate::state::*;


#[derive(Accounts)]
#[instruction(game_id: u64, next_round: u8)]

pub struct  AdvanceRound<'info> {
    #[account(mut)]
    pub creator : Signer<'info>,
    #[account(
        mut,
        seeds = [GAME_SEED,game_id.to_le_bytes().as_ref()],
        bump = game_state.bump,
        constraint = game_state.creator == creator.key() @ GameError::Unauthorized
    )]
    pub game_state : Account<'info,GameState>,
    #[account(
        init , 
        payer = creator,
        space = RoundResult::SIZE,
        seeds = [ROUND_RESULT_SEED,game_id.to_le_bytes().as_ref() , &[next_round]],
        bump
    )]
    pub next_round_result : Account<'info,RoundResult>,

    pub system_program : Program<'info,System>,
}

pub fn handler(ctx:Context<AdvanceRound>,game_id:u64,next_round:u8,start_price_btc:Option<u64>,start_price_sol:Option<u64>)->Result<()>{

    let game_state = &mut ctx.accounts.game_state;
    let next_round_result = &mut ctx.accounts.next_round_result;
    let clock = Clock::get()?;


    require!(
        game_state.status == GameStatus::Active,
        GameError::InvalidGameStatus
    );

    require!(next_round == game_state.current_round+1,GameError::InvalidRoundNumber);

    // check for last round 
    require!(next_round<=TOTAL_ROUNDS,GameError::InvalidRoundNumber);

    // update game state 
    game_state.current_round = next_round;

    next_round_result.game_id = game_id;
    next_round_result.round_number = next_round;
    next_round_result.round_type = game_state.round_types[(next_round - 1) as usize];
    next_round_result.start_price_btc = start_price_btc;
    next_round_result.end_price_btc = None;
    next_round_result.start_price_sol = start_price_sol;
    next_round_result.end_price_sol = None;
    next_round_result.price_change_btc = 0;
    next_round_result.price_change_sol = 0;
    next_round_result.correct_answer = None;
    next_round_result.round_start_ts = clock.unix_timestamp;
    next_round_result.round_end_ts = clock.unix_timestamp + ROUND_DURATION_SECONDS;
    next_round_result.evaluation_ts = None;
    next_round_result.total_predection = 0;
    next_round_result.correct_predection = 0;
    next_round_result.partial_correct = 0;
    next_round_result.wrong_predection = 0;
    next_round_result.bump = ctx.bumps.next_round_result;


    msg!("Round advanced!");
    msg!("Game ID: {}", game_id);
    msg!("New round: {}", next_round);
    msg!("Start price BTC: {:?}", start_price_btc);
    msg!("Start price SOL: {:?}", start_price_sol);

    Ok(())
}