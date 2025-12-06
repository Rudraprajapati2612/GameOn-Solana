use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;
pub mod constants;
pub mod errors;
// REPLACE THIS WITH YOUR ACTUAL PROGRAM ID FROM STEP 3
declare_id!("HkZW45NrfmzTjV7q7mrdj51BZ5ryBhaEZQet5oRfJmi");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Vault program initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
   

    pub system_program: Program<'info, System>,
}
