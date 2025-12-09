use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::errors::PrizeError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct ClaimPrize<'info> {
    /// Player claiming prize
    #[account(mut)]
    pub player: Signer<'info>,

    /// PrizePool PDA
    #[account(
        mut,
        seeds = [PRIZE_POOL_SEED, game_id.to_le_bytes().as_ref()],
        bump = prize_pool.bump,
    )]
    pub prize_pool: Account<'info, PrizePool>,

    /// ClaimRecord PDA (created per player)
    #[account(
        init,
        payer = player,
        space = ClaimRecord::SIZE,
        seeds = [CLAIM_RECORD_SEED, game_id.to_le_bytes().as_ref(), player.key().as_ref()],
        bump
    )]
    pub claim_record: Account<'info, ClaimRecord>,

    /// Game state (checked by other program)
    /// CHECK: Verified by game program via seeds
    pub game_state: AccountInfo<'info>,

    /// Player state (checked by other program)
    /// CHECK: Verified by game program
    pub player_state: AccountInfo<'info>,

    /// DEGEN mint
    pub token_mint: Account<'info, Mint>,

    /// Prize pool DEGEN token account
    #[account(
        mut,
        token::mint = token_mint,
    )]
    pub prize_pool_token_account: Account<'info, TokenAccount>,

    /// Player receiving the DEGEN prize
    #[account(
        mut,
        token::mint = token_mint,
        token::authority = player,
    )]
    pub player_token_account: Account<'info, TokenAccount>,

    /// PDA that signs token transfers
    /// CHECK: PDA authority for transfers
    pub prize_pool_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ClaimPrize>,
    game_id: u64,
    rank: u16,
    prize_amount: u64,
) -> Result<()> {
    let prize_pool = &mut ctx.accounts.prize_pool;
    let claim_record = &mut ctx.accounts.claim_record;
    let clock = Clock::get()?;

    // ---------------------------------------------------------------
    // VALIDATIONS
    // ---------------------------------------------------------------

    require!(rank >= 1 && rank <= 10, PrizeError::NotAWinner);

    let expected_amount = calculate_prize_amount(
        rank,
        prize_pool.total_pool,
        prize_pool.platform_fee,
    );

    require!(
        prize_amount == expected_amount,
        PrizeError::PrizeAmountMismatch
    );

    require!(
        ctx.accounts.prize_pool_token_account.amount >= prize_amount,
        PrizeError::InsufficientPrizePool
    );

    // ---------------------------------------------------------------
    // PDA SIGNER SEEDS — FIXED (no temporaries)
    // ---------------------------------------------------------------

    let game_id_bytes = game_id.to_le_bytes(); // <-- FIX

    let seeds: &[&[u8]] = &[
        b"game",
        &game_id_bytes,
        &[prize_pool.game_state_bump],
    ];

    let signer_seeds: &[&[&[u8]]] = &[seeds];

    // ---------------------------------------------------------------
    // TRANSFER PRIZE
    // ---------------------------------------------------------------

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.prize_pool_token_account.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.prize_pool_authority.to_account_info(),
            },
            signer_seeds,
        ),
        prize_amount,
    )?;

    // ---------------------------------------------------------------
    // UPDATE PRIZE POOL STATE
    // ---------------------------------------------------------------

    prize_pool.distributed_amount = prize_pool
        .distributed_amount
        .checked_add(prize_amount)
        .ok_or(PrizeError::ArithmeticOverflow)?;

    prize_pool.claims_processed = prize_pool
        .claims_processed
        .checked_add(1)
        .ok_or(PrizeError::ArithmeticOverflow)?;

    if prize_pool.first_claim_at.is_none() {
        prize_pool.first_claim_at = Some(clock.unix_timestamp);
    }

    prize_pool.last_claim_at = Some(clock.unix_timestamp);

    if prize_pool.claims_processed == prize_pool.total_winners {
        prize_pool.fully_distributed = true;
    }

    // ---------------------------------------------------------------
    // CREATE CLAIM RECORD
    // ---------------------------------------------------------------

    claim_record.game_id = game_id;
    claim_record.player = ctx.accounts.player.key();
    claim_record.rank = rank;
    claim_record.amount = prize_amount;
    claim_record.claimed_at = clock.unix_timestamp;
    claim_record.claim_signature = [0u8; 64];
    claim_record.claim_successful = true;
    claim_record.bump = ctx.bumps.claim_record;

    // ---------------------------------------------------------------
    // LOGS
    // ---------------------------------------------------------------

    msg!("Prize claimed successfully!");
    msg!("Player: {}", ctx.accounts.player.key());
    msg!("Rank: {}", rank);
    msg!("Prize amount: {}", prize_amount);
    msg!("Total distributed: {}", prize_pool.distributed_amount);
    msg!(
        "Claims processed: {}/{}",
        prize_pool.claims_processed,
        prize_pool.total_winners
    );

    Ok(())
}

//
// PRIZE CALCULATION
//

pub fn calculate_prize_amount(rank: u16, total_pool: u64, platform_fee: u64) -> u64 {
    let distributable_pool = total_pool.saturating_sub(platform_fee);

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
