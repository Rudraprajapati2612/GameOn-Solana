use anchor_lang::prelude::*;

#[error_code]
pub enum OracleError {
    #[msg("Price data is too old (stale), refresh required")]
    PriceStale,
    
    #[msg("Price confidence interval is too high (unreliable)")]
    LowConfidence,
    
    #[msg("Not enough publishers contributing to price")]
    InsufficientPublishers,
    
    #[msg("Pyth price feed account does not match configuration")]
    WrongPriceFeed,
    
    #[msg("Pyth reports price as unavailable or halted")]
    PriceNotAvailable,
    
    #[msg("Failed to deserialize Pyth price account")]
    PythDeserializationError,
    
    #[msg("Oracle is currently paused by admin")]
    OraclePaused,
    
    #[msg("Unauthorized: Only admin can perform this action")]
    Unauthorized,
    
    #[msg("Invalid asset type provided")]
    InvalidAssetType,
    
    #[msg("Price snapshot already exists for this round")]
    SnapshotAlreadyExists,
    
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    
    #[msg("Invalid staleness threshold (must be positive)")]
    InvalidStalenessThreshold,
    
    #[msg("Invalid confidence threshold")]
    InvalidConfidenceThreshold,
}