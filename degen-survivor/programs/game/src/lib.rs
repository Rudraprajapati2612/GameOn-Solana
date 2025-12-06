use anchor_lang::prelude::*;

// REPLACE THIS WITH YOUR ACTUAL PROGRAM ID FROM STEP 3
declare_id!("qhWdqP2HtsKTNs5QcDPKSvtjjcP9pmkjtyy8HU4yASt");

#[program]
pub mod game {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Game program initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}