use anchor_lang::prelude::*;

/// Global oracle configuration
/// Stores Pyth feed addresses and validation thresholds
#[account]
pub struct OracleConfig {
    /// Admin who can update configuration
    pub admin: Pubkey,
    
    /// Pyth BTC/USD price feed address
    pub btc_price_feed: Pubkey,
    
    /// Pyth SOL/USD price feed address
    pub sol_price_feed: Pubkey,
    
    /// Maximum age of price data in seconds (default: 60)
    pub staleness_threshold: i64,
    
    /// Maximum confidence interval (lower = better quality)
    pub confidence_threshold: u64,
    
    /// Minimum number of publishers required
    pub min_publishers: u8,
    
    /// Emergency pause flag (stops all price fetching)
    pub emergency_pause: bool,
    
    /// When config was last updated
    pub last_updated: i64,
    
    /// PDA bump
    pub bump: u8,
    
    /// Reserved for future use
    pub _reserved: [u8; 64],
}

impl OracleConfig {
    // 8 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 8 + 1 + 64 = 195 bytes
    pub const SIZE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 8 + 1 + 64;
}

/// Price snapshot for a specific round
/// Stores validated price data from Pyth
#[account]
pub struct PriceSnapshot {
    /// Which game this belongs to
    pub game_id: u64,
    
    /// Round number (1-5)
    pub round_number: u8,
    
    /// Asset type (BTC or SOL)
    pub asset_type: AssetType,
    
    /// Snapshot type (START or END)
    pub snapshot_type: SnapshotType,
    
    /// When this snapshot was created
    pub snapshot_time: i64,
    
    // ===== FROM PYTH =====
    
    /// Raw price from Pyth (with exponent)
    pub price: i64,
    
    /// Exponent (e.g., -2 means divide by 100)
    pub exponent: i32,
    
    /// Confidence interval from Pyth
    pub confidence: u64,
    
    /// When Pyth published this price
    pub publish_time: i64,
    
    /// Number of publishers contributing
    pub num_publishers: u32,
    
    /// Pyth price status
    pub pyth_status: u32,
    
    // ===== DERIVED DATA =====
    
    /// Normalized price in micro-dollars (price × 10^6)
    /// Example: $95,000.50 = 95_000_500_000
    pub price_normalized: u64,
    
    /// Staleness in seconds (snapshot_time - publish_time)
    pub staleness: i64,
    
    /// Validation status
    pub status: PriceStatus,
    
    /// PDA bump
    pub bump: u8,
}

impl PriceSnapshot {
    // 8 + 8 + 1 + 1 + 1 + 8 + 8 + 4 + 8 + 8 + 4 + 4 + 8 + 8 + 1 + 1 = 81 bytes
    pub const SIZE: usize = 8 + 8 + 1 + 1 + 1 + 8 + 8 + 4 + 8 + 8 + 4 + 4 + 8 + 8 + 1 + 1;
    
    /// Calculate normalized price from raw price and exponent
    pub fn normalize_price(price: i64, exponent: i32) -> u64 {
        // Convert to positive by applying exponent
        // Then multiply by 1,000,000 to get micro-dollars
        
        let price_f64 = price as f64 * 10_f64.powi(exponent);
        let micro_dollars = (price_f64 * 1_000_000.0) as u64;
        
        micro_dollars
    }
}

/// Asset types supported
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum AssetType {
    BTC,
    SOL,
}

/// Snapshot type (start or end of round)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum SnapshotType {
    START,
    END,
}

/// Price validation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceStatus {
    /// Price is valid and usable
    Valid,
    /// Price is too old
    Stale,
    /// Confidence interval too high
    LowConfidence,
    /// Not enough publishers
    InsufficientPublishers,
    /// Pyth reports price as unavailable
    Unavailable,
    /// Validation failed for other reason
    Failed,
}