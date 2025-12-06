use anchor_lang::prelude::*;

// REPLACE THIS WITH YOUR ACTUAL PROGRAM ID FROM STEP 3
declare_id!("BfCM2AwLS8UXboAExYh1p5Csx6RX8tb7QKYgNKvbAUje");

#[program]
pub mod prize {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Prize program initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}