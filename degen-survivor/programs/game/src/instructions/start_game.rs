use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GameError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct  StartGame <'info> {
    #[account(mut)]
    pub creator : Signer<'info>,
    #[account(
        mut,
        seeds = [GAME_SEED, game_id.to_le_bytes().as_ref()],
        bump = game_state.bump,
        constraint = game_state.creator == creator.key() @ GameError::Unauthorized,
    )]
    pub game_state : Account<'info,GameState>,
    #[account(
        init ,
        payer = creator,
        space = RoundResult::SIZE,
        seeds = [ROUND_RESULT_SEED,game_id.to_le_bytes().as_ref() , &[1u8]],
        bump ,
    )]
    pub round_result : Account<'info,RoundResult>,

    pub system_program : Program<'info,System>,
}
pub fn handeler(ctx:Context<StartGame>,start_btc_price:Option<u64>, start_sol_price:Option<u64>,game_id : u64)->Result<()>{
    
    let game_state  = &mut ctx.accounts.game_state;
    let round_result = &mut ctx.accounts.round_result;
    let clock = Clock::get()?;

    require!(game_state.status==GameStatus::Pending,
    GameError::InvalidGameStatus
    );

    require!(
        clock.unix_timestamp>game_state.start_time,
        GameError::GameNotStarted
    );
    // check for total player is greater than min player 
    require!(game_state.total_player >= MIN_PLAYER,
        GameError::InsufficientPlayers
    );

    // update game state result 

    game_state.status = GameStatus::Active;
    game_state.current_round = 1 ;
    game_state.actual_start_time = Some(clock.unix_timestamp);
    game_state.player_finalized= true;


    //  Initailize round 1 

    round_result.game_id = game_id;
    round_result.round_number = 1 ;
    round_result.round_type = game_state.round_types[0].clone();
    round_result.start_price_btc = start_btc_price ;
    round_result.start_price_sol = start_sol_price;
    round_result.end_price_btc = None ; 
    round_result.end_price_sol = None;
    round_result.price_change_btc = 0 ;
    round_result.price_change_sol = 0 ;
    round_result.round_start_ts = clock.unix_timestamp;
    round_result.round_end_ts = clock.unix_timestamp + ROUND_DURATION_SECONDS;
    round_result.evaluation_ts = None;
    round_result.total_predection = 0;
    round_result.correct_predection =0 ;
    round_result.wrong_predection = 0;
    round_result.bump = ctx.bumps.round_result;

    msg!("Game started!");
    msg!("Game ID: {}", game_id);
    msg!("Total players: {}", game_state.total_player);
    msg!("Prize pool: {}", game_state.prize_pool);
    msg!("Round 1 start price BTC: {:?}", start_btc_price);
    msg!("Round 1 start price SOL: {:?}", start_sol_price);
    
    Ok(())
}