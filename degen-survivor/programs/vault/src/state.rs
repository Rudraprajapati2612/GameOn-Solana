use anchor_lang::prelude::*;

#[account]

pub struct GlobalVault {
    // use to pause or unpause withdrawal
    pub admin : Pubkey,
    // degentoken mint address
    pub token_mint : Pubkey,
    // total sol is deposited 
    pub total_sol_deposited : u64,
    // total so that is withdrawal 
    pub total_sol_withdrawal : u64,
    // current balance (deposited - withdrawal)
    pub current_sol_balance : u64,
    //  use for 1 solana to how much Degen token
    pub conversion_rate : u64 , 
    // withdrawal fees in bias point (100 bps = 1%)
    pub withdrawal_fee_bps : u16,
    // paused emergency stop
    pub paused : bool,

    pub bump : u8,

    pub _reserved : [u8;64]

}


impl GlobalVault {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 2 + 1 + 1 + 64;
}

#[account]

pub struct UserVault {
    pub owner : Pubkey ,

    pub total_degen_balance:u64,
    // total deposte of users 
    pub total_deposite : u64,

    pub total_withdrawal : u64,
    // amount of degegn token is pending withdrawal
    pub pending_withdrawal_amount : u64,

    pub withdrawal_unlock_ts: i64,

    pub withdrawal_requested_at : i64,

    pub bump : u8,
     
    pub _reserved : [u8;64]

    
}


impl UserVault{
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 64;
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]

pub enum WithdrawalState {
    // no pending withdrawal 
    None ,
    // Pending Withdrawal
    Pending,
    // Ready to execute 
    Ready,
}

impl UserVault{
    pub fn has_pending_withdrawal(&self)-> bool{
        self.pending_withdrawal_amount > 0
    }

    pub fn is_withdrawal_ready(&self,current_ts : i64)->bool {
        self.has_pending_withdrawal() && current_ts>= self.withdrawal_unlock_ts
    }
    pub fn get_withdrawal_state(&self, current_ts: i64) -> WithdrawalState {
        if !self.has_pending_withdrawal() {
            WithdrawalState :: None
        }else if self.is_withdrawal_ready(current_ts) {
            WithdrawalState::Ready
        }else {
            WithdrawalState::Pending
        }
    }
}