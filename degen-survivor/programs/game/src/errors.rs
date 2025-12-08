use anchor_lang::prelude::*;
#[error_code]

pub enum  GameError {
    #[msg("Game has Not started yet")]
    GameNotStarted,
    #[msg("Game has already started, cannot join")]
    GameAlreadyStarted,
    
    #[msg("Game is full, maximum players reached")]
    GameFull,
    
    #[msg("Game has already been completed")]
    GameAlreadyCompleted,
    
    #[msg("Player has already joined this game")]
    PlayerAlreadyJoined,
    
    #[msg("Player has not joined this game")]
    PlayerNotInGame,

    #[msg("Player has already submitted prediction for this round")]
    AlreadyPredicted,
    
    #[msg("Prediction window has closed for this round")]
    PredictionWindowClosed,
    
    #[msg("Cannot predict in last 5 seconds (anti-cheat)")]
    PredictionTooLate,
    
    #[msg("Wrong round number provided")]
    InvalidRoundNumber,
    
    #[msg("Round has not ended yet, cannot evaluate")]
    RoundNotEnded,
    
    #[msg("Player did not make a prediction for this round")]
    NoPredictionFound,
    
    #[msg("This round has already been evaluated for this player")]
    AlreadyEvaluated,
    
    #[msg("Not all players have been evaluated yet")]
    PlayersNotEvaluated,
    
    #[msg("Leaderboard has not been finalized yet")]
    LeaderboardNotFinalized,
    
    #[msg("Leaderboard has already been finalized")]
    LeaderboardAlreadyFinalized,
    
    #[msg("Player is not a winner (rank > 10)")]
    NotAWinner,
    
    #[msg("Prize has already been claimed")]
    PrizeAlreadyClaimed,
    
    #[msg("Unauthorized: Only game creator can perform this action")]
    Unauthorized,
    
    #[msg("Game start time must be in the future")]
    InvalidStartTime,
    
    #[msg("Not enough players to start game")]
    InsufficientPlayers,
    
    #[msg("Username is too long (max 20 characters)")]
    UsernameTooLong,
    
    #[msg("Invalid game status for this operation")]
    InvalidGameStatus,
    
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    
    #[msg("Invalid prediction choice for this round type")]
    InvalidPredictionChoice,
    
    #[msg("Cannot join game, registration closes 2 minutes before start")]
    RegistrationClosed,
}

