use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GameError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64, round_number: u8)]

pub struct EvualatedRound <'info> {
    #[account(mut)]
    pub creator : Signer<'info>,
    #[account(
        seeds = [GAME_SEED , game_id.to_le_bytes().as_ref()],
        bump = game_state.bump,
        constraint = game_state.creator == creator.key() @ GameError::Unauthorized,
    )]
    pub game_state   : Account<'info,GameState>,

    #[account(
        mut,
        seeds = [PLAYER_SEED, game_id.to_le_bytes().as_ref(), player_state.player.key().as_ref()],
        bump = player_state.bump,
    )]
    pub player_state : Account<'info,PlayerState>,
    #[account(
        mut,
        seeds = [ROUND_RESULT_SEED, game_id.to_le_bytes().as_ref(), &[round_number]],
        bump = round_result.bump,
    )]
    pub round_result : Account<'info,RoundResult>,
}

pub fn handler(ctx:Context<EvualatedRound>,_gmae_id : u64,round_number:u8 )->Result<()>{
    let player_state = &mut ctx.accounts.player_state;
    let round_result = &mut ctx.accounts.round_result;
    let clock = Clock::get()?;

    // check round is ended 
    require!(clock.unix_timestamp> round_result.round_end_ts,GameError::RoundNotEnded);

    // check for predection 
    let predection = player_state.get_prediction(round_number).ok_or(GameError::NoPredictionFound)?;
    
     // Check not already evaluated
     require!(
        predection.point_earned == 0,
        GameError::AlreadyEvaluated
    );

    let correct_answer = round_result.correct_answer.ok_or(GameError::NoPredictionFound)?;
    let points = calculate_points(
        round_result.round_type,
        predection.choice,
        correct_answer,
        round_result.price_change_btc,
        round_result.price_change_sol,
    );

    // update player predection per round 

    if let Some(pred) = &mut player_state.predection[(round_number - 1) as usize] {
        pred.point_earned = points;
        pred.is_correct = points == POINT_EXCATE;
    }
    
     // Update player scores
     player_state.scores[(round_number - 1) as usize] = points;
     player_state.total_score = player_state.total_score
         .checked_add(points)
         .ok_or(GameError::ArithmeticOverflow)?;
     
     player_state.round_evaluated += 1;

     if player_state.round_evaluated == TOTAL_ROUNDS {
        player_state.all_round_completed = true;
        
        // Calculate average response time
        player_state.avg_response_time = 
            (player_state.total_reponse_time / TOTAL_ROUNDS as u64) as u32;
    }

    round_result.total_predection += 1;

     
    if points == POINT_EXCATE {
        round_result.correct_predection += 1;
    } else if points == POINT_PARTIAL || points == POINT_CLOSER {
        round_result.partial_correct += 1;
    } else {
        round_result.wrong_predection += 1;
    }
    Ok(())
}


fn calculate_points (round_type: RoundType,
        player_choice : PredectionChoice,
        correct_answer : PredectionChoice,
        _price_chage_btc :i64,
        _price_change_sol : i64
    )->u16{

        match round_type {
            RoundType::PriceDirection => {
                if player_choice == correct_answer {
                    POINT_EXCATE
                } else {
                    POINT_WRONG
                }
            }

                
        RoundType::Magnitude =>{
            calculate_magnitude_point(player_choice,correct_answer)
        }

        RoundType::Comperative => {
            if player_choice ==correct_answer {
                POINT_EXCATE
            }else {
                POINT_WRONG
            }
        }

        RoundType::Range =>{
            calculate_range_point(player_choice, correct_answer)
        }

        RoundType::Trend => {
            if player_choice ==correct_answer {
                POINT_EXCATE
            }
            else {
                POINT_WRONG
            }
        }

        }

    }

    fn calculate_magnitude_point(player_choice : PredectionChoice,correct_answer : PredectionChoice)->u16 {
        if player_choice == correct_answer {
            return POINT_EXCATE;
        }

        let is_adjacent = match(player_choice,correct_answer){
        (PredectionChoice::RangeA, PredectionChoice::RangeB) => true,
        (PredectionChoice::RangeB, PredectionChoice::RangeA) => true,
        (PredectionChoice::RangeB, PredectionChoice::RangeC) => true,
        (PredectionChoice::RangeC, PredectionChoice::RangeB) => true,
        (PredectionChoice::RangeC, PredectionChoice::RangeD) => true,
        (PredectionChoice::RangeD, PredectionChoice::RangeC) => true,
        _ => false,
        };

        if is_adjacent {
            POINT_PARTIAL
        } else {
            POINT_WRONG
        }
    }


    fn calculate_range_point(player_choice:PredectionChoice,
    correct_answer:PredectionChoice) ->u16 {
        if player_choice == correct_answer {
            return  POINT_EXCATE;
        }

        let get_zone_number = |choice : PredectionChoice| ->u8 {
            match choice{
                PredectionChoice::ZoneA =>1,
                PredectionChoice::ZoneB =>2,
                PredectionChoice::ZoneC =>3,
                PredectionChoice::ZoneD =>4,
                _=>0,
            }
        };


        let player_zone = get_zone_number(player_choice);
        let correct_zone = get_zone_number(correct_answer);
        
        let distance = player_zone.abs_diff(correct_zone);
        
        match distance {
            1 => POINT_CLOSER,  // 1 zone away
            2 => POINT_FAR,    // 2 zones away
            _ => POINT_WRONG,  // 3+ zones away
        }
    }