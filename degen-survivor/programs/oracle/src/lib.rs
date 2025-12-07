use anchor_lang::prelude::*;

// REPLACE THIS WITH YOUR ACTUAL PROGRAM ID FROM STEP 3
declare_id!("B4yqJH8RCYohHEbsZZXX7Pty84wYDG6kkav5U14zzvRb");

#[program]
pub mod oracle {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Oracle program initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}      