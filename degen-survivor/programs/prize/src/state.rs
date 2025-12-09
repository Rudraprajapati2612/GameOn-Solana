use anchor_lang::prelude::*;

/// Prize pool for a specific game
/// Tracks total pool, distribution, and claim history
#[account]
pub struct PrizePool {
    /// Which game this prize pool belongs to
    pub game_id: u64,
    
    /// Reference to game state account
    pub game_state: Pubkey,
    
    /// Total prize pool (entry fees collected)
    pub total_pool: u64,
    
    /// Amount distributed so far
    pub distributed_amount: u64,
    
    /// Platform fee (6% of total pool)
    pub platform_fee: u64,
    
    /// Platform fee collected flag
    pub platform_fee_collected: bool,
    
    /// Number of winners who have claimed
    pub claims_processed: u16,
    
    /// Total number of winners (always 10 for top 10)
    pub total_winners: u16,
    
    /// Whether all prizes have been distributed
    pub fully_distributed: bool,
    
    /// When prize pool was initialized
    pub created_at: i64,
    
    /// When first prize was claimed
    pub first_claim_at: Option<i64>,
    
    /// When last prize was claimed
    pub last_claim_at: Option<i64>,
    
    /// Admin who can collect platform fees
    pub admin: Pubkey,
    
    /// Game state PDA bump (for signing token transfers)
    pub game_state_bump: u8,
    
    /// PDA bump
    pub bump: u8,
    
    /// Reserved for future use
    pub _reserved: [u8; 64],
}

impl PrizePool {
    // 8 + 8 + 32 + 8 + 8 + 8 + 1 + 2 + 2 + 1 + 8 + 9 + 9 + 32 + 1 + 1 + 64
    pub const SIZE: usize = 8 + 8 + 32 + 8 + 8 + 8 + 1 + 2 + 2 + 1 + 8 + 9 + 9 + 32 + 1 + 1 + 64;
}

/// Individual claim record for a player
/// Prevents double-claiming and provides audit trail
#[account]
pub struct ClaimRecord {
    /// Which game this claim is for
    pub game_id: u64,
    
    /// Player who claimed
    pub player: Pubkey,
    
    /// Player's final rank (1-10)
    pub rank: u16,
    
    /// Amount claimed (in DEGEN tokens)
    pub amount: u64,
    
    /// When prize was claimed
    pub claimed_at: i64,
    
    /// Transaction signature (for reference)
    pub claim_signature: [u8; 64],
    
    /// Whether claim was successful
    pub claim_successful: bool,
    
    /// PDA bump
    pub bump: u8,
}

impl ClaimRecord {
    // 8 + 8 + 32 + 2 + 8 + 8 + 64 + 1 + 1
    pub const SIZE: usize = 8 + 8 + 32 + 2 + 8 + 8 + 64 + 1 + 1;
}

/// Platform fee accumulator
/// Tracks all fees collected across all games
#[account]
pub struct FeeCollector {
    /// Admin who can withdraw fees
    pub admin: Pubkey,
    
    /// Total fees accumulated (in DEGEN tokens)
    pub total_fees_collected: u64,
    
    /// Total fees withdrawn by admin
    pub total_fees_withdrawn: u64,
    
    /// Current available balance
    pub available_balance: u64,
    
    /// Number of games processed
    pub games_processed: u64,
    
    /// Last withdrawal timestamp
    pub last_withdrawal_at: Option<i64>,
    
    /// PDA bump
    pub bump: u8,
}

impl FeeCollector {
    // 8 + 32 + 8 + 8 + 8 + 8 + 9 + 1
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 9 + 1;
}