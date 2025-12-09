use anchor_lang::prelude::*;

#[error_code]
pub enum PrizeError {
    #[msg("Prize pool has not been initialized for this game")]
    PrizePoolNotInitialized,
    
    #[msg("Prize has already been claimed by this player")]
    AlreadyClaimed,
    
    #[msg("Player is not a winner (rank must be 1-10)")]
    NotAWinner,
    
    #[msg("Game is not completed yet, prizes cannot be claimed")]
    GameNotCompleted,
    
    #[msg("Leaderboard has not been finalized")]
    LeaderboardNotFinalized,
    
    #[msg("Prize pool has insufficient funds")]
    InsufficientPrizePool,
    
    #[msg("Platform fee has already been collected")]
    FeeAlreadyCollected,
    
    #[msg("Invalid rank provided (must be 1-10)")]
    InvalidRank,
    
    #[msg("Prize amount does not match calculated amount")]
    PrizeAmountMismatch,
    
    #[msg("Unauthorized: Only admin can perform this action")]
    Unauthorized,
    
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    
    #[msg("Game state verification failed")]
    GameStateVerificationFailed,
    
    #[msg("Player state verification failed")]
    PlayerStateVerificationFailed,
    
    #[msg("No platform fees available to withdraw")]
    NoFeesAvailable,
}