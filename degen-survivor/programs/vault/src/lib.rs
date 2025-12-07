use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("HkZW45NrfmzTjV7q7mrdj51BZ5ryBhaEZQet5oRfJmi");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, sol_amount: u64) -> Result<()> {
        deposit::handler(ctx, sol_amount)
    }

    pub fn request_withdrawal(
        ctx: Context<RequestWithdrawal>,
        degen_amount: u64,
    ) -> Result<()> {
        request_withdrawal::handler(ctx, degen_amount)
    }

    pub fn execute_withdrawal(ctx: Context<ExecuteWithdrawal>) -> Result<()> {
        execute_withdrawal::handeler(ctx)
    }

    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        admin::set_paused(ctx, paused)
    }
}
