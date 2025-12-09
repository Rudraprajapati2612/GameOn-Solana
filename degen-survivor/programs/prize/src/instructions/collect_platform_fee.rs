use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::errors::PrizeError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct CollectPlatformFee<'info> {
    /// Admin collecting fees
    #[account(mut)]
    pub admin: Signer<'info>,

    /// PrizePool account
    #[account(
        mut,
        seeds = [PRIZE_POOL_SEED, game_id.to_le_bytes().as_ref()],
        bump = prize_pool.bump,
        constraint = prize_pool.admin == admin.key() @ PrizeError::Unauthorized,
    )]
    pub prize_pool: Account<'info, PrizePool>,

    /// Global FeeCollector PDA
    #[account(
        init_if_needed,
        payer = admin,
        space = FeeCollector::SIZE,
        seeds = [FEE_COLLECTOR_SEED],
        bump
    )]
    pub fee_collector: Account<'info, FeeCollector>,

    /// DEGEN token mint
    pub token_mint: Account<'info, Mint>,

    /// Prize pool token account (source of tokens)
    #[account(
        mut,
        token::mint = token_mint,
    )]
    pub prize_pool_token_account: Account<'info, TokenAccount>,

    /// Admin token account (receiver of platform fee)
    #[account(
        mut,
        token::mint = token_mint,
        token::authority = admin,
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    /// PDA authority for prize pool
    /// CHECK: PDA signer
    pub prize_pool_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CollectPlatformFee>, game_id: u64) -> Result<()> {
    let prize_pool = &mut ctx.accounts.prize_pool;
    let fee_collector = &mut ctx.accounts.fee_collector;
    let clock = Clock::get()?;

    // -------------------------------------------------------------------------
    // VALIDATIONS
    // -------------------------------------------------------------------------

    require!(
        !prize_pool.platform_fee_collected,
        PrizeError::FeeAlreadyCollected
    );

    let platform_fee = prize_pool.platform_fee;

    require!(platform_fee > 0, PrizeError::NoFeesAvailable);

    // -------------------------------------------------------------------------
    // PDA SIGNER SEEDS FIX (no temporary values)
    // -------------------------------------------------------------------------

    let game_id_bytes = game_id.to_le_bytes(); // <-- FIXED

    let seeds: &[&[u8]] = &[
        PRIZE_POOL_SEED,
        &game_id_bytes,
        &[prize_pool.bump],
    ];

    let signer_seeds: &[&[&[u8]]] = &[seeds];

    // -------------------------------------------------------------------------
    // TRANSFER PLATFORM FEE TO ADMIN
    // -------------------------------------------------------------------------

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx
                    .accounts
                    .prize_pool_token_account
                    .to_account_info(),
                to: ctx.accounts.admin_token_account.to_account_info(),
                authority: ctx.accounts.prize_pool_authority.to_account_info(),
            },
            signer_seeds,
        ),
        platform_fee,
    )?;

    // Mark as collected
    prize_pool.platform_fee_collected = true;

    // -------------------------------------------------------------------------
    // INITIALIZE FEE COLLECTOR IF FIRST TIME
    // -------------------------------------------------------------------------

    if fee_collector.admin == Pubkey::default() {
        fee_collector.admin = ctx.accounts.admin.key();
        fee_collector.total_fees_collected = 0;
        fee_collector.total_fees_withdrawn = 0;
        fee_collector.available_balance = 0;
        fee_collector.games_processed = 0;
        fee_collector.last_withdrawal_at = None;
        fee_collector.bump = ctx.bumps.fee_collector;
    }

    // -------------------------------------------------------------------------
    // UPDATE FEE COLLECTOR STATS
    // -------------------------------------------------------------------------

    fee_collector.total_fees_collected = fee_collector
        .total_fees_collected
        .checked_add(platform_fee)
        .ok_or(PrizeError::ArithmeticOverflow)?;

    fee_collector.total_fees_withdrawn = fee_collector
        .total_fees_withdrawn
        .checked_add(platform_fee)
        .ok_or(PrizeError::ArithmeticOverflow)?;

    fee_collector.games_processed = fee_collector
        .games_processed
        .checked_add(1)
        .ok_or(PrizeError::ArithmeticOverflow)?;

    fee_collector.last_withdrawal_at = Some(clock.unix_timestamp);

    // -------------------------------------------------------------------------
    // LOGGING
    // -------------------------------------------------------------------------

    msg!("Platform fee collected successfully!");
    msg!("Game ID: {}", game_id);
    msg!("Fee amount: {}", platform_fee);
    msg!("Total fees collected: {}", fee_collector.total_fees_collected);
    msg!("Games processed: {}", fee_collector.games_processed);

    Ok(())
}
