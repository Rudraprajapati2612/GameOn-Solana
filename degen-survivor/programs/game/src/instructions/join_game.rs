use anchor_lang::prelude::*;
use anchor_spl::token::{ Burn, Mint, Token, TokenAccount, burn};

use crate::constants::*;
use crate::errors::GameError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id:u64)]

pub struct  JoinGame<'info>{
    #[account(mut)]
    pub player : Signer<'info>,

    #[account(
        mut,
        seeds = [GAME_SEED,game_id.to_le_bytes().as_ref()],
        bump = game_state.bump
    )]
    pub game_state: Account<'info,GameState>,

    #[account(
        init,
        payer = player,
        space = PlayerState::SIZE,
        seeds = [PLAYER_SEED,game_id.to_le_bytes().as_ref(),player.key().as_ref()],
        bump
    )]
    pub player_state : Account<'info,PlayerState>,

    // degen token min from vault program 
    #[account(mut)]
    pub token_mint :Account<'info,Mint>,
    
    pub player_token_account : Account<'info,TokenAccount>,
    
      /// Token program
    pub token_program: Program<'info, Token>,
    
      /// System program
    pub system_program: Program<'info, System>,
}

pub fn handeler (ctx:Context<JoinGame>,game_id:u64 , username:String)->Result<()>{
    let game_state = &mut ctx.accounts.game_state;
    let player_state = &mut ctx.accounts.player_state;

    let clock = Clock::get()?;


    require!(game_state.status == GameStatus::Pending , GameError :: GameAlreadyStarted);

    require!(game_state.total_player < game_state.max_player , GameError::GameFull);

    let regestration_deadline = game_state.start_time.checked_sub(REGISTRATION_CLOSE_BEFORE_START)
    .ok_or(GameError::ArithmeticOverflow)?;

    // check for current time is less than regerestraion deadline

    require!(
        clock.unix_timestamp< regestration_deadline,
        GameError::RegistrationClosed
    );

    // validate username 

    require!(
        username.len() <= MAX_USERNAME_LENGTH,
        GameError::UsernameTooLong
    );

    let token_burn_account = Burn{
        mint : ctx.accounts.token_mint.to_account_info(),
        from : ctx.accounts.player_token_account.to_account_info(),
        authority : ctx.accounts.player.to_account_info()
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(),token_burn_account );

    burn(cpi_ctx, game_state.entry_fee)?;

    player_state.game_id=game_id;
    player_state.player = ctx.accounts.player.key();
    player_state.username = username.clone();
    player_state.entry_slot = game_state.total_player as u64 + 1;
    player_state.predection= [None,None,None,None,None];
    player_state.scores = [0,0,0,0,0];
    player_state.total_score = 0;
    player_state.round_evaluated=0;
    player_state.all_round_completed=false;
    player_state.final_rank = None;
    player_state.prize_amount=0;
    player_state.prize_claimed=false;
    player_state.total_reponse_time=0;
    player_state.avg_response_time=0;
    player_state.first_prediction_ts=0;
    player_state.bump = ctx.bumps.player_state;

    // update game state 
    game_state.total_player+=1;
    game_state.prize_pool = game_state.prize_pool.checked_add(game_state.entry_fee).ok_or(GameError::ArithmeticOverflow)?;

    msg!("Player joined game!");
    msg!("Game ID: {}", game_id);
    msg!("Player: {}", ctx.accounts.player.key());
    msg!("Username: {}", username);
    msg!("Entry slot: {}", player_state.entry_slot);
    msg!("Total players: {}", game_state.total_player);
    msg!("Prize pool: {}", game_state.prize_pool);

    Ok(())
}
