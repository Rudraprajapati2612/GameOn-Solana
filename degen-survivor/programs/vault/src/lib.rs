use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("5D2EFjnokFHzGeVQ2AMpdMC6SazYaashKHAzMGWRzJVd");

#[program]
pub mod vault {
    use super::*;

    /// Initialize the vault and create DEGEN token mint
    /// This should be called once during deployment
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Deposit SOL and mint DEGEN tokens to user
    /// 
    /// # Arguments
    /// * `sol_amount` - Amount of SOL to deposit (in lamports)
    pub fn deposit(ctx: Context<Deposit>, sol_amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, sol_amount)
    }

    /// Request withdrawal - starts 24-hour timelock
    /// 
    /// # Arguments
    /// * `degen_amount` - Amount of DEGEN tokens to withdraw
    pub fn request_withdrawal(ctx: Context<RequestWithdrawal>, degen_amount: u64) -> Result<()> {
        instructions::request_withdrawal::handler(ctx, degen_amount)
    }

    /// Execute withdrawal after timelock expires
    /// Burns DEGEN tokens and returns SOL to user (minus 5% fee)
    pub fn execute_withdrawal(ctx: Context<ExecuteWithdrawal>) -> Result<()> {
        instructions::execute_withdrawal::handler(ctx)
    }

    /// Collect platform fee from a game (in SOL)
    /// Called by platform after game completes
    /// Converts DEGEN entry fees to SOL and transfers to platform
    pub fn collect_game_fee(
        ctx: Context<CollectGameFee>,
        game_id: u64,
        total_entry_fees_degen: u64,
    ) -> Result<()> {
        instructions::collect_game_fee::handler(ctx, game_id, total_entry_fees_degen)
    }

    /// Admin: Pause or unpause vault operations
    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        instructions::admin::set_paused(ctx, paused)
    }

    /// Admin: Update conversion rate
    pub fn update_conversion_rate(ctx: Context<UpdateConversionRate>, new_rate: u64) -> Result<()> {
        instructions::admin::update_conversion_rate(ctx, new_rate)
    }

    /// Admin: Update withdrawal fee
    pub fn update_withdrawal_fee(ctx: Context<UpdateWithdrawalFee>, new_fee_bps: u16) -> Result<()> {
        instructions::admin::update_withdrawal_fee(ctx, new_fee_bps)
    }

    /// Admin: Transfer admin authority to new address
    pub fn transfer_admin(ctx: Context<TransferAdmin>) -> Result<()> {
        instructions::admin::transfer_admin(ctx)
    }
}